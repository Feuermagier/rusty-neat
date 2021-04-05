use druid::{AppLauncher, PlatformError, WindowDesc};
use crate::view::genome_widget::Genome;

mod view;

fn main() -> Result<(), PlatformError> {
  let genome = Genome::new();
  AppLauncher::with_window(WindowDesc::new(view::build_ui).title("Rusty-Neat GUI")).launch(genome)?;
  Ok(())
}