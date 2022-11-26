#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use informant::Informant;
use egui::Vec2;

fn load_app_icon_data() -> eframe::IconData {
    let icon = image::open("assets/icon.png")
        .expect("Failed to open icon path")
        .into_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    eframe::IconData {
        rgba: icon.into_raw(),
        width: icon_width,
        height: icon_height,
    }
}

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        icon_data: Some(load_app_icon_data()),
        min_window_size: Some(Vec2::new(1200.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "informant",
        options,
        Box::new(|_cc| Box::new(Informant::default())),
    );
}
