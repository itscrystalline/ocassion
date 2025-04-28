use crate::errors::ConfigError;
use std::{
    collections::HashSet,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use chrono::{Month, Weekday};
use serde::{Deserialize, Serialize};

pub static CONFIG_VAR: &str = "OCASSION_CONFIG";
pub static CONFIG_FILE_NAME: &str = "ocassions.json";

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
    pub year: Option<HashSet<i32>>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DayOf {
    #[serde(rename = "week")]
    Week(HashSet<Weekday>),
    #[serde(rename = "month")]
    Month(HashSet<u8>),
}

impl Config {
    pub fn load_or_default() -> Result<Config, ConfigError> {
        match Config::load() {
            Ok(conf) => Ok(conf),
            Err(ConfigError::Io(err)) if err.kind() == ErrorKind::NotFound => {
                match Config::save_default() {
                    Ok(()) => Config::load(),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn load() -> Result<Config, ConfigError> {
        let file_path_str = std::env::var(CONFIG_VAR).unwrap_or(format!(
            "{}/{}",
            dirs::config_dir()
                .ok_or(ConfigError::UndeterminableConfigLocation)?
                .to_string_lossy(),
            CONFIG_FILE_NAME
        ));
        Self::load_from(&PathBuf::from(file_path_str))
    }

    fn load_from(path: &Path) -> Result<Config, ConfigError> {
        let contents = std::fs::read_to_string(path)?;

        Ok(serde_json::from_str(&contents)?)
    }

    fn save_default() -> Result<(), ConfigError> {
        let file_path_str = std::env::var(CONFIG_VAR).unwrap_or(format!(
            "{}/{}",
            dirs::config_dir()
                .ok_or(ConfigError::UndeterminableConfigLocation)?
                .to_string_lossy(),
            CONFIG_FILE_NAME
        ));
        Self::save_default_to(&PathBuf::from(file_path_str))
    }

    fn save_default_to(path: &Path) -> Result<(), ConfigError> {
        let json = serde_json::to_value(Config::default())?;
        let json_pretty = serde_json::to_string_pretty(&json)?;
        std::fs::write(path, json_pretty)?;
        Ok(())
    }
}

#[cfg(test)]
mod unit_tests {
    use std::env::current_dir;

    use map_macro::hash_set;

    use super::*;

    fn with_var<F: FnOnce()>(run: F) {
        let dir = current_dir().unwrap();
        let dir = dir.to_string_lossy();
        let file = format!("{dir}/{CONFIG_FILE_NAME}");
        temp_env::with_var(CONFIG_VAR, Some(file.clone()), move || {
            run();
            _ = std::fs::remove_file(file);
        });
    }

    #[test]
    /// A default `Config` should be created.
    fn serialize_default() {
        let test_config = Config::default();
        let json = serde_json::to_string(&test_config).unwrap();

        let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
        let dates = decoded["dates"].as_array().unwrap();
        assert!(dates.is_empty());
    }
    #[test]
    /// A properly made `Config` should serialize to a file properly.
    fn serialize_default_to_file() {
        with_var(|| {
            let save_res = Config::save_default();
            assert!(save_res.is_ok());
            let json = std::fs::read_to_string(std::env::var(CONFIG_VAR).unwrap()).unwrap();

            let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
            let dates = decoded["dates"].as_array().unwrap();
            assert!(dates.is_empty());
        });
    }
    #[test]
    /// A properly made `Config` should also deserialize properly.
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
    #[test]
    /// A properly made `Config` should also deserialize properly.
    fn deserialize_from_file() {
        with_var(|| {
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

            let json = serde_json::to_string(&test_config).unwrap();
            std::fs::write(std::env::var(CONFIG_VAR).unwrap(), &json).unwrap();

            let decoded_config = Config::load().unwrap();
            assert_eq!(test_config, decoded_config);
        });
    }
    #[test]
    /// A properly made `Config` should also deserialize properly.
    fn deserialize_from_broken_file() {
        with_var(|| {
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

            let mut json = serde_json::to_string(&test_config).unwrap();
            json.push_str("lalalalalalalal mreow :3");
            std::fs::write(std::env::var(CONFIG_VAR).unwrap(), &json).unwrap();

            let decoded_config = Config::load();
            assert!(matches!(decoded_config, Err(ConfigError::Deserialize(_))));
        });
    }

    #[test]
    fn deserialize_unreadable() {
        with_var(|| {
            // no written config
            let decoded_config = Config::load();
            assert!(matches!(decoded_config, Err(ConfigError::Io(_))));
        });
    }
}
