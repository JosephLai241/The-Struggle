//! Functions for formatting PrettyTable output.

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

/// Return the PrettyTable format for all information tables.
pub fn format_table() -> format::TableFormat {
    format::FormatBuilder::new()
        .borders('|')
        .column_separator('|')
        .padding(1, 1)
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
        assert_eq!(set_color("PENDING"), "Fbl");
        assert_eq!(set_color("IN PROGRESS"), "Fyl");
        assert_eq!(set_color("OFFER RECEIVED"), "Fml");
        assert_eq!(set_color("HIRED"), "Fgl");
        assert_eq!(set_color("REJECTED"), "Frl");
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
