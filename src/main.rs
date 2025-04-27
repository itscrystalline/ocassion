use std::io::ErrorKind;

use ocassion::{config::Config, errors::ConfigError};

fn main() {
    let config = match Config::load() {
        Ok(conf) => conf,
        Err(ConfigError::Io(err)) if err.kind() == ErrorKind::NotFound => {
            match Config::save_default() {
                Ok(()) => match Config::load() {
                    Ok(conf) => conf,
                    Err(_) => return,
                },
                Err(_) => return,
            }
        }
        _ => return,
    };
    for message in config.dates {
        let Some(msg) = message.try_message() else {
            continue;
        };
        print!("{msg}");
    }
}
