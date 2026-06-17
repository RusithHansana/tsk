use crate::task::*;
use std::fs;
use std::path::{Path, PathBuf};

fn get_storage_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME is not set");

    PathBuf::from(home).join(".tsk").join("tasks.json")
}

fn load_from(path: &Path) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(path)?;
    let tasks: Vec<Task> = serde_json::from_str(&contents)?;

    Ok(tasks)
}

fn save_to(path: &Path, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let pretty_json = serde_json::to_string_pretty(tasks)?;

    fs::write(path, pretty_json)?;

    Ok(())
}

fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id()).max().unwrap_or(0) + 1
}

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let path = get_storage_path();

    load_from(&path)
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_storage_path();

    save_to(&path, tasks)
}

pub fn get_next_id(tasks: &[Task]) -> u32 {
    next_id(tasks)
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

        let tasks = load_from(&path).unwrap();

        assert!(tasks.is_empty());
    }

    #[test]
    fn load_returns_error_on_corrupt_json() {
        let path = temp_path();
        fs::write(&path, b"this is not json {{{{{").unwrap();

        let result = load_from(&path);
        assert!(result.is_err());

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn save_then_load_round_trips() {
        let path = temp_path();
        let tasks = vec![Task::new(1, String::from("Tests"), Priority::High, None)];

        save_to(&path, &tasks).unwrap();
        let loaded = load_from(&path).unwrap();

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].title(), "Tests");
        assert!(matches!(loaded[0].priority(), Priority::High));

        let _ = fs::remove_file(&path); // cleanup
    }

    #[test]
    fn saved_file_is_human_readable_json() {
        let path = temp_path();
        let tasks = vec![Task::new(1, String::from("Tests"), Priority::High, None)];

        save_to(&path, &tasks).unwrap();
        let contents = std::fs::read_to_string(&path).unwrap();

        assert!(contents.contains('\n'));
        assert!(contents.contains("\"title\""));

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn next_id_is_one_for_empty_list() {
        let tasks: Vec<Task> = vec![];
        assert_eq!(next_id(&tasks), 1)
    }

    #[test]
    fn next_id_is_max_plus_one() {
        let tasks = vec![
            Task::new(1, String::from("A"), Priority::Medium, None),
            Task::new(3, String::from("B"), Priority::Medium, None), // gap — 2 was deleted
        ];

        assert_eq!(next_id(&tasks), 4);
    }

    #[test]
    fn next_id_after_single_task() {
        let tasks = vec![Task::new(1, String::from("Only"), Priority::Medium, None)];

        assert_eq!(next_id(&tasks), 2);
    }
}
