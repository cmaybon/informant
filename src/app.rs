use eframe::egui;
use egui::*;

use crate::workrave;

const WORKRAVE_HISTORYSTATS_FILENAME: &str = "historystats";

pub struct Informant {
    workrave_history: Option<workrave::WorkraveHistory>,
    current_tab: Tab,
    stats_tab: StatsTab,
    settings_tab: Settings,
}

impl Default for Informant {
    fn default() -> Self {
        Self {
            workrave_history: None,
            current_tab: Tab::Stats,
            stats_tab: StatsTab {
                current_day: None,
            },
            settings_tab: Settings {
                workrave_stats_path: Some("./historystats".to_string()),
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


struct StatsTab {
    current_day: Option<workrave::WorkraveDay>,
}


impl StatsTab {
    fn ui(&mut self, ui: &mut Ui) {
        if ui.button("Load stats file").clicked() {
            println!("Loading stats file")
        };
    }
}

struct Settings {
    workrave_stats_path: Option<String>,
}

impl Settings {
    // fn workrave_stats_path_is_valid(&mut self, path: &Path) -> bool {
    //     if let Ok(lines) = Settings::get_file_lines(&path) {
    //         for line in lines {
    //             if line.unwrap() == "WorkRaveStats 4" {
    //                 return true;
    //             }
    //             break;
    //         }
    //     }
    //     false
    // }
    //
    // fn get_file_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    //     let file = File::open(path)?;
    //     Ok(io::BufReader::new(file).lines())
    // }

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
                    // if let Some(path) = rfd::FileDialog::new()
                    //     .set_title(format!("Select a Workrave \"{}\" file", WORKRAVE_HISTORYSTATS_FILENAME).as_str())
                    //     .set_file_name(WORKRAVE_HISTORYSTATS_FILENAME)
                    //     .pick_file() {
                    //     if self.workrave_stats_path_is_valid(&path) {
                    //         self.workrave_stats_path = Some(path.display().to_string());
                    //     }
                    // }
                }
            });
        });
    }
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
                    StatsTab::ui(&mut self.stats_tab, ui);
                }
                Tab::Analytics => {}
                Tab::Settings => {
                    Settings::ui(&mut self.settings_tab, ui);
                }
            }
        });
    }
}
