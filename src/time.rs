use std::process::{Command, Output};

use chrono::{DateTime, Datelike, FixedOffset, Local};

use crate::config::{CustomCommand, DayOf, TimeRange, TimeRangeMessage};

impl TimeRange {
    pub fn evaluate(&self) -> bool {
        let now = Local::now().fixed_offset();
        self.eval_with_datetime(now)
    }

    fn eval_with_datetime(&self, dt: DateTime<FixedOffset>) -> bool {
        let match_year = match &self.year {
            None => true,
            Some(years) => years.iter().any(|&f| f == dt.year()),
        };
        let match_month = match &self.month {
            None => true,
            Some(months) => months.iter().any(|&m| m as u32 == dt.month0()),
        };
        let match_day = match &self.day_of {
            None => true,
            Some(DayOf::Week(weekdays)) => weekdays.iter().any(|&wk| wk == dt.weekday()),
            Some(DayOf::Month(days)) => days.iter().any(|&d| d as u32 == dt.day()),
        };

        match_year && match_month && match_day
    }
}

impl TimeRangeMessage {
    /// Evaluates the contained `TimeRange` and if it is true, return the configured message.
    /// Otherwise returns `None`.
    ///
    /// ```
    /// use occasion::config::{TimeRangeMessage, TimeRange, DayOf};
    /// use chrono::{Local, DateTime, Datelike};
    /// use std::collections::HashSet;
    ///
    /// let now = Local::now().fixed_offset();
    /// let range = TimeRangeMessage {
    ///     message: Some("hewwo !".to_string()),
    ///     command: None,
    ///     week_start_day: None,
    ///     time: TimeRange {
    ///         day_of: Some(DayOf::Month(HashSet::from_iter(vec![now.day() as u8].into_iter()))),
    ///         month: None,
    ///         year: None,
    ///     },
    /// };
    /// let result = range.try_message();
    /// assert!(result.is_some());
    /// assert_eq!(result.unwrap(), "hewwo !");
    /// ```
    pub fn try_message(&self) -> Option<String> {
        if self.time.evaluate() {
            self.message()
        } else {
            None
        }
    }

    fn message(&self) -> Option<String> {
        self.command.as_ref().map_or_else(
            || self.message.clone(),
            |CustomCommand {
                 run,
                 shell,
                 shell_flags,
             }| {
                // TODO: add defaults for windows and shell flags
                let mut cmd = Command::new(shell.clone().unwrap_or(
                    #[cfg(target_os = "windows")]
                    "cmd.exe".to_string(),
                    #[cfg(not(target_os = "windows"))]
                    "sh".to_string(),
                ));
                if let Some(shell_flags) = shell_flags {
                    cmd.args(shell_flags);
                } else {
                    cmd.arg(
                        #[cfg(target_os = "windows")]
                        "/C",
                        #[cfg(not(target_os = "windows"))]
                        "-c",
                    );
                }
                cmd.arg(run).output().map_or_else(
                    |_| self.message.clone(),
                    |Output { stdout, status, .. }| {
                        if status.success() | !stdout.is_empty() {
                            String::from_utf8(stdout).ok().map_or_else(
                                || self.message.clone(),
                                |mut str| {
                                    if str.ends_with("\n") {
                                        _ = str.pop();
                                    }
                                    Some(str)
                                },
                            )
                        } else {
                            self.message.clone()
                        }
                    },
                )
            },
        )
    }

