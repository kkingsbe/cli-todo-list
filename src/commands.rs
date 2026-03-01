//! Commands module for TaskForge.
//!
//! This module defines command handlers for CLI operations.

use crate::error::AppError;
use crate::filter::{TagFilter, TaskFilter, TaskSort};
use crate::models::{Priority, Tag, TagWithCount, Task};
use crate::repository::{Repository, RepositoryError};
use chrono::{DateTime, NaiveDate, Utc};
use std::io::Write;
use std::str::FromStr;
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
    repository: Arc<R>,
    filter: TaskFilter,
    sort: TaskSort,
) -> Result<Vec<Task>, AppError> {
    repository.list_tasks(&filter, &sort).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })
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
pub fn get_task_with_dyn(repository: &dyn Repository, id: String) -> Result<Task, AppError> {
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
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
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
                    crate::error::ValidationError::InvalidStatus(format!(
                        "Invalid status: {}. Use 'completed' or 'incomplete'.",
                        s
                    )),
                ));
            }
        }
    }

    // Update due date if provided
    if let Some(due) = due_date {
        let naive_date = NaiveDate::parse_from_str(&due, "%Y-%m-%d").map_err(|e| {
            AppError::Validation(crate::error::ValidationError::InvalidDate(format!(
                "Invalid date format: {}. Use YYYY-MM-DD.",
                e
            )))
        })?;
        let datetime: DateTime<Utc> = naive_date.and_hms_opt(23, 59, 59).unwrap().and_utc();
        task.due_date = Some(datetime);
        task.updated_at = Utc::now();
    }

    // Persist the updated task
    repository.update_task(&task).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
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
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
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
                    crate::error::ValidationError::InvalidStatus(format!(
                        "Invalid status: {}. Use 'completed' or 'incomplete'.",
                        s
                    )),
                ));
            }
        }
    }

    // Update due date if provided
    if let Some(due) = due_date {
        let naive_date = NaiveDate::parse_from_str(&due, "%Y-%m-%d").map_err(|e| {
            AppError::Validation(crate::error::ValidationError::InvalidDate(format!(
                "Invalid date format: {}. Use YYYY-MM-DD.",
                e
            )))
        })?;
        let datetime: DateTime<Utc> = naive_date.and_hms_opt(23, 59, 59).unwrap().and_utc();
        task.due_date = Some(datetime);
        task.updated_at = Utc::now();
    }

    // Persist the updated task
    repository.update_task(&task).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
        RepositoryError::NotFound(msg) => AppError::UserError(msg),
    })?;

    Ok(task)
}

/// Command handler for deleting a task.
pub fn delete_task<R: Repository>(
    repository: Arc<R>,
    id: String,
    force: bool,
) -> Result<(), AppError> {
    // Verify the task exists
    repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    // Prompt for confirmation if not forced
    if !force {
        print!("Delete task {}? [y/N]: ", id);
        std::io::stdout()
            .flush()
            .map_err(|e| AppError::UserError(e.to_string()))?;
        let mut confirmation = String::new();
        std::io::stdin()
            .read_line(&mut confirmation)
            .map_err(|e| AppError::UserError(e.to_string()))?;

        if !confirmation.trim().eq_ignore_ascii_case("y") {
            return Ok(());
        }
    }

    // Delete the task
    repository.delete_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    Ok(())
}

/// Command handler for deleting a task with a trait object.
/// This version accepts &dyn Repository for dynamic dispatch.
pub fn delete_task_with_dyn(
    repository: &dyn Repository,
    id: String,
    force: bool,
) -> Result<(), AppError> {
    // Verify the task exists
    repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    // Prompt for confirmation if not forced
    if !force {
        print!("Delete task {}? [y/N]: ", id);
        std::io::stdout()
            .flush()
            .map_err(|e| AppError::UserError(e.to_string()))?;
        let mut confirmation = String::new();
        std::io::stdin()
            .read_line(&mut confirmation)
            .map_err(|e| AppError::UserError(e.to_string()))?;

        if !confirmation.trim().eq_ignore_ascii_case("y") {
            return Ok(());
        }
    }

    // Delete the task
    repository.delete_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    Ok(())
}

