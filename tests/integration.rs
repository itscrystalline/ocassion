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
        let empty = Config::load_or_default(false).unwrap();
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
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
            ],
            multiple_behavior: Some(MultipleBehavior::default()),
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
                message: Some("hai".to_string()),
                time: Some(TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                    week: None,
                    month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                    year: Some(hash_set! { now.year() }),
                }),
                ..Default::default()
            }],
            multiple_behavior: Some(MultipleBehavior::default()),
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
                time: Some(TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                    week: None,
                    month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                    year: Some(hash_set! { now.year() }),
                }),
                ..Default::default()
            }],
            multiple_behavior: Some(MultipleBehavior::default()),
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 + 1 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
            ],
            multiple_behavior: Some(MultipleBehavior::default()),
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("yipee !! \n this is on a new line".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
            ],
            multiple_behavior: Some(MultipleBehavior::All {
                seperator: "\n".to_string(),
            }),
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("yipee !! \n this is on a new line".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
            ],
            multiple_behavior: Some(MultipleBehavior::First),
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("yipee !! \n this is on a new line".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
            ],
            multiple_behavior: Some(MultipleBehavior::Last),
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("hewwo :3".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
                TimeRangeMessage {
                    message: Some("mraow".to_string()),
                    time: Some(TimeRange {
                        day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                        week: None,
                        month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                        year: Some(hash_set! { now.year() }),
                    }),
                    ..Default::default()
                },
            ],
            multiple_behavior: Some(MultipleBehavior::Random),
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
                command: Some(CustomCommand {
                    run: "echo \"Hello! today is $DAY_OF_WEEK and it is the $DAY_IN_WEEK day of the week\"".to_string(),
                    ..Default::default()
                }),
                time: Some(TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                    week: None,
                    month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                    year: Some(hash_set! { now.year() }),
                }),
                ..Default::default()
            }],
            multiple_behavior: Some(MultipleBehavior::Random),
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
                command: Some(CustomCommand {
                    run: "echo \"Hello! today is $DAY_OF_WEEK and it is the $DAY_IN_WEEK day of the week\"".to_string(),
                    ..Default::default()
                }),
                time: Some(TimeRange {
                    day_of: Some(DayOf::Month(hash_set! { now.day() as u8 })),
                    week: None,
                    month: Some(hash_set! { Month::try_from(now.month() as u8).unwrap() }),
                    year: Some(hash_set! { now.year() }),
                }),
                ..Default::default()
            }],
            multiple_behavior: Some(MultipleBehavior::Random),
            week_start_day: Some(Weekday::Mon),
            ..Default::default()
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default(false).unwrap();

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
