# Story 02.3: Define Filter and Sort Structures

> Epic: epic-02 — Core Data Models
> Points: 3
> Sprint: 3
> Type: feature
> Risk: low
> Created: 2026-02-28T19:35:00Z

## User Story

**As a** developer,
**I want** filter and sort structures for querying tasks,
**So that** I can support listing, filtering, and sorting tasks in the CLI.

## Acceptance Criteria

1. TaskFilter struct with fields for status, priority, tags, due_date
   - **Test:** `cargo test filter` - All filter tests pass

2. TaskSort struct with fields for field (priority, created_at, due_date, title) and direction (asc, desc)
   - **Test:** `cargo test sort` - All sort tests pass

3. Filter and sort can be combined
   - **Test:** Create filter with multiple criteria and verify behavior

4. Default values are sensible (status: all, sort: created_at desc)
   - **Test:** Default filter returns all tasks sorted by created_at desc

## Technical Context

### Architecture Reference

From architecture.md §5 (filter.rs):
- **Purpose:** Filter and sort specifications
- **Public API:** `TaskFilter`, `TaskSort` structs
- **Dependencies:** models.rs
- **Data flow:** CLI filter args → Filter struct → repository query

From architecture.md §6 (Data Model):
- Task has: id, title, description, priority (P1-P4), status (Incomplete/Completed), created_at, updated_at, due_date
- Tag has: id, name, created_at

### Project Conventions

From project-context.md:
- Use `///` for public API documentation
- Error types use thiserror - Never use anyhow in library code
- Never use unwrap() or expect() outside tests - Return Result types properly
- Tests live in the same file as the code they test - Use `#[cfg(test)]` module
- Use `tracing` for logging - Never use println!
- All repository operations return Result

### Existing Code Context

The current `src/filter.rs` likely needs to be created (or modified if exists). Let me check the current source structure:

```
src/
├── main.rs              # Entry point, CLI setup
├── cli.rs               # Clap command definitions
├── commands.rs         # Command handlers
├── models.rs           # Data structures (Task, Tag entities)
├── task.rs             # Task entity and operations
├── tag.rs              # Tag entity and operations
├── filter.rs           # Filter structs (TO BE CREATED)
├── repository.rs       # Database operations
├── error.rs            # Error types (thiserror)
└── config.rs           # Configuration management
```

Looking at models.rs for the Task/Tag structures, the filter.rs module needs to provide:
- `TaskFilter` struct with optional fields for filtering
- `TaskSort` struct with field and direction
- Builder pattern or From implementations for ease of use

## Implementation Plan

1. **Create `src/filter.rs`** — Define TaskFilter and TaskSort structs
   - Add TaskFilter with optional status, priority, tags, due_date_before, due_date_after
   - Add TaskSort with field enum (Priority, CreatedAt, DueDate, Title) and direction (Asc, Desc)
   - Implement Default for TaskFilter and TaskSort
   - Add `#[cfg(test)]` module with unit tests

2. **Update `src/models.rs`** — Export filter types if needed

3. **Update `src/lib.rs`** — Add filter module to public API

4. **Run tests** — `cargo test` to verify everything passes

### Skills to Read
- `./skills/rust-best-practices/SKILL.md` — For idiomatic Rust patterns
- `./skills/test-driven-development/SKILL.md` — For testing best practices

### Dependencies
- 02.1: Define Task Entity (COMPLETE)

## Scope Boundaries

### This Story Includes
- Filter and sort data structures in src/filter.rs
- Unit tests for filter and sort functionality
- Basic builder/constructor functions

### This Story Does NOT Include
- Database query implementation (that's story 03.1)
- CLI argument parsing (that's in commands.rs)
- Integration with repository

### Files in Scope
- `src/filter.rs` — create
- `src/lib.rs` — modify (add filter module)
- `src/models.rs` — modify (export if needed)

### Files NOT in Scope
- `src/repository.rs` — Don't implement queries yet
- `src/commands.rs` — Don't add CLI parsing
- `src/cli.rs` — Don't modify CLI definitions
