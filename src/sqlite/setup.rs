//! Contains functions pertaining to setting up the SQLite instance.

use diesel::{sqlite::SqliteConnection, Connection};
use directories::ProjectDirs;

use crate::errors::FettersError;

/// Open a connnection to the SQLite instance. Creates a new SQLite instance if it does not already
/// exist.
pub fn open_sqlite() -> Result<SqliteConnection, FettersError> {
    let project_directory =
        ProjectDirs::from("", "", "fetters").ok_or(FettersError::ApplicationError)?;

    let sqlite_path = project_directory.data_dir().join("fetters.db3");
    let connection = SqliteConnection::establish(&sqlite_path.to_string_lossy())?;

    Ok(connection)
}
