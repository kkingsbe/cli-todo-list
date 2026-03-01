# Story 03.4: Get Task Details Command

> Epic: epic-03 — Task CRUD Operations
> Points: 2
> Sprint: 7
> Type: feature
> Risk: low
> Created: 2026-03-01T02:43:00Z

## User Story

**As a** user,
**I want** to view a single task with `task get <id>`,
**So that** I can see detailed information.

## Acceptance Criteria

1. `task get <uuid>` shows full task details
   - **Test:** Run `cargo build --release`, create a task with `cargo run -- add "Test"`, then run `cargo run -- get <id>` - should show title, description, priority, status, dates

2. Shows 404 error for unknown task ID
   - **Test:** Run `cargo run -- get invalid-uuid` - should show "Task not found" error

## Technical Context

### Architecture Reference

From [`architecture.md`](.switchboard/planning/architecture.md):
- **Module:** cli.rs - Define CLI commands and arguments using clap
- **Module:** commands.rs - Orchestrate business logic for each CLI command
- **Data Model:** Task entity with id, title, description, priority, status, created_at, updated_at, due_date
- **Repository:** SqliteRepository handles get_task operation

### Project Conventions

From [`project-context.md`](.switchboard/planning/project-context.md):
- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --check`
- **Never use unwrap()** - Return Result types properly
- **Use tracing for logging** - Never use println!

### Existing Code Context

**Current Show command in** [`src/cli.rs`](src/cli.rs:67):
```rust
/// Show a task.
Show {
    /// Task ID.
    id: String,
},
```

**Current get_task function in** [`src/commands.rs`](src/commands.rs:69):
```rust
pub fn get_task<R: Repository>(repository: Arc<R>, id: String) -> Result<Task, AppError> {
    repository.get_task(&id).map_err(|e| match e {
        RepositoryError::NotFound(msg) => AppError::NotFound(msg),
        RepositoryError::Database(msg) => {
            AppError::System(crate::error::SystemError::Database(msg))
        }
        RepositoryError::Constraint(msg) => AppError::UserError(msg),
    })
}
```

**Check main.rs for how Show command is handled - you need to see the current wiring:**

```
{output of: grep -n "Show\|get" src/main.rs | head -30}
```

## Implementation Plan

1. **Read** `src/main.rs` to see how Show command is currently wired (or not wired)

2. **If not wired:** Add the handler in main.rs to call `get_task` function:
   ```rust
   Commands::Show { id } => {
       // Handle show command - call get_task and print result
   }
   ```

3. **Ensure proper output formatting** - Show should display all task fields nicely

4. **Handle errors properly** - Show "Task not found" for invalid IDs

5. **Run build + tests** — Verify:
   - `cargo build --release` passes
   - `cargo test` passes
   - `cargo clippy -- -D warnings` passes

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling patterns

### Dependencies

Stories that must be complete. "None" if this is independent.
- Depends on: 03.3 (List Tasks Command) - complete

## Scope Boundaries

### This Story Includes
- Wiring up `task get <id>` command
- Displaying full task details
- Proper error handling for invalid IDs

### This Story Does NOT Include
- Any modifications to the data model
- Any database schema changes

### Files in Scope
- `src/main.rs` — wire up Show command
- `src/cli.rs` — already has Show command defined (verify)
- `src/commands.rs` — get_task already exists

### Files NOT in Scope
- `src/repository.rs` — don't modify
- `src/models.rs` — don't modify
