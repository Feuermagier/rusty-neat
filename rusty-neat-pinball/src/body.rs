use std::{cell::RefCell, rc::Rc};

use ggez::{
    graphics::{Color, DrawMode, FillOptions, Mesh, Rect},
    Context, GameResult,
};
use nalgebra::{Point2, Vector2};
use rapier2d::{dynamics::{BodyStatus, RigidBody, RigidBodyBuilder, RigidBodyHandle}, geometry::{Collider, ColliderBuilder, ColliderHandle, SharedShape}, na::Isometry2};

use crate::physics::PhysicsEngine;

pub struct Body {
    physics: Rc<RefCell<PhysicsEngine>>,
    body_handle: RigidBodyHandle,
    shapes: Vec<Shape>,
}

impl Body {
    pub fn new(
        physics: Rc<RefCell<PhysicsEngine>>,
        body: RigidBody
    ) -> Self {
        let body_handle = physics.borrow_mut().add_body(body);
        Self {
            physics,
            body_handle,
            shapes: Vec::with_capacity(1),
        }
    }

    pub(crate) fn shapes(&self) -> &[Shape] {
        &self.shapes
    }

    pub fn location(&self) -> mint::Vector2<f32> {
        self.physics
            .borrow()
            .get_body(self.body_handle)
            .position()
            .translation
            .vector
            .into()
    }

    pub fn rotation(&self) -> f32 {
        self.physics
            .borrow()
            .get_body(self.body_handle)
            .position()
            .rotation
            .angle()
    }

    // Nur für Dynamic
    pub fn apply_force(&self, force: mint::Vector2<f32>) {
        self.physics.borrow_mut().get_body_mut(self.body_handle).apply_force(force.into(), true);
    }

    // Nur für Dynamic
    pub fn apply_torque(&self, torque: f32) {
        self.physics.borrow_mut().get_body_mut(self.body_handle).apply_torque(torque, true);
    }

    // Nur für Dynamic
    pub fn set_velocity(&self, velocity: mint::Vector2<f32>) {
        self.physics.borrow_mut().get_body_mut(self.body_handle).set_linvel(velocity.into(), true);
    }

    // Nur für Dynamic
    pub fn set_angular_velocity(&self, velocity: f32) {
        self.physics.borrow_mut().get_body_mut(self.body_handle).set_angvel(velocity, true);
    }

    // Teleportiert und setzt alle Geschwindigkeiten zurück
    pub fn force_position(&self, position: Isometry2<f32>) {
        self.physics.borrow_mut().get_body_mut(self.body_handle).set_position(position, false);
    }

    pub(crate) fn add_shape(
        &mut self,
        shape_type: ShapeType,
        density: f32,
        friction: f32,
        restitution: f32,
    ) {
        let shape = Shape::new(
            Rc::clone(&self.physics),
            shape_type,
            density,
            friction,
            restitution,
            self.body_handle,
        );
        self.shapes.push(shape);
    }
}

pub struct Shape {
    physics: Rc<RefCell<PhysicsEngine>>,
    shape_type: ShapeType,
    handle: ColliderHandle,
}

impl Shape {
    pub fn new(
        physics: Rc<RefCell<PhysicsEngine>>,
        shape_type: ShapeType,
        density: f32,
        friction: f32,
        restituion: f32,
        body_handle: RigidBodyHandle,
    ) -> Self {
        let handle = physics.borrow_mut().add_collider(
            body_handle,
            shape_type.to_collider(density, friction, restituion),
        );

        Self {
            physics,
            shape_type,
            handle,
        }
    }

    pub fn get_global_position(&self) -> Isometry2<f32> {
        self.physics
            .borrow()
            .get_collider(self.handle)
            .position()
            .clone()
    }

    pub(crate) fn to_meshes(&self, ctx: &mut Context, color: Color) -> GameResult<Vec<Mesh>> {
        self.shape_type.to_meshes(ctx, color)
    }
}

#[derive(Debug, Clone)]
pub enum ShapeType {
    Rectangle {
        center_offset_x: f32,
        center_offset_y: f32,
        half_width: f32,
        half_height: f32,
        rotation: f32,
    },
    Circle {
        center_offset_x: f32,
        center_offset_y: f32,
        radius: f32,
    },
    Capsule {
        half_width: f32,
        radius: f32,
        rotation: f32
    },
}

impl ShapeType {
    pub(crate) fn to_meshes(&self, ctx: &mut Context, color: Color) -> GameResult<Vec<Mesh>> {
        match *self {
            Self::Rectangle {
                half_width,
                half_height,
                ..
            } => {
                let rect = Rect::new(
                    -half_width,
                    -half_height,
                    2.0 * half_width,
                    2.0 * half_height,
                );
                Ok(vec![Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), rect, color)?])
            }
            ShapeType::Circle {
                center_offset_x,
                center_offset_y,
                radius,
            } => Ok(vec![Mesh::new_circle(
                ctx,
                DrawMode::Fill(FillOptions::default()),
                Point2::new(center_offset_x, center_offset_y),
                radius,
                0.1,
                color,
            )?]),
            ShapeType::Capsule {
                half_width,
                radius,
                ..
            } => {
                let left_circle = Mesh::new_circle(
                    ctx,
                    DrawMode::Fill(FillOptions::default()),
                    Point2::new(-half_width, 0.0),
                    radius,
                    0.1,
                    color,
                )?;

                let right_circle = Mesh::new_circle(
                    ctx,
                    DrawMode::Fill(FillOptions::default()),
                    Point2::new(half_width, 0.0),
                    radius,
                    0.1,
                    color,
                )?;

                let rect = Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), Rect::new(
                    -half_width,
                    -radius,
                    2.0 * half_width,
                    2.0 * radius
                ), color)?;

                Ok(vec![left_circle, right_circle, rect])
            }
        }
    }

    fn to_collider(&self, density: f32, friction: f32, restitution: f32) -> Collider {
        match *self {
            Self::Rectangle {
                center_offset_x,
                center_offset_y,
                half_width,
                half_height,
                rotation,
            } => ColliderBuilder::new(SharedShape::cuboid(half_width, half_height))
                .translation(center_offset_x, center_offset_y)
                .rotation(rotation)
                .density(density)
                .friction(friction)
                .restitution(restitution)
                .build(),
            ShapeType::Circle {
                center_offset_x,
                center_offset_y,
                radius,
            } => ColliderBuilder::new(SharedShape::ball(radius))
                .translation(center_offset_x, center_offset_y)
                .density(density)
                .friction(friction)
                .restitution(restitution)
                .build(),
            ShapeType::Capsule {
                half_width,
                radius,
                rotation
            } => ColliderBuilder::capsule_x(half_width, radius)
                .rotation(rotation)
                .density(density)
                .friction(friction)
                .restitution(restitution)
                .build(),
        }
    }
}
