# Story 04.1: Create Tag with Task

> Epic: epic-04 — Tagging System
> Points: 3
> Sprint: 7
> Type: feature
> Risk: low
> Created: 2026-03-01T01:35:00Z

## User Story

**As a** user,
**I want** to add tags when creating a task,
**So that** I can organize tasks by category.

## Acceptance Criteria

1. `task add "Task" --tag work --tag urgent` creates task with tags
   - **Test:** Run `cargo run -- add "Buy groceries" --tag work --tag urgent`, then `cargo run -- list` should show the task with both tags

2. Tags are created automatically if they don't exist
   - **Test:** Run `cargo run -- add "New task" --tag project` - "project" tag should be created automatically

3. Multiple tags work correctly
   - **Test:** Create task with 3 tags, verify all 3 appear in list output

## Technical Context

### Architecture Reference

From architecture.md §5 - cli.rs module:
- **Purpose:** Define CLI commands and arguments using clap
- **Public API:** Functions to build CLI parser, get matches

From architecture.md §6 - Data Model:
```sql
CREATE TABLE task_tags (
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    tag_id TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, tag_id)
);
```

### Project Conventions

From project-context.md:
- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Never use unwrap()** - Return Result types properly
- **Use tracing for logging** - Never use println!

### Existing Code Context

Current Add command in cli.rs (lines 31-42):
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

**Missing:** The `--tag` flag is NOT present in the Add command!

Current Add handler in main.rs (lines 80-101):
```rust
Commands::Add {
    title,
    description,
    priority,
} => {
    let priority = match priority {
        1 => Priority::P1,
        2 => Priority::P2,
        3 => Priority::P3,
        4 => Priority::P4,
        _ => Priority::P3,
    };

    match create_task_with_dyn(repository.as_ref(), title, description, priority) {
        Ok(task) => {
            println!("Created task: {}", task.id);
        }
        Err(e) => {
            eprintln!("Error creating task: {}", e);
            std::process::exit(1);
        }
    }
}
```

**Missing:** Tag handling is NOT implemented!

Current repository.rs has these relevant methods:
- `create_task(&self, task: &Task) -> Result<(), RepositoryError>`
- `get_task(&self, id: &str) -> Result<Task, RepositoryError>`
- `list_tasks(&self, filter: &TaskFilter, sort: &TaskSort) -> Result<Vec<Task>, RepositoryError>`

**Missing:** Methods for creating tags and associating with tasks!

Current src/ directory structure:
```
src/
├── main.rs        # Entry point - needs Add --tag support
├── cli.rs         # Add command needs --tag flag
├── commands.rs    # Command handlers
├── models.rs      # Task, Tag entities
├── tag.rs         # Tag entity
├── filter.rs      # TaskFilter (already has tags field)
├── repository.rs  # Database operations
├── error.rs       # Error types
└── config.rs      # Configuration
```

## Implementation Plan

1. **Add --tag argument to Add command in src/cli.rs**
   - Add: `#[arg(long)] tag: Option<Vec<String>>` to the Add variant
   - Location: around line 31-42

2. **Add tag repository methods in src/repository.rs**
   - Add: `create_tag(&self, tag: &Tag) -> Result<(), RepositoryError>`
   - Add: `add_tag_to_task(&self, task_id: &str, tag_id: &str) -> Result<(), RepositoryError>`
   - Add: `get_or_create_tag(&self, name: &str) -> Result<Tag, RepositoryError>` - creates if not exists
   - Add: `list_tags(&self) -> Result<Vec<Tag>, RepositoryError>`

3. **Update src/commands.rs create_task function**
   - Add tag parameter: `tags: Option<Vec<String>>`
   - For each tag: call repository.get_or_create_tag()
   - After task creation: call repository.add_tag_to_task() for each tag

4. **Update src/main.rs Add handler**
   - Extract `tag` from command arguments
   - Pass tags to create_task command
   - After task created, show tags in output

5. **Update src/main.rs List handler**
   - Implement list_tasks to fetch from repository with filter
   - Include tags in output (join task_tags to get tag names)

6. **Run build + tests**
   - `cargo build --release`
   - `cargo test`
   - `cargo clippy -- -D warnings`

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling and Result types
- `./skills/test-driven-development/SKILL.md` — Writing tests first

### Dependencies

- 03.2: Create Task Command (complete)

## Scope Boundaries

### This Story Includes
- Adding `--tag` flag to Add command
- Auto-creating tags when they don't exist
- Associating multiple tags with a task on creation
- Displaying tags in list output

### This Story Does NOT Include
- Tag management commands (list, delete) - story 04.2, 04.3
- Adding/removing tags from existing tasks - story 04.4
- Filtering by tags - story 05.2 (already complete)

### Files in Scope
- `src/cli.rs` — Add --tag argument to Add command
- `src/repository.rs` — Add tag CRUD methods
- `src/commands.rs` — Update create_task to handle tags
- `src/main.rs` — Wire up tags in Add and List handlers

### Files NOT in Scope
- `src/tag.rs` — Entity already defined
- `src/filter.rs` — Already has tags field
- Any other CLI commands
