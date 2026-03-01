//! Repository module for TaskForge.
//!
//! This module defines the Repository trait for data persistence.

use crate::filter::{TagFilter, TaskFilter, TaskSort};
use crate::models::{Priority, Status, Tag, Task};
use chrono::{DateTime, Utc};
use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;

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

/// Helper function to convert DateTime<Utc> to String (ISO 8601)
fn datetime_to_string(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

/// Helper function to convert String (ISO 8601) to DateTime<Utc>
fn string_to_datetime(s: &str) -> Result<DateTime<Utc>, RepositoryError> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| RepositoryError::Database(format!("Invalid datetime format: {}", e)))
}

/// Helper function to convert Priority to i64
fn priority_to_i64(p: &Priority) -> i64 {
    match p {
        Priority::P1 => 1,
        Priority::P2 => 2,
        Priority::P3 => 3,
        Priority::P4 => 4,
    }
}

/// Helper function to convert i64 to Priority
fn i64_to_priority(n: i64) -> Priority {
    match n {
        1 => Priority::P1,
        2 => Priority::P2,
        3 => Priority::P3,
        4 => Priority::P4,
        _ => Priority::P3,
    }
}

/// Helper function to convert Status to String
fn status_to_string(s: &Status) -> String {
    match s {
        Status::Incomplete => "incomplete".to_string(),
        Status::Completed => "completed".to_string(),
    }
}

/// Helper function to convert String to Status
fn string_to_status(s: &str) -> Status {
    match s {
        "completed" => Status::Completed,
        _ => Status::Incomplete,
    }
}

/// SQLite implementation of the Repository
pub struct SqliteRepository {
    conn: Mutex<Connection>,
}

impl SqliteRepository {
    /// Creates a new SqliteRepository, creating the database file if it doesn't exist.
    pub fn new(path: &Path) -> Result<Self, RepositoryError> {
        let conn = Connection::open(path).map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Initializes the database schema, creating tables if they don't exist.
    pub fn initialize(&self) -> Result<(), RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                priority INTEGER NOT NULL DEFAULT 3,
                status TEXT NOT NULL DEFAULT 'incomplete',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                due_date TEXT
            );
            
            CREATE TABLE IF NOT EXISTS tags (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL UNIQUE COLLATE NOCASE,
                created_at TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS task_tags (
                task_id TEXT NOT NULL,
                tag_id TEXT NOT NULL,
                PRIMARY KEY (task_id, tag_id),
                FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
            CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority);
            CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date);
            CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at);
            CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name COLLATE NOCASE);
            "#,
        )
        .map_err(|e| RepositoryError::Database(e.to_string()))
    }
}

