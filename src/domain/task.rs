use gpui::SharedString;
use std::time::Instant;

/// The state of a task in its lifecycle.
/// Follows the "Wind & Rain" metaphor:
/// - Pending: Like wind, tasks are unsettled and in motion
/// - Completing: The rain falls, washing away the task  
/// - Done: Clear skies, the task has been overcome
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskState {
    /// Task is pending - represented as "wind" in the UI
    #[default]
    Pending,
    /// Task is being completed - the "rain drop" animation plays
    Completing,
    /// Task is done - moved to the "ocean" of completed tasks
    Done,
}

/// A unique identifier for a task
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);

impl TaskId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

/// Priority level for a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TaskPriority {
    /// Normal priority, like a gentle breeze
    #[default]
    Low,
    /// Medium priority, like a steady wind
    Medium,
    /// High priority, like a storm brew
    High,
}

/// A task entity representing something to be overcome.
#[derive(Debug, Clone)]
pub struct Task {
    /// Unique identifier
    pub id: TaskId,
    /// The content/description of the task
    pub content: SharedString,
    /// Current state of the task
    pub state: TaskState,
    /// Priority level
    pub priority: TaskPriority,
    /// Optional due date
    pub due_date: Option<chrono::DateTime<chrono::Local>>,
    /// When the task was created
    pub created_at: Instant,
    /// When the task state last changed
    pub updated_at: Instant,
}

impl Task {
    /// Create a new pending task
    pub fn new(content: impl Into<SharedString>) -> Self {
        let now = Instant::now();
        Self {
            id: TaskId::new(),
            content: content.into(),
            state: TaskState::Pending,
            priority: TaskPriority::default(),
            due_date: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Begin the completion animation
    pub fn begin_completing(&mut self) {
        if self.state == TaskState::Pending {
            self.state = TaskState::Completing;
            self.updated_at = Instant::now();
        }
    }

    /// Mark the task as fully completed
    pub fn complete(&mut self) {
        self.state = TaskState::Done;
        self.updated_at = Instant::now();
    }

    /// Check if task is in pending state
    pub fn is_pending(&self) -> bool {
        self.state == TaskState::Pending
    }

    /// Check if task is currently completing (animation playing)
    pub fn is_completing(&self) -> bool {
        self.state == TaskState::Completing
    }

    /// Check if task is done
    pub fn is_done(&self) -> bool {
        self.state == TaskState::Done
    }
}
