//! Contains functions pertaining to setting up the SQLite instance.

use directories::ProjectDirs;
use rusqlite::{params, Connection};

use crate::errors::FettersError;

/// Open a connnection to the SQLite instance. Creates a new SQLite instance if it does not already
/// exist. Also creates the following tables if they do not already exist:
/// - `job_data` - Contains all job listings.
/// - `stints` - Contains all stints (application phases).
pub fn open_sqlite() -> Result<Connection, FettersError> {
    match ProjectDirs::from("", "", "fetters") {
        Some(project_directory) => {
            let sqlite_path = project_directory.data_dir().join("fetters.db3");
            let connection = Connection::open(sqlite_path)?;

            connection.execute(
                "CREATE TABLE IF NOT EXISTS stints (
                    id INTEGER PRIMARY KEY,
                    date_added TEXT NOT NULL,
                    stint TEXT NOT NULL
                )",
                params![],
            )?;

            connection.execute(
                "CREATE TABLE IF NOT EXISTS job_data (
                    id INTEGER PRIMARY KEY,
                    company TEXT NOT NULL,
                    date_added TEXT NOT NULL,
                    link TEXT,
                    notes TEXT,
                    status TEXT NOT NULL,
                    stint INTEGER,
                    title TEXT NOT NULL,
                    FOREIGN KEY (stint) REFERENCES stints (id)
                )",
                params![],
            )?;

            Ok(connection)
        }
        None => Err(FettersError::ApplicationError),
    }
}
