use crate::{workrave, Informant, settings};
use eframe::egui;
use egui::*;
use plot::{Plot, Line, PlotPoints, PlotPoint, Legend};
use crate::workrave::{WorkraveHistory, WorkraveDay};
use chrono::{NaiveDate, Datelike};

pub struct StatsTab {
    pub current_day: Option<workrave::WorkraveDay>,
}

impl StatsTab {
    pub fn ui(&mut self, ui: &mut Ui, workrave_history: &Option<workrave::WorkraveHistory>) -> Response{
        let mut plot = Plot::new("testing")
            .include_y(0.0)
            .allow_boxed_zoom(false)
            .allow_drag(true)
            .label_formatter(StatsTab::label_formatter)
            .legend(Legend::default());
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

    fn label_formatter(plot_points_name: &str, plot_point: &PlotPoint) -> String {
        let date = match NaiveDate::from_num_days_from_ce_opt(plot_point.x as i32) {
            Some(value) => value,
            None => {
                return format!("DATE ERR")
            }
        };

        format!("{}\nDate:  {}-{}-{}\nValue: {}",
                plot_points_name,
                date.day(),
                date.month(),
                date.year(),
                plot_point.y)
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
                    total_keystrokes.push([date.num_days_from_ce() as f64, 0.0]);
                    total_mouse_clicks.push([date.num_days_from_ce() as f64, 0.0]);
                    continue;
                }
            };
            total_keystrokes.push([date.num_days_from_ce() as f64, valid_day.stats.total_keystrokes as f64]);
            total_mouse_clicks.push([date.num_days_from_ce() as f64, valid_day.stats.total_mouse_clicks as f64]);
        }

        vec![Line::new(PlotPoints::new(total_keystrokes)).name("Keystrokes").fill(0.0),
            Line::new(PlotPoints::new(total_mouse_clicks)).name("Mouse Clicks").fill(0.0)]
    }
}
