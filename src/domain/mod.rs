// Domain Layer - Core business logic
// This layer contains the heart of the Waloyo application:
// pure business rules with no dependencies on UI or infrastructure.

mod task;

pub use task::*;

/// Event emitted when a new task is submitted
#[derive(Clone)]
pub struct TaskSubmitted(pub String);
