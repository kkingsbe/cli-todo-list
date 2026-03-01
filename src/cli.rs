//! CLI module for TaskForge.
//!
//! This module defines the CLI argument structure using clap.

use clap::{Parser, Subcommand};

/// Output format for list command.
#[derive(Clone, Debug, Default, PartialEq, clap::ValueEnum)]
pub enum OutputFormat {
    #[default]
    Table,
    Plain,
    Json,
}

/// Command-line interface for TaskForge.
#[derive(Debug, Parser)]
#[command(name = "taskforge")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A CLI task management application", long_about = None)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI subcommands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new task.
    Add {
        /// Task title.
        title: String,

        /// Task description.
        #[arg(short, long)]
        description: Option<String>,

        /// Task priority (1-4, where 1 is highest).
        #[arg(short, long, default_value = "3")]
        priority: u8,

        /// Task tags (can be specified multiple times).
        #[arg(short, long)]
        tag: Option<Vec<String>>,
    },

    /// List tasks.
    List {
        /// Filter by status (complete/incomplete).
        #[arg(long)]
        status: Option<String>,

        /// Filter by priority.
        #[arg(short, long)]
        priority: Option<u8>,

        /// Search term.
        #[arg(long)]
        search: Option<String>,

        /// Sort field (created, priority, due, title).
        #[arg(short, long, default_value = "created")]
        sort_by: String,

        /// Sort order (asc/desc).
        #[arg(short, long, default_value = "asc")]
        order: String,

        /// Output format (table, plain, json).
        #[arg(long, value_enum, default_value = "table")]
        format: OutputFormat,

        /// Filter tasks due before this date (YYYY-MM-DD).
        #[arg(long)]
        due_before: Option<String>,

        /// Filter tasks due after this date (YYYY-MM-DD).
        #[arg(long)]
        due_after: Option<String>,

        /// Filter tasks due on this exact date (YYYY-MM-DD).
        #[arg(long)]
        due: Option<String>,
    },

    /// Show a task.
    #[command(alias = "get")]
    Show {
        /// Task ID.
        id: String,
    },

    /// Update a task.
    Edit {
        /// Task ID.
        id: String,

        /// New title.
        #[arg(short, long)]
        title: Option<String>,

        /// New description.
        #[arg(short, long)]
        description: Option<String>,

        /// New priority (1-4).
        #[arg(short, long)]
        priority: Option<u8>,

        /// New status (completed/incomplete).
        #[arg(long)]
        status: Option<String>,

        /// New due date (YYYY-MM-DD).
        #[arg(long)]
        due: Option<String>,

        /// Tag to add to the task.
        #[arg(long = "add-tag", value_name = "NAME")]
        add_tag: Option<String>,

        /// Tag to remove from the task.
        #[arg(long = "remove-tag", value_name = "NAME")]
        remove_tag: Option<String>,
    },

    /// Delete a task.
    Delete {
        /// Task ID.
        id: String,
        /// Force deletion without confirmation
        #[arg(long, short)]
        force: bool,
    },

    /// Complete a task.
    Complete {
        /// Task ID.
        id: String,
    },

    /// Reopen a completed task.
    Reopen {
        /// Task ID.
        id: String,
    },

    /// Tag management.
    Tag {
        #[command(subcommand)]
        command: TagCommands,
    },

    /// List all tags with usage count.
    Tags,
}

/// Tag-related subcommands.
#[derive(Debug, Subcommand)]
pub enum TagCommands {
    /// Create a new tag.
    Create {
        /// Tag name.
        name: String,

        /// Tag color (hex code).
        #[arg(short, long)]
        color: Option<String>,
    },

    /// List all tags.
    List,

    /// Delete a tag.
    Delete {
        /// Tag ID or name.
        identifier: String,
    },

    /// Add a tag to a task.
    Add {
        /// Task ID.
        task_id: String,

        /// Tag ID or name.
        tag_identifier: String,
    },

