use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBRError {
    #[error("config error {0}")]
    ConfigError(String),
}
