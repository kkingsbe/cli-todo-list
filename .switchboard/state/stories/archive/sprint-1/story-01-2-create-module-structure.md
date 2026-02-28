# Story 01.2: Create Module Structure

> Epic: epic-01 — Project Scaffolding
> Points: 2
> Sprint: 1
> Type: infrastructure
> Risk: low
> Created: 2026-02-28T16:29:20Z

## User Story

**As a** developer,
**I want** the module structure defined in architecture.md,
**So that** I can organize code properly.

## Acceptance Criteria

1. src/main.rs creates the module structure
   - **Test:** File contains `mod cli;`, `mod commands;`, etc.

2. Empty module files exist (cli.rs, commands.rs, models.rs, task.rs, tag.rs, filter.rs, repository.rs, error.rs, config.rs)
   - **Test:** `cargo check` succeeds

3. All modules compile without errors
   - **Test:** `cargo build` succeeds

## Technical Context

### Architecture Reference

From [architecture.md](.switchboard/planning/architecture.md):

**Project Structure:**
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

**Module Structure Rationale:**
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

### Project Conventions

From [project-context.md](.switchboard/planning/project-context.md):

**File Organization:**
- New modules go in `src/` as `src/{module}.rs`
- Tests go in `src/{module}.rs` as `#[cfg(test)] mod tests { ... }`
- Integration tests go in `tests/integration/`
- Database migrations go in `migrations/` directory

**Naming Conventions:**
- Functions: snake_case (e.g., `create_task`, `list_tasks`)
- Types: PascalCase (e.g., `Task`, `TaskFilter`, `AppError`)
- Files: snake_case (e.g., `task.rs`, `error.rs`)
- Enums: PascalCase with variants (e.g., `Priority::P1`)

### Existing Code Context

After Story 01.1, the project has:
- `Cargo.toml` with all dependencies
- `src/main.rs` with minimal hello world code

## Implementation Plan

1. **Update src/main.rs** — Add module declarations:
   ```rust
   mod cli;
   mod commands;
   mod models;
   mod task;
   mod tag;
   mod filter;
   mod repository;
   mod error;
   mod config;
   
   fn main() {
       println!("TaskForge - CLI Task Manager");
   }
   ```

2. **Create empty module files** — Create each file with minimal content:
   - `src/cli.rs` — `// CLI module placeholder`
   - `src/commands.rs` — `// Commands module placeholder`
   - `src/models.rs` — `// Models module placeholder`
   - `src/task.rs` — `// Task module placeholder`
   - `src/tag.rs` — `// Tag module placeholder`
   - `src/filter.rs` — `// Filter module placeholder`
   - `src/repository.rs` — `// Repository module placeholder`
   - `src/error.rs` — `// Error module placeholder`
   - `src/config.rs` — `// Config module placeholder`

3. **Verify compilation** — Run `cargo check` to ensure all modules compile

4. **Verify build** — Run `cargo build` to confirm full build succeeds

### Skills to Read

- [skills/rust-best-practices/SKILL.md](skills/rust-best-practices/SKILL.md) — For Rust coding standards and best practices
- [skills/rust-best-practices/references/chapter_01.md](skills/rust-best-practices/references/chapter_01.md) — Rust fundamentals

### Dependencies

- Story 01.1: Initialize Cargo Project — Must be complete before this story

## Scope Boundaries

### This Story Includes
- Adding module declarations to main.rs
- Creating empty placeholder modules
- Verifying all modules compile

### This Story Does NOT Include
- Implementing any module functionality
- Error handling implementation (Story 01.3)
- CLI argument parsing (Story 01.4)
- Business logic

### Files in Scope
- `src/main.rs` — modify (add module declarations)
- `src/cli.rs` — create (empty)
- `src/commands.rs` — create (empty)
- `src/models.rs` — create (empty)
- `src/task.rs` — create (empty)
- `src/tag.rs` — create (empty)
- `src/filter.rs` — create (empty)
- `src/repository.rs` — create (empty)
- `src/error.rs` — create (empty)
- `src/config.rs` — create (empty)

### Files NOT in Scope
- Tests directory
- Migration files
- Config files
- Any business logic implementation
