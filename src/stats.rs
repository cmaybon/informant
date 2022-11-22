use crate::{workrave, Informant, settings};
use eframe::egui;
use egui::*;
use plot::{Plot, Line, PlotPoints};
use crate::workrave::WorkraveHistory;

pub struct StatsTab {
    pub current_day: Option<workrave::WorkraveDay>,
}

impl StatsTab {
    pub fn ui(&mut self, ui: &mut Ui, workrave_history: &Option<workrave::WorkraveHistory>) -> Response{
        let mut plot = Plot::new("testing");
        plot.show(ui, |plot_ui| {
            match workrave_history {
                Some(history) => {
                    plot_ui.line(StatsTab::get_lines_from_history(&history));
                },
                None => {}
            }
        }).response
    }

    fn get_lines_from_history(history: &workrave::WorkraveHistory) -> Line {
        let mut total_keystrokes: Vec<[f64; 2]> = vec![];

        // Todo sort keys
        // let mut sorted_history = history.keys().clone();
        // sorted_history.sort();

        let mut count = 0;
        for (day, data) in &history.days {
            total_keystrokes.push([count as f64, data.stats.total_keystrokes as f64]);
            count += 1;
        }

        Line::new(PlotPoints::new(total_keystrokes))
    }
}
