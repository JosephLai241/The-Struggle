#[derive(Debug)]
pub struct Job {
    // Job attributes
    pub date: String,
    pub company: String,
    pub title: String,
    pub status: String,
    pub notes: String
}

impl Job {
    // Create new Job
    pub fn new_job(date: String, company: String, title: String, status: String,
        notes: String) -> Job {
            Job {date: date, company: company, title: title, status: status,
                notes: notes}
    }
}
