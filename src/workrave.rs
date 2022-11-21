use chrono::prelude::*;

struct WorkraveDayStats {
    datetime_start: DateTime<Local>,
    datetime_end: DateTime<Local>,
    total_active_time_seconds: u64,
    total_mouse_movement: u64,
    total_click_movement: u64,
    total_movement_time: u64,
    total_clicks: u64,
    total_keystrokes: u64,
}

#[derive(PartialEq, Debug)]
struct Stats {
    total_active_time_seconds: u64,
    total_mouse_movement: u64,
    total_click_movement: u64,
    total_movement_time: u64,
    total_clicks: u64,
    total_keystrokes: u64,
}

impl WorkraveDayStats {
    fn convert_line(&mut self, line: &str, day_stats: &mut WorkraveDayStats) {
        if line.starts_with("D") {
            let (datetime_start, datetime_end) = WorkraveDayStats::convert_date_line(line);
            self.datetime_start = datetime_start;
            self.datetime_end = datetime_end;
        } else if line.starts_with("B") {
            return;
        } else if line.starts_with("m") {
            WorkraveDayStats::convert_stats_line(line);
        }
    }

    fn convert_date_line(line: &str) -> (DateTime<Local>, DateTime<Local>) {
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
        (start, end)
    }

    fn convert_stats_line(line: &str) -> Stats {
        // Ignore the 'm' line identifier character
        let line: &str = &line[1..];
        let split_parsed: Vec<u64> = line.trim().split(" ").map(|s| s.parse().unwrap()).collect();
        Stats {
            total_active_time_seconds: split_parsed[1],
            total_mouse_movement: split_parsed[2],
            total_click_movement: split_parsed[3],
            total_movement_time: split_parsed[4],
            total_clicks: split_parsed[5],
            total_keystrokes: split_parsed[6],
        }
    }

    fn build_day_stats(stats: Stats, dates: (DateTime<Local>, DateTime<Local>)) -> WorkraveDayStats {
        WorkraveDayStats {
            datetime_start: dates[0],
            datetime_end: dates[1],
            total_active_time_seconds: stats.total_active_time_seconds,
            total_mouse_movement: stats.total_mouse_movement,
            total_click_movement: stats.total_click_movement,
            total_movement_time: stats.total_movement_time,
            total_clicks: stats.total_clicks,
            total_keystrokes: stats.total_keystrokes,
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

        assert_eq!(WorkraveDayStats::convert_date_line(&line), (datetime_start, datetime_end));
    }

    #[test]
    fn test_convert_stats_line() {
        let line = "m 6 338 28584 40231 29 104 33 ";
        let stats = Stats {
            total_active_time_seconds: 338,
            total_mouse_movement: 28584,
            total_click_movement: 40231,
            total_movement_time: 29,
            total_clicks: 104,
            total_keystrokes: 33,
        };

        assert_eq!(WorkraveDayStats::convert_stats_line(&line), stats);
    }
}
