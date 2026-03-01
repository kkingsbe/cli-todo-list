# Story 05.4: Sorting

> Epic: epic-05 — Filtering & Sorting
> Points: 2
> Sprint: 9
> Type: feature
> Risk: low
> Created: 2026-03-01T07:35:00Z

## User Story

**As a** user,
**I want** to sort tasks by different fields,
**So that** I can organize my view.

## Acceptance Criteria

1. `task list --sort priority` sorts by priority (1 first)
   - **Test:** Create tasks with P1, P2, P3, P4 priorities, run list with --sort priority, verify P1 appears first

2. `task list --sort priority --desc` sorts descending
   - **Test:** Same tasks with --desc flag, verify P4 appears first

3. Can sort by created_at, due_date, title
   - **Test:** Each sort field orders correctly

## Technical Context

### Architecture Reference

**Filter Module** (see architecture.md §5/filter.rs):
- `TaskSort` struct with `field: TaskSortField` and `order: SortOrder`
- `TaskSortField` enum: `CreatedAt`, `UpdatedAt`, `Priority`, `DueDate`, `Title`
- `SortOrder` enum: `Ascending`, `Descending`

**CLI Arguments** (see architecture.md §3/cli.rs):
- Already has `--sort-by` (default "created") and `--order` (default "asc") arguments

**Repository Pattern** (see architecture.md §7):
- `list_tasks` takes `filter: &TaskFilter` and `sort: &TaskSort` parameters

### Project Conventions

- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --check`
- All public functions must have doc comments (`///`)
- Error types use thiserror - never use `unwrap()` outside tests

### Existing Code Context

**src/filter.rs** - Current TaskSortField and SortOrder (lines 7-24):
```rust
/// Sort order for queries.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SortOrder {
    Ascending,
    #[default]
    Descending,
}

/// Field to sort tasks by.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum TaskSortField {
    #[default]
    CreatedAt,
    UpdatedAt,
    Priority,
    DueDate,
    Title,
}
```

**src/filter.rs** - Current TaskSort struct (lines 94-113):
```rust
/// Sort options for tasks.
#[derive(Debug, Clone, Default)]
pub struct TaskSort {
    /// Field to sort by.
    pub field: TaskSortField,
    /// Sort order.
    pub order: SortOrder,
}
```

**src/cli.rs** - Current List command arguments (lines 62-68):
```rust
/// Sort field (created, priority, due, title).
#[arg(short, long, default_value = "created")]
sort_by: String,

/// Sort order (asc/desc).
#[arg(short, long, default_value = "asc")]
order: String,
```

**src/repository.rs** - Current list_tasks signature (lines 206-210):
```rust
fn list_tasks(
    &self,
    filter: &TaskFilter,
    _sort: &TaskSort,  // Currently unused!
) -> Result<Vec<Task>, RepositoryError> {
```

## Implementation Plan

1. **Parse CLI sort arguments** in `src/cli.rs` or `src/commands.rs`
   - Map "priority" → `TaskSortField::Priority`
   - Map "created" → `TaskSortField::CreatedAt`
   - Map "due" → `TaskSortField::DueDate`
   - Map "title" → `TaskSortField::Title`
   - Map "asc" → `SortOrder::Ascending`, "desc" → `SortOrder::Descending`

2. **Implement sorting in repository** in `src/repository.rs`
   - Build ORDER BY clause based on sort field and order
   - Apply to `list_tasks` query

3. **Wire up command handler** in `src/commands.rs`
   - Pass sort options from CLI to repository

4. **Write tests in `src/filter.rs`**:
   - Test sort field parsing
   - Test sort order parsing

5. **Write integration tests** in repository module
   - Test ascending/descending priority sort
   - Test due date sort (handle nulls)
   - Test title sort (alphabetical)

6. **Run build + tests** - verify everything passes

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling patterns
- `./skills/test-driven-development/SKILL.md` — TDD approach

### Dependencies

- Story 05.1 (Filter by Status) — **complete**
- This story is independent of other Sprint 9 stories

## Scope Boundaries

### This Story Includes
- Implement sorting in list_tasks query
- Parse --sort-by and --order CLI arguments
- Handle null due_date in sorting (nulls last)

### This Story Does NOT Include
- Any new filter types
- Tag-based filtering (already in 05.2)
- Output format changes

### Files in Scope
- `src/repository.rs` — modify
- `src/commands.rs` — modify
- `src/cli.rs` — already has sort arguments
- `src/filter.rs` — may need helper methods

### Files NOT in Scope
- `src/task.rs`
- `src/tag.rs`
- `src/models.rs`
- `src/config.rs`
