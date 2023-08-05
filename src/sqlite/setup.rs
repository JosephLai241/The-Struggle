//! Contains functions pertaining to setting up the SQLite instance.

use diesel::{sqlite::SqliteConnection, Connection};
use directories::ProjectDirs;

use crate::errors::FettersError;

/// Open a connnection to the SQLite instance. Creates a new SQLite instance if it does not already
/// exist. Also creates the following tables if they do not already exist:
/// - `job_data` - Contains all job listings.
/// - `stints` - Contains all stints (application phases).
pub fn open_sqlite() -> Result<SqliteConnection, FettersError> {
    match ProjectDirs::from("", "", "fetters") {
        Some(project_directory) => {
            let sqlite_path = project_directory.data_dir().join("fetters.db3");
            let connection = SqliteConnection::establish(&sqlite_path.to_string_lossy())?;

            Ok(connection)
        }
        None => Err(FettersError::ApplicationError),
    }
}
