//! Contains an enum which handles all errors that may arise in `fetters`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FettersError {
    /// Something fucked up while attempting to access application-specific directories.
    #[error("Failed to access application-dependent directories!")]
    ApplicationError,

    /// Something fucked up while interacting with the application configuration.
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    /// Something fucked up while performing I/O operations.
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    /// Something fucked up while inquiring the user for an input.
    #[error("Inquire error: {0}")]
    InquireError(#[from] inquire::InquireError),

    /// Something fucked up while interacting with system paths.
    #[error("{0}")]
    PathError(String),

    /// Something fucked up while performing Regex operations.
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    /// Something fucked up while interacting with SQLite.
    #[error("SQLite error: {0}")]
    SQLiteError(#[from] rusqlite::Error),

    /// Something fucked up while interacting with TOML.
    #[error("TOML error: {0}")]
    TOMLError(#[from] toml::ser::Error),
}
