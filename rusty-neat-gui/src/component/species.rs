use std::sync::Arc;

use druid::{
    lens,
    widget::{Flex, Label, List, Scroll},
    Widget, WidgetExt,
};

use crate::commands::SELECT_GENOME;
use crate::model::{organism::Organism, species::Species};

pub fn species() -> impl Widget<Arc<Species>> {
    Flex::column()
        .with_child(Label::dynamic(|species: &Arc<Species>, _| {
            format!("Species {}", species.id)
        }))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(List::new(genome_entry).lens(lens::InArc::new(Species::organisms))),
            1.0,
        )
}

fn genome_entry() -> impl Widget<Arc<Organism>> {
    Label::dynamic(|organism: &Arc<Organism>, _| {
        format!("Fitness {}", organism.fitness.unwrap_or(-1.0))
    })
    .on_click(|ctx, organism, _env| {
        ctx.submit_command(SELECT_GENOME.with(Arc::clone(&organism.genome)))
    })
}
