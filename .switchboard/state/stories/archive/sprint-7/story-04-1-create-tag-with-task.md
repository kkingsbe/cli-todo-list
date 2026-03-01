# Story 04.1: Create Tag with Task

> Epic: epic-04 — Tagging System
> Points: 3
> Sprint: 7
> Type: feature
> Risk: low
> Created: 2026-03-01T02:43:00Z

## User Story

**As a** user,
**I want** to add tags when creating a task,
**So that** I can organize tasks by category.

## Acceptance Criteria

1. `task add "Task" --tag work --tag urgent` creates task with tags
   - **Test:** Run `cargo build --release`, then test: `cargo run -- add "Test task" --tag work --tag urgent`, verify task appears with both tags in list

2. Tags are created automatically if they don't exist
   - **Test:** Run `cargo run -- add "Test" --tag newtag`, verify "newtag" appears in `cargo run -- tag list`

3. Multiple tags work correctly
   - **Test:** Verify a task can have 3+ tags

## Technical Context

### Architecture Reference

From [`architecture.md`](.switchboard/planning/architecture.md):
- **Module:** cli.rs - Define CLI commands and arguments using clap
- **Module:** commands.rs - Orchestrate business logic for each CLI command
- **Data Model:** Task has many Tags via task_tags join table
- **Repository:** SqliteRepository handles all database operations

### Project Conventions

From [`project-context.md`](.switchboard/planning/project-context.md):
- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --check`
- **Never use unwrap()** - Return Result types properly
- **Use tracing for logging** - Never use println!

### Existing Code Context

**Current Add command in** [`src/cli.rs`](src/cli.rs:31):
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

**Current create_tag function in** [`src/commands.rs`](src/commands.rs:361):
```rust
pub fn create_tag<R: Repository>(
    _repository: Arc<R>,
    name: String,
    color: Option<String>,
) -> Result<Tag, AppError> {
    let tag = match color {
        Some(c) => Tag::with_color(name, c),
        None => Tag::new(name),
    };
    Ok(tag)
}
```

**Current add_tag_to_task function in** [`src/commands.rs`](src/commands.rs:391):
```rust
pub fn add_tag_to_task<R: Repository>(
    _repository: Arc<R>,
    task_id: String,
    tag_id: String,
) -> Result<(), AppError> {
    let _ = (task_id, tag_id);
    // Repository operations will be implemented in later stories
    Ok(())
}
```

## Implementation Plan

1. **Modify** `src/cli.rs` — Add `--tag` repeatable argument to Add command:
   ```rust
   /// Tags to add to the task (can be specified multiple times).
   #[arg(short, long)]
   tags: Vec<String>,
   ```

2. **Modify** `src/commands.rs` — Update `add_tag_to_task` to actually persist:
   - Get or create tag by name (case-insensitive)
   - Insert into task_tags join table
   - Return Result<(), AppError> properly

3. **Modify** `src/main.rs` — Wire up the tags argument in the add command handler

4. **Write tests** in `src/commands.rs` — Test tag creation and association

5. **Run build + tests** — Verify:
   - `cargo build --release` passes
   - `cargo test` passes
   - `cargo clippy -- -D warnings` passes

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling patterns
- `./skills/test-driven-development/SKILL.md` — TDD approach

### Dependencies

Stories that must be complete. "None" if this is independent.
- Depends on: 03.2 (Create Task Command) - complete

## Scope Boundaries

### This Story Includes
- Adding `--tag` flag to `task add` command
- Auto-creating tags if they don't exist
- Associating tags with tasks via join table

### This Story Does NOT Include
- Tag management commands (tag list, tag delete) - those are separate stories
- Adding/removing tags from existing tasks - that's story 04.4

### Files in Scope
- `src/cli.rs` — modify Add command
- `src/commands.rs` — modify add_tag_to_task function
- `src/main.rs` — wire up tags argument

### Files NOT in Scope
- `src/tag.rs` — don't modify (tags exist)
- `src/repository.rs` — assume repository methods exist