/// Command handler for completing a task.
pub fn complete_task<R: Repository>(repository: Arc<R>, id: String) -> Result<Task, AppError> {
    // Fetch the existing task
    let mut task = repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    // Mark the task as completed
    task.complete();

    // Persist the updated task
    repository.update_task(&task).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
        RepositoryError::NotFound(msg) => AppError::UserError(msg),
    })?;

    Ok(task)
}

/// Command handler for reopening a completed task.
pub fn reopen_task<R: Repository>(repository: Arc<R>, id: String) -> Result<Task, AppError> {
    // Fetch the existing task
    let mut task = repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    // Mark the task as incomplete
    task.reopen();

    // Persist the updated task
    repository.update_task(&task).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
        RepositoryError::NotFound(msg) => AppError::UserError(msg),
    })?;

    Ok(task)
}

/// Command handler for completing a task with a trait object.
/// This version accepts &dyn Repository for dynamic dispatch.
pub fn complete_task_with_dyn(repository: &dyn Repository, id: String) -> Result<Task, AppError> {
    // Fetch the existing task
    let mut task = repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    // Mark the task as completed
    task.complete();

    // Persist the updated task
    repository.update_task(&task).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
        RepositoryError::NotFound(msg) => AppError::UserError(msg),
    })?;

    Ok(task)
}

/// Command handler for reopening a completed task with a trait object.
/// This version accepts &dyn Repository for dynamic dispatch.
pub fn reopen_task_with_dyn(repository: &dyn Repository, id: String) -> Result<Task, AppError> {
    // Fetch the existing task
    let mut task = repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    // Mark the task as incomplete
    task.reopen();

    // Persist the updated task
    repository.update_task(&task).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
        RepositoryError::NotFound(msg) => AppError::UserError(msg),
    })?;

    Ok(task)
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
    repository: Arc<R>,
    _filter: TagFilter,
) -> Result<Vec<TagWithCount>, AppError> {
    repository
        .list_tags(&TagFilter::default())
        .map_err(|e| match e {
            RepositoryError::NotFound(msg) => AppError::NotFound(msg),
            RepositoryError::Database(msg) => {
                AppError::System(crate::error::SystemError::Database(msg))
            }
            RepositoryError::Constraint(msg) => AppError::UserError(msg),
        })
}

/// Command handler for deleting a tag.
pub fn delete_tag<R: Repository>(repository: Arc<R>, name: String) -> Result<(), AppError> {
    repository.delete_tag(&name).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;
    Ok(())
}

/// This version accepts &dyn Repository for dynamic dispatch.
pub fn delete_tag_with_dyn(repository: &dyn Repository, name: String) -> Result<(), AppError> {
    repository.delete_tag(&name).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;
    Ok(())
}

