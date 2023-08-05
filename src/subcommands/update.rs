//! Contains all functionality pertaining to the `update` subcommand.

use diesel::SqliteConnection;

use crate::errors::FettersError;

/// Update a job in the SQLite database.
pub fn update_job(connection: &mut SqliteConnection, query: String) -> Result<(), FettersError> {
    unimplemented!()
}
