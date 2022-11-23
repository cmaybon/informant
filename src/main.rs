#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use informant::Informant;
use egui::Vec2;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let mut options = eframe::NativeOptions::default();
    options.min_window_size = Some(Vec2::new(1600.0, 900.0));
    options.resizable = false;
    eframe::run_native(
        "Informant",
        options,
        Box::new(|_cc| Box::new(Informant::default())),
    );
}
