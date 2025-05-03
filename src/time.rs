use std::process::{Command, Output};

use chrono::{DateTime, Datelike, FixedOffset, Local, Weekday};

use crate::config::{
    CustomCommand, DayOf, MergeStratagy, RunCondition, TimeRange, TimeRangeMessage,
};

impl TimeRange {
    fn evaluate(&self, dt: DateTime<FixedOffset>) -> bool {
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
    ///     time: Some(TimeRange {
    ///         day_of: Some(DayOf::Month(HashSet::from_iter(vec![now.day() as u8].into_iter()))),
    ///         month: None,
    ///         year: None,
    ///     }),
    ///     ..Default::default()
    /// };
    /// let result = range.try_message(None);
    /// assert!(result.is_some());
    /// assert_eq!(result.unwrap(), "hewwo !");
    /// ```
    pub fn try_message(&self, week_start_day: Option<Weekday>) -> Option<String> {
        let week_start_day = week_start_day.unwrap_or(Weekday::Sun);
        if self.time.evaluate() {
            let now = Local::now().fixed_offset();
            self.message(now, week_start_day)
        } else {
            None
        }
    }

    fn message(&self, now: DateTime<FixedOffset>, week_start_day: Weekday) -> Option<String> {
        self.command.as_ref().map_or_else(
            || self.message.clone(),
            |command| command.run(now, week_start_day).or(self.message.clone()),
        )
    }

    /// similar to `try_message`, but takes a fixed DateTime. for testing.
    #[cfg(test)]
    fn try_with_datetime(
        &self,
        dt: DateTime<FixedOffset>,
        week_start_day: Option<Weekday>,
    ) -> Option<String> {
        let week_start_day = week_start_day.unwrap_or(Weekday::Sun);
        if self.time.eval_with_datetime(dt) {
            self.message(dt, week_start_day)
        } else {
            None
        }
    }
}

impl CustomCommand {
    /// Runs the input with the specified shell and shell_args, and returns the `stdout` of the
    /// command wrapped in `Some`, or `None` if the command fails and stdout is empty.
    fn run(&self, now: DateTime<FixedOffset>, week_start_day: Weekday) -> Option<String> {
        let CustomCommand {
            run,
            shell,
            shell_flags,
        } = self;
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
        cmd.envs([
            ("DAY_OF_WEEK", format!("{}", now.weekday())),
            (
                "DAY_IN_WEEK",
                format!("{}", now.weekday().days_since(week_start_day)),
            ),
            ("DAY_OF_MONTH", format!("{}", now.day())),
            ("WEEK", format!("{}", now.iso_week().week())),
            ("MONTH", format!("{}", now.month())),
            ("YEAR", format!("{}", now.year())),
        ]);
        cmd.arg(run)
            .output()
            .ok()
            .map(|Output { stdout, status, .. }| {
                if status.success() | !stdout.is_empty() {
                    let opt = String::from_utf8(stdout).ok().map(|mut str| {
                        if str.ends_with("\n") {
                            _ = str.pop();
                        }
                        Some(str)
                    });
                    opt.flatten()
                } else {
                    None
                }
            })?
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
        assert!(time.evaluate(now));
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
        assert!(time.evaluate(monday));
        assert!(time.evaluate(friday));
        assert!(!time.evaluate(sunday));
        assert!(time.evaluate(next_week));
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
        assert!(!time.evaluate(third));
        assert!(time.evaluate(first));
        assert!(time.evaluate(second));
        assert!(time.evaluate(fifth));
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
        assert!(!time.evaluate(april));
        assert!(time.evaluate(jan));
        assert!(time.evaluate(march));
        assert!(time.evaluate(sep));
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
        assert!(!time.evaluate(year24));
        assert!(time.evaluate(year22));
        assert!(time.evaluate(year23));
        assert!(time.evaluate(year25));
    }

