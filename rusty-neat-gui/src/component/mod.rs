use std::sync::Arc;

use druid::{
    commands::SHOW_OPEN_PANEL,
    widget::{Button, Flex, Label, List, Scroll},
    FileDialogOptions, Widget, WidgetExt,
};

use crate::{
    commands::SELECT_GENERATION,
    maybe::Maybe,
    model::{generation::Generation, neat::NeatModel, GUIModel},
};

pub mod generation;
pub mod genome;
pub mod species;

pub fn main() -> impl Widget<GUIModel> {
    Maybe::new(neat_component, no_data_component)
        .lens(GUIModel::neat)
        .padding(20.0)
}

fn neat_component() -> impl Widget<NeatModel> {
    Flex::row()
        .with_flex_child(
            Scroll::new(List::new(generation_entry).lens(NeatModel::generations)),
            1.0,
        )
        .with_default_spacer()
        .with_flex_child(
            Maybe::new(
                || generation::generation(),
                || Label::new("No generation selected"),
            )
            .lens(NeatModel::current_generation),
            1.0,
        )
        .with_default_spacer()
        .with_flex_child(
            Maybe::new(|| species::species(), || Label::new("No species selected"))
                .lens(NeatModel::current_species),
            1.0,
        )
        .with_default_spacer()
        .with_flex_child(
            Maybe::new(|| genome::genome(), || Label::new("No genome selected"))
                .lens(NeatModel::current_genome),
            9.0,
        )
}

fn generation_entry() -> impl Widget<Arc<Generation>> {
    Label::dynamic(|gen: &Arc<Generation>, _| format!("Generation {}", gen.generation))
        .on_click(|ctx, gen, _env| ctx.submit_command(SELECT_GENERATION.with(Arc::clone(gen))))
}

fn no_data_component() -> impl Widget<()> {
    Button::new("Open result")
        .on_click(|ctx, _data, _env| {
            ctx.submit_command(SHOW_OPEN_PANEL.with(FileDialogOptions::new().select_directories()));
        })
        .center()
}
