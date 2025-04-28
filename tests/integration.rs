use chrono::Month;
use chrono::{Datelike, Local};
use map_macro::hash_set;
use ocassion::config::{Config, DayOf, TimeRange, TimeRangeMessage};

mod common;

#[test]
fn integration_no_config() {
    common::with_config_var(|| {
        let empty = Config::load_or_default().unwrap();
        let res = ocassion::output_of(empty);
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
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = ocassion::output_of(config);
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
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = ocassion::output_of(config);
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
        };
        common::save_config(test_config).unwrap();

        let config = Config::load_or_default().unwrap();

        let res = ocassion::output_of(config);
        assert_eq!(res, "🐈");
    });
}
