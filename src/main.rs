use eframe::epaint::Vec2;
use lab::error::Error;

mod app;
use app::*;

fn main() -> Result<(), Error> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(800., 600.));
    native_options.resizable = true;
    native_options.centered = true;

    run_native(
        "Btree Showcase",
        native_options,
        Box::new(|cc| Box::new(Application::new(cc))),
    )?;

    Ok(())
}
