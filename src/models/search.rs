//! Contains all models pertaining to searching for an existing job application.

use super::job::Job;

/// This struct is used when interacting with any subcommand that allows users to query and filter
/// by job application attributes. This stores the modified `Job` struct and flags indicating
/// whether the job will be displayed depending on the user's filter query.
#[derive(Clone, Debug)]
pub struct SearchResult {
    /// The modified job application.
    pub job: Job,
    /// Whether the pattern matched the job's link.
    pub matched_link: bool,
    /// Whether the pattern matched the job's notes.
    pub matched_notes: bool,
    /// Whether the pattern matched the job's status.
    pub matched_status: bool,
    /// Whether the pattern matched the job's stint.
    pub matched_stint: bool,
    /// Whether the pattern matched the job's title.
    pub matched_title: bool,
    /// The painted stint name, if applicable. This is only here because the `Job`'s `stint` field
    /// is of type `Option<i32>`.
    pub painted_stint_name: String,
}

impl SearchResult {
    /// Create a new `SearchResult` from a `Job` and the given stint name.
    pub fn new_with_job_and_stint(job: Job, stint_name: String) -> Self {
        Self {
            job: job.clone(),
            matched_link: false,
            matched_notes: false,
            matched_status: false,
            matched_stint: false,
            matched_title: false,
            painted_stint_name: stint_name,
        }
    }
}
