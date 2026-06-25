use crate::{display::format_task_list, store::TaskStore, task::*};

pub fn handle_add(
    store: &mut TaskStore,
    title: String,
    priority: Priority,
    project: Option<String>,
) -> String {
    let message = format!("✓ Task added: {}", title);
    store.add_task(title, priority, project);

    message
}

pub fn handle_list(
    store: &TaskStore,
    project: Option<String>,
    priority: Option<Priority>,
    status: Option<Status>,
) -> String {
    let filtered_tasks = store.filter_tasks(project, priority, status);

    format_task_list(filtered_tasks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_add_confirms_with_a_success_message() {
        let mut store = TaskStore::new();
        let result = handle_add(
            &mut store,
            String::from("Learn Rust"),
            Priority::Medium,
            None,
        );

        assert_eq!(result, "✓ Task added: Learn Rust")
    }

    #[test]
    fn handle_list_returns_all_tasks_when_no_filters_given() {
        let mut store = TaskStore::new();

        store.add_task(String::from("Task 1"), Priority::Medium, None);
        store.add_task(String::from("Task 2"), Priority::Medium, None);
        store.add_task(String::from("Task 3"), Priority::Medium, None);

        let results = handle_list(&store, None, None, None);

        assert!(results.contains("Task 1"));
        assert!(results.contains("Task 2"));
        assert!(results.contains("Task 3"));
    }

    #[test]
    fn handle_list_returns_empty_message_when_no_tasks_exist() {
        let store = TaskStore::new();

        let results = handle_list(&store, None, None, None);

        assert_eq!(results, "No tasks found.");
    }

    #[test]
    fn handle_list_successfully_filters_by_project() {
        let mut store = TaskStore::new();

        store.add_task(
            String::from("Task 1"),
            Priority::Medium,
            Some(String::from("backend")),
        );
        store.add_task(
            String::from("Task 2"),
            Priority::Medium,
            Some(String::from("backend")),
        );
        store.add_task(String::from("Task 3"), Priority::Medium, None);

        let results = handle_list(&store, Some(String::from("backend")), None, None);

        assert!(results.contains("Task 1"));
        assert!(results.contains("Task 2"));
    }

    #[test]
    fn handle_list_successfully_filters_by_priority() {
        let mut store = TaskStore::new();

        store.add_task(String::from("Task 1"), Priority::High, None);
        store.add_task(String::from("Task 2"), Priority::Medium, None);
        store.add_task(String::from("Task 3"), Priority::High, None);

        let results = handle_list(&store, None, Some(Priority::High), None);

        assert!(results.contains("Task 1"));
        assert!(results.contains("Task 3"));
    }

    #[test]
    fn handle_list_successfully_filters_by_status() {
        let mut store = TaskStore::new();

        store.add_task(String::from("Task 1"), Priority::Medium, None);
        store.add_task(String::from("Task 2"), Priority::Medium, None);
        store.add_task(String::from("Task 3"), Priority::Medium, None);

        store.mark_done(1).unwrap();

        let results = handle_list(&store, None, None, Some(Status::Done));

        assert!(results.contains("Task 1"));
    }

    #[test]
    fn handle_list_successfully_filters_by_combined_filters() {
        let mut store = TaskStore::new();

        store.add_task(
            String::from("Task 1"),
            Priority::Medium,
            Some(String::from("backend")),
        );
        store.add_task(
            String::from("Task 2"),
            Priority::High,
            Some(String::from("frontend")),
        );
        store.add_task(
            String::from("Task 3"),
            Priority::High,
            Some(String::from("frontend")),
        );

        store.mark_done(3).unwrap();

        let results = handle_list(
            &store,
            Some(String::from("frontend")),
            Some(Priority::High),
            Some(Status::Done),
        );

        assert!(results.contains("Task 3"));
    }

    #[test]
    fn handle_list_returns_empty_message_when_no_matches() {
        let mut store = TaskStore::new();

        store.add_task(String::from("Task 1"), Priority::Medium, None);

        let result = handle_list(&store, Some(String::from("nonexistent")), None, None);

        assert_eq!(result, "No tasks found.")
    }

    #[test]
    fn handle_list_includes_header() {
        let mut store = TaskStore::new();

        store.add_task(String::from("Task 1"), Priority::Medium, None);

        let result = handle_list(&store, None, None, None);

        assert!(result.contains("ID"));
        assert!(result.contains("PRIORITY"));
        assert!(result.contains("STATUS"));
        assert!(result.contains("PROJECT"));
        assert!(result.contains("TITLE"));
    }
}
