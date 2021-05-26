//! The simplest possible example t i center_offset_x: (), center_offset_y: (), half_width: (), half_height: (), rotation: () d: (), shape: () center_offset_x: (), center_offset_y: (), half_width: (), half_height: (), rotation: () , color: ()  id: (), shape: (), color: ()  id: (), shape: (), color: ()  id: (), shape: (), color: () hat does something.

pub mod entity;

use std::cell::RefCell;
use std::rc::Rc;

use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::Font;
use ggez::graphics::Rect;
use ggez::graphics::Scale;
use ggez::graphics::Text;
use ggez::timer;
use ggez::{Context, GameResult};
use nalgebra::Vector2;

use crate::body::Body;
use crate::physics::PhysicsEngine;

use self::entity::GraphicsEntity;

const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 700.0;
const SCALE: f32 = 1.0 / 4.0;

struct Graphics {
    physics: Rc<RefCell<PhysicsEngine>>,
    entities: Vec<GraphicsEntity>,
    keydown_handler: Box<dyn Fn(event::KeyCode, &[GraphicsEntity]) -> bool>,
    keyup_handler: Box<dyn Fn(event::KeyCode, &[GraphicsEntity]) -> bool>,
}

impl Graphics {
    pub(self) fn new(
        physics: Rc<RefCell<PhysicsEngine>>,
        entities: Vec<GraphicsEntity>,
        keydown_handler: Box<dyn Fn(event::KeyCode, &[GraphicsEntity]) -> bool>,
        keyup_handler: Box<dyn Fn(event::KeyCode, &[GraphicsEntity]) -> bool>,
    ) -> Self {
        Self {
            physics,
            entities,
            keydown_handler,
            keyup_handler,
        }
    }
}

impl event::EventHandler for Graphics {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.physics.borrow_mut().step();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());

        graphics::set_screen_coordinates(
            ctx,
            Rect::new(
                0.0,
                0.0,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            ),
        )?;
        graphics::apply_transformations(ctx)?;

        let mut fps = Text::new(format!("FPS: {:.2}", timer::fps(ctx)));
        fps.set_font(Font::default(), Scale::uniform(20.0));
        graphics::draw(
            ctx,
            &fps,
            DrawParam::default().dest::<mint::Vector2<f32>>(Vector2::new(10.0, 10.0).into()).color(graphics::BLACK),
        )?;

        graphics::set_screen_coordinates(
            ctx,
            Rect::new(
                0.0,
                SCALE * WINDOW_HEIGHT,
                SCALE * WINDOW_WIDTH,
                SCALE * -WINDOW_HEIGHT,
            ),
        )?;
        graphics::apply_transformations(ctx)?;

        for entity in &self.entities {
            entity.draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if (self.keydown_handler)(keycode, &self.entities) {
            event::quit(ctx);
        }
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
    ) {
        if (self.keyup_handler)(keycode, &self.entities) {
            event::quit(ctx);
        }
    }
}

pub fn run(
    physics: Rc<RefCell<PhysicsEngine>>,
    entities: &[(Rc<Body>, Option<String>, Vec<Color>)],
    keydown_handler: Box<dyn Fn(event::KeyCode, &[GraphicsEntity]) -> bool>,
    keyup_handler: Box<dyn Fn(event::KeyCode, &[GraphicsEntity]) -> bool>,
) -> GameResult {
    let cb = ggez::ContextBuilder::new("rusty-neat-pinball", "")
        .window_setup(WindowSetup::default().title("rusty-neat-pinball"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    let (mut ctx, mut event_loop) = cb.build()?;

    let mut graphics_entities = Vec::with_capacity(entities.len());
    for (body, name, colors) in entities {
        graphics_entities.push(GraphicsEntity::new_with_name(
            Rc::clone(body),
            colors,
            &mut ctx,
            name.clone(),
        )?)
    }

    let mut graphics = Graphics::new(physics, graphics_entities, keydown_handler, keyup_handler);
    event::run(&mut ctx, &mut event_loop, &mut graphics)
}
