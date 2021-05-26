use std::rc::Rc;

use ggez::{
    graphics::{self, Color, DrawParam, Mesh},
    Context, GameError, GameResult,
};
use mint::Vector2;

use crate::body::Body;

pub struct GraphicsEntity {
    name: Option<String>,
    body: Rc<Body>,
    meshes: Vec<Vec<Mesh>>,
}

impl GraphicsEntity {
    pub(crate) fn new_with_name(body: Rc<Body>, colors: &[Color], ctx: &mut Context, name: Option<String>) -> GameResult<Self> {
        let mut meshes = Vec::with_capacity(body.shapes().len());
        for (shape, color) in body.shapes().iter().zip(colors.iter()) {
            meshes.push(shape.to_meshes(ctx, *color)?);
        }
        Ok(Self { name, body, meshes })
    }

    pub(crate) fn new(body: Rc<Body>, colors: &[Color], ctx: &mut Context) -> GameResult<Self> {
        Self::new_with_name(body, colors, ctx, None)
    }

    pub(crate) fn draw(&self, ctx: &mut Context) -> Result<(), GameError> {
        for (meshes, shape) in self.meshes.iter().zip(self.body.shapes().iter()) {
            let position = shape.get_global_position();
            let params = DrawParam::new().dest::<Vector2<f32>>(position.translation.vector.into()).rotation(position.rotation.angle());
            for mesh in meshes {
                graphics::draw(ctx, mesh, params)?;
            }
        }
        Ok(())
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    pub fn body(&self) -> &Rc<Body> {
        &self.body
    }
}

pub fn find_entity<'a>(entities: &'a [GraphicsEntity], name: &str) -> Option<&'a GraphicsEntity> {
    entities.iter().find(|entity| {
        if let Some(entity_name) = entity.name() {
            entity_name == name
        } else {
            false
        }
    })
}