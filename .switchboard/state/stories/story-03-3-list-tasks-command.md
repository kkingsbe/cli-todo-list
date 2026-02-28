# Story 03.3: List Tasks Command

> Epic: epic-03 — Task CRUD Operations
> Points: 3
> Sprint: 4
> Type: feature
> Risk: low
> Created: 2026-02-28T21:05:00Z

## User Story

**As a** user,
**I want** to view all tasks with `task list`,
**So that** I can see my current work.

## Acceptance Criteria

1. `task list` shows all tasks in table format
   - **Test:** Run `cargo run -- list` and verify output displays all task fields (id, title, priority, status, created_at)

2. Tasks sorted by created_at descending by default
   - **Test:** Create multiple tasks and verify most recent appear first

3. Pagination with --limit flag (default 50)
   - **Test:** Run `cargo run -- list --limit 10` and verify only 10 tasks shown

## Technical Context

### Architecture Reference

From `architecture.md`:

- **Module:** cli.rs, commands.rs, repository.rs
- **Data flow:** CLI input → filter → repository query → output
- **Repository trait** (`repository.rs:418-422`): 
  ```rust
  fn list_tasks(&self, filter: &TaskFilter, sort: &TaskSort) -> Result<Vec<Task>, RepositoryError>;
  fn list_tasks_all(&self) -> Result<Vec<Task>, RepositoryError>;
  ```

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

### Filter/Sort Context

From `filter.rs:69-86`:
```rust
pub struct TaskSort {
    pub field: TaskSortField,
    pub order: SortOrder,
}

pub enum TaskSortField {
    CreatedAt,  // default
    UpdatedAt,
    Priority,
    DueDate,
    Title,
}

pub enum SortOrder {
    Ascending,
    Descending,  // default
}
```

### Project Conventions

From `project-context.md`:

- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --check`

### Existing Code Context

**CLI Definition** (`src/cli.rs:36-56`):
```rust
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
```

**Command Handler** (`src/commands.rs:24-32`):
```rust
pub fn list_tasks<R: Repository>(
    _repository: Arc<R>,
    filter: TaskFilter,
    sort: TaskSort,
) -> Result<Vec<Task>, AppError> {
    // Repository operations will be implemented in later stories
    let _ = (filter, sort);
    Ok(Vec::new())
}
```

**Repository list_tasks_all** (`src/repository.rs:185-207`):
```rust
fn list_tasks_all(&self) -> Result<Vec<Task>, RepositoryError> {
    let conn = self.conn.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT id, title, description, priority, status, created_at, updated_at, due_date FROM tasks"
    ).map_err(|e| RepositoryError::Database(e.to_string()))?;

    let tasks = stmt.query_map([], |row| {
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
    }).map_err(|e| RepositoryError::Database(e.to_string()))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| RepositoryError::Database(e.to_string()))?;

    Ok(tasks)
}
```

**Main entry** (`src/main.rs:55-58`):
```rust
Commands::List { .. } => {
    // TODO: Implement list command
    tracing::info!("List command not yet implemented");
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

1. **Modify `src/main.rs`** — Wire up the List command:
   - Parse command arguments ( Liststatus, priority, search, sort_by, order)
   - Initialize repository if not already initialized
   - Call `list_tasks` command handler
   - Format and display tasks in table format

2. **Modify `src/commands.rs`** — Connect to repository:
   - Call `repository.list_tasks_all()` or `list_tasks(filter, sort)` depending on filters
   - Handle errors and return tasks

3. **Implement sorting** — Add ORDER BY to SQL query:
   - Parse sort_by and order from CLI
   - Add sorting logic to `list_tasks` or `list_tasks_all`

4. **Implement basic filtering** — Add WHERE clause:
   - Filter by status if provided
   - Filter by priority if provided
   - Filter by search term in title/description

5. **Implement pagination** — Add LIMIT:
   - Add --limit flag to CLI
   - Add LIMIT to SQL query

6. **Add table output** — Use a simple table format:
   - Print header row
   - Print each task as a row

7. **Add tests** — Test:
   - Listing tasks returns all tasks
   - Sorting works correctly
   - Filtering works correctly
   - Pagination limits results

8. **Run verification:**
   - `cargo build --release` - should compile
   - `cargo test` - all tests pass
   - `cargo clippy -- -D warnings` - no warnings
   - `cargo fmt --check` - formatted correctly

## Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling and code organization
- `./skills/test-driven-development/SKILL.md` — Writing tests

## Dependencies

Stories that must be complete:
- **03.2**: Create Task Command — must be complete (this story depends on task creation)

## Scope Boundaries

### This Story Includes
- Listing all tasks via `list` command
- Sorting by created_at (default descending)
- Basic filtering by status and priority
- Pagination with --limit flag
- Table format output

### This Story Does NOT Include
- Filtering by tags (story 05.2)
- Filtering by due date (story 05.3)
- Sorting by priority/due_date/title (story 05.4)
- JSON/other output formats (story 06.2)

### Files in Scope
- `src/main.rs` — modify to wire up list command
- `src/commands.rs` — modify list_tasks to use repository
- `src/repository.rs` — may need to add sorting/filtering to SQL
- `src/cli.rs` — List command already defined

### Files NOT in Scope
- `src/tag.rs` — Tag operations unrelated
- Output format changes (story 06.2)
- Filter/sort enhancements (epic-05)
