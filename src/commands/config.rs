//! Contains functions called by the CLI when editing the configuration file.

use crate::{config::configuration::Config, errors::FettersError};

/// Open the configuration file in the default `$EDITOR`.
pub fn edit_config() -> Result<(), FettersError> {
    edit::edit_file(Config::get_config_dir_path()?.join("fetters.toml"))?;
    Ok(())
}
