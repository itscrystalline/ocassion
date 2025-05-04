use occasion::{
    config::{CONFIG_FILE_NAME, CONFIG_VAR, Config},
    errors::ConfigError,
};
use std::env::temp_dir;

pub fn with_config_var<F: FnOnce()>(run: F) {
    let mut dir = temp_dir();
    dir.push(format!(
        "occasion-test-{}",
        fastrand::i128(i128::MIN..i128::MAX)
    ));
    let dir_str = dir.to_string_lossy();
    let file = format!("{dir_str}/{CONFIG_FILE_NAME}");
    _ = std::fs::create_dir_all(&dir);
    temp_env::with_var(CONFIG_VAR, Some(file.clone()), move || {
        run();
    });
    _ = std::fs::remove_dir_all(&dir);
}

pub fn save_config(config: Config) -> Result<(), ConfigError> {
    let file_path_str = std::env::var(CONFIG_VAR).unwrap_or(format!(
        "{}/{}",
        dirs::config_dir()
            .ok_or(ConfigError::UndeterminableConfigLocation)?
            .to_string_lossy(),
        CONFIG_FILE_NAME
    ));
    let json = serde_json::to_string_pretty(&config)?;
    std::fs::write(file_path_str, json)?;
    Ok(())
}

//pub fn date(year: i32, month: u32, day: u32) -> DateTime<FixedOffset> {
//    Local
//        .with_ymd_and_hms(year, month, day, 0, 0, 0)
//        .unwrap()
//        .fixed_offset()
//}
