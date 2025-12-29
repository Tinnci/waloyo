use crate::domain::{Task, TaskId, TaskState};
use crate::infrastructure::TaskStorage;
use chrono;

#[derive(Clone)]
enum TaskAction {
    Add(TaskId),
    Remove(Task),
    UpdateContent(TaskId, gpui::SharedString), // Stores OLD content
    Complete(TaskId),
}

/// Service for managing tasks
/// This represents the application's use cases for task management
pub struct TaskService {
    tasks: Vec<Task>,
    storage: TaskStorage,
    history: Vec<TaskAction>,
}

impl TaskService {
    pub fn new() -> Self {
        let storage = TaskStorage::new();
        let tasks = storage.load().unwrap_or_default();

        Self {
            tasks,
            storage,
            history: Vec::new(),
        }
    }

    /// Create with demo tasks (for first time use)
    pub fn new_with_defaults() -> Self {
        let storage = TaskStorage::new();
        let tasks = storage.load().unwrap_or_default();

        // Only add demo tasks if storage is empty
        if tasks.is_empty() {
            let mut service = Self {
                tasks,
                storage,
                history: Vec::new(),
            };
            service.add_task("Learn GPUI fundamentals !m");
            service.add_task("Build Waloyo task manager !h @today");
            service.add_task("Implement rain drop animation @tomorrow");
            service.add_task("Add wind swaying effect !l");
            service.add_task("Create clear sky celebration !h");
            // Clear history after initial defaults to avoid undoing them
            service.history.clear();
            return service;
        }

        Self {
            tasks,
            storage,
            history: Vec::new(),
        }
    }

    fn save(&self) {
        if let Err(e) = self.storage.save(&self.tasks) {
            eprintln!("Failed to save tasks: {}", e);
        }
    }

    /// Add a new task with smart parsing for metadata
    pub fn add_task(&mut self, content: impl Into<gpui::SharedString>) -> TaskId {
        let content_str = content.into();
        let mut task = Task::new(content_str.clone());
        let mut cleaned_content = content_str.to_string();

        // Simple parsing for priority: !h, !m, !l
        if cleaned_content.contains("!h") {
            task.priority = crate::domain::TaskPriority::High;
            cleaned_content = cleaned_content.replace("!h", "").trim().to_string();
        } else if cleaned_content.contains("!m") {
            task.priority = crate::domain::TaskPriority::Medium;
            cleaned_content = cleaned_content.replace("!m", "").trim().to_string();
        } else if cleaned_content.contains("!l") {
            task.priority = crate::domain::TaskPriority::Low;
            cleaned_content = cleaned_content.replace("!l", "").trim().to_string();
        }

        // Simple parsing for due date: @today, @tomorrow
        let now = chrono::Local::now();
        if cleaned_content.contains("@today") {
            task.due_date = Some(now);
            cleaned_content = cleaned_content.replace("@today", "").trim().to_string();
        } else if cleaned_content.contains("@tomorrow") {
            task.due_date = Some(now + chrono::Duration::days(1));
            cleaned_content = cleaned_content.replace("@tomorrow", "").trim().to_string();
        }

        task.content = gpui::SharedString::from(cleaned_content);

        let id = task.id;
        self.tasks.push(task);
        self.history.push(TaskAction::Add(id));
        self.save();
        id
    }

    /// Update task content
    pub fn update_task_content(
        &mut self,
        id: TaskId,
        content: impl Into<gpui::SharedString>,
    ) -> bool {
        let content = content.into();
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            let old_content = task.content.clone();
            if old_content != content {
                self.history
                    .push(TaskAction::UpdateContent(id, old_content));
                task.content = content;
                task.updated_at = std::time::Instant::now();
                self.save();
            }
            true
        } else {
            false
        }
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
            self.history.push(TaskAction::Complete(id));
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
            self.history.push(TaskAction::Remove(task.clone()));
            self.save();
            Some(task)
        } else {
            None
        }
    }

    /// Undo last action
    pub fn undo(&mut self) -> bool {
        if let Some(action) = self.history.pop() {
            match action {
                TaskAction::Add(id) => {
                    if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
                        self.tasks.remove(pos);
                    }
                }
                TaskAction::Remove(task) => {
                    self.tasks.push(task);
                }
                TaskAction::UpdateContent(id, old_content) => {
                    if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                        task.content = old_content;
                        task.updated_at = std::time::Instant::now();
                    }
                }
                TaskAction::Complete(id) => {
                    if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                        task.state = TaskState::Pending;
                        task.updated_at = std::time::Instant::now();
                    }
                }
            }
            self.save();
            true
        } else {
            false
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
