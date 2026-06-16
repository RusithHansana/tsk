use crate::{storage::get_next_id, task::*};

fn add_task(tasks: &mut Vec<Task>, title: String, priority: Priority, project: Option<String>) {
    let next_id = get_next_id(&tasks);
    let new_task = Task::new(next_id, title, priority, project);
    tasks.push(new_task);
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
