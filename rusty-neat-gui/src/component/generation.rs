
use std::sync::Arc;

use druid::{Widget, WidgetExt, widget::{Flex, Label, Scroll, List}, lens};

use crate::model::{generation::Generation, species::Species};
use crate::commands::SELECT_SPECIES;

pub fn generation() -> impl Widget<Arc<Generation>> {
  Flex::column()
    .with_child(Label::dynamic(|generation: &Arc<Generation>, _| format!("Generation {}", generation.generation)))
    .with_default_spacer()
    .with_flex_child(Scroll::new(List::new(genome_entry).lens(lens::InArc::new(Generation::species))), 1.0)
}

fn genome_entry() -> impl Widget<Arc<Species>> {
  Label::dynamic(|species: &Arc<Species>, _| format!("Species {}", species.id)).on_click(|ctx, species, _env| {
    ctx.submit_command(SELECT_SPECIES.with(Arc::clone(species)))
  })
}