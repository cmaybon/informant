use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::{ErrorKind, Read};
use eframe::egui;
use egui::*;
use whoami;
use crate::workrave;


const SETTINGS_FILENAME: &str = "settings.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub workrave_historystats_path: Option<String>,
    pub workrave_todaystats_path: Option<String>,
}

impl Settings {
    pub fn default() -> Self {
        let mut settings = Self {
            workrave_historystats_path: None,
            workrave_todaystats_path: None,
        };
        settings.init();
        settings
    }

    fn init(&mut self) {
        match fs::File::open(SETTINGS_FILENAME) {
            Ok(_) => {
                self.load_settings().unwrap();
            }
            Err(_) => {
                println!("No settings file found");
                self.workrave_historystats_path = Settings::try_workrave_appdata_path("historystats");
                self.workrave_todaystats_path = Settings::try_workrave_appdata_path("todaystats");
                self.save_settings().unwrap();
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
                self.workrave_todaystats_path = settings.workrave_todaystats_path;
                Ok(true)
            }
            Err(_) => {
                println!("No existing settings file, saving current settings...");
                match self.save_settings() {
                    Ok(_) => println!("Successfully saved settings"),
                    Err(error) => {
                        println!("Failed to save settings: {:?}", error);
                    }
                };
                Ok(false)
            }
        }
    }

    fn try_workrave_appdata_path(filename: &str) -> Option<String> {
        println!("Looking for {} in default AppData path...", filename);
        let path = format!("C:\\Users\\{}\\AppData\\Roaming\\Workrave\\{}", whoami::username(), filename);
        if workrave::WorkraveHistory::is_file_valid(&path) {
            println!("Found {}", filename);
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
                    }
                    Err(e) => panic!("Unknown failure when creating settings file: {:?}", e)
                },
                other_error => {
                    panic!("Unknown failure when opening settings file: {:?}", other_error)
                }
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
    fn filepath_ui(ui: &mut Ui, path: &Option<String>) {
        if let Some(saved_path) = &path {
            ui.code(saved_path);
        } else {
            ui.code("! no path set !");
        }
    }

    fn file_selection_dialog(ui: &mut Ui, dialog_title: &str, file_name: &str) -> Option<String> {
        if ui.button("Set Path").clicked() {
            Some(rfd::FileDialog::new()
                .set_title(dialog_title)
                .set_file_name(file_name)
                .pick_file()?
                .display()
                .to_string())
        } else {
            None
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let mut have_settings_changed = false;
        ui.vertical(|ui| {
            ui.heading(RichText::new("Workrave"));
            ui.separator();
            ui.columns(3, |columns| {
                columns[0].vertical(|ui| {
                    ui.label("historystats filepath");
                    ui.separator();
                    ui.label("todaystats filepath");
                });
                columns[1].vertical(|ui| {
                    SettingsTab::filepath_ui(ui, &self.settings.workrave_historystats_path);
                    ui.separator();
                    SettingsTab::filepath_ui(ui, &self.settings.workrave_todaystats_path);
                });
                columns[2].vertical(|ui| {
                    if let Some(path) = SettingsTab::file_selection_dialog(ui,
                                                       format!("Select a Workrave \"{}\" file", workrave::WORKRAVE_HISTORYSTATS_FILENAME).as_str(),
                                                       workrave::WORKRAVE_HISTORYSTATS_FILENAME) {
                        self.settings.workrave_historystats_path = Some(path);
                        have_settings_changed = true;
                    }
                    ui.separator();
                    if let Some(path) = SettingsTab::file_selection_dialog(ui,
                                                                           format!("Select a Workrave \"{}\" file", workrave::WORKRAVE_TODAYSTATS_FILENAME).as_str(),
                                                                           workrave::WORKRAVE_TODAYSTATS_FILENAME) {
                        self.settings.workrave_todaystats_path = Some(path);
                        have_settings_changed = true;
                    }
                });
            });
        });

        if have_settings_changed {
            self.settings.save_settings().unwrap();
        }
    }
}
