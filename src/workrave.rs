use chrono::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufRead};

pub struct WorkraveDay {
    datetime_range: DatetimeRange,
    stats: InputStats,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct DatetimeRange {
    start: DateTime<Local>,
    end: DateTime<Local>,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct InputStats {
    total_active_time_seconds: u64,
    total_mouse_movement: u64,
    total_click_movement: u64,
    total_movement_time: u64,
    total_clicks: u64,
    total_keystrokes: u64,
}

impl WorkraveDay {
    fn convert_date_line(line: &str) -> DatetimeRange {
        let line: &str = &line[1..];
        let split_parsed: Vec<u32> = line.trim().split(" ").map(|s| s.parse().unwrap()).collect();

        let start = Local.with_ymd_and_hms(split_parsed[2] as i32 + 1900,
                                           split_parsed[1],
                                           split_parsed[0],
                                           split_parsed[3],
                                           split_parsed[4],
                                           0).unwrap();
        let end = Local.with_ymd_and_hms(split_parsed[7] as i32 + 1900,
                                         split_parsed[6],
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
            total_mouse_movement: split_parsed[2],
            total_click_movement: split_parsed[3],
            total_movement_time: split_parsed[4],
            total_clicks: split_parsed[5],
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

pub struct WorkraveHistory {
    days: HashMap<DateTime<Local>, WorkraveDay>,
}

impl WorkraveHistory {
    fn is_file_valid(path: &str) -> Option<bool>{
        // error handling opening file
        // read first line
        // if first line is expected, return true
        Some(false)
    }

    fn load_historystats_file(file: &fs::File) -> WorkraveHistory {
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

        let mut days = HashMap::new();
        for (i, date) in dates.iter().enumerate() {
            days.insert(date.start,WorkraveDay::build_day(input_stats[i], *date));
        }
        WorkraveHistory {
            days
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_date_line() {
        let line = "D 8 10 122 22 39 8 10 122 22 44";
        let datetime_start = Local.with_ymd_and_hms(2022, 10, 8, 22, 39, 0).unwrap();
        let datetime_end = Local.with_ymd_and_hms(2022, 10, 8, 22, 44, 0).unwrap();
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
            total_mouse_movement: 28584,
            total_click_movement: 40231,
            total_movement_time: 29,
            total_clicks: 104,
            total_keystrokes: 33,
        };

        assert_eq!(WorkraveDay::convert_stats_line(&line), stats);
    }

    #[test]
    fn test_build_workrave_history() {
        // let expected = WorkraveHistory {}

        // assert_eq!()
    }
}
