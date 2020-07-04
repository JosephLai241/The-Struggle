//! Formatting PrettyTable output.

use prettytable::*;

/// Set the color of the match within the PrettyTable depending on the job
/// application status.
pub fn set_color(status: &str) -> String {
    match status {
        "PENDING" => return "bFbl".to_string(),
        "IN PROGRESS" => return "bFyl".to_string(),
        "OFFER RECEIVED" => return "bFml".to_string(),
        "HIRED" => return "bFgl".to_string(),
        "REJECTED" => return "bFrl".to_string(),
        _ => return "".to_string()
    };
}

/// Return a new vector of styled PrettyTable cells to add to the master table.
/// Using `AsRef` trait as an argument here so that we can accept all references 
/// that can be converted to `&str`.
pub fn convert_details<T: AsRef<str>>(job_details: &Vec<T>, style: &str) -> Vec<Cell> {
    let mut pt_row: Vec<Cell> = Vec::new();
    
    for job in job_details {
        pt_row.push(Cell::new(&job.as_ref()).style_spec(style));
    }

    pt_row
}

/// Set the PrettyTable format for all information tables.
pub fn format_table() -> format::TableFormat {
    format::FormatBuilder::new()
        .borders('|')
        .padding(1, 3)
        .separators(
            &[format::LinePosition::Bottom, format::LinePosition::Intern], 
            format::LineSeparator::new('-', '+', '+', '+'))
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Title], 
            format::LineSeparator::new('=', '=', '+', '+'))
        .build()
}

#[cfg(test)]
mod test_format {
    use super::*;

    #[test]
    fn test_set_color() {
        assert_eq!(set_color("PENDING"), "bFbl");
        assert_eq!(set_color("IN PROGRESS"), "bFyl");
        assert_eq!(set_color("OFFER RECEIVED"), "bFml");
        assert_eq!(set_color("HIRED"), "bFgl");
        assert_eq!(set_color("REJECTED"), "bFrl");
        assert_eq!(set_color("should return empty string"), "");
    }

    #[test]
    fn test_convert_details() {
        let job_details = vec![
            "07-02-2020 21:09:39",
            "ECorp",
            "Security Engineer",
            "HIRED",
            "My name is Elliot"
        ];
        let style = "Fbl";

        let results = vec![
            Cell::new(&job_details[0]).style_spec(style),
            Cell::new(&job_details[1]).style_spec(style),
            Cell::new(&job_details[2]).style_spec(style),
            Cell::new(&job_details[3]).style_spec(style),
            Cell::new(&job_details[4]).style_spec(style),
        ];

        assert_eq!(convert_details(&job_details, style), results);
    }
}
