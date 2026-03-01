//! Commands module for TaskForge.
//!
//! This module defines command handlers for CLI operations.

use chrono::{DateTime, NaiveDate, Utc};
use std::str::FromStr;
use crate::error::AppError;
use crate::filter::{TagFilter, TaskFilter, TaskSort};
use crate::models::{Priority, Tag, Task};
use crate::repository::{Repository, RepositoryError};
use std::sync::Arc;

/// Command handler for creating a new task.
/// This version accepts a concrete repository type.
pub fn create_task<R: Repository>(
    repository: Arc<R>,
    title: String,
    description: Option<String>,
    priority: Priority,
) -> Result<Task, AppError> {
    // Create the task with the given details
    let task = Task::with_details(title, description, priority);

    // Persist the task to the repository
    repository.create_task(&task).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        _ => AppError::UserError(e.to_string()),
    })?;

    Ok(task)
}

/// Command handler for creating a new task with a trait object.
/// This version accepts Arc<dyn Repository> for dynamic dispatch.
pub fn create_task_with_dyn(
    repository: &dyn Repository,
    title: String,
    description: Option<String>,
    priority: Priority,
) -> Result<Task, AppError> {
    // Create the task with the given details
    let task = Task::with_details(title, description, priority);

    // Persist the task to the repository
    repository.create_task(&task).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        _ => AppError::UserError(e.to_string()),
    })?;

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
pub fn get_task<R: Repository>(repository: Arc<R>, id: String) -> Result<Task, AppError> {
    repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })
}

/// Command handler for getting a task by ID with a trait object.
/// This version accepts Arc<dyn Repository> for dynamic dispatch.
pub fn get_task_with_dyn(
    repository: &dyn Repository,
    id: String,
) -> Result<Task, AppError> {
    repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })
}

/// Command handler for updating a task with a trait object.
/// This version accepts &dyn Repository for dynamic dispatch.
pub fn update_task_with_dyn(
    repository: &dyn Repository,
    id: String,
    title: Option<String>,
    description: Option<String>,
    priority: Option<u8>,
    status: Option<String>,
    due_date: Option<String>,
) -> Result<Task, AppError> {
    // Fetch the existing task
    let existing_task = repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => AppError::System(crate::error::SystemError::Database(msg)),
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    let mut task = existing_task;

    // Update title if provided
    if let Some(t) = title {
        task.update_title(t);
    }
    
    // Update priority if provided
    if let Some(p) = priority {
        let priority_enum = Priority::from_str(&format!("P{}", p))
            .map_err(|_| AppError::Validation(crate::error::ValidationError::InvalidPriority))?;
        task.update_priority(priority_enum);
    }
    
    // Update description if provided
    task.description = description.or(task.description);
    
    // Update status if provided
    if let Some(s) = status {
        match s.to_lowercase().as_str() {
            "completed" | "complete" => {
                task.complete();
            }
            "incomplete" | "pending" => {
                task.reopen();
            }
            _ => {
                return Err(AppError::Validation(
                    crate::error::ValidationError::InvalidStatus(
                        format!("Invalid status: {}. Use 'completed' or 'incomplete'.", s)
                    )
                ));
            }
        }
    }
    
    // Update due date if provided
    if let Some(due) = due_date {
        let naive_date = NaiveDate::parse_from_str(&due, "%Y-%m-%d")
            .map_err(|e| AppError::Validation(
                crate::error::ValidationError::InvalidDate(
                    format!("Invalid date format: {}. Use YYYY-MM-DD.", e)
                )
            ))?;
        let datetime: DateTime<Utc> = naive_date.and_hms_opt(23, 59, 59)
            .unwrap()
            .and_utc();
        task.due_date = Some(datetime);
        task.updated_at = Utc::now();
    }

    // Persist the updated task
    repository.update_task(&task).map_err(|e| match e {
        RepositoryError::Database(msg) => AppError::System(crate::error::SystemError::Database(msg)),
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
        RepositoryError::NotFound(msg) => AppError::UserError(msg),
    })?;

    Ok(task)
}

/// Command handler for updating a task.
pub fn update_task<R: Repository>(
    repository: Arc<R>,
    id: String,
    title: Option<String>,
    description: Option<String>,
    priority: Option<u8>,
    status: Option<String>,
    due_date: Option<String>,
) -> Result<Task, AppError> {
    // Fetch the existing task
    let existing_task = repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::UserError(msg),
        RepositoryError::Database(msg) => AppError::System(crate::error::SystemError::Database(msg)),
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    let mut task = existing_task;

    // Update title if provided
    if let Some(t) = title {
        task.update_title(t);
    }
    
    // Update priority if provided
    if let Some(p) = priority {
        let priority_enum = Priority::from_str(&format!("P{}", p))
            .map_err(|_| AppError::Validation(crate::error::ValidationError::InvalidPriority))?;
        task.update_priority(priority_enum);
    }
    
    // Update description if provided
    task.description = description.or(task.description);
    
    // Update status if provided
    if let Some(s) = status {
        match s.to_lowercase().as_str() {
            "completed" | "complete" => {
                task.complete();
            }
            "incomplete" | "pending" => {
                task.reopen();
            }
            _ => {
                return Err(AppError::Validation(
                    crate::error::ValidationError::InvalidStatus(
                        format!("Invalid status: {}. Use 'completed' or 'incomplete'.", s)
                    )
                ));
            }
        }
    }
    
    // Update due date if provided
    if let Some(due) = due_date {
        let naive_date = NaiveDate::parse_from_str(&due, "%Y-%m-%d")
            .map_err(|e| AppError::Validation(
                crate::error::ValidationError::InvalidDate(
                    format!("Invalid date format: {}. Use YYYY-MM-DD.", e)
                )
            ))?;
        let datetime: DateTime<Utc> = naive_date.and_hms_opt(23, 59, 59)
            .unwrap()
            .and_utc();
        task.due_date = Some(datetime);
        task.updated_at = Utc::now();
    }

    // Persist the updated task
    repository.update_task(&task).map_err(|e| match e {
        RepositoryError::Database(msg) => AppError::System(crate::error::SystemError::Database(msg)),
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
        RepositoryError::NotFound(msg) => AppError::UserError(msg),
    })?;

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
    use crate::repository::SqliteRepository;

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

    use tempfile::TempDir;

    #[test]
    fn test_get_task_returns_task_when_exists() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = Arc::new(SqliteRepository::new(&db_path).unwrap());
        repo.initialize().unwrap();

        // Create a task first
        let task = Task::with_details(
            "Test task".to_string(),
            Some("Test description".to_string()),
            Priority::P1,
        );
        repo.create_task(&task).unwrap();

        // Get the task using the command
        let result = get_task(repo, task.id.clone());

        assert!(result.is_ok());
        let retrieved = result.unwrap();
        assert_eq!(retrieved.id, task.id);
        assert_eq!(retrieved.title, "Test task");
        assert_eq!(retrieved.description, Some("Test description".to_string()));
        assert_eq!(retrieved.priority, Priority::P1);
    }

    #[test]
    fn test_get_task_returns_error_when_not_exists() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = Arc::new(SqliteRepository::new(&db_path).unwrap());
        repo.initialize().unwrap();

        // Try to get a task that doesn't exist
        let non_existent_id = "non-existent-task-id".to_string();
        let result = get_task(repo, non_existent_id.clone());

        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            AppError::NotFound(msg) => {
                assert!(msg.contains(&non_existent_id));
            }
            _ => panic!("Expected AppError::NotFound, got {:?}", error),
        }
    }
}
