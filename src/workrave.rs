use chrono::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufRead};

pub const WORKRAVE_HISTORYSTATS_FILENAME: &str = "historystats";
pub const WORKRAVE_TODAYSTATS_FILENAME: &str = "todaystats";
pub const WORKRAVE_MOVEMENT_TO_METERS: f32 = 4288.0;

#[derive(Debug)]
pub struct WorkraveDay {
    pub datetime_range: DatetimeRange,
    pub stats: InputStats,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct DatetimeRange {
    start: DateTime<Local>,
    end: DateTime<Local>,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct InputStats {
    pub total_active_time_seconds: u64,
    pub total_mouse_movement: f32,
    pub total_mouse_click_movement: f32,
    pub total_mouse_movement_time: u64,
    pub total_mouse_clicks: u64,
    pub total_keystrokes: u64,
}

impl WorkraveDay {
    fn round(x: f32, places: u32) -> f32 {
        let power = 10_i32.pow(places);
        (x * power as f32).round() / power as f32
    }

    fn convert_date_line(line: &str) -> DatetimeRange {
        let line: &str = &line[1..];
        let split_parsed: Vec<u32> = line.trim().split(" ").map(|s| s.parse().unwrap()).collect();

        // Month is indexed from 0
        let start = Local.with_ymd_and_hms(split_parsed[2] as i32 + 1900,
                                           split_parsed[1] + 1,
                                           split_parsed[0],
                                           split_parsed[3],
                                           split_parsed[4],
                                           0).unwrap();
        let end = Local.with_ymd_and_hms(split_parsed[7] as i32 + 1900,
                                         split_parsed[6] + 1,
                                         split_parsed[5],
                                         split_parsed[8],
                                         split_parsed[9],
                                         0).unwrap();
        DatetimeRange {
            start,
            end,
        }
    }

    fn convert_stats_line(line: &str) -> InputStats {
        // Ignore the 'm' line identifier character
        let line: &str = &line[1..];
        let split_parsed: Vec<u64> = line.trim().split(" ").map(|s| s.parse().unwrap()).collect();
        InputStats {
            total_active_time_seconds: split_parsed[1],
            total_mouse_movement: WorkraveDay::round(split_parsed[2] as f32 / WORKRAVE_MOVEMENT_TO_METERS, 2),
            total_mouse_click_movement: WorkraveDay::round(split_parsed[3] as f32 / WORKRAVE_MOVEMENT_TO_METERS, 2),
            total_mouse_movement_time: split_parsed[4],
            total_mouse_clicks: split_parsed[5],
            total_keystrokes: split_parsed[6],
        }
    }

    fn build_day(stats: InputStats, dates: DatetimeRange) -> WorkraveDay {
        WorkraveDay {
            datetime_range: dates,
            stats,
        }
    }
}

#[derive(Debug)]
pub struct WorkraveHistory {
    pub days: HashMap<NaiveDate, WorkraveDay>,
}

impl WorkraveHistory {
    pub fn is_file_valid(path: &str) -> bool {
        match fs::File::open(&path) {
            Ok(file) => {
                match BufReader::new(file).lines().next() {
                    Some(line_result) => {
                        match line_result {
                            Ok(line) => {
                                if line == "WorkRaveStats 4" {
                                    true
                                } else {
                                    false
                                }
                            }
                            Err(_) => false,
                        }
                    }
                    None => false
                }
            }
            Err(_) => {
                println!("Failed to open file, defaulting to invalid");
                false
            }
        }
    }

    pub fn load_historystats(path: &str) -> Option<WorkraveHistory> {
        if !WorkraveHistory::is_file_valid(&path) {
            return None
        };

        let valid_file = match fs::File::open(path) {
            Ok(file) => file,
            Err(_) => panic!("Failed to open historystats")
        };

        Some(WorkraveHistory::load_historystats_file(&valid_file))
    }

    pub fn load_historystats_file(file: &fs::File) -> WorkraveHistory {
        let reader = BufReader::new(file);

        let mut dates: Vec<DatetimeRange> = vec![];
        let mut input_stats: Vec<InputStats> = vec![];

        for (i, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(text) => text,
                Err(_) => continue,
            };

            let line = line.trim();
            if line.starts_with("D ") {
                dates.push(WorkraveDay::convert_date_line(&line));
            } else if line.starts_with("B ") {
                continue;
            } else if line.starts_with("m ") {
                input_stats.push(WorkraveDay::convert_stats_line(&line));
            } else {
                println!("Line {} ignored, contained: {}", i, line)
            }
        }

        if dates.len() != input_stats.len() {
            panic!("historystats file is unreadable, date and stat lines are not equal");
        }

        let mut days: HashMap<NaiveDate, WorkraveDay> = HashMap::new();
        for (i, date) in dates.iter().enumerate() {
            days.insert(date.start.date_naive(), WorkraveDay::build_day(input_stats[i], *date));
        }
        WorkraveHistory {
            days
        }
    }

    pub fn add_todaystats(&mut self, path: &str) {
        match WorkraveHistory::load_historystats(path) {
            Some(stats) => {
                self.days.extend(stats.days);
            },
            None => {
                println!("Failed to load todaystats")
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_date_line() {
        let line = "D 8 10 122 22 39 8 10 122 22 44";
        let datetime_start = Local.with_ymd_and_hms(2022, 11, 8, 22, 39, 0).unwrap();
        let datetime_end = Local.with_ymd_and_hms(2022, 11, 8, 22, 44, 0).unwrap();
        let date_range = DatetimeRange {
            start: datetime_start,
            end: datetime_end,
        };

        assert_eq!(WorkraveDay::convert_date_line(&line), date_range);
    }

    #[test]
    fn test_convert_stats_line() {
        let line = "m 6 338 28584 40231 29 104 33 ";
        let stats = InputStats {
            total_active_time_seconds: 338,
            total_mouse_movement: 6.67,
            total_mouse_click_movement: 9.38,
            total_mouse_movement_time: 29,
            total_mouse_clicks: 104,
            total_keystrokes: 33,
        };

        assert_eq!(WorkraveDay::convert_stats_line(&line), stats);
    }
}
