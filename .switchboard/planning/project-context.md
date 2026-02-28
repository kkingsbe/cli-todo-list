# Project Context — TaskForge

> This file is read by every development agent before implementation.
> Keep it concise. Only include rules that prevent common mistakes.

## Build & Test Commands

- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --check`

## Technology Stack

- **Language:** Rust 1.75+
- **CLI:** clap 4.x (derive macros)
- **Database:** SQLite via rusqlite 0.31.x
- **Date/Time:** chrono 0.4.x
- **Serialization:** serde 1.x
- **UUID:** uuid 1.x
- **Logging:** tracing 0.1.x
- **Errors:** thiserror

## Critical Rules

1. **All public functions must have doc comments** - Use `///` for public API documentation
2. **Error types use thiserror** - Never use anyhow in library code (only in main.rs)
3. **Never use unwrap() or expect() outside tests** - Return Result types properly
4. **Tests live in the same file as the code they test** - Use `#[cfg(test)]` module
5. **Use `tracing` for logging** - Never use println! or eprintln!
6. **All repository operations return Result** - Propagate errors up the stack
7. **Follow the module structure** in architecture.md - Each module has a specific purpose
8. **Write tests first (TDD)** - Red-Green-Refactor cycle, see skills/test-driven-development/SKILL.md

## File Organization

- New modules go in `src/` as `src/{module}.rs`
- Tests go in `src/{module}.rs` as `#[cfg(test)] mod tests { ... }`
- Integration tests go in `tests/integration/`
- Database migrations go in `migrations/` directory

## Naming Conventions

- **Functions:** snake_case (e.g., `create_task`, `list_tasks`)
- **Types:** PascalCase (e.g., `Task`, `TaskFilter`, `AppError`)
- **Files:** snake_case (e.g., `task.rs`, `error.rs`)
- **Enums:** PascalCase with variants (e.g., `Priority::P1`)

## Patterns to Follow

- **Error handling:** See architecture.md §7 - Use thiserror with UserError, ValidationError, SystemError
- **Module structure:** See architecture.md §3 - CLI → Commands → Domain → Repository
- **Data model:** See architecture.md §6 - Task, Tag entities with repository pattern

## Anti-Patterns (Do NOT)

- Do NOT use `unwrap()` in production code
- Do NOT add dependencies without updating architecture.md
- Do NOT modify existing public APIs without updating all callers
- Do NOT use println! for logging - use tracing!
- Do NOT use anyhow in library modules - use thiserror
