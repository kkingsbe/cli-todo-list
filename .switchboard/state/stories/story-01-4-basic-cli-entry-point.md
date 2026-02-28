# Story 01.4: Basic CLI Entry Point

> Epic: epic-01 — Project Scaffolding
> Points: 2
> Sprint: 2
> Type: infrastructure
> Risk: Low
> Created: 2026-02-28T18:35:03Z

## User Story

As a user,  
I want to see a help message when running `task --help`,  
So that I know how to use the CLI.

## Acceptance Criteria

1. CLI module defines --help and --version flags  
   - **Test:** `cargo run -- --help` shows help text

2. Basic clap derive setup in `src/cli.rs`  
   - **Test:** Binary runs and responds to --help

3. Empty main command that returns success  
   - **Test:** `cargo run` exits with code 0

## Technical Context

### Architecture Reference
- Layered architecture: CLI → Commands → Domain → Repository
- ADR-005: Command pattern for CLI operations

### Project Conventions
- Build/Test: `cargo build --release`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- Tech Stack: Rust 1.75+, clap 4.x
- All public functions must have doc comments (`///`)

### Existing Code Context

The file `src/cli.rs` already exists. Current content:

```rust
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
```

The file `src/main.rs` already exists. Current content:

```rust
#![allow(dead_code)]
//! TaskForge - CLI task management application
//!
//! Entry point for the TaskForge CLI tool.

use anyhow::Result;
use clap::Parser;
use std::env;

mod cli;
mod commands;
mod config;
mod error;
mod filter;
mod models;
mod repository;
mod tag;
mod task;

use crate::cli::{Cli, Commands};

fn main() -> Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // If no arguments provided (besides program name), exit successfully
    // This allows `cargo run` to exit cleanly
    if args.len() <= 1 {
        return Ok(());
    }

    // Parse command line arguments
    // Cli::parse() will handle --help and --version automatically
    // and display appropriate messages
    let cli = Cli::parse();

    // Handle the parsed commands
    match cli.command {
        Commands::Add { .. } => {
            // TODO: Implement add command
            println!("Add command not yet implemented");
        }
        Commands::List { .. } => {
            // TODO: Implement list command
            println!("List command not yet implemented");
        }
        Commands::Show { .. } => {
            // TODO: Implement show command
            println!("Show command not yet implemented");
        }
        Commands::Edit { .. } => {
            // TODO: Implement edit command
            println!("Edit command not yet implemented");
        }
        Commands::Delete { .. } => {
            // TODO: Implement delete command
            println!("Delete command not yet implemented");
        }
        Commands::Complete { .. } => {
            // TODO: Implement complete command
            println!("Complete command not yet implemented");
        }
        Commands::Reopen { .. } => {
            // TODO: Implement reopen command
            println!("Reopen command not yet implemented");
        }
        Commands::Tag { .. } => {
            // TODO: Implement tag command
            println!("Tag command not yet implemented");
        }
    }

    Ok(())
}
```

## Implementation Plan

1. **Review existing `src/cli.rs`** — Verify it has proper clap derive setup
2. **Review existing `src/main.rs`** — Verify CLI integration
3. **Run `cargo run -- --help`** — Verify help displays
4. **Run `cargo run -- --version`** — Verify version displays
5. **Run `cargo run`** — Verify exits with code 0
6. **Run `cargo clippy -- -D warnings`** — Fix any warnings

### Dependencies
- Story 01.3 (Setup Logging and Error Handling)

## Scope Boundaries

### This Story Includes
- Basic CLI entry point with --help and --version
- Clap derive setup

### This Story Does NOT Include
- Implementing actual commands (belongs to Epic 03)

### Files in Scope
- `src/cli.rs` — modify
- `src/main.rs` — modify
