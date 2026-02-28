# Story 01.3: Setup Logging and Error Handling

> Epic: epic-01 — Project Scaffolding
> Points: 2
> Sprint: 2
> Type: infrastructure
> Risk: Low
> Created: 2026-02-28T18:35:03Z

## User Story

As a developer,  
I want logging and error handling infrastructure in place,  
So that I can debug issues and handle errors gracefully.

## Acceptance Criteria

1. `src/error.rs` defines `AppError` using thiserror with `UserError`, `ValidationError`, `SystemError` variants  
   - **Test:** `cargo check` succeeds, error types are public

2. Logging is initialized in `src/main.rs` using tracing  
   - **Test:** Application runs without panic on startup

3. Error handling in `src/main.rs` propagates errors properly  
   - **Test:** Run with invalid args shows user-friendly error

## Technical Context

### Architecture Reference
- Layered architecture: CLI → Commands → Domain → Repository
- ADR-001: Use thiserror for library error types
- ADR-003: Synchronous database operations

### Project Conventions
- Build/Test: `cargo build --release`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- Tech Stack: Rust 1.75+, clap 4.x, rusqlite 0.31.x, chrono 0.4.x, serde 1.x, uuid 1.x, tracing 0.1.x, thiserror
- All public functions must have doc comments (`///`)
- Error types use thiserror (never anyhow in library code)
- Never use `unwrap()` or `expect()` outside tests
- Tests live in same file as code they test
- Use `tracing` for logging (never `println!`)

### Existing Code Context

The file `src/error.rs` already exists. Current content:

```rust
//! Error handling module for TaskForge.
//!
//! This module defines the application's error types using thiserror.

use thiserror::Error;

/// Main application error type that wraps all possible errors.
#[derive(Debug, Error)]
pub enum AppError {
    /// Error during validation of input data.
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// Error from the system layer (database, file I/O, etc.).
    #[error("System error: {0}")]
    System(#[from] SystemError),

    /// Entity not found error.
    #[error("Entity not found: {0}")]
    NotFound(String),
}

/// Validation errors for user input.
#[derive(Debug, Clone, Error)]
pub enum ValidationError {
    /// Title is empty or exceeds max length.
    #[error("Title cannot be empty or exceed 500 characters")]
    InvalidTitle,

    /// Description exceeds max length.
    #[error("Description exceeds maximum length of 10000 characters")]
    InvalidDescription,

    /// Tag name is invalid.
    #[error("Tag name must be 1-50 characters and contain only alphanumeric or underscore")]
    InvalidTagName,

    /// Priority is out of valid range.
    #[error("Priority must be between 1 and 4")]
    InvalidPriority,

    /// Empty required field.
    #[error("Required field '{0}' cannot be empty")]
    EmptyField(String),
}

/// System-level errors (database, file I/O, etc.).
#[derive(Debug, Error)]
pub enum SystemError {
    /// Database operation failed.
    #[error("Database error: {0}")]
    Database(String),

    /// File I/O error.
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_error_validation_displays_message() {
        let err = AppError::Validation(ValidationError::InvalidTitle);
        assert_eq!(
            err.to_string(),
            "Validation error: Title cannot be empty or exceed 500 characters"
        );
    }

    #[test]
    fn app_error_not_found_displays_message() {
        let err = AppError::NotFound("task".to_string());
        assert_eq!(err.to_string(), "Entity not found: task");
    }

    #[test]
    fn validation_error_invalid_title_message() {
        let err = ValidationError::InvalidTitle;
        assert_eq!(
            err.to_string(),
            "Title cannot be empty or exceed 500 characters"
        );
    }

    #[test]
    fn validation_error_empty_field_message() {
        let err = ValidationError::EmptyField("title".to_string());
        assert_eq!(err.to_string(), "Required field 'title' cannot be empty");
    }

    #[test]
    fn system_error_database_message() {
        let err = SystemError::Database("connection failed".to_string());
        assert_eq!(err.to_string(), "Database error: connection failed");
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

1. **Review existing `src/error.rs`** — Verify it has AppError with UserError, ValidationError, SystemError variants using thiserror
2. **Review existing `src/main.rs`** — Verify it has tracing initialization
3. **Run `cargo clippy -- -D warnings`** — Fix any warnings
4. **Run `cargo test`** — All tests pass
5. **Verify acceptance criteria** — Test error handling works

### Skills to Read
- `./skills/skills/rust-best-practices/SKILL.md` — Error handling patterns
- `./skills/skills/test-driven-development/SKILL.md` — Testing practices

### Dependencies
- Story 01.2 (Create Module Structure) — Complete

## Scope Boundaries

### This Story Includes
- Error types in error.rs with proper thiserror derive
- Tracing initialization in main.rs
- Error propagation from main.rs

### This Story Does NOT Include
- Database error handling (belongs to Epic 03)
- Task-specific validation errors

### Files in Scope
- `src/error.rs` — modify
- `src/main.rs` — modify

### Files NOT in Scope
- `src/repository.rs` — don't touch
- `src/models.rs` — don't touch
