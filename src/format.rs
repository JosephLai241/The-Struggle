use prettytable::*;

/// Set the color of the match within the PrettyTable depending on the job status.
pub fn set_color(status: &str) -> String {
    match status {
        "PENDING" => return "Fbl".to_string(),
        "IN PROGRESS" => return "Fyl".to_string(),
        "OFFER RECEIVED" => return "Fml".to_string(),
        "HIRED" => return "Fgl".to_string(),
        "REJECTED" => return "Frl".to_string(),
        _ => return "".to_string()
    };
}

/// Return a new vector of styled PrettyTable cells to add to the master table.
/// Using `AsRef` trait here so that we can accept all references that can be 
/// converted to `&str` as an argument.
pub fn convert_details<T: AsRef<str>>(job_details: &Vec<T>, style: &str) -> Vec<Cell> {
    let mut pt_row: Vec<Cell> = Vec::new();
    for job in job_details {
        pt_row.push(Cell::new(&job.as_ref()).style_spec(style));
    }

    pt_row
}
