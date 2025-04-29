use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("this error should never fire")]
    Unknown,
    #[error("malformed config file")]
    Malformed,
    #[error("cannot determine config dir, pass $OCCASION_CONFIG directly")]
    UndeterminableConfigLocation,
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("cannot parse: {0}")]
    Deserialize(#[from] serde_json::Error),
}
