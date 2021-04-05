use std::rc::Rc;

use druid::{Color, Data, Point, Rect, RenderContext, Size, Widget};
use im::Vector;

#[derive(Clone, Data)]
pub struct Genome {
  nodes: Vector<Rc<Node>>,
  connections: Vector<Rc<Connection>>
}

impl Genome {
  pub fn new() -> Self {
    Genome {
      nodes: Vector::new(),
      connections: Vector::new()
    }
  }
}

#[derive(Clone, Data)]
pub struct Node {
  bias: f64,
  activation: String,
  x: f64,
  y: f64
}

#[derive(Clone, Data)]
pub struct Connection {
  from: Rc<Node>,
  to: Rc<Node>,
  weight: f64,
  enabled: bool
}

pub struct GenomeWidget;

impl Widget<Genome> for GenomeWidget {
  fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut Genome, env: &druid::Env) {
    
  }

  fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &Genome, env: &druid::Env) {
    
  }

  fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &Genome, data: &Genome, env: &druid::Env) {
    
  }

  fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &Genome, env: &druid::Env) -> druid::Size {
    if bc.is_width_bounded() | bc.is_height_bounded() {
      let size = Size::new(100.0, 100.0);
      bc.constrain(size)
    } else {
        bc.max()
    }
  }

  fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Genome, env: &druid::Env) {
    let rect = Rect::from_origin_size(Point::ORIGIN, ctx.size());
    ctx.fill(rect, &Color::rgb8(100, 200, 0));
  }
}