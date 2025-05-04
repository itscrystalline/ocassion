use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("this error should never fire")]
    Unknown,
    #[error("the path in $OCCASION_CONFIG or one of it's imports is not a file path")]
    NotAFile,
    #[error("malformed config file")]
    Malformed,
    #[error("cannot determine config dir, pass $OCCASION_CONFIG directly")]
    UndeterminableConfigLocation,
    #[error("max import depth reached (3)")]
    MaxRecursionDepth,
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("cannot parse: {0}")]
    Deserialize(#[from] serde_json::Error),
}
