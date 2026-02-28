# Story 01.1: Initialize Cargo Project

> Epic: epic-01 — Project Scaffolding
> Points: 1
> Sprint: 1
> Type: infrastructure
> Risk: low
> Created: 2026-02-28T16:29:20Z

## User Story

**As a** developer,
**I want** a basic Rust project with Cargo.toml configured,
**So that** I can build and test the application.

## Acceptance Criteria

1. Cargo.toml exists with all required dependencies (clap, rusqlite, chrono, serde, uuid, tracing, thiserror)
   - **Test:** `cargo build --release` succeeds

2. Basic main.rs exists with minimal code
   - **Test:** Binary compiles and runs without errors

3. Project compiles without warnings
   - **Test:** `cargo clippy -- -D warnings` passes

## Technical Context

### Architecture Reference

From [architecture.md](.switchboard/planning/architecture.md):

**Technology Stack:**
- Language: Rust 1.75+
- CLI: clap 4.x (derive macros)
- Database: SQLite via rusqlite 0.31.x
- Date/Time: chrono 0.4.x
- Serialization: serde 1.x
- UUID: uuid 1.x
- Logging: tracing 0.1.x
- Errors: thiserror

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

**ADR-001:** Use thiserror for Library Error Types

### Project Conventions

From [project-context.md](.switchboard/planning/project-context.md):

**Build & Test Commands:**
- Build: `cargo build --release`
- Test: `cargo test`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt --check`

**Critical Rules:**
1. All public functions must have doc comments - Use `///` for public API documentation
2. Error types use thiserror - Never use anyhow in library code (only in main.rs)
3. Never use unwrap() or expect() outside tests - Return Result types properly
4. Tests live in the same file as the code they test - Use `#[cfg(test)]` module
5. Use `tracing` for logging - Never use println! or eprintln!
6. All repository operations return Result - Propagate errors up the stack

**Anti-Patterns (Do NOT):**
- Do NOT use `unwrap()` in production code
- Do NOT add dependencies without updating architecture.md
- Do NOT modify existing public APIs without updating all callers
- Do NOT use println! for logging - use tracing!
- Do NOT use anyhow in library modules - use thiserror

### Existing Code Context

This story creates the initial project from scratch. No existing code to reference.

## Implementation Plan

1. **Create Cargo.toml** — Define project with name "taskforge", type binary, Rust edition 2021. Add all required dependencies:
   - clap (with derive feature)
   - rusqlite (with bundled feature)
   - chrono (with serde feature)
   - serde (with derive feature)
   - uuid (with v4 and serde features)
   - tracing
   - tracing-subscriber (with env-filter feature)
   - thiserror
   - directories (for config path)

2. **Create src/main.rs** — Minimal hello world:
   ```rust
   fn main() {
       println!("TaskForge - CLI Task Manager");
   }
   ```

3. **Verify build** — Run `cargo build --release` to ensure it compiles

4. **Verify no warnings** — Run `cargo clippy -- -D warnings` and fix any issues

5. **Verify binary** — Run `cargo run` and confirm it exits cleanly

### Skills to Read

- [skills/rust-best-practices/SKILL.md](skills/rust-best-practices/SKILL.md) — For Rust coding standards and best practices
- [skills/rust-best-practices/references/chapter_01.md](skills/rust-best-practices/references/chapter_01.md) — Rust fundamentals

### Dependencies

None. This is the first story with no dependencies.

## Scope Boundaries

### This Story Includes
- Creating Cargo.toml with all dependencies
- Creating minimal main.rs that compiles
- Verifying build succeeds
- Verifying no clippy warnings

### This Story Does NOT Include
- Module structure (Story 01.2)
- Logging infrastructure (Story 01.3)
- CLI argument parsing (Story 01.4)
- Any business logic

### Files in Scope
- `Cargo.toml` — create
- `src/main.rs` — create

### Files NOT in Scope
- All other src/ files (not created yet)
- Any tests directory
- Any config files
