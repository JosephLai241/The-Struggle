//! Contains an enum encapsulating all errors that may occur while using `fetters`.

use thiserror::Error;

/// Contains variants for errors that may be raised throughout this program.
#[derive(Debug, Error)]
pub enum FettersError {
    /// Something went wrong when trying to get the application-specific directories.
    #[error("Could not retrieve system application directories!")]
    ApplicationError,

    /// Something went wrong when attempting to get the result after creating or updating a job in
    /// SQLite.
    #[error("Diesel query result error: {0}")]
    DieselResultError(#[from] diesel::result::Error),

    /// An IO error occurred.
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    /// Something went wrong when using the `Inquire` crate for prompts.
    #[error("Inquire error: {0}")]
    InquireError(#[from] inquire::error::InquireError),

    /// Something fucked up when running the SQLite migrations with `diesel_migrations`.
    #[error("Failed to run migrations!")]
    MigrationFailure,

    /// This error may be raised if the user tries to update or delete a job, but no job
    /// applications have been tracked for the current sprint.
    #[error("No job applications tracked for the current sprint [{0}]")]
    NoJobsAvailable(String),

    /// This error may be raised if the user attempts to create two new sprints in the same day,
    /// causing a sprint naming conflict (all sprint names should be unique).
    #[error("There is already a sprint with name {0}. Try renaming the sprint.")]
    SprintNameConflict(String),

    /// Something went wrong when trying to connect to the SQLite database.
    #[error("Failed to connect to SQLite database: {0}")]
    SQLiteConnectionError(#[from] diesel::ConnectionError),

    /// Something went wrong when deserializing TOML.
    #[error("TOML deserialization error: {0}")]
    TOMLDeserializationError(#[from] toml::de::Error),

    /// Something went wrong when serializing TOML.
    #[error("TOML serialization error: {0}")]
    TOMLSerializationError(#[from] toml::ser::Error),

    /// An unknown error occurred.
    #[error("{0}")]
    UnknownError(String),
}
