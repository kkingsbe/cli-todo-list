# Story 03.2: Create Task Command

> Epic: epic-03 — Task CRUD Operations
> Points: 3
> Sprint: 4
> Type: feature
> Risk: low
> Created: 2026-02-28T21:05:00Z

## User Story

**As a** user,
**I want** to create a task with `task add "title"`,
**So that** I can track new work.

## Acceptance Criteria

1. `task add "Buy groceries"` creates a task with the given title
   - **Test:** Run `cargo run -- add "Buy groceries"` and verify task appears in list after creation

2. Task has auto-generated UUID and timestamps
   - **Test:** Create task and verify it has valid UUID (format: 8-4-4-4-12 hex), created_at, updated_at

3. Default status is "incomplete", default priority is from config (or 3)
   - **Test:** Create task without priority flag, verify status is "incomplete" and priority is P3

4. Output shows task ID after creation
   - **Test:** Command output includes the new task ID in format like "Created task: {id}"

## Technical Context

### Architecture Reference

From `architecture.md`:

- **Module:** cli.rs, commands.rs, repository.rs
- **Data flow:** CLI input → validate → domain operations → repository → output
- **Repository trait** (`repository.rs:402-463`): Defines `create_task(&self, task: &Task) -> Result<(), RepositoryError>`
- **SqliteRepository** (`repository.rs:82-397`): Implements `create_task` at line 131

### Data Model

Task entity (from `task.rs:63-81`):
```rust
pub struct Task {
    pub id: String,           // UUID v4
    pub title: String,        // required, max 500 chars
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

Critical Rules:
1. All public functions must have doc comments - Use `///` for public API documentation
2. Error types use thiserror - Never use anyhow in library code
3. Never use unwrap() or expect() outside tests - Return Result types properly
4. Tests live in the same file as the code they test
5. Use `tracing` for logging - Never use println! or eprintln!

### Existing Code Context

**CLI Definition** (`src/cli.rs:22-33`):
```rust
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
```

**Command Handler** (`src/commands.rs:12-21`):
```rust
pub fn create_task<R: Repository>(
    _repository: Arc<R>,
    title: String,
    description: Option<String>,
    priority: Priority,
) -> Result<Task, AppError> {
    let task = Task::with_details(title, description, priority);
    // Repository operations will be implemented in later stories
    Ok(task)
}
```

**Task::with_details** (`src/task.rs:100-112`):
```rust
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
```

**Main entry** (`src/main.rs:51-54`):
```rust
Commands::Add { .. } => {
    // TODO: Implement add command
    tracing::info!("Add command not yet implemented");
}
```

**Repository create** (`src/repository.rs:131-148`):
```rust
fn create_task(&self, task: &Task) -> Result<(), RepositoryError> {
    let conn = self.conn.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
    conn.execute(
        "INSERT INTO tasks (id, title, description, priority, status, created_at, updated_at, due_date) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (
            &task.id,
            &task.title,
            &task.description,
            priority_to_string(&task.priority),
            status_to_string(&task.status),
            datetime_to_i64(&task.created_at),
            datetime_to_i64(&task.updated_at),
            task.due_date.as_ref().map(datetime_to_i64),
        ),
    ).map_err(|e| RepositoryError::Database(e.to_string()))?;
    Ok(())
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

1. **Modify `src/main.rs`** — Wire up the Add command:
   - Import the repository module
   - Initialize SqliteRepository at application startup
   - Call `create_task` command handler
   - Call `repository.create_task` to persist the task
   - Print success message with task ID

2. **Modify `src/commands.rs`** — Connect to repository:
   - Accept `Arc<R>` repository in `create_task` function
   - Call `repository.create_task(&task)` to persist
   - Handle errors and return the created task

3. **Add database initialization** — In `main.rs`:
   - Get database path from config or use default `~/.taskforge/tasks.db`
   - Create SqliteRepository
   - Call `repository.initialize()` to create tables

4. **Add tests** — In appropriate test modules:
   - Test creating a task persists to database
   - Test creating task with description
   - Test creating task with different priorities

5. **Run verification:**
   - `cargo build --release` - should compile
   - `cargo test` - all tests pass
   - `cargo clippy -- -D warnings` - no warnings
   - `cargo fmt --check` - formatted correctly

## Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling and code organization
- `./skills/test-driven-development/SKILL.md` — Writing tests

## Dependencies

Stories that must be complete:
- **03.1**: Database Repository Setup — complete ✅

## Scope Boundaries

### This Story Includes
- Creating tasks via CLI `add` command
- Persisting tasks to SQLite database
- Basic output showing task ID after creation
- Default values for status and priority

### This Story Does NOT Include
- Tag support when creating tasks (story 04.1)
- Due date support
- Task filtering or listing (story 03.3)
- Task update or delete

### Files in Scope
- `src/main.rs` — modify to wire up add command
- `src/commands.rs` — modify create_task to persist
- `src/repository.rs` — already has create_task implementation
- `src/task.rs` — existing Task::with_details

### Files NOT in Scope
- `src/cli.rs` — Already has Add command defined
- `src/tag.rs` — Tag creation unrelated
- Any new files in tests/integration/
