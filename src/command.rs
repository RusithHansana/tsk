use crate::{store::TaskStore, task::*};

// TODO:: Refractor from here onwards
fn filter_tasks(
    store: &TaskStore,
    project: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
) -> Vec<&Task> {
    store
        .tasks()
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

fn mark_done(store: &mut TaskStore, id: u32) -> Result<(), String> {
    let task = match store.tasks_mut().iter_mut().find(|t| t.id() == id) {
        Some(t) => t,
        None => return Err(format!("Could not find a task with the id: {id}")),
    };

    task.set_status(Status::Done);

    Ok(())
}

fn delete_task(store: &mut TaskStore, id: u32) -> Result<(), String> {
    let index = match store.tasks().iter().position(|t| t.id() == id) {
        Some(i) => i,
        None => return Err(format!("Could not find a task with the id: {id}")),
    };

    store.tasks_mut().remove(index);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_project() {
        let mut task_store = TaskStore::new();

        task_store.add_task(
            String::from("Test 1"),
            Priority::Medium,
            Some(String::from("backend")),
        );
        task_store.add_task(
            String::from("Test 2"),
            Priority::Medium,
            Some(String::from("backend")),
        );
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        let results = filter_tasks(&task_store, Some(String::from("backend")), None, None);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn filter_by_status_todo() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);

        task_store.tasks_mut()[0].set_status(Status::Done);

        let results = filter_tasks(&task_store, None, Some(Status::Todo), None);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn filter_by_priority() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::High, None);

        let results = filter_tasks(&task_store, None, None, Some(Priority::High));

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title(), "Test 3");
    }

    #[test]
    fn filter_combined_project_and_status() {
        let mut task_store = TaskStore::new();

        task_store.add_task(
            String::from("Test 1"),
            Priority::Medium,
            Some(String::from("backend")),
        );
        task_store.add_task(
            String::from("Test 2"),
            Priority::Medium,
            Some(String::from("backend")),
        );
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        task_store.tasks_mut()[1].set_status(Status::Done);

        let results = filter_tasks(
            &task_store,
            Some(String::from("backend")),
            Some(Status::Todo),
            None,
        );

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title(), "Test 1");
    }

    #[test]
    fn filter_no_match_returns_empty() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        let results = filter_tasks(&task_store, Some(String::from("nonexistent")), None, None);
        assert!(results.is_empty());
    }

    #[test]
    fn mark_done_updates_status() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test Task"), Priority::Medium, None);

        let result = mark_done(&mut task_store, 1);

        assert!(result.is_ok());
        assert!(matches!(task_store.tasks()[0].status(), Status::Done));
    }

    #[test]
    fn mark_done_returns_error_for_missing_id() {
        let mut task_store = TaskStore::new();

        let result = mark_done(&mut task_store, 99);

        assert!(result.is_err());
    }

    #[test]
    fn mark_done_does_not_affect_other_tasks() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        mark_done(&mut task_store, 1).unwrap();

        assert!(matches!(task_store.tasks()[1].status(), Status::Todo));
        assert!(matches!(task_store.tasks()[2].status(), Status::Todo));
    }

    #[test]
    fn delete_task_removes_it() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        let result = delete_task(&mut task_store, 2);

        assert!(result.is_ok());
        assert_eq!(task_store.tasks().len(), 2);
        assert_eq!(task_store.tasks()[0].id(), 1);
    }

    #[test]
    fn delete_task_returns_error_for_missing_id() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        let result = delete_task(&mut task_store, 99);

        assert!(result.is_err());
    }

    #[test]
    fn deleted_id_is_not_reused() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        delete_task(&mut task_store, 3).unwrap();

        task_store.add_task(String::from("Test 4"), Priority::Medium, None);

        // since we removed the `3` id the new task should have the id `4`
        assert_eq!(task_store.tasks().last().unwrap().id(), 4);
    }
}