impl Repository for SqliteRepository {
    fn create_task(&self, task: &Task) -> Result<(), RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute(
            "INSERT INTO tasks (id, title, description, priority, status, created_at, updated_at, due_date) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (
                &task.id,
                &task.title,
                &task.description,
                priority_to_i64(&task.priority),
                status_to_string(&task.status),
                datetime_to_string(&task.created_at),
                datetime_to_string(&task.updated_at),
                task.due_date.as_ref().map(datetime_to_string),
            ),
        ).map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }

    fn get_task(&self, id: &str) -> Result<Task, RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, title, description, priority, status, created_at, updated_at, due_date 
             FROM tasks WHERE id = ?1",
            )
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let task = stmt
            .query_row([id], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    priority: i64_to_priority(row.get(3)?),
                    status: string_to_status(&row.get::<_, String>(4)?),
                    created_at: string_to_datetime(&row.get::<_, String>(5)?)
                        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
                    updated_at: string_to_datetime(&row.get::<_, String>(6)?)
                        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
                    due_date: row
                        .get::<_, Option<String>>(7)?
                        .map(|s| {
                            string_to_datetime(&s)
                                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))
                        })
                        .transpose()?,
                })
            })
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => RepositoryError::NotFound(id.to_string()),
                _ => RepositoryError::Database(e.to_string()),
            })?;

        Ok(task)
    }

    fn list_tasks(
        &self,
        _filter: &TaskFilter,
        _sort: &TaskSort,
    ) -> Result<Vec<Task>, RepositoryError> {
        // For now, just return all tasks ignoring filter and sort
        self.list_tasks_all()
    }

    fn list_tasks_all(&self) -> Result<Vec<Task>, RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, description, priority, status, created_at, updated_at, due_date FROM tasks"
        ).map_err(|e| RepositoryError::Database(e.to_string()))?;

        let tasks = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    priority: i64_to_priority(row.get(3)?),
                    status: string_to_status(&row.get::<_, String>(4)?),
                    created_at: string_to_datetime(&row.get::<_, String>(5)?)
                        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
                    updated_at: string_to_datetime(&row.get::<_, String>(6)?)
                        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
                    due_date: row
                        .get::<_, Option<String>>(7)?
                        .map(|s| {
                            string_to_datetime(&s)
                                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))
                        })
                        .transpose()?,
                })
            })
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(tasks)
    }

    fn update_task(&self, task: &Task) -> Result<(), RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let rows_affected = conn
            .execute(
                "UPDATE tasks SET title = ?1, description = ?2, priority = ?3, status = ?4, 
             updated_at = ?5, due_date = ?6 WHERE id = ?7",
                (
                    &task.title,
                    &task.description,
                    priority_to_i64(&task.priority),
                    status_to_string(&task.status),
                    datetime_to_string(&task.updated_at),
                    task.due_date.as_ref().map(datetime_to_string),
                    &task.id,
                ),
            )
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        if rows_affected == 0 {
            return Err(RepositoryError::NotFound(format!(
                "Task with id {} not found",
                task.id
            )));
        }

        Ok(())
    }

    fn delete_task(&self, id: &str) -> Result<(), RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute("DELETE FROM tasks WHERE id = ?1", [id])
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }

    fn get_tasks_by_status(&self, status: Status) -> Result<Vec<Task>, RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, description, priority, status, created_at, updated_at, due_date FROM tasks WHERE status = ?1"
        ).map_err(|e| RepositoryError::Database(e.to_string()))?;

        let tasks = stmt
            .query_map([status_to_string(&status)], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    priority: i64_to_priority(row.get(3)?),
                    status: string_to_status(&row.get::<_, String>(4)?),
                    created_at: string_to_datetime(&row.get::<_, String>(5)?)
                        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
                    updated_at: string_to_datetime(&row.get::<_, String>(6)?)
                        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
                    due_date: row
                        .get::<_, Option<String>>(7)?
                        .map(|s| {
                            string_to_datetime(&s)
                                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))
                        })
                        .transpose()?,
                })
            })
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(tasks)
    }

    fn create_tag(&self, tag: &Tag) -> Result<(), RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute(
            "INSERT INTO tags (id, name, created_at) VALUES (?1, ?2, ?3)",
            (&tag.id, &tag.name, datetime_to_string(&tag.created_at)),
        )
        .map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                RepositoryError::Constraint(format!("Tag '{}' already exists", tag.name))
            } else {
                RepositoryError::Database(e.to_string())
            }
        })?;
        Ok(())
    }

    fn get_tag(&self, id: &str) -> Result<Tag, RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, name, created_at FROM tags WHERE id = ?1")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        stmt.query_row([id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: None,
                created_at: string_to_datetime(&row.get::<_, String>(2)?)
                    .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
            })
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => RepositoryError::NotFound(id.to_string()),
            _ => RepositoryError::Database(e.to_string()),
        })
    }

    fn get_tag_by_name(&self, name: &str) -> Result<Tag, RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, name, created_at FROM tags WHERE name = ?1")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        stmt.query_row([name.to_lowercase()], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: None,
                created_at: string_to_datetime(&row.get::<_, String>(2)?)
                    .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
            })
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => RepositoryError::NotFound(name.to_string()),
            _ => RepositoryError::Database(e.to_string()),
        })
    }

    fn update_tag(&self, _tag: &Tag) -> Result<(), RepositoryError> {
        Err(RepositoryError::Database("Not implemented".to_string()))
    }

    fn delete_tag(&self, _id: &str) -> Result<(), RepositoryError> {
        Err(RepositoryError::Database("Not implemented".to_string()))
    }

    fn list_tags(&self, _filter: &TagFilter) -> Result<Vec<Tag>, RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, name, created_at FROM tags")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let tags = stmt
            .query_map([], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: None,
                    created_at: string_to_datetime(&row.get::<_, String>(2)?)
                        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
                })
            })
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(tags)
    }

    fn add_tag_to_task(&self, task_id: &str, tag_id: &str) -> Result<(), RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute(
            "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
            [task_id, tag_id],
        )
        .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }

    fn remove_tag_from_task(&self, task_id: &str, tag_id: &str) -> Result<(), RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = conn
            .execute(
                "DELETE FROM task_tags WHERE task_id = ?1 AND tag_id = ?2",
                [task_id, tag_id],
            )
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        if rows == 0 {
            return Err(RepositoryError::NotFound(format!(
                "Tag {} not found on task {}",
                tag_id, task_id
            )));
        }
        Ok(())
    }

    fn get_task_tags(&self, task_id: &str) -> Result<Vec<Tag>, RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.created_at \n            FROM tags t \n            INNER JOIN task_tags tt ON t.id = tt.tag_id \n            WHERE tt.task_id = ?1"
        ).map_err(|e| RepositoryError::Database(e.to_string()))?;

        let tags = stmt
            .query_map([task_id], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: None,
                    created_at: string_to_datetime(&row.get::<_, String>(2)?)
                        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
                })
            })
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(tags)
    }

    fn get_tasks_by_tag(&self, tag_id: &str) -> Result<Vec<Task>, RepositoryError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT t.id, t.title, t.description, t.priority, t.status, t.created_at, t.updated_at, t.due_date \n            FROM tasks t \n            INNER JOIN task_tags tt ON t.id = tt.task_id \n            WHERE tt.tag_id = ?1"
        ).map_err(|e| RepositoryError::Database(e.to_string()))?;

        let tasks = stmt
            .query_map([tag_id], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    priority: i64_to_priority(row.get(3)?),
                    status: string_to_status(&row.get::<_, String>(4)?),
                    created_at: string_to_datetime(&row.get::<_, String>(5)?)
                        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
                    updated_at: string_to_datetime(&row.get::<_, String>(6)?)
                        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?,
                    due_date: row
                        .get::<_, Option<String>>(7)?
                        .map(|s| {
                            string_to_datetime(&s)
                                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))
                        })
                        .transpose()?,
                })
            })
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(tasks)
    }
}

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

    /// Lists all tasks without filtering or sorting.
    fn list_tasks_all(&self) -> Result<Vec<Task>, RepositoryError>;

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

    use tempfile::TempDir;

    #[test]
    fn test_sqlite_repository_new_creates_database_file() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = SqliteRepository::new(&db_path).unwrap();

        assert!(db_path.exists(), "Database file should be created");
    }

    #[test]
    fn test_sqlite_repository_initializes_schema() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = SqliteRepository::new(&db_path).unwrap();
        repo.initialize().unwrap();

        let conn = rusqlite::Connection::open(&db_path).unwrap();

        // Verify tables exist
        conn.execute("SELECT * FROM tasks", []).unwrap();
        conn.execute("SELECT * FROM tags", []).unwrap();
        conn.execute("SELECT * FROM task_tags", []).unwrap();
    }

    #[test]
    fn test_create_and_retrieve_task() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = SqliteRepository::new(&db_path).unwrap();
        repo.initialize().unwrap();

        let task = Task::new("Test Task".to_string());
        repo.create_task(&task).unwrap();

        let retrieved = repo.get_task(&task.id).unwrap();

        assert_eq!(retrieved.id, task.id);
        assert_eq!(retrieved.title, task.title);
    }

    #[test]
    fn test_create_and_retrieve_tag() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = SqliteRepository::new(&db_path).unwrap();
        repo.initialize().unwrap();

        let tag = Tag::new("test-tag".to_string());
        repo.create_tag(&tag).unwrap();

        let retrieved = repo.get_tag(&tag.id).unwrap();

        assert_eq!(retrieved.id, tag.id);
        assert_eq!(retrieved.name, tag.name);
    }

    #[test]
    fn test_list_tasks_returns_all() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = SqliteRepository::new(&db_path).unwrap();
        repo.initialize().unwrap();

        let task1 = Task::new("Task 1".to_string());
        let task2 = Task::new("Task 2".to_string());
        repo.create_task(&task1).unwrap();
        repo.create_task(&task2).unwrap();

        let tasks = repo.list_tasks_all().unwrap();

        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_tag_task_relationship() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let repo = SqliteRepository::new(&db_path).unwrap();
        repo.initialize().unwrap();

        let task = Task::new("Test Task".to_string());
        let tag = Tag::new("test-tag".to_string());
        repo.create_task(&task).unwrap();
        repo.create_tag(&tag).unwrap();

        repo.add_tag_to_task(&task.id, &tag.id).unwrap();

        let tags = repo.get_task_tags(&task.id).unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "test-tag");

        let tasks = repo.get_tasks_by_tag(&tag.id).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Test Task");
    }
}
