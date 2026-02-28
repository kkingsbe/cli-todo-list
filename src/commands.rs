//! Commands module for TaskForge.
//!
//! This module defines command handlers for CLI operations.

use crate::error::AppError;
use crate::filter::{TagFilter, TaskFilter, TaskSort};
use crate::models::{Priority, Tag, Task};
use crate::repository::Repository;
use std::sync::Arc;

/// Command handler for creating a new task.
pub fn create_task<R: Repository>(
    _repository: Arc<R>,
    title: String,
    description: Option<String>,
    priority: Priority,
) -> Result<Task, AppError> {
    let task = Task::with_details(title, description, priority);
    // Repository operations will be implemented in later stories
    Ok(task)
}

/// Command handler for listing tasks.
pub fn list_tasks<R: Repository>(
    _repository: Arc<R>,
    filter: TaskFilter,
    sort: TaskSort,
) -> Result<Vec<Task>, AppError> {
    // Repository operations will be implemented in later stories
    let _ = (filter, sort);
    Ok(Vec::new())
}

/// Command handler for getting a task by ID.
pub fn get_task<R: Repository>(_repository: Arc<R>, id: String) -> Result<Task, AppError> {
    let _ = id;
    // Repository operations will be implemented in later stories
    Err(AppError::NotFound("Task not implemented yet".to_string()))
}

/// Command handler for updating a task.
pub fn update_task<R: Repository>(
    _repository: Arc<R>,
    mut task: Task,
    title: Option<String>,
    description: Option<String>,
    priority: Option<Priority>,
) -> Result<Task, AppError> {
    if let Some(t) = title {
        task.update_title(t);
    }
    if let Some(p) = priority {
        task.update_priority(p);
    }
    task.description = description.or(task.description);
    Ok(task)
}

/// Command handler for deleting a task.
pub fn delete_task<R: Repository>(_repository: Arc<R>, id: String) -> Result<(), AppError> {
    let _ = id;
    // Repository operations will be implemented in later stories
    Ok(())
}

/// Command handler for completing a task.
pub fn complete_task<R: Repository>(_repository: Arc<R>, id: String) -> Result<Task, AppError> {
    let _ = id;
    // Repository operations will be implemented in later stories
    Err(AppError::NotFound("Task not found".to_string()))
}

/// Command handler for creating a new tag.
pub fn create_tag<R: Repository>(
    _repository: Arc<R>,
    name: String,
    color: Option<String>,
) -> Result<Tag, AppError> {
    let tag = match color {
        Some(c) => Tag::with_color(name, c),
        None => Tag::new(name),
    };
    Ok(tag)
}

/// Command handler for listing tags.
pub fn list_tags<R: Repository>(
    _repository: Arc<R>,
    filter: TagFilter,
) -> Result<Vec<Tag>, AppError> {
    let _ = filter;
    // Repository operations will be implemented in later stories
    Ok(Vec::new())
}

/// Command handler for deleting a tag.
pub fn delete_tag<R: Repository>(_repository: Arc<R>, id: String) -> Result<(), AppError> {
    let _ = id;
    // Repository operations will be implemented in later stories
    Ok(())
}

/// Command handler for adding a tag to a task.
pub fn add_tag_to_task<R: Repository>(
    _repository: Arc<R>,
    task_id: String,
    tag_id: String,
) -> Result<(), AppError> {
    let _ = (task_id, tag_id);
    // Repository operations will be implemented in later stories
    Ok(())
}

/// Command handler for removing a tag from a task.
pub fn remove_tag_from_task<R: Repository>(
    _repository: Arc<R>,
    task_id: String,
    tag_id: String,
) -> Result<(), AppError> {
    let _ = (task_id, tag_id);
    // Repository operations will be implemented in later stories
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task_creates_task_with_given_details() {
        let task = Task::with_details(
            "Test task".to_string(),
            Some("Description".to_string()),
            Priority::P1,
        );
        assert_eq!(task.title, "Test task");
        assert_eq!(task.description, Some("Description".to_string()));
        assert_eq!(task.priority, Priority::P1);
    }

    #[test]
    fn create_tag_creates_tag() {
        let tag = Tag::new("work".to_string());
        assert_eq!(tag.name, "work");
    }

    #[test]
    fn create_tag_with_color_creates_tag_with_color() {
        let tag = Tag::with_color("work".to_string(), "#FF0000".to_string());
        assert_eq!(tag.name, "work");
        assert_eq!(tag.color, Some("#FF0000".to_string()));
    }

    #[test]
    fn update_task_updates_title() {
        let mut task = Task::new("Old title".to_string());
        task.title = "New title".to_string();
        assert_eq!(task.title, "New title");
    }

    #[test]
    fn update_task_updates_priority() {
        let mut task = Task::new("Test".to_string());
        task.priority = Priority::P1;
        assert_eq!(task.priority, Priority::P1);
    }
}
