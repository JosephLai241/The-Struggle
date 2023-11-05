//! Contains miscellaneous utilities for subcommands.

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// The regex expression used for extracting the job ID from the square brackets in the
    /// `Select` menu's option list.
    pub static ref JOB_ID_REGEX: Regex = Regex::new(r"\[(\d+)\]")
        .expect("FAILED TO CREATE THE REGEX EXPRESSION TO MATCH JOB IDS FROM THE SELECTION MENU!");
}
