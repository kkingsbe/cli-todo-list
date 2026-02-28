//! Task entity module for TaskForge.
//!
//! This module defines the Task struct and related types.

use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Priority levels for tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    /// Highest priority (P1)
    P1 = 1,
    /// High priority (P2)
    P2 = 2,
    /// Medium priority (P3)
    #[default]
    P3 = 3,
    /// Low priority (P4)
    P4 = 4,
}

impl Priority {
    /// Returns the numeric value of the priority.
    pub fn value(&self) -> u8 {
        match self {
            Priority::P1 => 1,
            Priority::P2 => 2,
            Priority::P3 => 3,
            Priority::P4 => 4,
        }
    }
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "P1" => Ok(Priority::P1),
            "P2" => Ok(Priority::P2),
            "P3" => Ok(Priority::P3),
            "P4" => Ok(Priority::P4),
            _ => Err(format!(
                "Invalid priority: {}. Expected P1, P2, P3, or P4.",
                s
            )),
        }
    }
}

/// Status of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Task is incomplete.
    #[default]
    Incomplete,
    /// Task is completed.
    Completed,
}

/// A task entity with all its fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier (UUID v4).
    pub id: String,
    /// Task title (required, max 500 chars).
    pub title: String,
    /// Optional task description.
    pub description: Option<String>,
    /// Task priority (P1-P4).
    pub priority: Priority,
    /// Task completion status.
    pub status: Status,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
    /// Optional due date.
    pub due_date: Option<DateTime<Utc>>,
}

impl Task {
    /// Creates a new task with the given title.
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description: None,
            priority: Priority::default(),
            status: Status::default(),
            created_at: now,
            updated_at: now,
            due_date: None,
        }
    }

    /// Creates a task with all fields specified.
    pub fn with_details(title: String, description: Option<String>, priority: Priority) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description,
            priority,
            status: Status::default(),
            created_at: now,
            updated_at: now,
            due_date: None,
        }
    }

    /// Marks the task as completed.
    pub fn complete(&mut self) {
        self.status = Status::Completed;
        self.updated_at = Utc::now();
    }

    /// Marks the task as incomplete.
    pub fn reopen(&mut self) {
        self.status = Status::Incomplete;
        self.updated_at = Utc::now();
    }

    /// Updates the task title.
    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    /// Updates the task priority.
    pub fn update_priority(&mut self, priority: Priority) {
        self.priority = priority;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_new_creates_task_with_defaults() {
        let task = Task::new("Test task".to_string());
        assert!(!task.id.is_empty());
        assert_eq!(task.title, "Test task");
        assert_eq!(task.priority, Priority::P3);
        assert_eq!(task.status, Status::Incomplete);
        assert!(task.description.is_none());
    }

    #[test]
    fn task_with_details_creates_task() {
        let task = Task::with_details(
            "Test task".to_string(),
            Some("Description".to_string()),
            Priority::P1,
        );
        assert!(!task.id.is_empty());
        assert_eq!(task.title, "Test task");
        assert_eq!(task.description, Some("Description".to_string()));
        assert_eq!(task.priority, Priority::P1);
    }

    #[test]
    fn task_complete_changes_status() {
        let mut task = Task::new("Test".to_string());
        assert_eq!(task.status, Status::Incomplete);
        task.complete();
        assert_eq!(task.status, Status::Completed);
    }

    #[test]
    fn task_reopen_changes_status() {
        let mut task = Task::new("Test".to_string());
        task.complete();
        task.reopen();
        assert_eq!(task.status, Status::Incomplete);
    }

    #[test]
    fn task_update_title_changes_title() {
        let mut task = Task::new("Old title".to_string());
        task.update_title("New title".to_string());
        assert_eq!(task.title, "New title");
    }

    #[test]
    fn task_update_priority_changes_priority() {
        let mut task = Task::new("Test".to_string());
        task.update_priority(Priority::P1);
        assert_eq!(task.priority, Priority::P1);
    }

    #[test]
    fn priority_value_returns_correct_value() {
        assert_eq!(Priority::P1.value(), 1);
        assert_eq!(Priority::P2.value(), 2);
        assert_eq!(Priority::P3.value(), 3);
        assert_eq!(Priority::P4.value(), 4);
    }

    #[test]
    fn priority_default_returns_p3() {
        assert_eq!(Priority::default(), Priority::P3);
    }

    #[test]
    fn status_default_returns_incomplete() {
        assert_eq!(Status::default(), Status::Incomplete);
    }

    #[test]
    fn priority_from_str_parses_p1() {
        assert_eq!("P1".parse::<Priority>(), Ok(Priority::P1));
    }

    #[test]
    fn priority_from_str_parses_all_values() {
        assert_eq!("P1".parse::<Priority>(), Ok(Priority::P1));
        assert_eq!("P2".parse::<Priority>(), Ok(Priority::P2));
        assert_eq!("P3".parse::<Priority>(), Ok(Priority::P3));
        assert_eq!("P4".parse::<Priority>(), Ok(Priority::P4));
    }

    #[test]
    fn priority_from_str_returns_error_for_invalid() {
        assert!("P5".parse::<Priority>().is_err());
        assert!("invalid".parse::<Priority>().is_err());
    }

    #[test]
    fn status_serializes_to_lowercase() {
        use serde_json;
        let incomplete = Status::Incomplete;
        let completed = Status::Completed;

        let incomplete_json = serde_json::to_string(&incomplete).unwrap();
        let completed_json = serde_json::to_string(&completed).unwrap();

        assert_eq!(incomplete_json, "\"incomplete\"");
        assert_eq!(completed_json, "\"completed\"");
    }
}
