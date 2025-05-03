use chrono::{Datelike, Local};
use chrono::{Month, Weekday};
use map_macro::hash_set;
use occasion::config::{
    Config, CustomCommand, DayOf, MultipleBehavior, TimeRange, TimeRangeMessage,
};

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
                    message: Some("hai".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: Some(MultipleBehavior::default()),
            week_start_day: None,
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = occasion::output_of(&config);
        assert_eq!(res, "haihewwo :3");
    });
}
#[test]
fn integration_with_config_multiple_default_behavior() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![
                TimeRangeMessage {
                    message: Some("hai".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: None,
            week_start_day: None,
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
                command: None,
                message: Some("hai".to_string()),
                time: TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                    month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                    year: Some(hash_set! { now.year() }),
                },
            }],
            multiple_behavior: Some(MultipleBehavior::default()),
            week_start_day: None,
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
                message: Some("🐈".to_string()),
                command: None,
                time: TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                    month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                    year: Some(hash_set! { now.year() }),
                },
            }],
            multiple_behavior: Some(MultipleBehavior::default()),
            week_start_day: None,
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
                    message: Some("hai".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 + 1 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: Some(MultipleBehavior::default()),
            week_start_day: None,
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
                    message: Some("hai".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("yipee !! \n this is on a new line".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: Some(MultipleBehavior::All {
                seperator: "\n".to_string(),
            }),
            week_start_day: None,
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
                    message: Some("hai".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("yipee !! \n this is on a new line".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: Some(MultipleBehavior::First),
            week_start_day: None,
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
                    message: Some("hai".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("yipee !! \n this is on a new line".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: Some(MultipleBehavior::Last),
            week_start_day: None,
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
                    message: Some("hai".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
                TimeRangeMessage {
                    message: Some("mraow".to_string()),
                    command: None,
                    time: TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    },
                },
            ],
            multiple_behavior: Some(MultipleBehavior::Random),
            week_start_day: None,
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        for _ in 0..10 {
            let res = occasion::output_of(&config);
            assert!(matches!(res.as_str(), "hai" | "hewwo :3" | "mraow"));
        }
    });
}
#[test]
fn integration_with_shell_commands_with_vars() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![TimeRangeMessage {
                message: None,
                command: Some(CustomCommand {
                    run: "echo \"Hello! today is $DAY_OF_WEEK and it is the $DAY_IN_WEEK day of the week\"".to_string(),
                    ..Default::default()
                }),
                time: TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                    month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                    year: Some(hash_set! { now.year() }),
                },
            }],
            multiple_behavior: Some(MultipleBehavior::Random),
            week_start_day: None,
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = occasion::output_of(&config);
        assert_eq!(
            res,
            format!(
                "Hello! today is {} and it is the {} day of the week",
                now.weekday(),
                now.weekday().days_since(Weekday::Sun)
            )
        );
    });
}
#[test]
fn integration_with_shell_commands_with_vars_and_custom_week_start() {
    common::with_config_var(|| {
        let now = Local::now().fixed_offset();
        let test_config = Config {
            dates: vec![TimeRangeMessage {
                message: None,
                command: Some(CustomCommand {
                    run: "echo \"Hello! today is $DAY_OF_WEEK and it is the $DAY_IN_WEEK day of the week\"".to_string(),
                    ..Default::default()
                }),
                time: TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                    month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                    year: Some(hash_set! { now.year() }),
                },
            }],
            multiple_behavior: Some(MultipleBehavior::Random),
            week_start_day: Some(Weekday::Mon),
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = occasion::output_of(&config);
        assert_eq!(
            res,
            format!(
                "Hello! today is {} and it is the {} day of the week",
                now.weekday(),
                now.weekday().days_since(Weekday::Mon)
            )
        );
    });
}
