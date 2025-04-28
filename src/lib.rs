use chrono::{DateTime, FixedOffset};
use config::Config;

pub mod config;
pub mod errors;
pub mod time;

pub fn output_of(config: Config) -> String {
    let mut out = String::new();
    for message in config.dates {
        let Some(msg) = message.try_message() else {
            continue;
        };
        out.push_str(&msg);
    }
    out
}
