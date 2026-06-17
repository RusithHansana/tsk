use crate::{storage::TaskStore, task::*};

fn add_task(store: &mut TaskStore, title: String, priority: Priority, project: Option<String>) {
    store.add_task(title, priority, project);
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

fn mark_done(tasks: &mut [Task], id: u32) -> Result<(), String> {
    let task = match tasks.iter_mut().find(|t| t.id() == id) {
        Some(t) => t,
        None => return Err(format!("Could not find a task with the id: {id}")),
    };

    task.set_status(Status::Done);

    Ok(())
}

fn delete_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    let index = match tasks.iter().position(|t| t.id() == id) {
        Some(i) => i,
        None => return Err(format!("Could not find a task with the id: {id}")),
    };

    tasks.remove(index);

    Ok(())
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
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("New Task"), Priority::Medium, None);

        assert_eq!(task_store.tasks().len(), 1);
        assert_eq!(task_store.tasks()[0].title(), "New Task");
    }

    #[test]
    fn add_task_assigns_sequential_ids() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("first"), Priority::Medium, None);
        task_store.add_task(String::from("second"), Priority::Medium, None);

        assert_eq!(task_store.tasks()[0].id(), 1);
        assert_eq!(task_store.tasks()[1].id(), 2);
    }

    #[test]
    fn add_task_status_is_always_todo() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("first"), Priority::Medium, None);

        assert!(matches!(task_store.tasks()[0].status(), Status::Todo));
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

    #[test]
    fn mark_done_updates_status() {
        let mut tasks = vec![Task::new(1, String::from("Test"), Priority::Medium, None)];

        let result = mark_done(&mut tasks, 1);

        assert!(result.is_ok());
        assert!(matches!(tasks[0].status(), Status::Done));
    }

    #[test]
    fn mark_done_returns_error_for_missing_id() {
        let mut tasks = sample_tasks();

        let result = mark_done(&mut tasks, 99);

        assert!(result.is_err());
    }

    #[test]
    fn mark_done_does_not_affect_other_tasks() {
        let mut tasks = sample_tasks();

        mark_done(&mut tasks, 1).unwrap();

        assert!(matches!(tasks[1].status(), Status::Todo));
        assert!(matches!(tasks[2].status(), Status::Todo));
    }

    #[test]
    fn delete_task_removes_it() {
        let mut tasks = sample_tasks();

        let result = delete_task(&mut tasks, 2);

        assert!(result.is_ok());
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id(), 1);
    }

    #[test]
    fn delete_task_returns_error_for_missing_id() {
        let mut tasks = sample_tasks();

        let result = delete_task(&mut tasks, 99);

        assert!(result.is_err());
    }

    #[test]
    fn deleted_id_is_not_reused() {
        let mut tasks = sample_tasks();
        delete_task(&mut tasks, 3).unwrap();

        // add_task(&mut tasks, String::from("New Task"), Priority::Medium, None);

        // since we removed the `3` id the new task should have the id `4`
        assert_eq!(tasks.last().unwrap().id(), 4);
    }
}
