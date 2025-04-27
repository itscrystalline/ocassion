use chrono::{DateTime, Datelike, FixedOffset, Local};

use crate::config::{DayOf, TimeRange, TimeRangeMessage};

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
    /// use ocassion::config::TimeRangeMessage;
    /// use chrono::{Local, DateTime};
    ///
    /// let now = Local::now().fixed_offset();
    /// let range = TimeRangeMessage {
    ///     message: "hewwo !".to_string(),
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
            Some(self.message.clone())
        } else {
            None
        }
    }

    /// similar to `try_message`, but takes a fixed DateTime. for testing.
    fn try_with_datetime(&self, dt: DateTime<FixedOffset>) -> Option<String> {
        if self.time.eval_with_datetime(dt) {
            Some(self.message.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use map_macro::hash_set;

    use crate::config::DayOf;
    use chrono::{Local, TimeZone, Weekday};

    use super::*;

    fn date(year: i32, month: u32, day: u32) -> DateTime<FixedOffset> {
        Local
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .unwrap()
            .fixed_offset()
    }

    #[test]
    fn eval_datetime_days_of_week() {
        let time = TimeRange {
            day_of: Some(DayOf::Week(hash_set! {Weekday::Mon, Weekday::Fri})),
            month: None,
            year: None,
        };
        let monday = date(2025, 4, 28);
        let friday = date(2025, 5, 2);
        let next_week = date(2025, 5, 5);
        assert!(time.eval_with_datetime(monday));
        assert!(time.eval_with_datetime(friday));
        assert!(time.eval_with_datetime(next_week));
    }
}
