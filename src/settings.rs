use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::{ErrorKind, Write, Read, Error};
use eframe::egui;
use egui::*;
use tracing_subscriber::util::SubscriberInitExt;
use std::fs::File;
use whoami;
use crate::workrave;


const SETTINGS_FILENAME: &str = "settings.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub workrave_historystats_path: Option<String>,
}

impl Settings {
    pub fn default() -> Self {
        let mut settings = Self {
            workrave_historystats_path: None,
        };
        settings.init();
        settings
    }

    fn init(&mut self) {
        match fs::File::open(SETTINGS_FILENAME) {
            Ok(_) => self.load_settings(),
            Err(_) =>  {
                println!("No settings file found");
                self.workrave_historystats_path = Settings::try_workrave_appdata_path();
                self.save_settings()
            }
        };
    }

    fn load_settings(&mut self) -> io::Result<bool> {
        match fs::File::open(SETTINGS_FILENAME) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let settings: Settings = serde_json::from_str(&contents)?;
                self.workrave_historystats_path = settings.workrave_historystats_path;
                Ok(true)
            },
            Err(error) => {
                println!("No existing settings file, saving current settings...");
                self.save_settings();
                Ok(false)
            },
        }
    }

    fn try_workrave_appdata_path() -> Option<String> {
        println!("Looking for historystats in default AppData path...");
        let path = format!("C:\\Users\\{}\\AppData\\Roaming\\Workrave\\historystats", whoami::username());
        if workrave::WorkraveHistory::is_file_valid(&path) {
            println!("Found");
            Some(path)
        } else {
            None
        }
    }

    fn save_settings(&self) -> io::Result<bool> {
        let serialised = serde_json::to_string(&self)?;
        match fs::File::open(SETTINGS_FILENAME) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match fs::File::create(SETTINGS_FILENAME) {
                    Ok(new_file) => {
                        println!("Settings file created");
                        new_file
                    },
                    Err(e) => panic!("Unknown failure when creating settings file: {:?}", e)
                },
                other_error=> {
                    panic!("Unknown failure when opening settings file: {:?}", other_error)
                },
            }
        };

        match fs::write(SETTINGS_FILENAME, serialised.as_bytes()) {
            Ok(_) => Ok(true),
            Err(error) => panic!("Failed to write settings to file: {:?}", error)
        }
    }
}

pub struct SettingsTab {
    pub settings: Settings,
}

impl SettingsTab {
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
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title(format!("Select a Workrave \"{}\" file", workrave::WORKRAVE_HISTORYSTATS_FILENAME).as_str())
                        .set_file_name(workrave::WORKRAVE_HISTORYSTATS_FILENAME.as_str())
                        .pick_file() {
                        let path = path.display().to_string();
                        if workrave::WorkraveHistory::is_file_valid(&path) {
                            self.settings.workrave_historystats_path = Some(path);
                        } else {
                            self.settings.workrave_historystats_path = None;
                        }
                    }
                }
            });
            if ui.button("Save settings").clicked() {
                println!("Saving settings...");
                self.settings.save_settings();
            };
            if ui.button("Load settings").clicked() {
                println!("Loading settings...");
                self.settings.load_settings();
            }
        });
    }
}
