use std::collections::HashSet;

use chrono::{Month, Weekday};

pub struct Config {
    dates: Vec<TimeRangeMessage>,
}

pub struct TimeRangeMessage {
    message: String,
    time: TimeRange,
}

pub struct TimeRange {
    day_of: Option<DayOf>,
    month: Option<Month>,
    year: Option<u32>,
}
pub enum DayOf {
    Week(HashSet<Weekday>),
    Month(HashSet<u8>),
}
