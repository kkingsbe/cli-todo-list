# Story 03.5: Update Task Command

> Epic: epic-03 — Task CRUD Operations
> Points: 5
> Sprint: 7
> Type: feature
> Risk: medium
> Created: 2026-03-01T01:35:00Z

## User Story

**As a** user,
**I want** to update a task with `task update <id> [options]`,
**So that** I can modify task details.

## Acceptance Criteria

1. Can update title with --title flag
   - **Test:** `cargo run -- edit <task-id> --title "New title"` changes the task title

2. Can update description with --description flag
   - **Test:** `cargo run -- edit <task-id> --description "New description"` updates the description

3. Can update priority with --priority flag (1-4)
   - **Test:** `cargo run -- edit <task-id> --priority 1` changes priority to P1

4. Can mark complete/incomplete with --status flag
   - **Test:** `cargo run -- edit <task-id> --status completed` changes status to Completed

5. Can update due date with --due flag
   - **Test:** `cargo run -- edit <task-id> --due 2026-03-15` sets due date

6. updated_at timestamp updates on any change
   - **Test:** Edit a task, verify updated_at is newer than before

## Technical Context

### Architecture Reference

From architecture.md §5 - cli.rs module:
- **Purpose:** Define CLI commands and arguments using clap
- **Public API:** Functions to build CLI parser, get matches

From architecture.md §7 - Error Handling:
- Use thiserror for application-specific errors
- UserError for invalid input, not found
- ValidationError for constraint violations
- SystemError for database errors

### Project Conventions

From project-context.md:
- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Never use unwrap()** - Return Result types properly
- **Use tracing for logging** - Never use println!

### Existing Code Context

Current Edit command in cli.rs (lines 74-89):
```rust
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
```

**Missing:** --status and --due flags!

Current Edit handler in main.rs (lines 132-135):
```rust
Commands::Edit { .. } => {
    // TODO: Implement edit command
    tracing::info!("Edit command not yet implemented");
}
```

Current update_task function in commands.rs (lines 93-108):
```rust
pub fn update_task<R: Repository>(
    _repository: Arc<R>,
    mut task: Task,
    title: Option<String>,
    description: Option<String>,
    priority: Option<Priority>,
) -> Result<Task, AppError> {
    if let Some(t) = title {
        task.update_title(t);
    }
    if let Some(p) = priority {
        task.update_priority(p);
    }
    task.description = description.or(task.description);
    Ok(task)
}
```

**Missing:** status and due_date handling, and persistence to repository!

Current repository.rs has update_task method:
- `update_task(&self, task: &Task) -> Result<(), RepositoryError>`

Current Task entity in task.rs has:
- `update_title(&mut self, title: String)`
- `update_priority(&mut self, priority: Priority)`
- `status` field: `Status` enum
- `due_date` field: `Option<DateTime<Utc>>`

## Implementation Plan

1. **Add --status and --due flags to Edit command in src/cli.rs**
   - Add: `#[arg(long)] status: Option<String>` - accept "complete" or "incomplete"
   - Add: `#[arg(long)] due: Option<String>` - accept ISO date format "YYYY-MM-DD"
   - Location: around lines 74-89

2. **Update src/commands.rs update_task function**
   - Add parameters: `status: Option<Status>`, `due_date: Option<DateTime<Utc>>`
   - Add logic to update status if provided
   - Add logic to update due_date if provided
   - Update updated_at timestamp on any change

3. **Add repository update method in src/repository.rs**
   - Ensure `update_task` persists changes to database
   - Verify it updates all fields including status and due_date

4. **Update src/main.rs Edit handler**
   - Extract all Edit arguments (id, title, description, priority, status, due)
   - Parse status string to Status enum
   - Parse due date string to DateTime<Utc>
   - Call repository.get_task() to fetch existing task
   - Call commands::update_task() to modify
   - Call repository.update_task() to persist
   - Print success message with updated task info

5. **Run build + tests**
   - `cargo build --release`
   - `cargo test`
   - `cargo clippy -- -D warnings`

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling and Result types
- `./skills/test-driven-development/SKILL.md` — Writing tests first

### Dependencies

- 03.4: Get Task Details Command (APPROVED - pending final status update)

## Scope Boundaries

### This Story Includes
- Updating task title, description, priority
- Updating task status (complete/incomplete)
- Updating task due date
- Automatic updated_at timestamp

### This Story Does NOT Include
- Tag management on tasks (story 04.4)
- Filtering by due date (story 05.3)

### Files in Scope
- `src/cli.rs` — Add --status and --due flags to Edit
- `src/commands.rs` — Update update_task function
- `src/repository.rs` — Ensure update persists
- `src/main.rs` — Wire up Edit command

### Files NOT in Scope
- Any tag-related functionality
- List command changes
- Filter functionality
