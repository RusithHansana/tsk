use crate::task::Task;
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

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let path = get_storage_path();

    load_from(&path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_path() -> PathBuf {
        std::env::temp_dir().join(format!("tsk_test_{}.json", std::process::id()))
    }

    #[test]
    fn load_returns_empty_vec_when_file_is_missing() {
        let path = temp_path();

        // making sure that file will not exist
        let _ = std::fs::remove_file(&path);

        let tasks = load_from(&path).unwrap();

        assert!(tasks.is_empty());
    }
}
