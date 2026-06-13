use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Todo,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
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

    #[test]
    fn task_serializes_to_json() {
        let task = Task::new(1, String::from("Tests"), Priority::Medium, None);

        let json = serde_json::to_string(&task).unwrap();

        assert!(json.contains("\"title\":\"Tests\""));
        assert!(json.contains("\"status\":\"Todo\""));
    }

    #[test]
    fn task_deserializes_from_json() {
        let json = r#"{
            "id": 1,
            "title": "Learn Rust",
            "status": "Todo",
            "priority": "Medium",
            "project": null,
            "created_at": "2026-06-08"
        }"#;

        let task: Task = serde_json::from_str(json).unwrap();

        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Learn Rust");
    }

    #[test]
    fn vec_of_tasks_round_trips() {
        let tasks = vec![
            Task::new(1, String::from("Task one"), Priority::Medium, None),
            Task::new(
                2,
                String::from("Task two"),
                Priority::Low,
                Some(String::from("work")),
            ),
        ];

        let json = serde_json::to_string(&tasks).unwrap();
        let loaded: Vec<Task> = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].title, "Task one");
        assert_eq!(loaded[1].project, Some(String::from("work")));
    }
}
