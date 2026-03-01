# Story 06.1: Delete Task Command

> Epic: epic-06 — Delete Task and Output Formats
> Points: 3
> Sprint: 9
> Type: feature
> Risk: medium
> Created: 2026-03-01T07:35:00Z
> Status: complete

## User Story

**As a** user,
**I want** to delete a task with `task delete <id>`,
**So that** I can remove unwanted tasks.

## Acceptance Criteria

1. `task delete <id>` prompts for confirmation
   - **Test:** Run `task delete <id>` - should ask for confirmation

2. `task delete <id> --force` deletes without confirmation
   - **Test:** Run `task delete <id> --force` - should delete immediately

3. Deleted task no longer appears in list
   - **Test:** After deletion, `task list` should not show the task

4. Returns error for non-existent task ID
   - **Test:** Run `task delete invalid-id` - should show error message

## Technical Context

### Architecture Reference

**Repository Pattern** (see architecture.md §7):
- `delete_task` method already exists in repository trait
- Returns `Result<(), RepositoryError>`

**CLI Structure** (see architecture.md §3/cli.rs):
- `Commands::Delete` variant with `id: String` field

**Task Entity** (see architecture.md §6):
- Task has `id: String` (UUID v4)

### Project Conventions

- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --check`
- All public functions must have doc comments (`///`)
- Error types use thiserror - never use `unwrap()` outside tests

### Existing Code Context

**src/cli.rs** - Current Delete command (lines 128-132):
```rust
/// Delete a task.
Delete {
    /// Task ID.
    id: String,
},
```

**src/repository.rs** - Current delete_task implementation (lines 362-369):
```rust
fn delete_task(&self, id: &str) -> Result<(), RepositoryError> {
    let conn = self
        .conn
        .lock()
        .map_err(|e| RepositoryError::Database(e.to_string()))?;
    conn.execute("DELETE FROM tasks WHERE id = ?1", [id])
        .map_err(|e| RepositoryError::Database(e.to_string()))?;
    Ok(())
}
```

**src/task.rs** - Task struct (lines 66-84):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
}
```

## Implementation Plan

1. **Add --force flag to CLI** in `src/cli.rs`
   - Add `#[arg(long, short)] force: bool` to Delete command

2. **Implement confirmation prompt** in `src/commands.rs`
   - If not --force, prompt user with "Delete task '<id>'? (y/n): "
   - Only proceed if user enters "y" or "Y"

3. **Wire up delete command handler**
   - Call repository.delete_task()
   - Handle RepositoryError::NotFound for non-existent IDs
   - Print success/error messages

4. **Add confirmation interaction** 
   - Use std::io::stdin().read_line() for confirmation
   - Handle empty input as "no"

5. **Write tests**:
   - Test --force flag works
   - Test error for non-existent task
   - Test confirmation prompt behavior

6. **Run build + tests** - verify everything passes

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling patterns
- `./skills/test-driven-development/SKILL.md` — TDD approach

### Dependencies

- Story 05.1 (Filter by Status) — **complete**
- This story is independent of other Sprint 9 stories

## Scope Boundaries

### This Story Includes
- `task delete <id>` command with confirmation
- `--force` flag to skip confirmation
- Error handling for non-existent task

### This Story Does NOT Include
- Bulk delete
- Undo functionality
- Any output format changes

### Files in Scope
- `src/cli.rs` — modify (add --force flag/commands.rs` — modify ()
- `srcadd handler)
- `src/repository.rs` — already implemented

### Files NOT in Scope
- `src/task.rs`
- `src/tag.rs`
- `src/filter.rs`
- `src/models.rs`
- `src/config.rs`