    /// similar to `try_message`, but takes a fixed DateTime. for testing.
    #[cfg(test)]
    fn try_with_datetime(&self, dt: DateTime<FixedOffset>) -> Option<String> {
        if self.time.eval_with_datetime(dt) {
            self.message()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use map_macro::hash_set;

    use crate::config::{CustomCommand, DayOf};
    use chrono::{Local, Month, TimeZone, Weekday};

    use super::*;

    fn date(year: i32, month: u32, day: u32) -> DateTime<FixedOffset> {
        Local
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .unwrap()
            .fixed_offset()
    }

    #[test]
    fn eval_now() {
        let now = Local::now().fixed_offset();
        let time = TimeRange {
            day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
            month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
            year: Some(hash_set! { now.year() }),
        };
        assert!(time.evaluate());
    }

    #[test]
    fn eval_datetime_days_of_week() {
        let time = TimeRange {
            day_of: Some(DayOf::Week(hash_set! { Weekday::Mon, Weekday::Fri })),
            month: None,
            year: None,
        };
        let monday = date(2025, 4, 28);
        let friday = date(2025, 5, 2);
        let sunday = date(2025, 5, 4);
        let next_week = date(2025, 5, 5);
        assert!(time.eval_with_datetime(monday));
        assert!(time.eval_with_datetime(friday));
        assert!(!time.eval_with_datetime(sunday));
        assert!(time.eval_with_datetime(next_week));
    }
    #[test]
    fn eval_datetime_days_of_month() {
        let time = TimeRange {
            day_of: Some(DayOf::Month(hash_set! { 1, 2, 5 })),
            month: None,
            year: None,
        };
        let first = date(2020, 1, 1);
        let second = date(2027, 3, 2);
        let third = date(2012, 12, 3);
        let fifth = date(2021, 9, 5);
        assert!(!time.eval_with_datetime(third));
        assert!(time.eval_with_datetime(first));
        assert!(time.eval_with_datetime(second));
        assert!(time.eval_with_datetime(fifth));
    }
    #[test]
    fn eval_datetime_month() {
        let time = TimeRange {
            day_of: None,
            month: Some(hash_set! { Month::January, Month::March, Month::September }),
            year: None,
        };
        let jan = date(2020, 1, 1);
        let march = date(2027, 3, 2);
        let april = date(2012, 4, 3);
        let sep = date(2021, 9, 5);
        assert!(!time.eval_with_datetime(april));
        assert!(time.eval_with_datetime(jan));
        assert!(time.eval_with_datetime(march));
        assert!(time.eval_with_datetime(sep));
    }
    #[test]
    fn eval_datetime_year() {
        let time = TimeRange {
            day_of: None,
            month: None,
            year: Some(hash_set! { 2022, 2023, 2025 }),
        };
        let year22 = date(2022, 1, 1);
        let year23 = date(2023, 3, 2);
        let year24 = date(2024, 4, 3);
        let year25 = date(2025, 9, 5);
        assert!(!time.eval_with_datetime(year24));
        assert!(time.eval_with_datetime(year22));
        assert!(time.eval_with_datetime(year23));
        assert!(time.eval_with_datetime(year25));
    }

    #[test]
    fn message_now() {
        let now = Local::now().fixed_offset();
        let range = TimeRangeMessage {
            message: Some("hewwo !".to_string()),
            command: None,
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                month: None,
                year: None,
            },
        };
        let range_tmrw = TimeRangeMessage {
            message: Some("hewwo !".to_string()),
            command: None,
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { now.day() as u8 + 1 })),
                month: None,
                year: None,
            },
        };
        let result = range.try_message();
        assert_eq!(result.unwrap(), "hewwo !");
        let result = range_tmrw.try_message();
        assert!(result.is_none());
    }
    #[test]
    fn message() {
        let range = TimeRangeMessage {
            message: Some("hewwo !".to_string()),
            command: None,
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3, 5, 9 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };

        let first_june = date(2025, 6, 1);
        let third_june = date(2025, 6, 3);
        let fifth_june = date(2025, 6, 5);
        let ninth_june = date(2025, 6, 9);
        let third_july = date(2025, 7, 3);

        assert!(range.try_with_datetime(first_june).is_none());
        assert!(range.try_with_datetime(third_july).is_none());
        assert_eq!(range.try_with_datetime(third_june).unwrap(), "hewwo !");
        assert_eq!(range.try_with_datetime(fifth_june).unwrap(), "hewwo !");
        assert_eq!(range.try_with_datetime(ninth_june).unwrap(), "hewwo !");
    }
    #[test]
    fn command_with_default_shell() {
        let range = TimeRangeMessage {
            message: None,
            command: Some(CustomCommand {
                run: "echo 'hi!'".to_string(),
                shell: None,
                shell_flags: None,
            }),
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(range.try_with_datetime(third_june).unwrap(), "hi!");
    }
    #[test]
    fn command_with_env_vars() {
        let range = TimeRangeMessage {
            message: None,
            command: Some(CustomCommand {
                run: "echo '$DAY_OF_WEEK $DAY_IN_WEEK $DAY_OF_MONTH $WEEK $MONTH $YEAR'"
                    .to_string(),
                shell: None,
                shell_flags: None,
            }),
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(
            range.try_with_datetime(third_june).unwrap(),
            format!(
                "{} {} {} {} {} {}",
                third_june.weekday(),
                third_june.weekday().days_since(Weekday::Sun),
                third_june.day(),
                third_june.iso_week().week(),
                third_june.month(),
                third_june.year()
            )
        );
    }

    #[test]
    fn command_strip_only_trailing_newline() {
        let with_spaces = TimeRangeMessage {
            message: None,
            command: Some(CustomCommand {
                run: "echo 'hi!    '".to_string(),
                shell: None,
                shell_flags: None,
            }),
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };
        let with_no_newline = TimeRangeMessage {
            message: None,
            command: Some(CustomCommand {
                run: "echo -n 'hi! this will not have a newline'".to_string(),
                shell: None,
                shell_flags: None,
            }),
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };
        let third_june = date(2025, 6, 3);

        assert_eq!(
            with_spaces.try_with_datetime(third_june).unwrap(),
            "hi!    "
        );
        assert_eq!(
            with_no_newline.try_with_datetime(third_june).unwrap(),
            "hi! this will not have a newline"
        );
    }
    #[test]
    fn command_with_bash() {
        let range = TimeRangeMessage {
            message: None,
            command: Some(CustomCommand {
                run: "echo 'hi!'".to_string(),
                shell: Some("bash".to_string()),
                shell_flags: None,
            }),
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(range.try_with_datetime(third_june).unwrap(), "hi!");
    }
    #[test]
    fn command_overtakes_message() {
        let range = TimeRangeMessage {
            message: Some("and not this one".to_string()),
            command: Some(CustomCommand {
                run: "echo 'this will get printed'".to_string(),
                shell: None,
                shell_flags: None,
            }),
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(
            range.try_with_datetime(third_june).unwrap(),
            "this will get printed"
        );
    }
    #[test]
    fn command_failure_fallback() {
        let range = TimeRangeMessage {
            message: Some("it will fall back to this".to_string()),
            command: Some(CustomCommand {
                run: "ls non_existing".to_string(),
                shell: None,
                shell_flags: None,
            }),
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(
            range.try_with_datetime(third_june).unwrap(),
            "it will fall back to this"
        );
    }
    #[test]
    fn command_failure_stdout_no_fallback() {
        std::fs::write("existing", "meow :3").unwrap();
        let range = TimeRangeMessage {
            message: Some("it will not fall back to this".to_string()),
            command: Some(CustomCommand {
                run: "ls non_existing existing".to_string(),
                shell: None,
                shell_flags: None,
            }),
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(range.try_with_datetime(third_june).unwrap(), "existing");
        std::fs::remove_file("existing").unwrap();
    }
    #[test]
    fn command_custom_with_flags() {
        let range = TimeRangeMessage {
            message: Some("it will not fall back to this".to_string()),
            command: Some(CustomCommand {
                run: "print('hello world!')".to_string(),
                shell: Some("python".to_string()),
                shell_flags: Some(vec!["-c".to_string()]),
            }),
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(range.try_with_datetime(third_june).unwrap(), "hello world!");
    }
    #[test]
    fn both_none() {
        let range = TimeRangeMessage {
            message: None,
            command: None,
            week_start_day: None,
            time: TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            },
        };

        let third_june = date(2025, 6, 3);

        assert!(range.try_with_datetime(third_june).is_none());
    }
}
