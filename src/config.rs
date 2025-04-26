use std::collections::HashSet;

use chrono::{Month, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Config {
    pub dates: Vec<TimeRangeMessage>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TimeRangeMessage {
    pub message: String,
    pub time: TimeRange,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TimeRange {
    pub day_of: Option<DayOf>,
    pub month: Option<HashSet<Month>>,
    pub year: Option<HashSet<u32>>,
}
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum DayOf {
    #[serde(rename = "week")]
    Week(HashSet<Weekday>),
    #[serde(rename = "month")]
    Month(HashSet<u8>),
}
