use serde::{Deserialize, Serialize};

use crate::task::*;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct TaskStore {
    next_id: u32,
    tasks: Vec<Task>,
}

impl TaskStore {
    pub fn new() -> TaskStore {
        TaskStore {
            next_id: 1,
            tasks: Vec::new(),
        }
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut Vec<Task> {
        &mut self.tasks
    }

    pub fn next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_task(&mut self, title: String, priority: Priority, project: Option<String>) {
        let id = self.next_id();
        let task = Task::new(id, title, priority, project);

        self.tasks.push(task);
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let pretty_json = serde_json::to_string_pretty(self)?;

        fs::write(path, pretty_json)?;

        Ok(())
    }

    pub fn load(path: &Path) -> Result<TaskStore, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Ok(TaskStore::new());
        }

        let contents = fs::read_to_string(path)?;
        let store: TaskStore = serde_json::from_str(&contents)?;

        Ok(store)
    }
}

fn get_storage_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME is not set");

    PathBuf::from(home).join(".tsk").join("tasks.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path() -> PathBuf {
        let thread_id = std::thread::current().id();
        std::env::temp_dir().join(format!("tsk_test_{:?}.json", thread_id))
    }

    #[test]
    fn load_returns_empty_vec_when_file_is_missing() {
        let path = temp_path();

        // making sure that file will not exist
        let _ = fs::remove_file(&path);

        let task_store = TaskStore::load(&path).unwrap();

        assert!(task_store.tasks().is_empty());
    }

    #[test]
    fn load_returns_error_on_corrupt_json() {
        let path = temp_path();
        fs::write(&path, b"this is not json {{{{{").unwrap();

        let result = TaskStore::load(&path);
        assert!(result.is_err());

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn save_then_load_round_trips() {
        let path = temp_path();
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Tests"), Priority::High, None);

        task_store.save(&path).unwrap();

        let loaded = TaskStore::load(&path).unwrap();

        assert_eq!(loaded.tasks().len(), 1);
        assert_eq!(loaded.tasks()[0].title(), "Tests");
        assert!(matches!(loaded.tasks[0].priority(), Priority::High));

        let _ = fs::remove_file(&path); // cleanup
    }

    #[test]
    fn saved_file_is_human_readable_json() {
        let path = temp_path();
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Tests"), Priority::High, None);

        task_store.save(&path).unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();

        assert!(contents.contains('\n'));
        assert!(contents.contains("\"title\""));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn next_id_is_one_for_empty_list() {
        let task_store = TaskStore::new();
        assert_eq!(task_store.next_id, 1)
    }

    #[test]
    fn next_id_increments_after_each_call() {
        let mut task_store = TaskStore::new();

        assert_eq!(task_store.next_id(), 1);
        assert_eq!(task_store.next_id(), 2);
    }

    #[test]
    fn add_task_increments_the_next_id() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("first"), Priority::Medium, None);
        assert_eq!(task_store.next_id, 2);

        task_store.add_task(String::from("second"), Priority::Medium, None);
        assert_eq!(task_store.next_id, 3);
    }

    #[test]
    fn add_task_assigns_correct_counter_value_as_next_id() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("first"), Priority::Medium, None);
        task_store.add_task(String::from("second"), Priority::Medium, None);

        assert_eq!(task_store.tasks[0].id(), 1);
        assert_eq!(task_store.tasks[1].id(), 2);
    }
}
