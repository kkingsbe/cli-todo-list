//! Commands module for TaskForge.
//!
//! This module defines command handlers for CLI operations.

use crate::cli::OutputFormat;
use crate::error::AppError;
use crate::filter::{TagFilter, TaskFilter, TaskSort};
use crate::models::{Priority, Tag, Task};
use crate::repository::{Repository, RepositoryError};
use std::sync::Arc;

/// Format and print a list of tasks in the specified output format.
pub fn format_task_list(tasks: &[Task], format: OutputFormat, limit: u32) {
    match format {
        OutputFormat::Table => {
            // Print table header
            println!(
                "{:<38} | {:<30} | {:^8} | {:^12}",
                "ID", "TITLE", "PRIORITY", "STATUS"
            );
            println!(
                "{:-<38}-+-{:-<30 }-+-{:-<8 }-+-{:-<12}",
                "", "", "", ""
            );

            // Print each task
            for task in tasks {
                let title = if task.title.len() > 30 {
                    format!("{}...", &task.title[..27])
                } else {
                    task.title.clone()
                };
                println!(
                    "{:<38} | {:<30} | {:^8} | {:^12}",
                    &task.id[..8],
                    title,
                    format!("{:?}", task.priority),
                    format!("{:?}", task.status)
                );
            }

            println!("\nTotal: {} task(s)", limit);
        }
        OutputFormat::Plain => {
            // Print plain text output
            for task in tasks {
                println!(
                    "[#{}] {} - Status: {:?}, Priority: {:?}",
                    &task.id[..8],
                    task.title,
                    task.status,
                    task.priority
                );
            }
            println!("\nTotal: {} task(s)", limit);
        }
        OutputFormat::Json => {
            // Output as JSON array
            let json = serde_json::to_string_pretty(&tasks).expect("Failed to serialize tasks");
            println!("{}", json);
        }
    }
}

/// Format and print a single task's details.
pub fn format_task_detail(task: &Task) {
    println!("Task Details:");
    println!("  ID:          {}", task.id);
    println!("  Title:       {}", task.title);
    println!(
        "  Description: {}",
        task.description.as_deref().unwrap_or("N/A")
    );
    println!("  Priority:    {:?}", task.priority);
    println!("  Status:      {:?}", task.status);
    println!("  Created:     {}", task.created_at);
    println!("  Updated:     {}", task.updated_at);
    println!(
        "  Due Date:    {}",
        task.due_date
            .map(|d| d.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );
}

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
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        _ => AppError::UserError(e.to_string()),
    })
}

/// Command handler for listing tasks with a trait object.
/// This version accepts Arc<dyn Repository> for dynamic dispatch.
pub fn list_tasks_with_dyn(
    repository: &dyn Repository,
    filter: TaskFilter,
    sort: TaskSort,
) -> Result<Vec<Task>, AppError> {
    repository.list_tasks(&filter, &sort).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        _ => AppError::UserError(e.to_string()),
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

/// Command handler for getting or creating a tag by name.
/// If the tag exists, returns the existing tag. If not, creates a new tag.
pub fn get_or_create_tag<R: Repository>(repository: Arc<R>, name: String) -> Result<Tag, AppError> {
    // Try to get the tag by name first
    match repository.get_tag_by_name(&name) {
        Ok(tag) => Ok(tag),
        Err(e) => match e {
            RepositoryError::NotFound(_) => {
                // Tag not found, create a new one
                let tag = Tag::new(name);
                repository.create_tag(&tag).map_err(|e| match e {
                    RepositoryError::Database(msg) => {
                        AppError::System(crate::error::SystemError::Database(msg))
                    }
                    RepositoryError::Constraint(msg) => AppError::UserError(msg),
                    _ => AppError::UserError(e.to_string()),
                })?;
                Ok(tag)
            }
            RepositoryError::Database(msg) => {
                Err(AppError::System(crate::error::SystemError::Database(msg)))
            }
            RepositoryError::Constraint(msg) => Err(AppError::UserError(msg)),
        },
    }
}

/// Command handler for getting or creating a tag by name with a trait object.
/// If the tag exists, returns the existing tag. If not, creates a new tag.
pub fn get_or_create_tag_with_dyn(
    repository: &dyn Repository,
    name: String,
) -> Result<Tag, AppError> {
    // Try to get the tag by name first
    match repository.get_tag_by_name(&name) {
        Ok(tag) => Ok(tag),
        Err(e) => match e {
            RepositoryError::NotFound(_) => {
                // Tag not found, create a new one
                let tag = Tag::new(name);
                repository.create_tag(&tag).map_err(|e| match e {
                    RepositoryError::Database(msg) => {
                        AppError::System(crate::error::SystemError::Database(msg))
                    }
                    RepositoryError::Constraint(msg) => AppError::UserError(msg),
                    _ => AppError::UserError(e.to_string()),
                })?;
                Ok(tag)
            }
            RepositoryError::Database(msg) => {
                Err(AppError::System(crate::error::SystemError::Database(msg)))
            }
            RepositoryError::Constraint(msg) => Err(AppError::UserError(msg)),
        },
    }
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
    repository: Arc<R>,
    task_id: String,
    tag_id: String,
) -> Result<(), AppError> {
    repository.add_tag_to_task(&task_id, &tag_id).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        _ => AppError::UserError(e.to_string()),
    })
}

/// Command handler for adding a tag to a task with a trait object.
pub fn add_tag_to_task_with_dyn(
    repository: &dyn Repository,
    task_id: String,
    tag_id: String,
) -> Result<(), AppError> {
    repository.add_tag_to_task(&task_id, &tag_id).map_err(|e| match e {
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        _ => AppError::UserError(e.to_string()),
    })
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
