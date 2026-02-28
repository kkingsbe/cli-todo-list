# Story 01.4: Basic CLI Entry Point

> Epic: epic-01 — Project Scaffolding
> Points: 2
> Sprint: 1
> Type: infrastructure
> Risk: low
> Created: 2026-02-28T16:29:20Z

## User Story

**As a** user,
**I want** to see a help message when running `task --help`,
**So that** I know how to use the CLI.

## Acceptance Criteria

1. CLI module defines --help and --version flags
   - **Test:** `task --help` shows help text

2. Basic clap derive setup in cli.rs
   - **Test:** Binary runs and responds to --help

3. Empty main command that returns success
   - **Test:** `cargo run` exits with code 0

## Technical Context

### Architecture Reference

From [architecture.md](.switchboard/planning/architecture.md):

**Technology Stack:**
- CLI: clap 4.x (derive macros)

**Module: cli.rs**
- Purpose: Define CLI commands and arguments using clap
- Public API: Functions to build CLI parser, get matches
- Dependencies: clap, serde (for derive)
- Data flow: User input → clap → parsed arguments → commands.rs

**ADR-005:** Command Pattern for CLI Operations
- Decision: Use clap's derive macros to define commands. Each command is a function in commands.rs.

### Project Conventions

From [project-context.md](.switchboard/planning/project-context.md):

**Critical Rules:**
1. All public functions must have doc comments - Use `///` for public API documentation

**Naming Conventions:**
- Types: PascalCase (e.g., `Task`, `TaskFilter`, `AppError`)
- Files: snake_case (e.g., `task.rs`, `error.rs`)

### Existing Code Context

After Story 01.3, the project has:
- `Cargo.toml` with all dependencies (including clap)
- `src/main.rs` with logging initialization and error handling
- `src/error.rs` with AppError enum
- Empty module files

**Current src/main.rs:**
```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod commands;
mod models;
mod task;
mod tag;
mod filter;
mod repository;
mod error;
mod config;

fn main() -> Result<(), error::AppError> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    tracing::info!("TaskForge starting up...");
    
    // Placeholder - CLI parsing will be added here
    println!("TaskForge - CLI Task Manager");
    
    Ok(())
}
```

**Current src/error.rs:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid input: {0}")]
    UserError(String),
    
    #[error("Validation failed: {0}")]
    ValidationError(String),
    
    #[error("System error: {0}")]
    SystemError(#[from] std::io::Error),
}
```

## Implementation Plan

1. **Implement src/cli.rs** — Define basic CLI with clap derive:
   ```rust
   use clap::{Parser, Subcommand};
   
   #[derive(Parser)]
   #[command(name = "task")]
   #[command(version = "0.1.0")]
   #[command(about = "TaskForge - CLI Task Manager", long_about = None)]
   pub struct Cli {
       #[command(subcommand)]
       pub command: Option<Commands>,
   }
   
   #[derive(Subcommand, Debug)]
   pub enum Commands {
       /// List all tasks
       List,
       /// Add a new task
       Add {
           /// Task title
           title: String,
       },
   }
   
   /// Parse CLI arguments
   pub fn parse() -> Cli {
       Cli::parse()
   }
   ```

2. **Update src/main.rs** — Integrate CLI parsing:
   ```rust
   use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
   
   mod cli;
   mod commands;
   mod models;
   mod task;
   mod tag;
   mod filter;
   mod repository;
   mod error;
   mod config;
   
   fn main() -> Result<(), error::AppError> {
       // Initialize logging
       tracing_subscriber::registry()
           .with(tracing_subscriber::EnvFilter::new(
               std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
           ))
           .with(tracing_subscriber::fmt::layer())
           .init();
       
       tracing::info!("TaskForge starting up...");
       
       // Parse CLI arguments
       let cli = cli::parse();
       
       // Handle commands (placeholder for now)
       match cli.command {
           Some(_) => {
               // Commands will be implemented in later stories
               tracing::info!("Command executed");
           }
           None => {
               // No command given, show help
               cli::Cli::command().print_help().ok();
           }
       }
       
       Ok(())
   }
   ```

3. **Verify help works** — Run `cargo run -- --help` and confirm help text appears

4. **Verify version works** — Run `cargo run -- --version` and confirm version appears

5. **Verify build** — Run `cargo build` to confirm full build succeeds

6. **Verify clean exit** — Run `cargo run` and confirm it exits with code 0

### Skills to Read

- [skills/rust-best-practices/SKILL.md](skills/rust-best-practices/SKILL.md) — For Rust coding standards and best practices

### Dependencies

- Story 01.3: Setup Logging and Error Handling — Must be complete before this story

## Scope Boundaries

### This Story Includes
- Implementing cli.rs with basic clap derive setup
- Updating main.rs to parse CLI arguments
- Supporting --help and --version flags
- Supporting empty command (shows help)

### This Story Does NOT Include
- Implementing any actual commands (List, Add, etc.)
- Implementing task CRUD operations
- Implementing tag operations
- Implementing filtering/sorting

### Files in Scope
- `src/cli.rs` — modify (implement CLI parsing)
- `src/main.rs` — modify (integrate CLI parsing)

### Files NOT in Scope
- commands.rs (will be implemented in later stories)
- models.rs
- task.rs
- tag.rs
- filter.rs
- repository.rs
- config.rs
