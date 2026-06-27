use crate::task::*;

fn format_task(task: &Task) -> String {
    format!(
        "{:<4} {:<12} {:<9} {:<7} {:<10} {}",
        task.id(),
        task.created_at(),
        format!("{:?}", task.priority()).to_uppercase(),
        format!("{:?}", task.status()).to_lowercase(),
        task.project().unwrap_or("(none)"),
        task.title()
    )
}

pub fn format_task_list(tasks: Vec<&Task>) -> String {
    if tasks.is_empty() {
        return String::from("No tasks found.");
    }

    let mut list = format!(
        "{:<4} {:<12} {:<9} {:<7} {:<10} {}",
        "ID", "CREATED AT", "PRIORITY", "STATUS", "PROJECT", "TITLE"
    );

    list.push('\n');

    for task in tasks {
        list.push_str(&format_task(task));
        list.push('\n');
    }

    list
}

pub fn format_summary(summary: &Summary) -> String {
    let mut output = String::new();

    let total = format!("{:<10} {}\n", "Total:", summary.total);
    let todo = format!("{:<10} {}\n", "Todo:", summary.todo);
    let done = format!("{:<10} {}\n", "Done:", summary.done);

    output.push_str(&total);
    output.push_str(&todo);
    output.push_str(&done);

    output.push_str("\nBy Project:\n");

    for (project, count) in &summary.by_project {
        output.push_str(&format!("  {:<10} {}\n", project, count));
    }

    output.push_str("\nBy Priority:\n");

    for (priority, count) in &summary.by_priority {
        output.push_str(&format!("  {:<10} {}\n", format!("{:?}", priority), count));
    }

    output
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
