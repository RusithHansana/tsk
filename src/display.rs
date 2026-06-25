use crate::task::*;

pub fn format_task(task: &Task) -> String {
    format!(
        "{:<4} {:<9} {:<7} {:<10} {}",
        task.id(),
        format!("{:?}", task.priority()).to_uppercase(),
        format!("{:?}", task.status()).to_lowercase(),
        task.project().unwrap_or("(none)"),
        task.title()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_row_pads_to_fixed_widths() {
        let task = Task::new(
            1,
            String::from("Build API"),
            Priority::High,
            Some(String::from("backend")),
        );
        let row = format_task(&task);

        assert!(row.starts_with("1   ")); // ID - 4 wide
        assert!(row.contains("HIGH    "));
    }

    #[test]
    fn format_row_handles_no_project() {
        let task = Task::new(1, String::from("Build API"), Priority::High, None);
        let row = format_task(&task);
        assert!(row.contains("(none)"));
    }
}
