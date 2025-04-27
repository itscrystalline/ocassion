use crate::errors::ConfigError;
use std::{
    collections::HashSet,
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
    pub fn load() -> Result<Config, ConfigError> {
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

    pub fn save_default() -> Result<(), ConfigError> {
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
        let json_str = json.to_string();
        std::fs::write(path, json_str)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use map_macro::hash_set;

    use super::*;

    fn setup() {
        let pwd = std::env::current_dir().unwrap();
        let pwd = pwd.to_str().unwrap();
        unsafe {
            // SAFETY: this app is single threaded.
            // https://doc.rust-lang.org/std/env/fn.set_var.html#safety
            std::env::set_var(CONFIG_VAR, format!("{pwd}/{CONFIG_FILE_NAME}"));
        }
    }
    fn teardown() {
        let Ok(config_path) = std::env::var(CONFIG_VAR) else {
            return;
        };
        _ = std::fs::remove_file(config_path);
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
        setup();

        let save_res = Config::save_default();
        assert!(save_res.is_ok());
        let json = std::fs::read_to_string(std::env::var(CONFIG_VAR).unwrap()).unwrap();

        let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
        let dates = decoded["dates"].as_array().unwrap();
        assert!(dates.is_empty());

        teardown();
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
        setup();

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

        teardown();
    }
}
