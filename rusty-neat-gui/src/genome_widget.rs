use druid::Widget;
use im::Vector;

pub struct Genome<'a> {
    nodes: Vector<Node>,
    connections: Vector<Connection<'a>>,
}

pub struct Node {
    id: usize,
    x: f64,
    y: f64,
    activation: String,
    bias: f64,
}

pub struct Connection<'genome> {
    start: &'genome Node,
    end: &'genome Node,
    enabled: bool,
    weight: f64,
}

struct GenomeWidget;

impl Widget<Genome> for GenomeWidget {
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
