use crate::{workrave, Informant, settings};
use eframe::egui;
use egui::*;
use plot::{Plot, Line, PlotPoints};
use crate::workrave::{WorkraveHistory, WorkraveDay};
use chrono::NaiveDate;

pub struct StatsTab {
    pub current_day: Option<workrave::WorkraveDay>,
}

impl StatsTab {
    pub fn ui(&mut self, ui: &mut Ui, workrave_history: &Option<workrave::WorkraveHistory>) -> Response{
        let mut plot = Plot::new("testing");
        plot.show(ui, |plot_ui| {
            match workrave_history {
                Some(history) => {
                    for line in StatsTab::get_lines_from_history(&history) {
                        plot_ui.line(line);
                    }
                },
                None => {}
            }
        }).response
    }

    fn get_lines_from_history(history: &workrave::WorkraveHistory) -> Vec<Line> {
        let mut total_keystrokes: Vec<[f64; 2]> = vec![];
        let mut total_mouse_clicks: Vec<[f64; 2]> = vec![];

        let mut sort_dates: Vec<&NaiveDate> = history.days.keys().clone().collect();
        sort_dates.sort();

        for (i, date) in sort_dates.iter().enumerate() {
            let valid_day = match history.days.get(date) {
                Some(day) => day,
                None => {
                    total_keystrokes.push([i as f64, 0.0]);
                    total_mouse_clicks.push([i as f64, 0.0]);
                    continue;
                }
            };
            total_keystrokes.push([i as f64, valid_day.stats.total_keystrokes as f64]);
            total_mouse_clicks.push([i as f64, valid_day.stats.total_mouse_clicks as f64]);
        }

        vec![Line::new(PlotPoints::new(total_keystrokes)).name("Keystrokes"),
            Line::new(PlotPoints::new(total_mouse_clicks)).name("Mouse Clicks")]
    }
}