/// Command handler for adding a tag to a task.
pub fn add_tag_to_task<R: Repository>(
    repository: Arc<R>,
    task_id: String,
    tag_name: String,
) -> Result<(), AppError> {
    // First, verify the task exists
    let _task = repository.get_task(&task_id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    // Try to get the tag by name (case-insensitive lookup via Tag::new)
    // Tag::new normalizes the name to lowercase
    let tag_name_lower = tag_name.to_lowercase();
    let tag = match repository.get_tag_by_name(&tag_name_lower) {
        Ok(existing_tag) => existing_tag,
        Err(RepositoryError::NotFound(_)) => {
            // Tag doesn't exist, create it
            let new_tag = Tag::new(tag_name_lower);
            repository.create_tag(&new_tag).map_err(|e| match e {
                RepositoryError::NotFound(msg) => AppError::NotFound(msg),
                RepositoryError::Database(msg) => {
                    AppError::System(crate::error::SystemError::Database(msg))
                }
                RepositoryError::Constraint(msg) => AppError::UserError(msg),
            })?;
            new_tag
        }
        Err(e) => {
            return Err(match e {
                RepositoryError::NotFound(msg) => AppError::NotFound(msg),
                RepositoryError::Database(msg) => {
                    AppError::System(crate::error::SystemError::Database(msg))
                }
                RepositoryError::Constraint(msg) => AppError::UserError(msg),
            });
        }
    };

    // Add the tag to the task
    repository
        .add_tag_to_task(&task_id, &tag.id)
        .map_err(|e| match e {
            RepositoryError::NotFound(msg) => AppError::NotFound(msg),
            RepositoryError::Database(msg) => {
                AppError::System(crate::error::SystemError::Database(msg))
            }
            RepositoryError::Constraint(msg) => AppError::UserError(msg),
        })
}

/// Command handler for removing a tag from a task.
pub fn remove_tag_from_task<R: Repository>(
    repository: Arc<R>,
    task_id: String,
    tag_name: String,
) -> Result<(), AppError> {
    // First, verify the task exists
    let _task = repository.get_task(&task_id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    // Get the tag by name (case-insensitive lookup)
    let tag_name_lower = tag_name.to_lowercase();
    let tag = repository
        .get_tag_by_name(&tag_name_lower)
        .map_err(|e| match e {
            RepositoryError::NotFound(msg) => AppError::NotFound(msg),
            RepositoryError::Database(msg) => {
                AppError::System(crate::error::SystemError::Database(msg))
            }
            RepositoryError::Constraint(msg) => AppError::UserError(msg),
        })?;

    // Get all tags on the task to verify the tag is actually on the task
    let task_tags = repository.get_task_tags(&task_id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })?;

    // Verify the tag is actually on the task
    let tag_exists_on_task = task_tags.iter().any(|t| t.id == tag.id);
    if !tag_exists_on_task {
        return Err(AppError::UserError(format!(
            "Tag '{}' is not attached to task '{}'",
            tag_name, task_id
        )));
    }

    // Remove the tag from the task
    repository
        .remove_tag_from_task(&task_id, &tag.id)
        .map_err(|e| match e {
            RepositoryError::NotFound(msg) => AppError::NotFound(msg),
            RepositoryError::Database(msg) => {
                AppError::System(crate::error::SystemError::Database(msg))
            }
            RepositoryError::Constraint(msg) => AppError::UserError(msg),
        })
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

    // ============ Tag Functionality Tests ============

    #[test]
    fn test_create_task_with_single_tag_creates_tag_and_links() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = Arc::new(SqliteRepository::new(&db_path).unwrap());
        repo.initialize().unwrap();

        // Create a task
        let task = Task::with_details(
            "Test task with tag".to_string(),
            Some("Description".to_string()),
            Priority::P1,
        );
        repo.create_task(&task).unwrap();

        // Create a tag and link to task
        let tag = Tag::new("work".to_string());
        repo.create_tag(&tag).unwrap();
        repo.add_tag_to_task(&task.id, &tag.id).unwrap();

        // Verify the tag was created in the database
        let retrieved_tag = repo.get_tag(&tag.id).unwrap();
        assert_eq!(retrieved_tag.name, "work");
        assert_eq!(retrieved_tag.id, tag.id);

        // Verify the tag is linked to the task
        let task_tags = repo.get_task_tags(&task.id).unwrap();
        assert_eq!(task_tags.len(), 1);
        assert_eq!(task_tags[0].name, "work");
    }

    #[test]
    fn test_create_task_with_multiple_tags() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = Arc::new(SqliteRepository::new(&db_path).unwrap());
        repo.initialize().unwrap();

        // Create a task
        let task = Task::with_details(
            "Test task with multiple tags".to_string(),
            Some("Description".to_string()),
            Priority::P2,
        );
        repo.create_task(&task).unwrap();

        // Create multiple tags and link to task
        let tag1 = Tag::new("work".to_string());
        let tag2 = Tag::new("urgent".to_string());
        let tag3 = Tag::new("project-a".to_string());

        repo.create_tag(&tag1).unwrap();
        repo.create_tag(&tag2).unwrap();
        repo.create_tag(&tag3).unwrap();

        repo.add_tag_to_task(&task.id, &tag1.id).unwrap();
        repo.add_tag_to_task(&task.id, &tag2.id).unwrap();
        repo.add_tag_to_task(&task.id, &tag3.id).unwrap();

        // Verify all tags are linked to the task
        let task_tags = repo.get_task_tags(&task.id).unwrap();
        assert_eq!(task_tags.len(), 3);

        let tag_names: Vec<String> = task_tags.iter().map(|t| t.name.clone()).collect();
        assert!(tag_names.contains(&"work".to_string()));
        assert!(tag_names.contains(&"urgent".to_string()));
        assert!(tag_names.contains(&"project-a".to_string()));
    }

    #[test]
    fn test_create_task_with_existing_tag_reuses_it() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = Arc::new(SqliteRepository::new(&db_path).unwrap());
        repo.initialize().unwrap();

        // Create first tag
        let existing_tag = Tag::new("work".to_string());
        repo.create_tag(&existing_tag).unwrap();

        // Create first task and link tag
        let task1 = Task::with_details(
            "First task".to_string(),
            Some("Description".to_string()),
            Priority::P1,
        );
        repo.create_task(&task1).unwrap();
        repo.add_tag_to_task(&task1.id, &existing_tag.id).unwrap();

        // Try to create another tag with same name - should fail due to unique constraint
        let result = repo.create_tag(&Tag::new("work".to_string()));
        assert!(result.is_err());

        // Instead, get the existing tag by name
        let retrieved_tag = repo.get_tag_by_name("work").unwrap();
        assert_eq!(retrieved_tag.name, "work");
        assert_eq!(retrieved_tag.id, existing_tag.id);

        // Create second task and link to existing tag
        let task2 = Task::with_details(
            "Second task".to_string(),
            Some("Description".to_string()),
            Priority::P2,
        );
        repo.create_task(&task2).unwrap();
        repo.add_tag_to_task(&task2.id, &retrieved_tag.id).unwrap();

        // Verify both tasks have the same tag
        let task1_tags = repo.get_task_tags(&task1.id).unwrap();
        let task2_tags = repo.get_task_tags(&task2.id).unwrap();

        assert_eq!(task1_tags.len(), 1);
        assert_eq!(task2_tags.len(), 1);
        assert_eq!(task1_tags[0].id, task2_tags[0].id);

        // Verify only one tag exists in the database
        let all_tags = repo
            .list_tags(&crate::filter::TagFilter::default())
            .unwrap();
        assert_eq!(all_tags.len(), 1);
    }

    #[test]
    fn test_tags_are_properly_associated_with_tasks() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = Arc::new(SqliteRepository::new(&db_path).unwrap());
        repo.initialize().unwrap();

        // Create tags
        let tag1 = Tag::new("work".to_string());
        let tag2 = Tag::new("personal".to_string());
        repo.create_tag(&tag1).unwrap();
        repo.create_tag(&tag2).unwrap();

        // Create two tasks with different tags
        let task1 = Task::with_details(
            "Work task".to_string(),
            Some("Work description".to_string()),
            Priority::P1,
        );
        let task2 = Task::with_details(
            "Personal task".to_string(),
            Some("Personal description".to_string()),
            Priority::P3,
        );

        repo.create_task(&task1).unwrap();
        repo.create_task(&task2).unwrap();

        // Link tags to tasks
        repo.add_tag_to_task(&task1.id, &tag1.id).unwrap();
        repo.add_tag_to_task(&task2.id, &tag2.id).unwrap();

        // Verify task1 has only "work" tag
        let task1_tags = repo.get_task_tags(&task1.id).unwrap();
        assert_eq!(task1_tags.len(), 1);
        assert_eq!(task1_tags[0].name, "work");

        // Verify task2 has only "personal" tag
        let task2_tags = repo.get_task_tags(&task2.id).unwrap();
        assert_eq!(task2_tags.len(), 1);
        assert_eq!(task2_tags[0].name, "personal");

        // Verify we can get tasks by tag
        let work_tasks = repo.get_tasks_by_tag(&tag1.id).unwrap();
        assert_eq!(work_tasks.len(), 1);
        assert_eq!(work_tasks[0].id, task1.id);

        let personal_tasks = repo.get_tasks_by_tag(&tag2.id).unwrap();
        assert_eq!(personal_tasks.len(), 1);
        assert_eq!(personal_tasks[0].id, task2.id);
    }

    #[test]
    fn test_remove_tag_case_insensitivity() {
        // Test that removing a tag works case-insensitively
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = Arc::new(SqliteRepository::new(&db_path).unwrap());
        repo.initialize().unwrap();

        // Create a task
        let task = Task::with_details(
            "Test task for case insensitive tag removal".to_string(),
            Some("Description".to_string()),
            Priority::P1,
        );
        repo.create_task(&task).unwrap();

        // Create a tag with mixed case - but it will be normalized to lowercase
        let tag = Tag::new("WorkHighPriority".to_string());
        repo.create_tag(&tag).unwrap();

        // Verify tag name is normalized to lowercase
        assert_eq!(tag.name, "workhighpriority");

        // Add tag to task
        repo.add_tag_to_task(&task.id, &tag.id).unwrap();

        // Verify tag is on task
        let task_tags_before = repo.get_task_tags(&task.id).unwrap();
        assert_eq!(task_tags_before.len(), 1);
        assert_eq!(task_tags_before[0].name, "workhighpriority");

        // Now remove the tag using lowercase (different case than original)
        let result = remove_tag_from_task(
            repo.clone(),
            task.id.clone(),
            "workhighpriority".to_string(),
        );
        assert!(
            result.is_ok(),
            "Should be able to remove tag with lowercase name"
        );

        // Verify tag is removed
        let task_tags_after = repo.get_task_tags(&task.id).unwrap();
        assert_eq!(task_tags_after.len(), 0);
    }

    #[test]
    fn test_remove_tag_with_different_case() {
        // Test removing a tag using uppercase when it was created with mixed case
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = Arc::new(SqliteRepository::new(&db_path).unwrap());
        repo.initialize().unwrap();

        // Create a task
        let task = Task::with_details(
            "Test task for uppercase removal".to_string(),
            Some("Description".to_string()),
            Priority::P2,
        );
        repo.create_task(&task).unwrap();

        // Create a tag with mixed case
        let tag = Tag::new("Urgent".to_string());
        repo.create_tag(&tag).unwrap();

        // Add tag to task
        repo.add_tag_to_task(&task.id, &tag.id).unwrap();

        // Try to remove using uppercase
        let result = remove_tag_from_task(repo.clone(), task.id.clone(), "URGENT".to_string());
        assert!(
            result.is_ok(),
            "Should be able to remove tag with uppercase name"
        );

        // Verify tag is removed
        let task_tags_after = repo.get_task_tags(&task.id).unwrap();
        assert_eq!(task_tags_after.len(), 0);
    }

    // ============ Delete Task Tests ============

    #[test]
    fn test_delete_task_with_dyn_deletes_existing_task() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = Arc::new(SqliteRepository::new(&db_path).unwrap());
        repo.initialize().unwrap();

        // Create a task first
        let task = Task::with_details(
            "Task to delete".to_string(),
            Some("Description".to_string()),
            Priority::P2,
        );
        repo.create_task(&task).unwrap();
        let task_id = task.id.clone();

        // Delete the task using delete_task_with_dyn with force=true
        let result = delete_task_with_dyn(repo.as_ref(), task_id.clone(), true);

        assert!(
            result.is_ok(),
            "Expected delete to succeed, got {:?}",
            result
        );

        // Verify the task is deleted
        let get_result = repo.get_task(&task_id);
        assert!(get_result.is_err());
    }

    #[test]
    fn test_delete_task_with_dyn_returns_not_found_for_non_existent() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = Arc::new(SqliteRepository::new(&db_path).unwrap());
        repo.initialize().unwrap();

        // Try to delete a task that doesn't exist
        let non_existent_id = "non-existent-task-id".to_string();
        let result = delete_task_with_dyn(repo.as_ref(), non_existent_id.clone(), true);

        assert!(result.is_err(), "Expected error for non-existent task");
        match result.unwrap_err() {
            AppError::NotFound(msg) => {
                assert!(msg.contains(&non_existent_id));
            }
            _ => panic!("Expected AppError::NotFound, got different error"),
        }
    }
}
