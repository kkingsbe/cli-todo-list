# Story 03.1: Database Repository Setup

> Epic: epic-03 — Task CRUD Operations
> Points: 3
> Sprint: 3
> Type: infrastructure
> Risk: medium
> Created: 2026-02-28T19:35:00Z

## User Story

**As a** developer,
**I want** a repository that handles database operations,
**So that** I can persist and retrieve tasks.

## Acceptance Criteria

1. SqliteRepository implements Repository trait
   - **Test:** `cargo test repository` - Can insert and retrieve a Task from SQLite

2. Database schema matches architecture.md (tasks, tags, task_tags tables)
   - **Test:** Tables created on first run with correct indexes

3. Database stored at ~/.taskforge/tasks.db
   - **Test:** Database file created in correct location

## Technical Context

### Architecture Reference

From architecture.md §5 (repository.rs):
- **Purpose:** Database abstraction layer
- **Public API:** `Repository` trait, `SqliteRepository` implementation
- **Dependencies:** rusqlite, models.rs
- **Data flow:** Domain operations → repository → SQLite

From architecture.md §6 (Data Model - Storage):
```sql
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    priority INTEGER NOT NULL DEFAULT 3,
    status TEXT NOT NULL DEFAULT 'incomplete',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    due_date TEXT
);

CREATE TABLE tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE,
    created_at TEXT NOT NULL
);

CREATE TABLE task_tags (
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    tag_id TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, tag_id)
);

-- Indexes
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_priority ON tasks(priority);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);
CREATE INDEX idx_tags_name ON tags(name COLLATE NOCASE);
```

### Project Conventions

From project-context.md:
- Use `///` for public API documentation
- Error types use thiserror - Never use anyhow in library code
- Never use unwrap() or expect() outside tests - Return Result types properly
- Tests live in the same file as the code they test - Use `#[cfg(test)]` module
- Use `tracing` for logging - Never use println!
- All repository operations return Result

### Existing Code Context

Current source structure:
```
src/
├── main.rs              # Entry point, CLI setup
├── cli.rs               # Clap command definitions
├── commands.rs         # Command handlers
├── models.rs           # Data structures (Task, Tag entities exist)
├── task.rs             # Task entity and operations
├── tag.rs              # Tag entity and operations
├── filter.rs           # Filter structs (story 02.3 - concurrent)
├── repository.rs       # Database operations (TO BE IMPLEMENTED)
├── error.rs            # Error types (thiserror - exists)
└── config.rs           # Configuration management
```

Check `src/repository.rs` - likely empty or minimal. Check `src/error.rs` for existing error types.

## Implementation Plan

1. **Define Repository trait in `src/repository.rs`** — Abstract database operations
   - `init() -> Result<()>` - Initialize database and create tables
   - `create_task(task: &Task) -> Result<()>` - Insert a task
   - `get_task(id: &str) -> Result<Option<Task>>` - Retrieve a task
   - `list_tasks() -> Result<Vec<Task>>` - List all tasks
   - Similar for tags

2. **Implement SqliteRepository** — Concrete SQLite implementation
   - Open database at ~/.taskforge/tasks.db
   - Create tables on init if not exist
   - Implement all Repository trait methods

3. **Handle directory creation** — Create ~/.taskforge/ if it doesn't exist

4. **Add tests in `src/repository.rs`** — `#[cfg(test)]` module
   - Test database initialization
   - Test create and get task
   - Test error cases

5. **Run tests** — `cargo test` to verify everything passes

### Skills to Read
- `./skills/rust-best-practices/SKILL.md` — For idiomatic Rust patterns
- `./skills/test-driven-development/SKILL.md` — For testing best practices
- `./skills/rust-async-patterns/SKILL.md` — (Note: we're using sync rusqlite per ADR-003)

### Dependencies
- 02.1: Define Task Entity (COMPLETE)
- 02.2: Define Tag Entity (COMPLETE)

## Scope Boundaries

### This Story Includes
- Repository trait definition
- SqliteRepository implementation
- Database initialization and schema creation
- Basic CRUD operations (create, get, list)
- Unit tests for repository

### This Story Does NOT Include
- Complex queries with filters (that's later)
- Tag-task relationship management
- Configuration loading for db path (use hardcoded ~/.taskforge for now)
- Integration with CLI commands

### Files in Scope
- `src/repository.rs` — modify (implement Repository trait and SqliteRepository)

### Files NOT in Scope
- `src/commands.rs` — Don't wire up CLI commands
- `src/cli.rs` — Don't add CLI arguments
- `src/task.rs` — Don't add domain logic (just use Task from models)
