use eframe::egui;
use egui::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use chrono::prelude::*;

const WORKRAVE_HISTORYSTATS_FILENAME: &str = "historystats";

pub struct Informant {
    current_tab: Tab,
    today_stats: DayStats,
    settings: Settings,
}

impl Default for Informant {
    fn default() -> Self {
        Self {
            current_tab: Tab::Today,
            today_stats: DayStats {
                keystrokes: 100,
                mouse_clicks: 100,
                mouse_usage_seconds: 100,
                mouse_distance: 10.0,
                mouse_effective_distance: 10.0,
            },
            settings: Settings {
                workrave_stats_path: Some("./historystats".to_string()),
            },
        }
    }
}

#[derive(PartialEq, Eq)]
enum Tab {
    Today,
    Analytics,
    Settings,
}

struct DayStats {
    keystrokes: u64,
    mouse_clicks: u64,
    mouse_usage_seconds: u64,
    mouse_distance: f64,
    mouse_effective_distance: f64,
}

impl Default for DayStats {
    fn default() -> Self {
        Self {
            keystrokes: 0,
            mouse_clicks: 0,
            mouse_usage_seconds: 0,
            mouse_distance: 0.0,
            mouse_effective_distance: 0.0,
        }
    }
}

impl DayStats {
    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Keystrokes: ");
                ui.label(format!("{}", self.keystrokes));
            });
            ui.separator();
            ui.vertical(|ui| {
                ui.label("Usage Time: ");
                ui.label("5hr 35min 47s");
            });
        });
    }
}

struct Settings {
    workrave_stats_path: Option<String>,
}

impl Settings {
    fn workrave_stats_path_is_valid(&mut self, path: &Path) -> bool {
        if let Ok(lines) = Settings::get_file_lines(&path) {
            for line in lines {
                if line.unwrap() == "WorkRaveStats 4" {
                    return true;
                }
                break;
            }
        }
        false
    }

    fn get_file_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
        let file = File::open(path)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading(RichText::new("Workrave"));
            ui.horizontal(|ui| {
                ui.label("\"historystats\" filepath: ");

                if let Some(saved_path) = &self.workrave_stats_path {
                    ui.monospace(saved_path);
                } else {
                    ui.monospace("! no path set !");
                }

                if ui.button("Set").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title(format!("Select a Workrave \"{}\" file", WORKRAVE_HISTORYSTATS_FILENAME).as_str())
                        .set_file_name(WORKRAVE_HISTORYSTATS_FILENAME)
                        .pick_file() {
                        if self.workrave_stats_path_is_valid(&path) {
                            self.workrave_stats_path = Some(path.display().to_string());
                        }
                    }
                }
            });
        });
    }
}

impl eframe::App for Informant {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").min_height(75.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Today, "Today");
                ui.selectable_value(&mut self.current_tab, Tab::Analytics, "Analytics");
                ui.selectable_value(&mut self.current_tab, Tab::Settings, "Settings");
            });
            match self.current_tab {
                Tab::Today => {
                    DayStats::ui(&mut self.today_stats, ui);
                }
                Tab::Analytics => {}
                Tab::Settings => {
                    Settings::ui(&mut self.settings, ui);
                }
            }
        });
    }
}
