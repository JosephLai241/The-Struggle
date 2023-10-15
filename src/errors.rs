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

    /// Something fucked up while connecting to SQLite via `diesel`.
    #[error("Diesel connection error: {0}")]
    DieselConnectionError(#[from] diesel::ConnectionError),

    /// Something fucked up while executing a SQL query via `diesel`.
    #[error("Diesel error: {0}")]
    DieselError(#[from] diesel::result::Error),

    /// Generic error variant to display whatever string is provided.
    #[error("{0}")]
    GenericError(String),

    /// Something fucked up while performing I/O operations.
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    /// Something fucked up while inquiring the user for an input.
    #[error("Inquire error: {0}")]
    InquireError(#[from] inquire::InquireError),

    /// Something fucked up while attempting to parse the job ID from the `Select` menu's selected
    /// option.
    #[error("Job ID parse error: Failed to capture the job ID from the selected job!")]
    JobIDCaptureError,

    /// Something fucked up when attempting to parse a string into an integer.
    #[error("Failed to parse the stringified job ID to an integer! {0}")]
    JobIdParseError(std::num::ParseIntError),

    /// Something fucked up when attempting to select a particular job.
    #[error("Job selection error: {0}")]
    JobSelectionError(String),

    /// Something fucked up while interacting with system paths.
    #[error("{0}")]
    PathError(String),

    /// Something fucked up while performing Regex operations.
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    /// Something fucked up while deserializing TOML.
    #[error("TOML error: {0}")]
    TOMLDeError(#[from] toml::de::Error),

    /// Something fucked up while serializing TOML.
    #[error("TOML error: {0}")]
    TOMLSerError(#[from] toml::ser::Error),
}

impl From<std::num::ParseIntError> for FettersError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::JobIdParseError(value)
    }
}
