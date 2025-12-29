use crate::domain::{Task, TaskId, TaskState};
use crate::infrastructure::TaskStorage;

/// Service for managing tasks
/// This represents the application's use cases for task management
pub struct TaskService {
    tasks: Vec<Task>,
    storage: TaskStorage,
}

impl TaskService {
    pub fn new() -> Self {
        let storage = TaskStorage::new();
        let tasks = storage.load().unwrap_or_default();

        Self { tasks, storage }
    }

    /// Create with demo tasks (for first time use)
    pub fn new_with_defaults() -> Self {
        let storage = TaskStorage::new();
        let tasks = storage.load().unwrap_or_default();

        // Only add demo tasks if storage is empty
        if tasks.is_empty() {
            let mut service = Self { tasks, storage };
            service.add_task("Learn GPUI fundamentals");
            service.add_task("Build Waloyo task manager");
            service.add_task("Implement rain drop animation");
            service.add_task("Add wind swaying effect");
            service.add_task("Create clear sky celebration");
            return service;
        }

        Self { tasks, storage }
    }

    fn save(&self) {
        if let Err(e) = self.storage.save(&self.tasks) {
            eprintln!("Failed to save tasks: {}", e);
        }
    }

    /// Add a new task
    pub fn add_task(&mut self, content: impl Into<gpui::SharedString>) -> TaskId {
        let task = Task::new(content);
        let id = task.id;
        self.tasks.push(task);
        self.save();
        id
    }

    /// Get all pending tasks
    #[allow(dead_code)]
    pub fn pending_tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter().filter(|t| t.state == TaskState::Pending)
    }

    /// Get all completed tasks
    #[allow(dead_code)]
    pub fn completed_tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter().filter(|t| t.state == TaskState::Done)
    }

    /// Get all tasks (for rendering)
    pub fn all_tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Begin completing a task (starts animation)
    pub fn begin_completing(&mut self, id: TaskId) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.begin_completing();
            true
        } else {
            false
        }
    }

    /// Finish completing a task (after animation)
    pub fn finish_completing(&mut self, id: TaskId) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.complete();
            self.save();
            true
        } else {
            false
        }
    }

    /// Remove a task
    pub fn remove_task(&mut self, id: TaskId) -> Option<Task> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            let task = self.tasks.remove(pos);
            self.save();
            Some(task)
        } else {
            None
        }
    }

    /// Get count of pending tasks
    pub fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.is_pending()).count()
    }

    /// Get count of completed tasks  
    pub fn completed_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.is_done()).count()
    }

    /// Check if all tasks are completed (clear sky!)
    pub fn all_overcome(&self) -> bool {
        !self.tasks.is_empty() && self.tasks.iter().all(|t| t.is_done())
    }
}

impl Default for TaskService {
    fn default() -> Self {
        Self::new_with_defaults()
    }
}
