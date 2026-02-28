# Architecture — TaskForge

> Generated: 2026-02-28T16:15:00 UTC
> Source: .switchboard/input/PRD.md
> Scale: medium

## 1. System Overview

TaskForge is a command-line interface (CLI) task management application built in Rust. It provides fast, efficient, and distraction-free task organization directly in the terminal. The system uses a modular architecture with clear separation between CLI parsing, business logic, domain models, and data persistence.

The architecture follows a layered approach:
- **CLI Layer**: User interface using clap for parsing
- **Command Layer**: Business argument logic orchestration
- **Domain Layer**: Core entities (Task, Tag, Filter)
- **Data Access Layer**: Repository pattern with SQLite persistence

## 2. Technology Stack

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Language | Rust 1.75+ | As specified in PRD, high performance CLI |
| CLI Parsing | clap 4.x | As specified in PRD, mature CLI framework |
| Database | SQLite via rusqlite 0.31.x | As specified in PRD, zero-config, portable |
| Date/Time | chrono 0.4.x | As specified in PRD, robust datetime handling |
| Serialization | serde 1.x | As specified in PRD, Rust-native serialization |
| UUID Generation | uuid 1.x | As specified in PRD, unique ID generation |
| Logging | tracing 0.1.x | As specified in PRD, structured logging |
| Error Handling | thiserror | Per skills/rust-best-practices for library code |
| Testing | tokio + mockall | As specified in PRD |

## 3. Project Structure

```
taskforge/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, CLI setup
│   ├── cli.rs               # Clap command definitions
│   ├── commands.rs         # Command handlers
│   ├── models.rs           # Data structures
│   ├── task.rs             # Task entity and operations
│   ├── tag.rs              # Tag entity and operations
│   ├── filter.rs           # Filter structs
│   ├── repository.rs       # Database operations
│   ├── error.rs            # Error types (thiserror)
│   └── config.rs           # Configuration management
├── tests/
│   ├── integration/         # Integration tests
│   └── unit/               # Unit tests
├── migrations/             # Database migrations
└── config.toml             # Example configuration
```

### Module Structure Rationale

- **main.rs**: Application entry, initializes logging point, handles CLI
- **cli.rs**: All clap command/argument definitions - single source of truth for CLI
- **commands.rs**: Orchestrates business logic, coordinates between modules
- **models.rs**: Core data structures shared across modules
- **task.rs**: Task domain logic (CRUD operations, validation)
- **tag.rs**: Tag domain logic (CRUD, tag-task relationships)
- **filter.rs**: Filter and sort structures for querying
- **repository.rs**: Database abstraction, connection management
- **error.rs**: Application error types using thiserror
- **config.rs**: Configuration loading from ~/.taskforge/config.toml

## 4. Key Design Decisions

### ADR-001: Use thiserror for Library Error Types

- **Status:** Accepted
- **Context:** Rust offers thiserror and anyhow for error handling. The application is a CLI binary but may expose library functions.
- **Decision:** Use thiserror for application errors. For library-style reusable code, return concrete error types. Use `anyhow` only in main.rs for convenient error propagation at the top level.
- **Consequences:** Consistent error handling across codebase. Clear error hierarchies. Easy to extend.
- **Alternatives Considered:** anyhow - rejected because it hides error types and makes it harder to match on specific errors.

### ADR-002: Repository Pattern for Database Abstraction

- **Status:** Accepted
- **Context:** Need to abstract database operations for testability and flexibility.
- **Decision:** Implement a Repository trait that abstracts all database operations. Use rusqlite directly for implementation.
- **Consequences:** Easy to mock in tests. Clear separation of concerns. Swap storage backend if needed.
- **Alternatives Considered:** Using raw SQL in commands - rejected due to poor testability.

### ADR-003: Synchronous Database Operations

- **Status:** Accepted
- **Context:** The application is a CLI tool, not a long-running server. The PRD doesn't require async operations.
- **Decision:** Use synchronous rusqlite operations. The CLI nature means blocking I/O is acceptable.
- **Consequences:** Simpler code, no async overhead, meets performance requirements (< 500ms for filters).
- **Alternatives Considered:** Using tokio with async SQL - rejected as overkill for CLI tool.

### ADR-004: UUID v4 for Task and Tag IDs

- **Status:** Accepted
- **Context:** Need unique identifiers for tasks and tags.
- **Decision:** Use UUID v4 (random) for all entity IDs. Store as String in database.
- **Consequences:** No ID collision, good distribution, human-readable IDs.
- **Alternatives Considered:** Auto-increment integers - rejected for distributed feel and collision concerns.

### ADR-005: Command Pattern for CLI Operations

