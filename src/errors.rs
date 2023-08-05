//! Contains an enum which handles all errors that may arise in `fetters`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FettersError {
    /// Something fucked up while attempting to access application-specific directories.
    #[error("Failed to access application-dependent directories!")]
    ApplicationError,

    /// Something fucked up while interacting with ChatGPT.
    #[error("ChatGPT error: {0}")]
    ChatGPTError(#[from] chatgpt::err::Error),

    /// Something fucked up while parsing a string to DateTime.
    #[error("Parse error: {0}")]
    ChronoParseError(#[from] chrono::ParseError),

    /// Generic error variant to display whatever string is provided.
    #[error("{0}")]
    GenericError(String),

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

    /// Something fucked up while deserializing TOML.
    #[error("TOML error: {0}")]
    TOMLDeError(#[from] toml::de::Error),

    /// Something fucked up while serializing TOML.
    #[error("TOML error: {0}")]
    TOMLSerError(#[from] toml::ser::Error),
}
