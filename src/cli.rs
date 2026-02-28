//! CLI module for TaskForge.
//!
//! This module defines the CLI argument structure using clap.

use clap::{Parser, Subcommand};

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
    },

    /// Show a task.
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
    },

    /// Delete a task.
    Delete {
        /// Task ID.
        id: String,
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

    #[test]
    fn cli_parse_add_command() {
        let cli = Cli::parse_from(&["taskforge", "add", "My task", "--priority", "1"]);
        match cli.command {
            Commands::Add { title, priority, .. } => {
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
            Commands::Delete { id } => {
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
}