- **Status:** Accepted
- **Context:** Need to organize CLI operations cleanly.
- **Decision:** Use clap's derive macros to define commands. Each command is a function in commands.rs.
- **Consequences:** Clean separation between CLI parsing and business logic. Easy to test command handlers.
- **Alternatives Considered:** Manual argument parsing - rejected for maintenance burden.

## 5. Module Specifications

### cli.rs

- **Purpose:** Define CLI commands and arguments using clap
- **Public API:** Functions to build CLI parser, get matches
- **Dependencies:** clap, serde (for derive)
- **Data flow:** User input → clap → parsed arguments → commands.rs

### commands.rs

- **Purpose:** Orchestrate business logic for each CLI command
- **Public API:** Functions like `run_add`, `run_list`, `run_delete`
- **Dependencies:** task.rs, tag.rs, filter.rs, repository.rs, config.rs
- **Data flow:** CLI input → validate → domain operations → repository → output

### task.rs

- **Purpose:** Task domain logic
- **Public API:** `Task` struct, `TaskManager` for CRUD
- **Dependencies:** models.rs, error.rs
- **Data flow:** Create/validate Task → repository

### tag.rs

- **Purpose:** Tag domain logic and tag-task relationships
- **Public API:** `Tag` struct, `TagManager` for CRUD
- **Dependencies:** models.rs, error.rs
- **Data flow:** Create/validate Tag → repository

### filter.rs

- **Purpose:** Filter and sort specifications
- **Public API:** `TaskFilter`, `TaskSort` structs
- **Dependencies:** models.rs
- **Data flow:** CLI filter args → Filter struct → repository query

### repository.rs

- **Purpose:** Database abstraction layer
- **Public API:** `Repository` trait, `SqliteRepository` implementation
- **Dependencies:** rusqlite, models.rs
- **Data flow:** Domain operations → repository → SQLite

### error.rs

- **Purpose:** Application error types
- **Public API:** `AppError` enum with thiserror
- **Dependencies:** None (core)
- **Data flow:** Propagated from all modules

### config.rs

- **Purpose:** Configuration management
- **Public API:** `Config` struct, `load_config()` function
- **Dependencies:** serde, directories
- **Data flow:** config.toml → Config struct → modules

## 6. Data Model

### Entities

#### Task
- **id**: String (UUID v4)
- **title**: String (required, max 500 chars)
- **description**: Option<String>
- **priority**: Priority enum (P1=1, P2=2, P3=3, P4=4)
- **status**: Status enum (Incomplete, Completed)
- **created_at**: DateTime<Utc>
- **updated_at**: DateTime<Utc>
- **due_date**: Option<DateTime<Utc>>

#### Tag
- **id**: String (UUID v4)
- **name**: String (unique, case-insensitive)
- **created_at**: DateTime<Utc>

#### TaskTag (relationship)
- **task_id**: String (FK)
- **tag_id**: String (FK)

### Storage

SQLite database stored at `~/.taskforge/tasks.db`:

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

## 7. Error Handling Strategy

- **Pattern:** Use thiserror for application-specific errors
- **Error Types:**
  - `UserError`: Invalid input, not found, empty fields
  - `ValidationError`: Constraint violations
  - `SystemError`: Database errors, file I/O errors
- **User-facing errors:** Display friendly messages with suggestions
- **Internal errors:** Log details, show generic message to user

## 8. Testing Strategy

- **Unit tests:** In-module `#[cfg(test)]` modules testing individual functions
- **Integration tests:** In `tests/integration/` testing CLI commands end-to-end
- **Test Commands:**
  - Build: `cargo build --release`
  - Test: `cargo test`
  - Lint: `cargo clippy -- -D warnings`
  - Format: `cargo fmt --check`

## 9. Non-Functional Requirements

| NFR | Architectural Response |
|-----|----------------------|
| Task creation < 2 seconds | Synchronous SQLite, minimal validation |
| Filter operations < 500ms | Database indexes on filtered columns |
| Binary size < 10MB | Release build with LTO, minimal dependencies |
| Zero runtime dependencies | Static binary, SQLite bundled |

## 10. Scope Boundaries

### In Scope
- CLI task management (CRUD operations)
- Tag management (create, view, delete, rename)
- Task filtering (status, priority, tags, due date)
- Task sorting (priority, created, due date, title)
- Configuration management (config.toml)
- Multiple output formats (table, plain, JSON)

### Out of Scope
- Web interface or API
- Multi-user/multi-tenant support
- Cloud synchronization
- Task dependencies/wizards
- Recurring tasks
- Notifications

### Future Considerations
- Task dependencies (blocking)
- Recurring tasks
- Subtasks/checklists
- Priority inheritance
