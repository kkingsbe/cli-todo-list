# Story 05.2: Filter by Priority and Tags

> Epic: epic-05 — Filtering & Sorting
> Points: 3
> Sprint: 5
> Type: feature
> Risk: low
> Created: 2026-02-28T22:40:00Z

## User Story

**As a** user,
**I want** to filter tasks by priority and tags,
**So that** I can focus on important work.

## Acceptance Criteria

1. `task list --priority 1` shows only P1 tasks
   - **Test:** `cargo run -- list --priority 1` should show only tasks with priority P1

2. `task list --tag work` shows tasks with "work" tag
   - **Test:** Create task with `cargo run -- add "Test" --tag work`, then `cargo run -- list --tag work` should show it

3. Multiple tags use AND logic
   - **Test:** `cargo run -- list --tag work --tag urgent` shows only tasks with both tags

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

Current list command in cli.rs supports:
```rust
--status <STATUS>      Filter by status (complete/incomplete)
-p, --priority <PRIORITY>  Filter by priority
--search <SEARCH>      Search term
-s, --sort-by <SORT_BY>    Sort field
-o, --order <ORDER>        Sort order
-l, --limit <LIMIT>        Max tasks
```

The repository.rs already has filtering implemented for status and priority. Need to add tag filtering.

Current src/cli.rs filter options (lines 50-60):
```rust
#[derive(Parser, Debug)]
pub struct ListArgs {
    /// Filter by status (complete/incomplete)
    #[arg(long)]
    pub status: Option<String>,
    
    /// Filter by priority
    #[arg(short, long)]
    pub priority: Option<i32>,
    
    /// Search term
    #[arg(long)]
    pub search: Option<String>,
    
    // ... sorting and limit options
}
```

## Implementation Plan

1. **Add --tag argument to ListArgs in src/cli.rs**
   - Add: `#[arg(long)] pub tag: Option<Vec<String>>`
   
2. **Update filter.rs TaskFilter struct**
   - Add: `tags: Option<Vec<String>>` field
   
3. **Update repository.rs to filter by tags**
   - Modify `list_tasks` to join with task_tags table
   - Add WHERE clause for tag filtering (AND logic for multiple tags)

4. **Run build + tests**
   - `cargo build --release`
   - `cargo test`
   - `cargo clippy -- -D warnings`

5. **Manual testing**
   - Create tasks with tags
   - Test single tag filter
   - Test multiple tags (AND logic)

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling and Result types
- `./skills/test-driven-development/SKILL.md` — Writing tests first

### Dependencies

- 05.1: Filter by Status (complete)

## Scope Boundaries

### This Story Includes
- Adding `--tag` filter to list command
- Filtering tasks by single tag
- Filtering tasks by multiple tags (AND logic)
- Repository-level tag filtering

### This Story Does NOT Include
- Due date filtering (story 05.3)
- Output format changes (story 06.2)
- Any changes to tag creation/deletion

### Files in Scope
- `src/cli.rs` — Add --tag argument
- `src/filter.rs` — Add tags to TaskFilter
- `src/repository.rs` — Implement tag filtering in list_tasks

### Files NOT in Scope
- `src/commands.rs` — No changes needed
- `src/main.rs` — No changes needed
- Any tag management files
