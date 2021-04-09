use std::rc::Rc;

use druid::{Affine, Color, Data, Event, FontFamily, Lens, Point, Rect, RenderContext, Selector, Size, Widget, kurbo::{Circle, Line}, piet::{Text, TextLayout, TextLayoutBuilder}};
use im::Vector;

use crate::commands;

const BACKGROUND_COLOR: &str = "fff8dc";

const NODE_RADIUS: f64 = 25.0;
const INPUT_NODE_COLOR: &str = "000000";
const HIDDEN_NODE_COLOR: &str = "000000";
const OUTPUT_NODE_COLOR: &str = "000000";

const POSITIVE_CONNECTION: &str = "b31537";
const NEGATIVE_CONNECTION: &str = "1628b5";
const DISABLED_CONNECTION: &str = "8c8486";

const CONNECTION_SCALE: f64 = 2.0;
const MIN_CONNECTION_THICKNESS: f64 = 1.0;
const MAX_CONNECTION_THICKNESS: f64 = 20.0;

const TEXT_COLOR: &str = "ffffff";
const FONT_SIZE: f64 = 12.0;

#[derive(Clone, Lens, Data)]
pub struct Genome {
    id: usize,
    nodes: Vector<Rc<Node>>,
    connections: Vector<Rc<Connection>>,
}

impl Genome {
    pub fn new(id: usize, nodes: Vector<Rc<Node>>, connections: Vector<Rc<Connection>>) -> Self {
        Self {
            id,
            nodes, 
            connections
        }
    }
}

impl PartialEq for Genome {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Data)]
pub struct Node {
    pub id: u32,
    pub position: Point,
    pub activation: String,
    pub bias: f64,
    pub node_type: NodeType
}

impl Node {
    fn actual_position(&self, bounding_rect: Size) -> Point {
        Point::new(self.position.x * bounding_rect.width, self.position.y * bounding_rect.height)
    }
}

#[derive(Clone, Data)]
pub struct Connection {
    pub start: Rc<Node>,
    pub end: Rc<Node>,
    pub innovation: u32,
    pub enabled: bool,
    pub weight: f64,
}

#[derive(Clone, Data)]
pub enum NodeType {
    INPUT(u32),
    OUTPUT(u32),
    HIDDEN
}

pub struct GenomeWidget {
    current_transformation: Affine,
    last_drag_position: Option<Point>
}

impl GenomeWidget {
    pub fn new() -> Self {
        Self {
            current_transformation: Affine::scale(1.0),
            last_drag_position: Option::None
        }
    }
}

impl Widget<Option<Genome>> for GenomeWidget {

    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        _data: &mut Option<Genome>,
        _env: &druid::Env,
    ) {
        if let Event::Wheel(event) = event {
            if event.wheel_delta.y != 0.0 {
                let zoom_factor = {
                    if event.wheel_delta.y < 0.0 {
                        1.1
                    } else {
                        0.9
                    }
                };
                self.current_transformation = Affine::translate(event.pos.to_vec2()) * Affine::scale(zoom_factor) * Affine::translate(event.pos.to_vec2() * -1.0) * self.current_transformation;
                ctx.request_paint();
            }
        }

        if let Event::MouseDown(vent) = event {
            ctx.set_active(true);
            self.last_drag_position = Option::Some(vent.pos);
        }

        if let Event::MouseUp(_) = event {
            ctx.set_active(false);
            self.last_drag_position = Option::None;
        }

        if let Event::MouseMove(event) = event {
            if ctx.is_active() && self.last_drag_position.is_some() {
                self.current_transformation = Affine::translate(event.pos - self.last_drag_position.unwrap()) * self.current_transformation;
                self.last_drag_position = Option::Some(event.pos);
                ctx.request_paint();
            }
        }

        if let Event::Command(command) = event {
            if command.is::<()>(Selector::new(commands::RECENTER_GENOME_SLECTOR)) {
                self.current_transformation = Affine::scale(1.0);
            }
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &Option<Genome>,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &Option<Genome>,
        data: &Option<Genome>,
        _env: &druid::Env,
    ) {
        if old_data.ne(data) {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &Option<Genome>,
        _env: &druid::Env,
    ) -> druid::Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Option<Genome>, _env: &druid::Env) {
        let background = Rect::from_origin_size(Point::ORIGIN, ctx.size());
        ctx.fill(background, &Color::from_hex_str(BACKGROUND_COLOR).unwrap());

        if data.is_none() {
            return;
        }

        let data = data.as_ref().unwrap();

        ctx.save().unwrap();
        ctx.transform(self.current_transformation);

        // Draw connection first so they are placed below notes
        for connection in &data.connections {
            let line = Line::new(connection.start.actual_position(ctx.size()), connection.end.actual_position(ctx.size()));
            let color = Color::from_hex_str({
                if !connection.enabled || connection.weight == 0.0 {
                    DISABLED_CONNECTION
                } else if connection.weight > 0.0 {
                    POSITIVE_CONNECTION
                } else {
                    NEGATIVE_CONNECTION
                }
            }).unwrap();
            ctx.stroke(line, &color, (connection.weight.abs() * CONNECTION_SCALE).clamp(MIN_CONNECTION_THICKNESS, MAX_CONNECTION_THICKNESS));
        }

        for node in &data.nodes {
            let pixel_position = node.actual_position(ctx.size());

            let circle = Circle::new(pixel_position, NODE_RADIUS);
            let color = match node.node_type {
                NodeType::INPUT(_) => Color::from_hex_str(INPUT_NODE_COLOR).unwrap(),
                NodeType::OUTPUT(_) => Color::from_hex_str(OUTPUT_NODE_COLOR).unwrap(),
                NodeType::HIDDEN => Color::from_hex_str(HIDDEN_NODE_COLOR).unwrap()
            };
            ctx.fill(circle, &color);

            let text = match node.node_type {
                NodeType::INPUT(id) => node.id.to_string() + " (In " + id.to_string().as_str() + ")",
                NodeType::OUTPUT(id) => node.id.to_string() + " (Out " + id.to_string().as_str() + ")",
                NodeType::HIDDEN => node.id.to_string()
            };
            let text_layout = ctx.text().new_text_layout(text)
                .font(FontFamily::SANS_SERIF, FONT_SIZE)
                .text_color(Color::from_hex_str(TEXT_COLOR).unwrap())
                .build().unwrap();
            ctx.draw_text(&text_layout, pixel_position - (text_layout.size().width / 2.0, text_layout.size().height / 2.0));
        }

        ctx.restore().unwrap();
    }
}
