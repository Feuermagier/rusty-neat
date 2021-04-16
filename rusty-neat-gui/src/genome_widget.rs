use std::sync::Arc;

use druid::{Affine, Color, Event, FontFamily, Point, Rect, RenderContext, Selector, Widget, kurbo::{Circle, Line}, piet::{Text, TextLayout, TextLayoutBuilder}};
use rusty_neat_interchange::gene_pool::PrintableNodeType;

use crate::{commands, model::genome::Genome};

const BACKGROUND_COLOR: &str = "fff8dc";

const NODE_RADIUS: f64 = 25.0;
const INPUT_NODE_COLOR: &str = "000000";
const HIDDEN_NODE_COLOR: &str = "000000";
const OUTPUT_NODE_COLOR: &str = "000000";

const OFFSET: f64 = 30.0;

const POSITIVE_CONNECTION: &str = "b31537";
const NEGATIVE_CONNECTION: &str = "1628b5";
const DISABLED_CONNECTION: &str = "8c8486";

const CONNECTION_SCALE: f64 = 2.0;
const MIN_CONNECTION_THICKNESS: f64 = 1.0;
const MAX_CONNECTION_THICKNESS: f64 = 20.0;

const TEXT_COLOR: &str = "ffffff";
const FONT_SIZE: f64 = 12.0;

pub struct GenomeWidget {
    current_transformation: Affine,
    last_drag_position: Option<Point>
}

impl GenomeWidget {
    pub fn new() -> Self {
        Self {
            current_transformation: initial_transform(),
            last_drag_position: Option::None
        }
    }
}

impl Widget<Arc<Genome>> for GenomeWidget {

    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        _data: &mut Arc<Genome>,
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
            if command.is::<()>(Selector::new(commands::RECENTER_GENOME)) {
                self.current_transformation = initial_transform();
            }
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &Arc<Genome>,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &Arc<Genome>,
        data: &Arc<Genome>,
        _env: &druid::Env,
    ) {
        if old_data.ne(data) {
            self.current_transformation = initial_transform();
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &Arc<Genome>,
        _env: &druid::Env,
    ) -> druid::Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Arc<Genome>, _env: &druid::Env) {
        let background = Rect::from_origin_size(Point::ORIGIN, ctx.size());
        ctx.fill(background, &Color::from_hex_str(BACKGROUND_COLOR).unwrap());

        ctx.save().unwrap();
        ctx.transform(self.current_transformation);

        // Draw connection first so they are placed below notes
        for connection in &data.connections {
            let line = Line::new(connection.start.actual_position(ctx.size(), OFFSET), connection.end.actual_position(ctx.size(), OFFSET));
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
            let pixel_position = node.actual_position(ctx.size(), OFFSET);

            let circle = Circle::new(pixel_position, NODE_RADIUS);
            let color = match node.node_type.as_ref() {
                PrintableNodeType::Input(_) => Color::from_hex_str(INPUT_NODE_COLOR).unwrap(),
                PrintableNodeType::Output(_) => Color::from_hex_str(OUTPUT_NODE_COLOR).unwrap(),
                PrintableNodeType::Hidden => Color::from_hex_str(HIDDEN_NODE_COLOR).unwrap()
            };
            ctx.fill(circle, &color);

            let text = match node.node_type.as_ref() {
                PrintableNodeType::Input(id) => node.id.to_string() + " (In " + id.to_string().as_str() + ")",
                PrintableNodeType::Output(id) => node.id.to_string() + " (Out " + id.to_string().as_str() + ")",
                PrintableNodeType::Hidden => node.id.to_string()
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

fn initial_transform() -> Affine {
    Affine::translate((0.0, 0.0))
}