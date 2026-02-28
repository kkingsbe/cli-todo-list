# Story 03.4: Get Task Details Command

> Epic: epic-03 — Task CRUD Operations
> Points: 2
> Sprint: 4
> Type: feature
> Risk: low
> Created: 2026-02-28T21:05:00Z

## User Story

**As a** user,
**I want** to view a single task with `task get <id>`,
**So that** I can see detailed information.

## Acceptance Criteria

1. `task get <uuid>` shows full task details
   - **Test:** Run `cargo run -- get <task-id>` and verify output shows title, description, priority, status, dates

2. Shows 404 error for unknown task ID
   - **Test:** Run `cargo run -- get invalid-uuid` and verify "Task not found" error

## Technical Context

### Architecture Reference

From `architecture.md`:

- **Module:** cli.rs, commands.rs, repository.rs
- **Data flow:** CLI input → repository get → format output
- **Repository trait** (`repository.rs:409`): `fn get_task(&self, id: &str) -> Result<Task, RepositoryError>;`

### Data Model

Task entity (from `task.rs:63-81`):
```rust
pub struct Task {
    pub id: String,           // UUID v4
    pub title: String,        // required
    pub description: Option<String>,
    pub priority: Priority,   // P1, P2, P3, P4
    pub status: Status,       // Incomplete, Completed
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
}
```

### Project Conventions

From `project-context.md`:

- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --check`

### Existing Code Context

**CLI Definition** (`src/cli.rs:58-62`):
```rust
Show {
    /// Task ID.
    id: String,
},
```

**Command Handler** (`src/commands.rs:35-39`):
```rust
pub fn get_task<R: Repository>(_repository: Arc<R>, id: String) -> Result<Task, AppError> {
    let _ = id;
    // Repository operations will be implemented in later stories
    Err(AppError::NotFound("Task not implemented yet".to_string()))
}
```

**Repository get_task** (`src/repository.rs:150-174`):
```rust
fn get_task(&self, id: &str) -> Result<Task, RepositoryError> {
    let conn = self.conn.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT id, title, description, priority, status, created_at, updated_at, due_date 
         FROM tasks WHERE id = ?1"
    ).map_err(|e| RepositoryError::Database(e.to_string()))?;

    let task = stmt.query_row([id], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            priority: string_to_priority(&row.get::<_, String>(3)?),
            status: string_to_status(&row.get::<_, String>(4)?),
            created_at: i64_to_datetime(row.get(5)?),
            updated_at: i64_to_datetime(row.get(6)?),
            due_date: row.get::<_, Option<i64>>(7)?.map(i64_to_datetime),
        })
    }).map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => RepositoryError::NotFound(id.to_string()),
        _ => RepositoryError::Database(e.to_string()),
    })?;

    Ok(task)
}
```

**Main entry** (`src/main.rs:59-62`):
```rust
Commands::Show { .. } => {
    // TODO: Implement show command
    tracing::info!("Show command not yet implemented");
}
```

**Error types** (`src/error.rs`):
```rust
pub enum AppError {
    NotFound(String),
    // ...
}
```

**Current directory structure:**
```
src/
├── main.rs        # Entry point, CLI setup
├── cli.rs         # Clap command definitions
├── commands.rs   # Command handlers
├── models.rs     # Data structures
├── task.rs       # Task entity
├── tag.rs        # Tag entity
├── filter.rs     # Filter structs
├── repository.rs # Database operations
├── error.rs      # Error types
└── config.rs    # Configuration
```

## Implementation Plan

1. **Modify `src/main.rs`** — Wire up the Show command:
   - Get task ID from CLI argument
   - Call `get_task` command handler
   - Display full task details in formatted output

2. **Modify `src/commands.rs`** — Connect to repository:
   - Call `repository.get_task(&id)` 
   - Return the task or error

3. **Handle errors** — Convert RepositoryError to AppError:
   - RepositoryError::NotFound → AppError::NotFound
   - RepositoryError::Database → AppError::System

4. **Format output** — Display all task fields:
   - ID, Title, Description
   - Priority (P1-P4)
   - Status (incomplete/completed)
   - Created at, Updated at
   - Due date (if set)

5. **Add tests** — Test:
   - Get existing task returns task details
   - Get non-existent task returns error

6. **Run verification:**
   - `cargo build --release` - should compile
   - `cargo test` - all tests pass
   - `cargo clippy -- -D warnings` - no warnings
   - `cargo fmt --check` - formatted correctly

## Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling and code organization

## Dependencies

Stories that must be complete:
- **03.3**: List Tasks Command — must be complete (this story adds get/details after list)

## Scope Boundaries

### This Story Includes
- Getting single task by ID via `get` or `show` command
- Displaying full task details
- Error handling for non-existent tasks

### This Story Does NOT Include
- Editing/updating task (story 03.5)
- Adding/removing tags from task (story 04.4)
- Completing a task (story 06.3)

### Files in Scope
- `src/main.rs` — wire up show command
- `src/commands.rs` — modify get_task to use repository
- `src/error.rs` — may need to add error variants

### Files NOT in Scope
- `src/cli.rs` — Show command already defined
- `src/repository.rs` — get_task already implemented
- Tag operations (epic-04)