    /// Remove a tag from a task.
    Remove {
        /// Task ID.
        task_id: String,

        /// Tag ID or name.
        tag_identifier: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Task;
    use crate::task::{Priority, Status};
    use chrono::Utc;

    #[test]
    fn cli_parse_add_command() {
        let cli = Cli::parse_from(&["taskforge", "add", "My task", "--priority", "1"]);
        match cli.command {
            Commands::Add {
                title, priority, ..
            } => {
                assert_eq!(title, "My task");
                assert_eq!(priority, 1);
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn cli_parse_list_command() {
        let cli = Cli::parse_from(&["taskforge", "list", "--status", "incomplete"]);
        match cli.command {
            Commands::List { status, .. } => {
                assert_eq!(status, Some("incomplete".to_string()));
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn cli_parse_delete_command() {
        let cli = Cli::parse_from(&["taskforge", "delete", "task-123"]);
        match cli.command {
            Commands::Delete { id, .. } => {
                assert_eq!(id, "task-123");
            }
            _ => panic!("Expected Delete command"),
        }
    }

    #[test]
    fn cli_parse_tag_create() {
        let cli = Cli::parse_from(&["taskforge", "tag", "create", "work", "--color", "#FF0000"]);
        match cli.command {
            Commands::Tag { command } => match command {
                TagCommands::Create { name, color } => {
                    assert_eq!(name, "work");
                    assert_eq!(color, Some("#FF0000".to_string()));
                }
                _ => panic!("Expected Tag Create command"),
            },
            _ => panic!("Expected Tag command"),
        }
    }

    // ========== Output Format Tests ==========

    #[test]
    fn cli_parse_list_format_table() {
        let cli = Cli::parse_from(&["taskforge", "list", "--format", "table"]);
        match cli.command {
            Commands::List { format, .. } => {
                assert_eq!(format, OutputFormat::Table);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn cli_parse_list_format_plain() {
        let cli = Cli::parse_from(&["taskforge", "list", "--format", "plain"]);
        match cli.command {
            Commands::List { format, .. } => {
                assert_eq!(format, OutputFormat::Plain);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn cli_parse_list_format_json() {
        let cli = Cli::parse_from(&["taskforge", "list", "--format", "json"]);
        match cli.command {
            Commands::List { format, .. } => {
                assert_eq!(format, OutputFormat::Json);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn cli_list_default_format_is_table() {
        let cli = Cli::parse_from(&["taskforge", "list"]);
        match cli.command {
            Commands::List { format, .. } => {
                assert_eq!(format, OutputFormat::Table);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn output_format_default_is_table() {
        assert_eq!(OutputFormat::default(), OutputFormat::Table);
    }

    #[test]
    fn output_format_display_table() {
        assert_eq!(format!("{:?}", OutputFormat::Table), "Table");
    }

    #[test]
    fn output_format_display_plain() {
        assert_eq!(format!("{:?}", OutputFormat::Plain), "Plain");
    }

    #[test]
    fn output_format_display_json() {
        assert_eq!(format!("{:?}", OutputFormat::Json), "Json");
    }

    #[test]
    fn output_format_clone() {
        let format = OutputFormat::Json;
        let cloned = format.clone();
        assert_eq!(format, cloned);
    }

    #[test]
    fn output_format_partial_eq() {
        assert_eq!(OutputFormat::Table, OutputFormat::Table);
        assert_eq!(OutputFormat::Plain, OutputFormat::Plain);
        assert_eq!(OutputFormat::Json, OutputFormat::Json);
        assert_ne!(OutputFormat::Table, OutputFormat::Plain);
        assert_ne!(OutputFormat::Plain, OutputFormat::Json);
    }

    #[test]
    fn test_json_format_produces_valid_json() {
        // Create a test task
        let task = Task {
            id: "test-id-123".to_string(),
            title: "Test Task".to_string(),
            description: Some("Test description".to_string()),
            priority: Priority::P1,
            status: Status::Incomplete,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_date: None,
        };

        // Serialize to JSON
        let json_output = serde_json::to_string(&task).expect("Failed to serialize task to JSON");

        // Verify it's valid JSON by parsing it back
        let parsed: Task =
            serde_json::from_str(&json_output).expect("Failed to parse JSON back to Task");

        // Verify the parsed task matches the original
        assert_eq!(parsed.id, task.id);
        assert_eq!(parsed.title, task.title);
        assert_eq!(parsed.description, task.description);
        assert_eq!(parsed.priority, task.priority);
        assert_eq!(parsed.status, task.status);
    }

    #[test]
    fn test_json_format_produces_valid_json_for_multiple_tasks() {
        // Create multiple test tasks
        let tasks = vec![
            Task {
                id: "task-1".to_string(),
                title: "First Task".to_string(),
                description: Some("Description 1".to_string()),
                priority: Priority::P1,
                status: Status::Incomplete,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                due_date: None,
            },
            Task {
                id: "task-2".to_string(),
                title: "Second Task".to_string(),
                description: None,
                priority: Priority::P2,
                status: Status::Completed,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                due_date: None,
            },
        ];

        // Serialize to JSON (array)
        let json_output = serde_json::to_string(&tasks).expect("Failed to serialize tasks to JSON");

        // Verify the JSON array starts with [ and ends with ]
        assert!(json_output.starts_with('['));
        assert!(json_output.ends_with(']'));

        // Verify it's valid JSON by parsing it back
        let parsed: Vec<Task> =
            serde_json::from_str(&json_output).expect("Failed to parse JSON back to Tasks");

        // Verify we got both tasks back
        assert_eq!(parsed.len(), 2);
    }

    #[test]
    fn test_json_format_empty_array() {
        let empty_tasks: Vec<Task> = vec![];
        let json_output =
            serde_json::to_string(&empty_tasks).expect("Failed to serialize empty tasks to JSON");

        assert_eq!(json_output, "[]");
    }
}
