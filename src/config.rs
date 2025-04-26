use std::{collections::HashSet, path::Path};

use chrono::{Month, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Config {
    pub dates: Vec<TimeRangeMessage>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TimeRangeMessage {
    pub message: String,
    pub time: TimeRange,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TimeRange {
    pub day_of: Option<DayOf>,
    pub month: Option<HashSet<Month>>,
    pub year: Option<HashSet<u32>>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DayOf {
    #[serde(rename = "week")]
    Week(HashSet<Weekday>),
    #[serde(rename = "month")]
    Month(HashSet<u8>),
}
#[cfg(test)]
mod tests {
    use map_macro::hash_set;

    use super::*;

    #[test]
    /// A properly made `Config` should serialize properly.
    fn serialize() {
        let test_config = Config {
            dates: vec![TimeRangeMessage {
                time: TimeRange {
                    day_of: Some(DayOf::Week(hash_set! { Weekday::Mon, Weekday::Fri })),
                    month: Some(hash_set! { Month::September }),
                    year: None,
                },
                message: "hello!".to_string(),
            }],
        };
        let json = serde_json::to_string(&test_config).unwrap();

        let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();

        let dates = decoded["dates"].as_array().unwrap();
        let time_range_msg = dates[0].as_object().unwrap();

        let message = time_range_msg.get("message");
        assert_eq!(message.unwrap().as_str().unwrap(), "hello!");

        let time = time_range_msg.get("time").unwrap().as_object().unwrap();
        assert!(time.get("year").unwrap().is_null());
        assert!(time.get("month").unwrap().is_array());
        assert!(time.get("day_of").unwrap().is_object());
    }

    #[test]
    fn deserialize() {
        let test_config = Config {
            dates: vec![TimeRangeMessage {
                message: "hai :3".to_string(),
                time: TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { 1, 3, 5, 7, 9 })),
                    month: Some(hash_set! { Month::January, Month::June, Month::July }),
                    year: Some(hash_set! { 2016, 2017, 2018, 2022, 2024, 2005, 2030 }),
                },
            }],
        };
        let test_config_2 = Config {
            dates: vec![TimeRangeMessage {
                message: "hewwo !".to_string(),
                time: TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { 2 })),
                    month: None,
                    year: None,
                },
            }],
        };
        let json = serde_json::to_string(&test_config).unwrap();
        let json_2 = serde_json::to_string(&test_config_2).unwrap();

        let decoded_config: Config = serde_json::from_str(&json).unwrap();
        let decoded_config_2: Config = serde_json::from_str(&json_2).unwrap();
        assert_eq!(test_config, decoded_config);
        assert_eq!(test_config_2, decoded_config_2);
    }
}
