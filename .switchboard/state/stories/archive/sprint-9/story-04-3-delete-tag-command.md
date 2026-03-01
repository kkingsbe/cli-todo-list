# Story 04.3: Delete Tag Command

> Epic: epic-04 — Tagging System
> Points: 2
> Sprint: 9
> Type: feature
> Risk: medium
> Created: 2026-03-01T07:35:00Z
> Status: complete

## User Story

**As a** user,
**I want** to delete a tag with `task tag delete <name>`,
**So that** I can remove unused tags.

## Acceptance Criteria

1. `task tag delete <name>` removes the tag
   - **Test:** Run `cargo run -- tag delete work`, then verify tag is gone with `cargo run -- tags`

2. Tag is removed from all associated tasks
   - **Test:** Create task with tag "work", delete "work" tag, verify task no longer has that tag

3. Error if tag doesn't exist
   - **Test:** Run `task tag delete nonexistent` - should show error message

## Technical Context

### Architecture Reference

**Repository Pattern** (see architecture.md §7):
- Repository trait in `src/repository.rs` defines `delete_tag` method
- Currently returns `Err(RepositoryError::Database("Not implemented".to_string()))`

**CLI Structure** (see architecture.md §3/cli.rs` -):
- `src Tag subcommands defined in `TagCommands::Delete`
- Currently has `identifier: String` field

**Data Model** (see architecture.md §6):
- Tags stored in `tags` table with `id`, `name`, `created_at`
- Junction table `task_tags` with `task_id`, `tag_id` (CASCADE deletes)

### Project Conventions

- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --check`
- All public functions must have doc comments (`///`)
- Error types use thiserror - never use `unwrap()` outside tests
- Use `tracing` for logging - never use `println!`

### Existing Code Context

**src/repository.rs** - Current delete_tag stub (lines 480-482):
```rust
fn delete_tag(&self, _id: &str) -> Result<(), RepositoryError> {
    Err(RepositoryError::Database("Not implemented".to_string()))
}
```

**src/repository.rs** - Existing delete_task implementation for reference (lines 362-369):
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

**src/cli.rs** - Current Tag Delete command (lines 172-176):
```rust
/// Delete a tag.
Delete {
    /// Tag ID or name.
    identifier: String,
},
```

**src/tag.rs** - Tag struct (lines 20-30):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

## Implementation Plan

1. **Implement `Repository::delete_tag`** in `src/repository.rs`
   - Look up tag by name (case-insensitive) using `get_tag_by_name`
   - Delete from `task_tags` junction table first (or rely on CASCADE)
   - Delete from `tags` table
   - Return error if tag not found

2. **Add `delete_tag` method to repository module** - wire up the trait implementation

3. **Add command handler in `src/commands.rs`** 
   - Parse tag name from CLI argument
   - Call repository delete method
   - Handle errors with user-friendly messages

4. **Write tests in `src/repository.rs`**:
   - Test successful tag deletion
   - Test deletion removes tag associations
   - Test error for non-existent tag

5. **Run build + tests** - verify everything passes

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling patterns
- `./skills/test-driven-development/SKILL.md` — TDD approach

### Dependencies

- Story 04.2 (List Tags Command) — **complete**
- This story is independent of other Sprint 9 stories

## Scope Boundaries

### This Story Includes
- `task tag delete <name>` command implementation
- Repository delete_tag method implementation
- Proper error handling for non-existent tags

### This Story Does NOT Include
- Tag renaming functionality
- Tag color management
- Any changes to task CRUD operations
- Output format changes

### Files in Scope
- `src/repository.rs` — modify
- `src/commands.rs` — modify
- `src/cli.rs` — no changes needed (already has Delete variant)
- `src/tag.rs` — no changes needed

### Files NOT in Scope
- `src/task.rs`
- `src/filter.rs`
- `src/models.rs`
- `src/config.rs`
