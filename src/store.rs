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

    pub fn filter_tasks(
        &self,
        project: Option<String>,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Vec<&Task> {
        self.tasks()
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

    pub fn search_tasks(&self, keyword: &str) -> Vec<&Task> {
        let lowercase_keyword = keyword.to_lowercase();
        self.tasks()
            .iter()
            .filter(|t| t.title().to_lowercase().contains(&lowercase_keyword))
            .collect()
    }

    pub fn edit_task(
        &mut self,
        id: u32,
        title: Option<String>,
        priority: Option<Priority>,
        project: Option<Option<String>>, // to handle clearing project -> None - No change, Some(None) - Clear Project, Some(Some(s)) - update
    ) -> Result<(), String> {
        self.tasks_mut()
            .iter_mut()
            .find(|t| t.id() == id)
            .map(|t| {
                if let Some(new_title) = title {
                    t.set_title(new_title);
                }

                if let Some(new_priority) = priority {
                    t.set_priority(new_priority);
                }

                if let Some(new_project) = project {
                    t.set_project(new_project);
                }
            })
            .ok_or_else(|| format!("Could not find a task with id: {}", id))
    }

    pub fn mark_done(&mut self, id: u32) -> Result<(), String> {
        self.tasks_mut()
            .iter_mut()
            .find(|t| t.id() == id)
            .map(|t| {
                t.set_status(Status::Done);
            })
            .ok_or_else(|| format!("Could not find a task with the id: {}", id))
    }

    pub fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(index) = self.tasks().iter().position(|t| t.id() == id) {
            self.tasks_mut().remove(index);
            Ok(())
        } else {
            Err(format!("Could not find a task with id: {}", id))
        }
    }

    pub fn summary(&self) -> Summary {
        use std::collections::HashMap;

        let total = self.tasks().len();
        let todo = self
            .tasks()
            .iter()
            .filter(|t| t.status() == Status::Todo)
            .count();
        let done = total - todo;

        // by priority
        let mut priority_counts: HashMap<Priority, usize> = HashMap::new();

        for task in self.tasks() {
            *priority_counts.entry(task.priority()).or_insert(0) += 1;
        }

        let mut by_priority: Vec<_> = priority_counts.into_iter().collect();
        by_priority.sort_by_key(|(p, _)| *p);

        // by priority
        let mut project_counts: HashMap<String, usize> = HashMap::new();

        for task in self.tasks() {
            let key = task.project().unwrap_or("(none)").to_string();
            *project_counts.entry(key).or_insert(0) += 1;
        }

        let mut by_project: Vec<_> = project_counts.into_iter().collect();
        by_project.sort_by(|a, b| b.1.cmp(&a.1));

        Summary {
            total,
            todo,
            done,
            by_project,
            by_priority,
        }
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

pub fn get_storage_path() -> PathBuf {
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
    fn load_returns_default_store_when_file_is_missing() {
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
        assert!(matches!(loaded.tasks()[0].priority(), Priority::High));

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

    #[test]
    fn add_task_status_is_always_todo() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("first"), Priority::Medium, None);

        assert!(matches!(task_store.tasks()[0].status(), Status::Todo));
    }

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

        let results = task_store.filter_tasks(Some(String::from("backend")), None, None);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn filter_by_status_todo() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);

        task_store.tasks_mut()[0].set_status(Status::Done);

        let results = task_store.filter_tasks(None, None, Some(Status::Todo));

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn filter_by_priority() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::High, None);

        let results = task_store.filter_tasks(None, Some(Priority::High), None);

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

        let results =
            task_store.filter_tasks(Some(String::from("backend")), None, Some(Status::Todo));

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title(), "Test 1");
    }

    #[test]
    fn filter_no_match_returns_empty() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        let results = task_store.filter_tasks(Some(String::from("nonexistent")), None, None);
        assert!(results.is_empty());
    }

    #[test]
    fn mark_done_updates_status() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test Task"), Priority::Medium, None);

        let result = task_store.mark_done(1);

        assert!(result.is_ok());
        assert!(matches!(task_store.tasks()[0].status(), Status::Done));
    }

    #[test]
    fn mark_done_returns_error_for_missing_id() {
        let mut task_store = TaskStore::new();

        let result = task_store.mark_done(99);

        assert!(result.is_err());
    }

    #[test]
    fn mark_done_does_not_affect_other_tasks() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        task_store.mark_done(1).unwrap();

        assert!(matches!(task_store.tasks()[1].status(), Status::Todo));
        assert!(matches!(task_store.tasks()[2].status(), Status::Todo));
    }

    #[test]
    fn mark_done_on_empty_list_does_not_panic() {
        let mut task_store = TaskStore::new();
        let result = task_store.mark_done(1);
        assert!(result.is_err());
    }

    #[test]
    fn delete_task_removes_it() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        let result = task_store.delete_task(2);

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

        let result = task_store.delete_task(99);

        assert!(result.is_err());
    }

    #[test]
    fn deleted_id_is_not_reused() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Test 1"), Priority::Medium, None);
        task_store.add_task(String::from("Test 2"), Priority::Medium, None);
        task_store.add_task(String::from("Test 3"), Priority::Medium, None);

        task_store.delete_task(3).unwrap();

        task_store.add_task(String::from("Test 4"), Priority::Medium, None);

        // since we removed the `3` id the new task should have the id `4`
        assert_eq!(task_store.tasks().last().unwrap().id(), 4);
    }

    #[test]
    fn delete_on_empty_list_does_not_panic() {
        let mut task_store = TaskStore::new();
        let result = task_store.delete_task(1);
        assert!(result.is_err());
    }

    #[test]
    fn search_is_case_insensitive() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Learn Rust"), Priority::Medium, None);
        task_store.add_task(String::from("Build Api"), Priority::Medium, None);
        task_store.add_task(String::from("Deploy"), Priority::Medium, None);

        let result = task_store.search_tasks("rust");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id(), 1);
    }

    #[test]
    fn search_matches_partial_word() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Learn Rust"), Priority::Medium, None);
        task_store.add_task(String::from("Build Api"), Priority::Medium, None);
        task_store.add_task(String::from("Deploy"), Priority::Medium, None);

        let result = task_store.search_tasks("rus");

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn search_no_match_returns_empty() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Learn Rust"), Priority::Medium, None);
        task_store.add_task(String::from("Build Api"), Priority::Medium, None);
        task_store.add_task(String::from("Deploy"), Priority::Medium, None);

        let result = task_store.search_tasks("python");

        assert!(result.is_empty());
    }

    #[test]
    fn edit_updates_title() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Old Title"), Priority::Medium, None);

        task_store
            .edit_task(1, Some(String::from("New Title")), None, None)
            .unwrap();

        assert_eq!(task_store.tasks()[0].title(), "New Title");
    }

    #[test]
    fn edit_updates_priority_only() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Same Title"), Priority::Medium, None);

        task_store
            .edit_task(1, None, Some(Priority::High), None)
            .unwrap();

        assert_eq!(task_store.tasks()[0].title(), "Same Title");
        assert!(matches!(task_store.tasks()[0].priority(), Priority::High));
    }

    #[test]
    fn edit_returns_error_for_missing_id() {
        let mut task_store = TaskStore::new();

        let result = task_store.edit_task(99, None, Some(Priority::High), None);

        assert!(result.is_err());
    }

    #[test]
    fn edit_updates_project() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Build API"), Priority::Medium, None);

        task_store
            .edit_task(1, None, None, Some(Some(String::from("backend"))))
            .unwrap();

        assert_eq!(task_store.tasks()[0].project(), Some("backend"));
    }

    #[test]
    fn summary_counts_total_todo_done() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Learn Rust"), Priority::High, None);
        task_store.add_task(String::from("Build Api"), Priority::Medium, None);
        task_store.add_task(String::from("Deploy"), Priority::Low, None);

        task_store.tasks_mut()[2].set_status(Status::Done);

        let result = task_store.summary();

        assert_eq!(result.total, 3);
        assert_eq!(result.todo, 2);
        assert_eq!(result.done, 1);
    }

    #[test]
    fn summary_counts_by_priority() {
        let mut task_store = TaskStore::new();

        task_store.add_task(String::from("Learn Rust"), Priority::High, None);
        task_store.add_task(String::from("Build Api"), Priority::High, None);
        task_store.add_task(String::from("Deploy"), Priority::Low, None);

        let result = task_store.summary();

        assert_eq!(result.priority_count(Priority::High), 2);
        assert_eq!(result.priority_count(Priority::Medium), 0);
        assert_eq!(result.priority_count(Priority::Low), 1);
    }

    #[test]
    fn summary_counts_by_project() {
        let mut task_store = TaskStore::new();

        task_store.add_task(
            String::from("Learn Rust"),
            Priority::High,
            Some(String::from("backend")),
        );
        task_store.add_task(
            String::from("Build Api"),
            Priority::High,
            Some(String::from("backend")),
        );
        task_store.add_task(String::from("Deploy"), Priority::Low, None);

        let result = task_store.summary();

        assert_eq!(result.project_count("backend"), 2);
        assert_eq!(result.project_count("(none)"), 1);
    }
}
