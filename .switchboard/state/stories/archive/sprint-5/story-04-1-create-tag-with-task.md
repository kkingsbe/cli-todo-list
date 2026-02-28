# Story 04.1: Create Tag with Task

> Epic: epic-04 — Tagging System
> Points: 3
> Sprint: 4
> Type: feature
> Risk: low
> Created: 2026-02-28T21:05:00Z

## User Story

**As a** user,
**I want** to add tags when creating a task,
**So that** I can organize tasks by category.

## Acceptance Criteria

1. `task add "Task" --tag work --tag urgent` creates task with tags
   - **Test:** Run `cargo run -- add "My Task" --tag work --tag urgent` and verify task appears in list with both tags

2. Tags are created automatically if they don't exist
   - **Test:** Create new tag "project" with task - verify tag appears in `task tags` list

3. Multiple tags work correctly
   - **Test:** Create task with 3+ tags, verify all appear on task

## Technical Context

### Architecture Reference

From `architecture.md`:

- **Module:** cli.rs, commands.rs, repository.rs, tag.rs
- **Data flow:** CLI input → validate → create tag → create task → add tag-task relationship
- **Repository trait** (`src/repository.rs`):
  - `fn create_tag(&self, tag: &Tag) -> Result<(), RepositoryError>;`
  - `fn get_tag_by_name(&self, name: &str) -> Result<Tag, RepositoryError>;`
  - `fn add_tag_to_task(&self, task_id: &str, tag_id: &str) -> Result<(), RepositoryError>;`

### Data Model

Tag entity (from `src/tag.rs`):
```rust
pub struct Tag {
    pub id: String,           // UUID v4
    pub name: String,         // unique, case-insensitive
    pub color: Option<String>, // optional color
    pub created_at: DateTime<Utc>,
}
```

Task-Tag relationship (`src/repository.rs:118-124`):
```sql
CREATE TABLE task_tags (
    task_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (task_id, tag_id),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
```

### Project Conventions

From `project-context.md`:

- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --check`

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

Need to add to Cli struct:
```rust
/// Tags to add to the task.
#[arg(long)]
tag: Vec<String>,
```

**Tag entity** (`src/tag.rs`):
```rust
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Tag {
    pub fn new(name: String) -> Self { ... }
    pub fn with_color(name: String, color: String) -> Self { ... }
}
```

**Repository methods** (`src/repository.rs`):
```rust
fn create_tag(&self, tag: &Tag) -> Result<(), RepositoryError>;  // line 244
fn get_tag_by_name(&self, name: &str) -> Result<Tag, RepositoryError>;  // line 283
fn add_tag_to_task(&self, task_id: &str, tag_id: &str) -> Result<(), RepositoryError>;  // line 330
```

**create_tag implementation** (`src/repository.rs:244-262`):
```rust
fn create_tag(&self, tag: &Tag) -> Result<(), RepositoryError> {
    let conn = self.conn.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
    conn.execute(
        "INSERT INTO tags (id, name, color, created_at) VALUES (?1, ?2, ?3, ?4)",
        (
            &tag.id,
            &tag.name,
            &tag.color,
            datetime_to_i64(&tag.created_at),
        ),
    ).map_err(|e| {
        if e.to_string().contains("UNIQUE constraint") {
            RepositoryError::Constraint(format!("Tag '{}' already exists", tag.name))
        } else {
            RepositoryError::Database(e.to_string())
        }
    })?;
    Ok(())
}
```

**get_tag_by_name implementation** (`src/repository.rs:283-300`):
```rust
fn get_tag_by_name(&self, name: &str) -> Result<Tag, RepositoryError> {
    let conn = self.conn.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT id, name, color, created_at FROM tags WHERE name = ?1"
    ).map_err(|e| RepositoryError::Database(e.to_string()))?;

    stmt.query_row([name.to_lowercase()], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
            created_at: i64_to_datetime(row.get(3)?),
        })
    }).map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => RepositoryError::NotFound(name.to_string()),
        _ => RepositoryError::Database(e.to_string()),
    })
}
```

**add_tag_to_task implementation** (`src/repository.rs:330-337`):
```rust
fn add_tag_to_task(&self, task_id: &str, tag_id: &str) -> Result<(), RepositoryError> {
    let conn = self.conn.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
    conn.execute(
        "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
        [task_id, tag_id],
    ).map_err(|e| RepositoryError::Database(e.to_string()))?;
    Ok(())
}
```

**Current directory structure:**
```
src/
├── main.rs        # Entry point
├── cli.rs         # Clap definitions
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

1. **Modify `src/cli.rs`** — Add --tag flag to Add command:
   ```rust
   /// Tags to add to the task.
   #[arg(long)]
   tag: Vec<String>,
   ```

2. **Modify `src/main.rs`** — Pass tags to command:
   - Parse --tag arguments
   - For each tag: get or create tag, then add to task after creation

3. **Modify `src/commands.rs`** — Create helper for get_or_create_tag:
   - Try to get tag by name
   - If not found, create new tag
   - Return the tag

4. **Add tag association** — After creating task:
   - For each tag name, call get_or_create_tag
   - Call repository.add_tag_to_task(task_id, tag_id)

5. **Modify task list output** — Show tags:
   - When listing tasks, include tags in output
   - May need to call `repository.get_task_tags(task_id)`

6. **Add tests** — Test:
   - Creating task with new tag creates both task and tag
   - Creating task with existing tag reuses tag
   - Creating task with multiple tags associates all
   - Listing tasks shows tags

7. **Run verification:**
   - `cargo build --release` - should compile
   - `cargo test` - all tests pass
   - `cargo clippy -- -D warnings` - no warnings
   - `cargo fmt --check` - formatted correctly

## Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling and code organization

## Dependencies

Stories that must be complete:
- **03.2**: Create Task Command — must be complete (this adds tags to task creation)

## Scope Boundaries

### This Story Includes
- Adding --tag flag to add command
- Auto-creating tags if they don't exist
- Associating tags with tasks on creation
- Displaying tags in task list

### This Story Does NOT Include
- Tag management commands (story 04.2, 04.3)
- Adding tags to existing tasks (story 04.4)
- Filtering by tags (story 05.2)

### Files in Scope
- `src/cli.rs` — add --tag flag
- `src/main.rs` — wire up tag handling
- `src/commands.rs` — add get_or_create_tag helper
- `src/tag.rs` — Tag entity
- `src/repository.rs` — already has necessary methods

### Files NOT in Scope
- Tag list command (story 04.2)
- Tag delete command (story 04.3)
- Managing tags on existing tasks (story 04.4)
