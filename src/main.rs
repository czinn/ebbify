mod app;
mod components;
mod data;
mod error;
mod result;
mod ui_state;
mod widgets;

use crate::app::{App, APP_NAME};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .unwrap();
}
