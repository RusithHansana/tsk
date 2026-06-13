enum Priority {
    Low,
    Medium,
    High,
}

enum Status {
    Todo,
    Done,
}

struct Task {
    id: u32,
    title: String,
    priority: Priority,
    status: Status,
    project: Option<String>,
    created_at: String,
}

impl Task {
    fn new(id: u32, title: String, priority: Priority, project: Option<String>) -> Task {
        Task {
            id,
            title,
            priority,
            status: Status::Todo,
            project,
            created_at: String::from("2026-06-13"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task_has_todo_status() {
        let task = Task::new(1, String::from("Tests"), Priority::Medium, None);
        assert!(matches!(task.status, Status::Todo));
    }

    #[test]
    fn new_task_stores_title() {
        let task = Task::new(1, String::from("Tests"), Priority::Medium, None);
        assert_eq!(task.title, "Tests");
    }

    #[test]
    fn new_task_stores_priority() {
        let task = Task::new(1, String::from("Tests"), Priority::High, None);
        assert!(matches!(task.priority, Priority::High));
    }

    #[test]
    fn new_task_with_project() {
        let task = Task::new(
            1,
            String::from("Tests"),
            Priority::Medium,
            Some(String::from("Testing")),
        );

        assert_eq!(task.project, Some(String::from("Testing")));
    }

    #[test]
    fn new_task_without_project_is_none() {
        let task = Task::new(1, String::from("Tests"), Priority::Medium, None);

        assert!(task.project.is_none());
    }
}
