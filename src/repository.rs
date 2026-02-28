//! Repository module for TaskForge.
//!
//! This module defines the Repository trait for data persistence.

use crate::filter::{TagFilter, TaskFilter, TaskSort};
use crate::models::{Status, Tag, Task};

/// Errors that can occur during repository operations.
#[derive(Debug)]
pub enum RepositoryError {
    /// Database error.
    Database(String),
    /// Entity not found.
    NotFound(String),
    /// Constraint violation.
    Constraint(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::Database(msg) => write!(f, "Database error: {}", msg),
            RepositoryError::NotFound(msg) => write!(f, "Not found: {}", msg),
            RepositoryError::Constraint(msg) => write!(f, "Constraint violation: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

/// Repository trait for data persistence operations.
///
/// This trait defines the interface for CRUD operations on tasks and tags.
pub trait Repository: Send + Sync {
    // Task operations

    /// Creates a new task.
    fn create_task(&self, task: &Task) -> Result<(), RepositoryError>;

    /// Gets a task by ID.
    fn get_task(&self, id: &str) -> Result<Task, RepositoryError>;

    /// Updates an existing task.
    fn update_task(&self, task: &Task) -> Result<(), RepositoryError>;

    /// Deletes a task by ID.
    fn delete_task(&self, id: &str) -> Result<(), RepositoryError>;

    /// Lists tasks with optional filter and sort.
    fn list_tasks(
        &self,
        filter: &TaskFilter,
        sort: &TaskSort,
    ) -> Result<Vec<Task>, RepositoryError>;

    /// Gets all tasks with a specific status.
    fn get_tasks_by_status(&self, status: Status) -> Result<Vec<Task>, RepositoryError>;

    // Tag operations

    /// Creates a new tag.
    fn create_tag(&self, tag: &Tag) -> Result<(), RepositoryError>;

    /// Gets a tag by ID.
    fn get_tag(&self, id: &str) -> Result<Tag, RepositoryError>;

    /// Gets a tag by name.
    fn get_tag_by_name(&self, name: &str) -> Result<Tag, RepositoryError>;

    /// Updates an existing tag.
    fn update_tag(&self, tag: &Tag) -> Result<(), RepositoryError>;

    /// Deletes a tag by ID.
    fn delete_tag(&self, id: &str) -> Result<(), RepositoryError>;

    /// Lists tags with optional filter.
    fn list_tags(&self, filter: &TagFilter) -> Result<Vec<Tag>, RepositoryError>;

    // Tag-Task relationship operations

    /// Adds a tag to a task.
    fn add_tag_to_task(&self, task_id: &str, tag_id: &str) -> Result<(), RepositoryError>;

    /// Removes a tag from a task.
    fn remove_tag_from_task(&self, task_id: &str, tag_id: &str) -> Result<(), RepositoryError>;

    /// Gets all tags for a task.
    fn get_task_tags(&self, task_id: &str) -> Result<Vec<Tag>, RepositoryError>;

    /// Gets all tasks with a specific tag.
    fn get_tasks_by_tag(&self, tag_id: &str) -> Result<Vec<Task>, RepositoryError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_error_display_database() {
        let err = RepositoryError::Database("connection failed".to_string());
        assert_eq!(err.to_string(), "Database error: connection failed");
    }

    #[test]
    fn repository_error_display_not_found() {
        let err = RepositoryError::NotFound("task 123".to_string());
        assert_eq!(err.to_string(), "Not found: task 123");
    }

    #[test]
    fn repository_error_display_constraint() {
        let err = RepositoryError::Constraint("unique name".to_string());
        assert_eq!(err.to_string(), "Constraint violation: unique name");
    }
}
