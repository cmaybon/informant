use crate::{workrave};
use eframe::egui;
use egui::*;
use plot::{Plot, PlotPoint, Legend, Bar, BarChart};
use chrono::{NaiveDate, Datelike};
use std::ops::RangeInclusive;

pub struct StatsTab {
    pub workrave_history: Option<workrave::WorkraveHistory>,
    pub current_day: Option<workrave::WorkraveDay>,
    first_history_load: bool,
}

struct PlotData {
    key_strokes: Vec<BarChart>,
    mouse_movement: Vec<BarChart>,
    activity_time: Vec<BarChart>,
}

impl StatsTab {
    pub fn default() -> Self {
        Self {
            workrave_history: None,
            current_day: None,
            first_history_load: true,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, historystats_path: &Option<String>, todaystats_path: &Option<String>, frame: &eframe::Frame) -> Response {
        if let Some(path) = historystats_path {
            if Option::is_none(&self.workrave_history) && self.first_history_load {
                self.workrave_history = workrave::WorkraveHistory::load_historystats(path);
                if let Some(today_path) = todaystats_path {
                    self.workrave_history.as_mut().unwrap().add_todaystats(today_path);
                    println!("Loaded todaystats");
                }

                self.first_history_load = false;
            }
        }

        let history = match &self.workrave_history {
            Some(data) => data,
            None => {
                return self.no_history_data_ui(ui, historystats_path);
            }
        };

        ui.vertical_centered(|ui| {
            let link_axis_group = plot::LinkedAxisGroup::new(true, false);

            let window_size = frame.info().window_info.size;
            let large_plot_size = egui::Vec2::new(window_size.x - 15.0,
                                                  (window_size.y - 15.0) * 0.5);
            let small_plot_size = egui::Vec2::new((&large_plot_size.x - 10.0) * 0.5,
                                                  window_size.y * 0.40);
            let plot_data = StatsTab::build_plot_data(&history);

            ui.heading("Keystrokes");
            let mut keystrokes_plot = Plot::new("keystrokes_plot")
                .width(large_plot_size.x)
                .height(large_plot_size.y)
                .link_axis(link_axis_group.clone());
            keystrokes_plot = StatsTab::configure_plot_settings(keystrokes_plot);

            keystrokes_plot.show(ui, |plot_ui| {
                for chart in plot_data.key_strokes {
                    plot_ui.bar_chart(chart);
                }
            });

            ui.vertical(|ui| {
                ui.columns(2, |columns| {
                    columns[0].vertical_centered(|ui| {
                        ui.heading("Mouse Movement");
                    });
                    columns[1].vertical_centered(|ui| {
                        ui.heading("Activity Time");
                    });
                });

                ui.horizontal(|ui| {
                    let mut movement_plot = Plot::new("movement_plot")
                        .width(small_plot_size.x)
                        .height(small_plot_size.y)
                        .link_axis(link_axis_group.clone());
                    movement_plot = StatsTab::configure_plot_settings(movement_plot);
                    movement_plot.show(ui, |plot_ui| {
                        for chart in plot_data.mouse_movement {
                            plot_ui.bar_chart(chart);
                        }
                    });

                    let mut time_plot = Plot::new("time_plot")
                        .width(small_plot_size.x)
                        .height(small_plot_size.y)
                        .link_axis(link_axis_group.clone());
                    time_plot = StatsTab::configure_plot_settings(time_plot);
                    time_plot.show(ui, |plot_ui| {
                        for chart in plot_data.activity_time {
                            plot_ui.bar_chart(chart);
                        }
                    });
                });
            });
        }).response
    }

    fn no_history_data_ui(&mut self, ui: &mut Ui, path: &Option<String>) -> Response {
        ui.vertical_centered(|ui| {
            ui.heading("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nNo history data loaded");
            if ui.button("Try load data").clicked() {
                match path {
                    Some(s) => {
                        self.workrave_history = workrave::WorkraveHistory::load_historystats(s);
                    },
                    None => println!("Failed to load data, no path given"),
                };
            }
        }).response
    }

    fn build_plot_data(history: &workrave::WorkraveHistory) -> PlotData {
        fn create_bar_chart(bars: Vec<Bar>, name: &str, color: Color32, stacked_on: Option<&BarChart>, y_is_time: bool) -> BarChart {
            let chart = match y_is_time {
                true => {
                    BarChart::new(bars).name(name).color(color).element_formatter(Box::new(StatsTab::active_time_element_formatter))
                },
                false => {
                    BarChart::new(bars).name(name).color(color).element_formatter(Box::new(StatsTab::box_chart_element_formatter))
                }
            };

            match stacked_on {
                Some(s) => {
                    chart.stack_on(&[s])
                }
                None => {
                    chart
                }
            }
        }

        let mut total_keystrokes: Vec<Bar> = vec![];
        let mut total_mouse_clicks: Vec<Bar> = vec![];
        let mut total_movement: Vec<Bar> = vec![];
        let mut total_click_movement: Vec<Bar> = vec![];
        let mut total_active_time: Vec<Bar> = vec![];
        let mut total_mouse_time: Vec<Bar> = vec![];

        let sorted_dates = StatsTab::get_sorted_dates(history);
        for date in sorted_dates {
            match history.days.get(date) {
                Some(day) => {
                    let x = date.num_days_from_ce() as f64;
                    let stats = &day.stats;

                    total_keystrokes.push(Bar::new(x, stats.total_keystrokes as f64).name("Keystrokes"));
                    total_mouse_clicks.push(Bar::new(x, stats.total_mouse_clicks as f64).name("Mouse Clicks"));
                    total_movement.push(Bar::new(x, stats.total_mouse_movement as f64).name("Movement"));
                    total_click_movement.push(Bar::new(x, stats.total_mouse_click_movement as f64).name("Click Movement"));
                    total_active_time.push(Bar::new(x, stats.total_active_time_seconds as f64).name("Active Time"));
                    total_mouse_time.push(Bar::new(x, stats.total_mouse_movement_time as f64).name("Active Mouse Time"));
                }
                None => continue
            }
        };

        let mouse_clicks_chart = create_bar_chart(total_mouse_clicks,
                                                  "Mouse Clicks",
                                                  Color32::from_rgb(0, 202, 252),
                                                  None,
                                                  false);
        let keystrokes_chart = create_bar_chart(total_keystrokes,
                                                "Keystrokes",
                                                Color32::from_rgb(221, 18, 101),
                                                Some(&mouse_clicks_chart),
                                                false);

        let movement_chart = create_bar_chart(total_movement,
                                              "Movement",
                                              Color32::from_rgb(206, 145, 254),
                                              None,
                                              false);
        let click_movement_chart = create_bar_chart(total_click_movement,
                                                    "Click Movement",
                                                    Color32::from_rgb(255, 195, 29),
                                                    Some(&movement_chart),
                                                    false);

        let active_time_chart = create_bar_chart(total_active_time,
                                                 "Active Time",
                                                 Color32::from_rgb(220, 120, 244),
                                                 None,
                                                 true);
        let mouse_time_chart = create_bar_chart(total_mouse_time,
                                                "Mouse Time",
                                                Color32::from_rgb(255, 128, 7),
                                                Some(&active_time_chart),
                                                true);

        PlotData {
            key_strokes: vec![mouse_clicks_chart, keystrokes_chart],
            mouse_movement: vec![movement_chart, click_movement_chart],
            activity_time: vec![active_time_chart, mouse_time_chart],
        }
    }

    fn get_sorted_dates(history: &workrave::WorkraveHistory) -> Vec<&NaiveDate> {
        let mut sorted_dates: Vec<&NaiveDate> = history.days.keys().clone().collect();
        sorted_dates.sort();
        sorted_dates
    }

    fn configure_plot_settings(plot: Plot) -> Plot {
        plot.include_y(0.0)
            .label_formatter(StatsTab::general_label_formatter)
            .allow_boxed_zoom(false)
            .allow_drag(true)
            .x_axis_formatter(StatsTab::x_axis_formatter)
            .legend(Legend::default())
    }

    fn general_label_formatter(plot_points_name: &str, plot_point: &PlotPoint) -> String {
        let date = match NaiveDate::from_num_days_from_ce_opt(plot_point.x as i32) {
            Some(value) => value,
            None => {
                return format!("DATE ERR");
            }
        };

        let date = "Date:     ".to_owned() + &StatsTab::naive_date_to_string(&date);
        if plot_points_name.is_empty() {
            date
        } else {
            format!("{}\n{}\nValue:    {}",
                    plot_points_name,
                    date,
                    plot_point.y)
        }
    }

    fn box_chart_element_formatter(bar: &Bar, _chart: &BarChart) -> String {
        let date = match NaiveDate::from_num_days_from_ce_opt(bar.argument as i32) {
            Some(value) => value,
            None => {
                return format!("DATE ERR");
            }
        };

        let date = "Date:     ".to_owned() + &StatsTab::naive_date_to_string(&date);
        format!("{}\n{}\n{:.2}", bar.name, date, bar.value)
    }

    fn active_time_element_formatter(bar: &Bar, _chart: &BarChart) -> String {
        match NaiveDate::from_num_days_from_ce_opt(bar.argument as i32) {
            Some(date) => {
                let date = "Date:     ".to_owned() + &StatsTab::naive_date_to_string(&date);
                format!("{}\n{}\nValue:    {}hr {}min {}s",
                        bar.name,
                        date,
                        format!("{:.0}", (bar.value / 60.0) / 60.0),
                        format!("{:.0}", (bar.value / 60.0) % 60.0),
                        format!("{:.0}", bar.value % 60.0))
            },
            None => {
                return format!("DATE ERR");
            }
        }
    }

    fn x_axis_formatter(x: f64, _range: &RangeInclusive<f64>) -> String {
        let date = match NaiveDate::from_num_days_from_ce_opt(x as i32) {
            Some(value) => value,
            None => {
                return format!("DATE ERR");
            }
        };
        StatsTab::naive_date_to_string(&date)
    }

    fn naive_date_to_string(date: &NaiveDate) -> String {
        format!("{}-{}-{}",
                date.day(),
                date.month(),
                date.year())
    }
}
