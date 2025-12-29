use crate::domain::{Task, TaskId, TaskState};
use gpui::SharedString;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

/// Serializable version of Task for JSON persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub id: u64,
    pub content: String,
    pub state: String,
}

impl From<&Task> for TaskData {
    fn from(task: &Task) -> Self {
        Self {
            id: task.id.0,
            content: task.content.to_string(),
            state: match task.state {
                TaskState::Pending => "pending".to_string(),
                TaskState::Completing => "pending".to_string(), // Save as pending since completing is transient
                TaskState::Done => "done".to_string(),
            },
        }
    }
}

impl TaskData {
    pub fn into_task(self) -> Task {
        let now = Instant::now();
        Task {
            id: TaskId(self.id),
            content: SharedString::from(self.content),
            state: match self.state.as_str() {
                "done" => TaskState::Done,
                _ => TaskState::Pending,
            },
            created_at: now,
            updated_at: now,
        }
    }
}

/// Storage data format
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageData {
    pub version: u32,
    pub tasks: Vec<TaskData>,
}

impl StorageData {
    pub fn new() -> Self {
        Self {
            version: 1,
            tasks: Vec::new(),
        }
    }
}

/// Task storage service for JSON file persistence
pub struct TaskStorage {
    file_path: PathBuf,
}

impl TaskStorage {
    pub fn new() -> Self {
        let file_path = Self::get_storage_path();
        Self { file_path }
    }

    fn get_storage_path() -> PathBuf {
        // Use ~/.waloyo/tasks.json
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".waloyo");
        path.push("tasks.json");
        path
    }

    /// Ensure the storage directory exists
    fn ensure_directory(&self) -> std::io::Result<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    /// Load tasks from storage
    pub fn load(&self) -> Result<Vec<Task>, String> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("Failed to read storage file: {}", e))?;

        let data: StorageData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse storage file: {}", e))?;

        Ok(data.tasks.into_iter().map(|t| t.into_task()).collect())
    }

    /// Save tasks to storage
    pub fn save(&self, tasks: &[Task]) -> Result<(), String> {
        self.ensure_directory()
            .map_err(|e| format!("Failed to create storage directory: {}", e))?;

        let data = StorageData {
            version: 1,
            tasks: tasks.iter().map(TaskData::from).collect(),
        };

        let content = serde_json::to_string_pretty(&data)
            .map_err(|e| format!("Failed to serialize tasks: {}", e))?;

        fs::write(&self.file_path, content)
            .map_err(|e| format!("Failed to write storage file: {}", e))?;

        Ok(())
    }
}

impl Default for TaskStorage {
    fn default() -> Self {
        Self::new()
    }
}
