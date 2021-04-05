use druid::{AppLauncher, PlatformError, WindowDesc};

mod view;

fn main() -> Result<(), PlatformError> {
  AppLauncher::with_window(WindowDesc::new(view::build_ui).title("Rusty-Neat graph viewer")).launch(())?;
  Ok(())
}