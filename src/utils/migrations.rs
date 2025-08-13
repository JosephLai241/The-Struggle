//! Contains a utility function for running migrations on startup.

use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use owo_colors::OwoColorize;

use crate::errors::FettersError;

/// Migrations to include with `fetters`. These migrations will be run on startup.
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

/// Run the SQLite migrations specified in the `migrations/` directory.
pub fn run_migrations(connection: &mut SqliteConnection) -> Result<(), FettersError> {
    if let Err(error) = connection.run_pending_migrations(MIGRATIONS) {
        println!(
            "{}",
            format!("FAILED TO RUN SQLITE MIGRATIONS: {}", error.to_string())
                .red()
                .bold()
        );

        return Err(FettersError::MigrationFailure);
    }

    Ok(())
}
