//! Defining the Job schema that will be used for reading and writing jobs to the
//! spreadsheet.

/// Job schema.
#[derive(Debug, PartialEq)]
pub struct Job {
    pub date: String,
    pub company: String,
    pub title: String,
    pub status: String,
    pub notes: String
}

impl Job {
    /// Create a new Job object using the Job struct.
    pub fn new_job(
        date: String, 
        company: String, 
        title: String, 
        status: String,
        notes: String) -> Job {
            Job {
                date: date, 
                company: company, 
                title: title, 
                status: status,
                notes: notes
            }
    }
}

#[cfg(test)]
mod test_model {
    use super::*;

    #[test]
    fn test_new_job() {
        let test_job = Job {
            date: "07-02-2020 21:09:39".to_string(),
            company: "ECorp".to_string(),
            title: "Security Engineer".to_string(),
            status: "HIRED".to_string(),
            notes: "My name is Elliot".to_string()
        };

        assert_eq!(Job::new_job(
            "07-02-2020 21:09:39".to_string(),
            "ECorp".to_string(),
            "Security Engineer".to_string(),
            "HIRED".to_string(),
            "My name is Elliot".to_string()), test_job);
    }
}
