use chrono::Month;
use chrono::{Datelike, Local};
use map_macro::hash_set;
use occasion::config::{Config, DayOf, MultipleBehavior, TimeRange, TimeRangeMessage};

mod common;

#[test]
fn integration_no_config() {
    common::with_config_var(|| {
        let empty = Config::load_or_default().unwrap();
        let res = occasion::output_of(&empty);
        assert!(res.is_empty())
    });
}

#[test]
fn integration_with_config_multiple() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![
                TimeRangeMessage {
                    message: "hai".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: "hewwo :3".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: MultipleBehavior::default(),
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = occasion::output_of(&config);
        assert_eq!(res, "haihewwo :3");
    });
}

#[test]
fn integration_with_config_single() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![TimeRangeMessage {
                message: "hai".to_string(),
                time: TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                    month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                    year: Some(hash_set! { now.year() }),
                },
            }],
            multiple_behavior: MultipleBehavior::default(),
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = occasion::output_of(&config);
        assert_eq!(res, "hai");
    });
}

#[test]
fn integration_with_config_emoji() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![TimeRangeMessage {
                message: "🐈".to_string(),
                time: TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                    month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                    year: Some(hash_set! { now.year() }),
                },
            }],
            multiple_behavior: MultipleBehavior::default(),
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = occasion::output_of(&config);
        assert_eq!(res, "🐈");
    });
}
#[test]
fn integration_with_matching_and_nonmatching() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![
                TimeRangeMessage {
                    message: "hai".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: "hewwo :3".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 + 1 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: MultipleBehavior::default(),
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = occasion::output_of(&config);
        assert_eq!(res, "hai");
    });
}
#[test]
fn integration_with_all_custom_seperator() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![
                TimeRangeMessage {
                    message: "hai".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: "hewwo :3".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: "yipee !! \n this is on a new line".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: MultipleBehavior::All {
                seperator: "\n".to_string(),
            },
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = occasion::output_of(&config);
        assert_eq!(res, "hai\nhewwo :3\nyipee !! \n this is on a new line");
    });
}
#[test]
fn integration_with_first() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![
                TimeRangeMessage {
                    message: "hai".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: "hewwo :3".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: "yipee !! \n this is on a new line".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: MultipleBehavior::First,
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = occasion::output_of(&config);
        assert_eq!(res, "hai");
    });
}
#[test]
fn integration_with_last() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![
                TimeRangeMessage {
                    message: "hai".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: "hewwo :3".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: "yipee !! \n this is on a new line".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: MultipleBehavior::Last,
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = occasion::output_of(&config);
        assert_eq!(res, "yipee !! \n this is on a new line");
    });
}
#[test]
fn integration_with_random() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![
                TimeRangeMessage {
                    message: "hai".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: "hewwo :3".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: "mraow".to_string(),
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: MultipleBehavior::Random,
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        for _ in 0..10 {
            let res = occasion::output_of(&config);
            assert!(matches!(res.as_str(), "hai" | "hewwo :3" | "mraow"));
        }
    });
}
