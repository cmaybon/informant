use eframe::egui;

pub struct Informant {
    current_tab: Tab,
}

#[derive(PartialEq, Eq)]
enum Tab {
    Today,
    Analytics,
    Settings,
}

impl Default for Informant {
    fn default() -> Self {
        Self { current_tab: Tab::Today}
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
        });
    }
}
