use eframe::egui;
use egui::*;
use std::fs;

use crate::workrave;
use crate::stats;
use crate::settings;
use std::collections::BTreeMap;

pub struct Informant {
    current_tab: Tab,
    pub stats_tab: stats::StatsTab,
    pub settings_tab: settings::SettingsTab,
}

impl Informant {
    fn top_panel_style() -> BTreeMap<TextStyle, FontId>{
        return [(egui::TextStyle::Button,
                 egui::FontId::new(24.0, egui::FontFamily::Proportional))]
            .into()
    }
}

impl Default for Informant {
    fn default() -> Self {
        Self {
            current_tab: Tab::Stats,
            stats_tab: stats::StatsTab::default(),
            settings_tab: settings::SettingsTab {
                settings: settings::Settings::default()
            },
        }
    }
}

#[derive(PartialEq, Eq)]
enum Tab {
    Stats,
    Settings,
}

impl eframe::App for Informant {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().text_styles = Informant::top_panel_style();

                ui.selectable_value(&mut self.current_tab, Tab::Stats, "Stats");
                ui.selectable_value(&mut self.current_tab, Tab::Settings, "Settings");
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Stats => {
                    self.stats_tab.ui(ui, &self.settings_tab.settings.workrave_historystats_path, &frame);
                }
                Tab::Settings => {
                    self.settings_tab.ui(ui);
                }
            }
        });
    }
}
