use std::fmt::{Display, Formatter};

/// Shared error type used by the application during bootstrap and command wiring.
#[derive(Debug)]
pub enum AppError {
    Config(String),
    Io(std::io::Error),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(msg) => write!(f, "configuration error: {msg}"),
            Self::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<config::ConfigError> for AppError {
    fn from(value: config::ConfigError) -> Self {
        Self::Config(value.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::Config(value.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Config(value.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
