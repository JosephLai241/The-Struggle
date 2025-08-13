//! Contains all functionality pertaining to interacting with SQLite.

use diesel::Connection;
use diesel::sqlite::SqliteConnection;

use crate::errors::FettersError;

/// Contains all functionality pertaining to interacting with the SQLite database.
pub struct Database {
    /// The SQLite connection.
    pub connection: SqliteConnection,
}

impl Database {
    /// Create a new connection to the SQLite database.
    pub fn new_connection(db_path: &str) -> Result<Database, FettersError> {
        let connection = SqliteConnection::establish(db_path)?;
        Ok(Database { connection })
    }
}
