//! Contains all models for job inslghts.

use tabled::Tabled;

/// Contains the name of the field, the total count for the field, as well as the percentage over
/// the total number of jobs or in the target sprint.
#[derive(Debug, Tabled)]
pub struct CountAndPercentage {
    /// The name of the field.
    #[tabled(rename = "Label")]
    pub label: String,
    /// The count of job applications in the field.
    #[tabled(rename = "# of Jobs")]
    pub count: i64,
    /// The percentage of job applications over the number of jobs in the sprint.
    #[tabled(rename = "% in Current Sprint")]
    pub sprint_percentage: String,
    /// The percentage of job applications over the total number of jobs.
    #[tabled(rename = "% Overall")]
    pub overall_percentage: String,
}
