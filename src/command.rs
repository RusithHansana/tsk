use crate::{storage::get_next_id, task::*};

fn add_task(tasks: &mut Vec<Task>, title: String, priority: Priority, project: Option<String>) {
    let next_id = get_next_id(&tasks);
    let new_task = Task::new(next_id, title, priority, project);
    tasks.push(new_task);
}

fn filter_tasks(
    tasks: &[Task],
    project: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
) -> Vec<&Task> {
    tasks
        .iter()
        .filter(|t| {
            let project_matches = match &project {
                Some(p) => t.project().as_deref() == Some(p.as_str()),
                None => true,
            };

            let status_matches = match &status {
                Some(s) => t.status() == *s,
                None => true,
            };

            let priority_matches = match &priority {
                Some(p) => t.priority() == *p,
                None => true,
            };

            project_matches && status_matches && priority_matches
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tasks() -> Vec<Task> {
        vec![
            Task::new(
                1,
                String::from("Build API"),
                Priority::High,
                Some(String::from("backend")),
            ),
            Task::new(
                2,
                String::from("Write docs"),
                Priority::Medium,
                Some(String::from("backend")),
            ),
            Task::new(
                3,
                String::from("Read Rust book"),
                Priority::Low,
                Some(String::from("learning")),
            ),
        ]
    }

    #[test]
    fn add_task_increases_count() {
        let mut tasks: Vec<Task> = vec![];

        add_task(&mut tasks, String::from("New Task"), Priority::Medium, None);

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title(), "New Task");
    }

    #[test]
    fn add_task_assigns_sequential_ids() {
        let mut tasks: Vec<Task> = vec![];

        add_task(&mut tasks, String::from("first"), Priority::Medium, None);
        add_task(&mut tasks, String::from("second"), Priority::Medium, None);

        assert_eq!(tasks[0].id(), 1);
        assert_eq!(tasks[1].id(), 2);
    }

    #[test]
    fn add_task_status_is_always_todo() {
        let mut tasks: Vec<Task> = vec![];

        add_task(&mut tasks, String::from("first"), Priority::Medium, None);

        assert!(matches!(tasks[0].status(), Status::Todo));
    }

    #[test]
    fn filter_by_project() {
        let tasks = sample_tasks();
        let results = filter_tasks(&tasks, Some(String::from("backend")), None, None);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn filter_by_status_todo() {
        let mut tasks = sample_tasks();
        tasks[0].set_status(Status::Done);
        let results = filter_tasks(&tasks, None, Some(Status::Todo), None);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn filter_by_priority() {
        let tasks = sample_tasks();
        let results = filter_tasks(&tasks, None, None, Some(Priority::High));

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title(), "Build API");
    }

    #[test]
    fn filter_combined_project_and_status() {
        let mut tasks = sample_tasks();
        tasks[1].set_status(Status::Done);
        let results = filter_tasks(
            &tasks,
            Some(String::from("backend")),
            Some(Status::Todo),
            None,
        );

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title(), "Build API");
    }

    #[test]
    fn filter_no_match_returns_empty() {
        let tasks = sample_tasks();
        let results = filter_tasks(&tasks, Some(String::from("nonexistent")), None, None);
        assert!(results.is_empty());
    }
}
