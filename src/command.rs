use crate::{store::TaskStore, task::*};

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
