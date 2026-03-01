# Story 06.3: Complete and Toggle Status

> Epic: epic-06 — Delete Task and Output Formats
> Points: 2
> Sprint: 7
> Type: feature
> Risk: low
> Created: 2026-03-01T01:35:00Z

## User Story

**As a** user,
**I want** quick commands to mark tasks complete,
**So that** I can track progress efficiently.

## Acceptance Criteria

1. `task complete <id>` marks task as completed
   - **Test:** `cargo run -- complete <task-id>` changes task status to Completed

2. `task reopen <id>` marks task as incomplete
   - **Test:** `cargo run -- reopen <task-id>` changes task status to Incomplete

3. Output confirms the status change
   - **Test:** Success message shown after command

## Technical Context

### Architecture Reference

From architecture.md §5 - cli.rs module:
- **Purpose:** Define CLI commands and arguments using clap
- **Public API:** Functions to build CLI parser, get matches

From architecture.md §7 - Error Handling:
- Use thiserror for application-specific errors
- UserError for invalid input, not found

### Project Conventions

From project-context.md:
- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Never use unwrap()** - Return Result types properly
- **Use tracing for logging** - Never use println!

### Existing Code Context

Current Complete and Reopen commands in cli.rs (lines 98-107):
```rust
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
```

**Already implemented:** CLI commands exist!

Current Complete handler in main.rs (lines 140-143):
```rust
Commands::Complete { .. } => {
    // TODO: Implement complete command
    tracing::info!("Complete command not yet implemented");
}
```

Current Reopen handler in main.rs (lines 144-147):
```rust
Commands::Reopen { .. } => {
    // TODO: Implement reopen command
    tracing::info!("Reopen command not yet implemented");
}
```

**Missing:** Implementation!

Current commands.rs has complete_task stub (lines 118-122):
```rust
pub fn complete_task<R: Repository>(_repository: Arc<R>, id: String) -> Result<Task, AppError> {
    let _ = id;
    // Repository operations will be implemented in later stories
    Err(AppError::NotFound("Task not found".to_string()))
}
```

**Missing:** Full implementation!

Current repository.rs has:
- `get_task(&self, id: &str) -> Result<Task, RepositoryError>`
- `update_task(&self, task: &Task) -> Result<(), RepositoryError>`

Current Task entity in task.rs has:
- `status: Status` field
- Status enum: `Incomplete`, `Completed`

## Implementation Plan

1. **Update src/commands.rs complete_task function**
   - Fetch task from repository
   - Update status to Completed
   - Update updated_at timestamp
   - Persist to repository
   - Return updated task

2. **Add reopen_task function in src/commands.rs**
   - Fetch task from repository
   - Update status to Incomplete
   - Update updated_at timestamp
   - Persist to repository
   - Return updated task

3. **Update src/main.rs Complete handler**
   - Extract id from command
   - Call commands::complete_task()
   - Print success message with task details

4. **Update src/main.rs Reopen handler**
   - Extract id from command
   - Call commands::reopen_task()
   - Print success message with task details

5. **Run build + tests**
   - `cargo build --release`
   - `cargo test`
   - `cargo clippy -- -D warnings`

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling and Result types
- `./skills/test-driven-development/SKILL.md` — Writing tests first

### Dependencies

- 05.1: Filter by Status (will be unblocked by 04.1 completion)

## Scope Boundaries

### This Story Includes
- Implementing task complete command
- Implementing task reopen command
- Status toggle functionality

### This Story Does NOT Include
- Task deletion (story 06.1)
- Output format changes (story 06.2)
- Any other CRUD operations

### Files in Scope
- `src/commands.rs` — Implement complete_task and reopen_task
- `src/main.rs` — Wire up Complete and Reopen commands

### Files NOT in Scope
- Any CLI changes (already defined)
- Filter functionality
- Tag management
