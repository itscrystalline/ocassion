use config::{Config, MultipleBehavior};

pub mod config;
pub mod errors;
pub mod formatting;
pub mod time;

pub fn output_of(config: &Config) -> String {
    let behavior = &config.multiple_behavior;

    let outputs: Vec<String> = config
        .dates
        .iter()
        .filter_map(|message| message.try_message(config.week_start_day))
        .collect();
    match behavior {
        Some(MultipleBehavior::All { seperator }) => outputs
            .into_iter()
            .reduce(|mut str, curr| {
                str.push_str(seperator);
                str.push_str(&curr);
                str
            })
            .unwrap_or_default(),
        None => outputs
            .into_iter()
            .reduce(|mut str, curr| {
                str.push_str(&curr);
                str
            })
            .unwrap_or_default(),
        Some(MultipleBehavior::First) => outputs.first().map_or("", |v| v).to_string(),
        Some(MultipleBehavior::Last) => outputs.last().map_or("", |v| v).to_string(),
        Some(MultipleBehavior::Random) => outputs[fastrand::usize(..outputs.len())].clone(),
    }
}
