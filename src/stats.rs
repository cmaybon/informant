use crate::workrave;
use eframe::egui;
use egui::*;

pub struct StatsTab {
    pub current_day: Option<workrave::WorkraveDay>,
}

impl StatsTab {
    pub fn ui(&mut self, ui: &mut Ui) {
        if ui.button("Load stats file").clicked() {
            println!("Loading stats file")
        };
    }
}
