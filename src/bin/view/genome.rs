use std::rc::Rc;

use druid::{Widget, widget::Label};

pub(crate) fn build_widget() -> impl Widget<()> {
  
}

struct Genome {
  
}

struct Node {
  bias: f64,
  activation: String
}

struct Connection {
  from: Rc<Node>,
  to: Rc<Node>,
  weight: f64,
  enabled: bool
}

struct GenomeWidget;

impl Widget for GenomeWidget {
  fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut T, env: &druid::Env) {
    todo!()
  }

  fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &T, env: &druid::Env) {
    todo!()
  }

  fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {
    todo!()
  }

  fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &T, env: &druid::Env) -> druid::Size {
    todo!()
  }

  fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
    todo!()
  }
}