    #[test]
    fn message_now() {
        let now = Local::now().fixed_offset();
        let range = TimeRangeMessage {
            message: Some("hewwo !".to_string()),
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                month: None,
                year: None,
            }),
            ..Default::default()
        };
        let range_tmrw = TimeRangeMessage {
            message: Some("hewwo !".to_string()),
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { now.day() as u8 + 1 })),
                month: None,
                year: None,
            }),
            ..Default::default()
        };
        let result = range.try_message(None);
        assert_eq!(result.unwrap(), "hewwo !");
        let result = range_tmrw.try_message(None);
        assert!(result.is_none());
    }
    #[test]
    fn message() {
        let range = TimeRangeMessage {
            message: Some("hewwo !".to_string()),
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3, 5, 9 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };

        let first_june = date(2025, 6, 1);
        let third_june = date(2025, 6, 3);
        let fifth_june = date(2025, 6, 5);
        let ninth_june = date(2025, 6, 9);
        let third_july = date(2025, 7, 3);

        assert!(range.try_with_datetime(first_june, None).is_none());
        assert!(range.try_with_datetime(third_july, None).is_none());
        assert_eq!(
            range.try_with_datetime(third_june, None).unwrap(),
            "hewwo !"
        );
        assert_eq!(
            range.try_with_datetime(fifth_june, None).unwrap(),
            "hewwo !"
        );
        assert_eq!(
            range.try_with_datetime(ninth_june, None).unwrap(),
            "hewwo !"
        );
    }
    #[test]
    fn command_with_default_shell() {
        let range = TimeRangeMessage {
            command: Some(CustomCommand {
                run: "echo 'hi!'".to_string(),
                shell: None,
                shell_flags: None,
            }),
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(range.try_with_datetime(third_june, None).unwrap(), "hi!");
    }
    #[test]
    fn command_with_env_vars() {
        let range = TimeRangeMessage {
            command: Some(CustomCommand {
                run: "echo $DAY_OF_WEEK $DAY_IN_WEEK $DAY_OF_MONTH $WEEK $MONTH $YEAR".to_string(),
                shell: None,
                shell_flags: None,
            }),
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(
            range.try_with_datetime(third_june, None).unwrap(),
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
    fn command_with_custom_week_start() {
        let range = TimeRangeMessage {
            command: Some(CustomCommand {
                run: "echo $DAY_OF_WEEK $DAY_IN_WEEK $DAY_OF_MONTH $WEEK $MONTH $YEAR".to_string(),
                shell: None,
                shell_flags: None,
            }),
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(
            range
                .try_with_datetime(third_june, Some(Weekday::Tue))
                .unwrap(),
            format!(
                "{} {} {} {} {} {}",
                third_june.weekday(),
                third_june.weekday().days_since(Weekday::Tue),
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
            command: Some(CustomCommand {
                run: "echo 'hi!    '".to_string(),
                shell: None,
                shell_flags: None,
            }),
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };
        let with_no_newline = TimeRangeMessage {
            message: None,
            command: Some(CustomCommand {
                run: "echo -n 'hi! this will not have a newline'".to_string(),
                shell: None,
                shell_flags: None,
            }),
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };
        let third_june = date(2025, 6, 3);

        assert_eq!(
            with_spaces.try_with_datetime(third_june, None).unwrap(),
            "hi!    "
        );
        assert_eq!(
            with_no_newline.try_with_datetime(third_june, None).unwrap(),
            "hi! this will not have a newline"
        );
    }
    #[test]
    fn command_with_bash() {
        let range = TimeRangeMessage {
            command: Some(CustomCommand {
                run: "echo 'hi!'".to_string(),
                shell: Some("bash".to_string()),
                shell_flags: None,
            }),
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(range.try_with_datetime(third_june, None).unwrap(), "hi!");
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
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(
            range.try_with_datetime(third_june, None).unwrap(),
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
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(
            range.try_with_datetime(third_june, None).unwrap(),
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
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(
            range.try_with_datetime(third_june, None).unwrap(),
            "existing"
        );
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
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };

        let third_june = date(2025, 6, 3);

        assert_eq!(
            range.try_with_datetime(third_june, None).unwrap(),
            "hello world!"
        );
    }
    #[test]
    fn both_none() {
        let range = TimeRangeMessage {
            message: None,
            command: None,
            time: Some(TimeRange {
                day_of: Some(DayOf::Month(hash_set! { 3 })),
                month: Some(hash_set! { Month::June }),
                year: None,
            }),
            ..Default::default()
        };

        let third_june = date(2025, 6, 3);

        assert!(range.try_with_datetime(third_june, None).is_none());
    }
}
