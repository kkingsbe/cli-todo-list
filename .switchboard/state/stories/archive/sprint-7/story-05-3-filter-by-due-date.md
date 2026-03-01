# Story 05.3: Filter by Due Date

> Epic: epic-05 — Filtering & Sorting
> Points: 3
> Sprint: 7
> Type: feature
> Risk: low
> Created: 2026-03-01T01:35:00Z

## User Story

**As a** user,
**I want** to filter tasks by due date,
**So that** I can see overdue or upcoming tasks.

## Acceptance Criteria

1. `task list --due-before 2026-03-01` shows tasks due before date
   - **Test:** Create tasks with different due dates, verify filtering works

2. `task list --due-after 2026-03-01` shows tasks due after date
   - **Test:** Verify tasks with due dates after specified date appear

3. `task list --due 2026-03-15` shows tasks due on specific date
   - **Test:** Verify only tasks with exact matching due date appear

## Technical Context

### Architecture Reference

From architecture.md §5 - cli.rs module:
- **Purpose:** Define CLI commands and arguments using clap
- **Public API:** Functions to build CLI parser, get matches

From architecture.md §6 - Data Model:
```sql
CREATE TABLE tasks (
    ...
    due_date TEXT
);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
```

### Project Conventions

From project-context.md:
- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Never use unwrap()** - Return Result types properly
- **Use tracing for logging** - Never use println!

### Existing Code Context

Current List command in cli.rs (lines 45-65):
```rust
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
```

**Missing:** Due date filter flags!

Current filter.rs already has due date fields (lines 35-38):
```rust
/// Filter by due date (tasks due before this date).
pub due_before: Option<chrono::DateTime<chrono::Utc>>,
/// Filter by due date (tasks due after this date).
pub due_after: Option<chrono::DateTime<chrono::Utc>>,
```

**Already implemented:** The TaskFilter struct already has due_before and due_after fields!

Current List handler in main.rs (lines 103-106):
```rust
Commands::List { .. } => {
    // TODO: Implement list command
    tracing::info!("List command not yet implemented");
}
```

**Missing:** Implementation of list command with filtering!

Current repository.rs has list_tasks method:
```rust
fn list_tasks(&self, filter: &TaskFilter, sort: &TaskSort) -> Result<Vec<Task>, RepositoryError>;
```

## Implementation Plan

1. **Add due date filter flags to List command in src/cli.rs**
   - Add: `#[arg(long)] due_before: Option<String>` - format "YYYY-MM-DD"
   - Add: `#[arg(long)] due_after: Option<String>` - format "YYYY-MM-DD"
   - Add: `#[arg(long)] due: Option<String>` - format "YYYY-MM-DD" (exact match)
   - Location: around lines 45-65

2. **Update src/main.rs List handler**
   - Extract due_before, due_after, due from command args
   - Parse date strings to chrono::DateTime<Utc>
   - Build TaskFilter with due date criteria
   - Call repository.list_tasks(filter, sort)
   - Format and display results

3. **Update src/repository.rs list_tasks method**
   - Add SQL WHERE clause for due_before: `due_date < ?`
   - Add SQL WHERE clause for due_after: `due_date > ?`
   - Add SQL WHERE clause for due (exact): `due_date = ?`
   - Handle NULL due_date (tasks without due dates)

4. **Run build + tests**
   - `cargo build --release`
   - `cargo test`
   - `cargo clippy -- -D warnings`

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling and Result types
- `./skills/test-driven-development/SKILL.md` — Writing tests first

### Dependencies

- 05.2: Filter by Priority and Tags (complete)

## Scope Boundaries

### This Story Includes
- Adding --due-before, --due-after, --due flags to List command
- Filtering tasks by due date in repository
- Handling NULL due_date values properly

### This Story Does NOT Include
- Sorting by due date (story 05.4)
- Any changes to other commands

### Files in Scope
- `src/cli.rs` — Add due date filter flags
- `src/repository.rs` — Implement due date filtering in SQL
- `src/main.rs` — Wire up due date filters

### Files NOT in Scope
- Any tag-related functionality
- Task update/create commands
- Output format changes
