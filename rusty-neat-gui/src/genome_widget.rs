use std::rc::Rc;

use druid::{Widget, Lens, Data};
use im::Vector;

#[derive(Clone, Lens, Data)]
pub struct Genome {
    nodes: Vector<Rc<Node>>,
    connections: Vector<Rc<Connection>>,
}

#[derive(Clone, Lens, Data)]
pub struct Node {
    id: usize,
    x: f64,
    y: f64,
    activation: String,
    bias: f64,
}

#[derive(Clone, Lens, Data)]
pub struct Connection {
    start: Rc<Node>,
    end: Rc<Node>,
    enabled: bool,
    weight: f64,
}

struct GenomeWidget;

impl <'a> Widget<Genome<'a>> for GenomeWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Genome,
        env: &druid::Env,
    ) {
        todo!()
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Genome,
        env: &druid::Env,
    ) {
        todo!()
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &Genome,
        data: &Genome,
        env: &druid::Env,
    ) {
        todo!()
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Genome,
        env: &druid::Env,
    ) -> druid::Size {
        todo!()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Genome, env: &druid::Env) {
        todo!()
    }
}
