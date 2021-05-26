mod body;
mod display;
mod physics;
use std::cell::RefCell;
use std::rc::Rc;

use body::Body;
use body::ShapeType;
use ggez::event::KeyCode;
use ggez::graphics;
use nalgebra::Isometry2;
use nalgebra::Vector2;
use physics::PhysicsEngine;
use rapier2d::dynamics::RigidBodyBuilder;

use crate::display::entity;

const CENTER_OFFSET_X: f32 = 100.0;
const CENTER_OFFSET_Y: f32 = 100.0;

fn main() {
    let physics = Rc::new(RefCell::new(PhysicsEngine::new(-3.0)));

    let table = create_table(Rc::clone(&physics));
    let ball = create_ball(Rc::clone(&physics));
    let (left_flipper, right_flipper) = create_flippers(Rc::clone(&physics));
    left_flipper.set_angular_velocity(1.0);
    display::run(
        physics,
        &vec![
            (
                Rc::new(table),
                None,
                vec![
                    graphics::BLACK,
                    graphics::BLACK,
                    graphics::BLACK,
                    graphics::BLACK,
                ],
            ),
            (Rc::new(left_flipper), None, vec![[0.0, 0.0, 1.0, 1.0].into()]),
            (Rc::new(right_flipper), None, vec![[0.0, 0.0, 1.0, 1.0].into()]),
            (Rc::new(ball), Some("ball".to_string()), vec![graphics::BLACK]),
        ],
        Box::new(|code, entities| {
            match code {
                KeyCode::W => {
                    entity::find_entity(entities, "ball").unwrap().body().apply_force(Vector2::new(0.0, 100.0).into())
                },
                KeyCode::A => {
                    entity::find_entity(entities, "ball").unwrap().body().apply_force(Vector2::new(-100.0, 0.0).into())
                },
                KeyCode::S => {
                    entity::find_entity(entities, "ball").unwrap().body().apply_force(Vector2::new(0.0, -100.0).into())
                },
                KeyCode::D => {
                    entity::find_entity(entities, "ball").unwrap().body().apply_force(Vector2::new(100.0, 0.0).into())
                }
                _ => {}
            }
            false
        }),
        Box::new(|_, _| false),
    )
    .expect("Game error");
}

fn create_table(physics: Rc<RefCell<PhysicsEngine>>) -> Body {
    let mut wall = Body::new(
        Rc::clone(&physics),
        RigidBodyBuilder::new_static()
            .position(Isometry2::new(
                Vector2::new(CENTER_OFFSET_X, CENTER_OFFSET_Y),
                0.0,
            ))
            .build(),
    );

    // Top
    wall.add_shape(
        ShapeType::Rectangle {
            center_offset_x: 0.0,
            center_offset_y: 55.0,
            half_width: 35.0,
            half_height: 5.0,
            rotation: 0.0,
        },
        1.0,
        0.0,
        0.0,
    );

    // Bottom
    wall.add_shape(
        ShapeType::Rectangle {
            center_offset_x: 0.0,
            center_offset_y: -55.0,
            half_width: 35.0,
            half_height: 5.0,
            rotation: 0.0,
        },
        1.0,
        0.0,
        0.0,
    );

    // Left
    wall.add_shape(
        ShapeType::Rectangle {
            center_offset_x: -30.0,
            center_offset_y: 0.0,
            half_width: 5.0,
            half_height: 50.0,
            rotation: 0.0,
        },
        1.0,
        0.0,
        0.0,
    );

    // Right
    wall.add_shape(
        ShapeType::Rectangle {
            center_offset_x: 30.0,
            center_offset_y: 0.0,
            half_width: 5.0,
            half_height: 50.0,
            rotation: 0.0,
        },
        1.0,
        0.0,
        0.0,
    );

    wall
}

fn create_ball(physics: Rc<RefCell<PhysicsEngine>>) -> Body {
    let mut ball = Body::new(
        Rc::clone(&physics),
        RigidBodyBuilder::new_dynamic()
            .position(Isometry2::new(
                Vector2::new(CENTER_OFFSET_X, CENTER_OFFSET_Y),
                0.0,
            ))
            .linvel(1.0, 2.0)
            .build(),
    );

    ball.add_shape(
        ShapeType::Circle {
            center_offset_x: 0.0,
            center_offset_y: 0.0,
            radius: 1.0,
        },
        1.0,
        0.0,
        1.0,
    );

    ball
}

fn create_flippers(physics: Rc<RefCell<PhysicsEngine>>) -> (Body, Body) {
    let mut left_flipper = Body::new(
        Rc::clone(&physics),
        RigidBodyBuilder::new_dynamic()
            .position(Isometry2::new(
                Vector2::new(CENTER_OFFSET_X - 7.5, CENTER_OFFSET_Y - 45.0),
                core::f32::consts::PI * -0.2,
            ))
            .build(),
    );
    left_flipper.add_shape(
        ShapeType::Capsule {
            half_width: 5.0,
            radius: 2.0,
            rotation: 0.0,
        },
        1.0,
        0.0,
        0.0,
    );

    let mut right_flipper = Body::new(
        Rc::clone(&physics),
        RigidBodyBuilder::new_dynamic()
            .position(Isometry2::new(
                Vector2::new(CENTER_OFFSET_X + 7.5, CENTER_OFFSET_Y - 45.0),
                core::f32::consts::PI * -0.2,
            ))
            .build(),
    );
    right_flipper.add_shape(
        ShapeType::Capsule {
            half_width: 5.0,
            radius: 2.0,
            rotation: 0.0,
        },
        1.0,
        0.0,
        0.0,
    );

    (left_flipper, right_flipper)
}
