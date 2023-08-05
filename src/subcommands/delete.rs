//! Contains all functionality pertaining to the `delete` subcommand.

use diesel::SqliteConnection;

use crate::errors::FettersError;

/// Delete a job from the SQLite database.
pub fn delete_job(connection: &mut SqliteConnection, query: String) -> Result<(), FettersError> {
    unimplemented!()
}
