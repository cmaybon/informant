use eframe::egui;
use egui::*;
use std::fs;

use crate::workrave;
use crate::stats;
use crate::settings;

const WORKRAVE_HISTORYSTATS_FILENAME: &str = "historystats";

pub struct Informant {
    pub workrave_history: Option<workrave::WorkraveHistory>,
    current_tab: Tab,
    pub stats_tab: stats::StatsTab,
    pub settings_tab: settings::SettingsTab,
}

impl Informant {
    fn load_workrave_stats(&mut self) {
        match &self.settings_tab.settings.workrave_historystats_path {
            Some(path) => {
                let valid_file = match fs::File::open(path) {
                    Ok(file) => file,
                    Err(_) => panic!("Failed to open historystats")
                };

                let history_data = workrave::WorkraveHistory::load_historystats_file(&valid_file);
                self.workrave_history = Some(history_data);
                println!("{:#?}", self.workrave_history);
            }
            None => {
                println!("No path to historystats");
            }
        }
    }
}

impl Default for Informant {
    fn default() -> Self {
        Self {
            workrave_history: None,
            current_tab: Tab::Stats,
            stats_tab: stats::StatsTab {
                current_day: None,
            },
            settings_tab: settings::SettingsTab {
                settings: settings::Settings::default()
            },
        }
    }
}

#[derive(PartialEq, Eq)]
enum Tab {
    Stats,
    Analytics,
    Settings,
}

impl eframe::App for Informant {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").min_height(75.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Stats, "Stats");
                ui.selectable_value(&mut self.current_tab, Tab::Analytics, "Analytics");
                ui.selectable_value(&mut self.current_tab, Tab::Settings, "Settings");
                ui.separator();
                if ui.button("Load Stats").clicked() {
                    println!("Loading stats file");
                    self.load_workrave_stats();
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Stats => {
                    self.stats_tab.ui(ui, &self.workrave_history);
                }
                Tab::Analytics => {}
                Tab::Settings => {
                    self.settings_tab.ui(ui);
                }
            }
        });
    }
}
