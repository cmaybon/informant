use eframe::egui;
use egui::*;

use crate::workrave;
use crate::stats;
use crate::settings;

const WORKRAVE_HISTORYSTATS_FILENAME: &str = "historystats";

pub struct Informant {
    workrave_history: Option<workrave::WorkraveHistory>,
    current_tab: Tab,
    stats_tab: stats::StatsTab,
    settings_tab: settings::SettingsTab,
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
            });
            match self.current_tab {
                Tab::Stats => {
                    self.stats_tab.ui(ui);
                }
                Tab::Analytics => {}
                Tab::Settings => {
                    self.settings_tab.ui(ui);
                }
            }
        });
    }
}
