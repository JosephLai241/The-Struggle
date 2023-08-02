//! Contains all models pertaining to searching for an existing job application.

use super::job::Job;

/// This struct is used when interacting with any subcommand that allows users to query and filter
/// by job application attributes. This stores the modified `Job` struct and flags indicating
/// whether the job will be displayed depending on the user's filter query.
#[derive(Clone, Debug)]
pub struct SearchResult {
    /// The modified job application.
    pub job: Job,
    /// The painted stint name, if applicable. This is only here because the `Job`'s `stint` field
    /// is of type `Option<i32>`.
    pub painted_stint_name: String,
}

impl SearchResult {
    /// Create a new `SearchResult` from a `Job` and the given stint name.
    pub fn new_with_job_and_stint(job: Job, stint_name: String) -> Self {
        Self {
            job,
            painted_stint_name: stint_name,
        }
    }
}
