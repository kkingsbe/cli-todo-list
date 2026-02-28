# Epic 01: Project Scaffolding

> Priority: 1
> Depends on: None
> Estimated stories: 4
> Status: not-started

## Description

Set up the foundational project structure including Cargo.toml, basic module structure, dependencies, and a passing "hello world" build. This ensures all subsequent stories have a green build to start from.

## Stories

### Story 01.1: Initialize Cargo Project

- **Points:** 1
- **Depends on:** None
- **Risk:** Low
- **Type:** infrastructure

**As a** developer,
**I want** a basic Rust project with Cargo.toml configured,
**So that** I can build and test the application.

**Acceptance Criteria:**

1. Cargo.toml exists with all required dependencies (clap, rusqlite, chrono, serde, uuid, tracing, thiserror)
   - Verification: `cargo build --release` succeeds

2. Basic main.rs exists with minimal code
   - Verification: Binary compiles and runs without errors

3. Project compiles without warnings
   - Verification: `cargo clippy -- -D warnings` passes

---

### Story 01.2: Create Module Structure

- **Points:** 2
- **Depends on:** 01.1
- **Risk:** Low
- **Type:** infrastructure

**As a** developer,
**I want** the module structure defined in architecture.md,
**So that** I can organize code properly.

**Acceptance Criteria:**

1. src/main.rs creates the module structure
   - Verification: File contains `mod cli;`, `mod commands;`, etc.

2. Empty module files exist (cli.rs, commands.rs, models.rs, task.rs, tag.rs, filter.rs, repository.rs, error.rs, config.rs)
   - Verification: `cargo check` succeeds

3. All modules compile without errors
   - Verification: `cargo build` succeeds

---

### Story 01.3: Setup Logging and Error Handling

- **Points:** 2
- **Depends on:** 01.2
- **Risk:** Low
- **Type:** infrastructure

**As a** developer,
**I want** logging and error handling infrastructure in place,
**So that** I can debug issues and handle errors gracefully.

**Acceptance Criteria:**

1. error.rs defines AppError using thiserror with UserError, ValidationError, SystemError variants
   - Verification: `cargo check` succeeds, error types are public

2. Logging is initialized in main.rs using tracing
   - Verification: Application runs without panic on startup

3. Error handling in main.rs propagates errors properly
   - Verification: Run with invalid args shows user-friendly error

---

### Story 01.4: Basic CLI Entry Point

- **Points:** 2
- **Depends on:** 01.3
- **Risk:** Low
- **Type:** infrastructure

**As a** user,
**I want** to see a help message when running `task --help`,
**So that** I know how to use the CLI.

**Acceptance Criteria:**

1. CLI module defines --help and --version flags
   - Verification: `task --help` shows help text

2. Basic clap derive setup in cli.rs
   - Verification: Binary runs and responds to --help

3. Empty main command that returns success
   - Verification: `cargo run` exits with code 0
