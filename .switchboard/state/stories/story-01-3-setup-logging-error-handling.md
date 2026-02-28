# Story 01.3: Setup Logging and Error Handling

> Epic: epic-01 — Project Scaffolding
> Points: 2
> Sprint: 1
> Type: infrastructure
> Risk: low
> Created: 2026-02-28T16:29:20Z

## User Story

**As a** developer,
**I want** logging and error handling infrastructure in place,
**So that** I can debug issues and handle errors gracefully.

## Acceptance Criteria

1. error.rs defines AppError using thiserror with UserError, ValidationError, SystemError variants
   - **Test:** `cargo check` succeeds, error types are public

2. Logging is initialized in main.rs using tracing
   - **Test:** Application runs without panic on startup

3. Error handling in main.rs propagates errors properly
   - **Test:** Run with invalid args shows user-friendly error

## Technical Context

### Architecture Reference

From [architecture.md](.switchboard/planning/architecture.md):

**Error Handling Strategy:**
- Pattern: Use thiserror for application-specific errors
- Error Types:
  - `UserError`: Invalid input, not found, empty fields
  - `ValidationError`: Constraint violations
  - `SystemError`: Database errors, file I/O errors
- User-facing errors: Display friendly messages with suggestions
- Internal errors: Log details, show generic message to user

**Module: error.rs**
- Purpose: Application error types
- Public API: `AppError` enum with thiserror
- Dependencies: None (core)
- Data flow: Propagated from all modules

### Project Conventions

From [project-context.md](.switchboard/planning/project-context.md):

**Critical Rules:**
1. All public functions must have doc comments - Use `///` for public API documentation
2. Error types use thiserror - Never use anyhow in library code (only in main.rs)
3. Never use unwrap() or expect() outside tests - Return Result types properly
4. Use `tracing` for logging - Never use println! or eprintln!
5. All repository operations return Result - Propagate errors up the stack

**Anti-Patterns (Do NOT):**
- Do NOT use `unwrap()` in production code
- Do NOT use println! for logging - use tracing!
- Do NOT use anyhow in library modules - use thiserror

### Existing Code Context

After Story 01.2, the project has:
- `Cargo.toml` with all dependencies (including tracing, thiserror)
- `src/main.rs` with module declarations and hello world
- Empty module files: cli.rs, commands.rs, models.rs, task.rs, tag.rs, filter.rs, repository.rs, error.rs, config.rs

**Current src/main.rs:**
```rust
mod cli;
mod commands;
mod models;
mod task;
mod tag;
mod filter;
mod repository;
mod error;
mod config;

fn main() {
    println!("TaskForge - CLI Task Manager");
}
```

## Implementation Plan

1. **Implement src/error.rs** — Define error types using thiserror:
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

2. **Update src/main.rs** — Add logging initialization and error handling:
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
       
       // Placeholder - actual CLI will go in Story 01.4
       println!("TaskForge - CLI Task Manager");
       
       Ok(())
   }
   ```

3. **Verify compilation** — Run `cargo check` to ensure error types compile

4. **Verify build** — Run `cargo build` to confirm full build succeeds

5. **Test logging** — Run `RUST_DEBUG=1 cargo run` and verify logging works

### Skills to Read

- [skills/rust-best-practices/SKILL.md](skills/rust-best-practices/SKILL.md) — For Rust coding standards and best practices
- [skills/rust-best-practices/references/chapter_03.md](skills/rust-best-practices/references/chapter_03.md) — Error handling patterns

### Dependencies

- Story 01.2: Create Module Structure — Must be complete before this story

## Scope Boundaries

### This Story Includes
- Implementing error.rs with AppError enum
- Adding logging initialization to main.rs
- Basic error propagation setup

### This Story Does NOT Include
- CLI argument parsing (Story 01.4)
- Implementing any business logic errors
- Implementing repository errors
- Implementing config errors

### Files in Scope
- `src/error.rs` — modify (implement error types)
- `src/main.rs` — modify (add logging and error handling)

### Files NOT in Scope
- cli.rs (Story 01.4)
- commands.rs
- models.rs
- task.rs
- tag.rs
- filter.rs
- repository.rs
- config.rs
