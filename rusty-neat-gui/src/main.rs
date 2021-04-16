mod commands;
mod component;
mod delegate;
mod genome_widget;
mod maybe;
mod model;
mod reader;
use delegate::Delegate;
use druid::{AppLauncher, PlatformError, WindowDesc};
use model::GUIModel;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(component::main)
        .title("rusty-neat-gui")
        .window_size((1200.0, 600.0));

    let model = GUIModel { neat: None };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .delegate(Delegate)
        .launch(model)
}