use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::{ErrorKind, Write};
use eframe::egui;
use egui::*;
use tracing_subscriber::util::SubscriberInitExt;

const SETTINGS_FILENAME: &str = "settings.json";


#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub workrave_historystats_path: Option<String>,
}

impl Settings {
    fn settings_init(&self) {
        // see if settings file exists
        // exists
        //      read and set fields
        // else
        self.save_settings();
    }

    fn load_settings(&mut self) {
        match fs::File::open(SETTINGS_FILENAME) {
            Ok(file) => {
                // read
                // set fields
            },
            Err(error ) => {
                println!("No existing settings file, saving current settings...");
                self.save_settings();
                return
            },
        };
    }

    fn save_settings(&self) -> io::Result<bool> {
        let serialised = serde_json::to_string(&self)?;
        let mut valid_file = match fs::File::open(SETTINGS_FILENAME) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match fs::File::create(SETTINGS_FILENAME) {
                    Ok(new_file) => new_file,
                    Err(e) => panic!("Unknown failure when creating settings file: {:?}", e)
                },
                other_error=> {
                    panic!("Unknown failure when opening settings file: {:?}", other_error)
                },
            }
        };

        match valid_file.write(serialised.as_bytes()) {
            Ok(_) => Ok(true),
            Err(error) => panic!("Failed to write settings: {:?}", error),
        }
    }
}

pub struct SettingsTab {
    pub settings: Settings,
}

impl SettingsTab {
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

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading(RichText::new("Workrave"));
            ui.horizontal(|ui| {
                ui.label("\"historystats\" filepath: ");

                if let Some(saved_path) = &self.settings.workrave_historystats_path {
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
            if ui.button("Save settings").clicked() {
                println!("Saving settings...");
                self.settings.save_settings();
            };
        });
    }
}
