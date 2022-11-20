use eframe::egui;
use egui::*;

pub struct Informant {
    current_tab: Tab,
    today_stats: DayStats,
}

#[derive(PartialEq, Eq)]
enum Tab {
    Today,
    Analytics,
    Settings,
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
            }
        }
    }
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


impl eframe::App for Informant {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui | {
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
                Tab::Settings => {}
            }
        });
    }
}
