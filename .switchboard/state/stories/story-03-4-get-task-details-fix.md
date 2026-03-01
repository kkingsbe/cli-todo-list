# Story 03.4: Get Task Details Command (Scope Fix)

> Epic: epic-03 — Task CRUD Operations
> Points: 2
> Sprint: 10
> Type: feature
> Risk: low
> Created: 2026-03-01

## User Story

**As a** user,
**I want** to view a single task with `task get <id>`,
**So that** I can see detailed information.

## Acceptance Criteria

1. `task get <uuid>` shows full task details
   - Output shows title, description, priority, status, dates
   - **Test:** Run `cargo run -- get <valid-uuid>` and verify detailed output

2. Shows 404 error for unknown task ID
   - Invalid ID shows "Task not found" error
   - **Test:** Run `cargo run -- get <invalid-uuid>` and verify error message

## Technical Context

### Issue Summary
The original implementation exceeded story scope by implementing complete/reopen commands that belong to a different story. These out-of-scope changes must be reverted.

### Review Feedback (from REVIEW_QUEUE.md)
- **Status:** CHANGES_REQUESTED
- **Issue:** Scope Violation — Changes to `src/commands.rs` and `src/main.rs` implementing complete/reopen functionality are outside the story's scope
- **Files in Scope:** `src/main.rs` (wire up Show), `src/cli.rs` (Show command), `src/commands.rs` (get_task already exists)
- **Actual changes (MUST REVERT):** Added `complete_task`, `reopen_task`, `complete_task_with_dyn`, `reopen_task_with_dyn` in commands.rs; Implemented `Commands::Complete` and `Commands::Reopen` in main.rs

### Current Code State

**In commands.rs - Functions to REMOVE:**
- Lines 347-370: `complete_task<R: Repository>()`
- Lines 373-396: `reopen_task<R: Repository>()`
- Lines 400-423: `complete_task_with_dyn()`
- Lines 427-450: `reopen_task_with_dyn()`

**In commands.rs - Functions to KEEP (already working):**
- Lines 74-82: `get_task<R: Repository>()`
- Lines 86-94: `get_task_with_dyn()`

**In main.rs - Imports to REMOVE:**
- Line 25: `use crate::commands::complete_task_with_dyn;`
- Line 29: `use crate::commands::reopen_task_with_dyn;`

**In main.rs - Commands to REMOVE:**
- Lines 471-487: `Commands::Complete { id }`
- Lines 488-504: `Commands::Reopen { id }`

**In main.rs - Already implemented (KEEP):**
- Lines 311-343: `Commands::Show { id }` using `commands::get_task_with_dyn()`

**In cli.rs - Variants to REMOVE:**
- Lines 137-141: `Commands::Complete`
- Lines 143-147: `Commands::Reopen`

**In cli.rs - Already defined (KEEP):**
- Lines 87-92: `Commands::Show` with alias `get`

### Architecture Reference
- **Module:** CLI → Commands → Domain → Repository
- **Entry Point:** main.rs wires up CLI commands
- **Command Definition:** cli.rs defines command structure
- **Handler:** commands.rs contains business logic
- **Get task functionality:** Already exists and works

### Project Conventions
- All public functions must have doc comments (`///`)
- Error types use thiserror (never anyhow in library code)
- Never use `unwrap()`/`expect()` outside tests
- Use `tracing` for logging
- Build: `cargo build --release`
- Test: `cargo test`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt --check`

## Implementation Plan

1. **Remove out-of-scope code from src/commands.rs**
   - Delete `complete_task` function (lines 347-370)
   - Delete `reopen_task` function (lines 373-396)
   - Delete `complete_task_with_dyn` function (lines 400-423)
   - Delete `reopen_task_with_dyn` function (lines 427-450)

2. **Remove out-of-scope imports from src/main.rs**
   - Remove `use crate::commands::complete_task_with_dyn;`
   - Remove `use crate::commands::reopen_task_with_dyn;`

3. **Remove out-of-scope command handlers from src/main.rs**
   - Delete `Commands::Complete { id }` match arm (lines 471-487)
   - Delete `Commands::Reopen { id }` match arm (lines 488-504)

4. **Remove out-of-scope command variants from src/cli.rs**
   - Delete `Commands::Complete` variant (lines 137-141)
   - Delete `Commands::Reopen` variant (lines 143-147)

5. **Verify and test**
   - Run `cargo build --release` - should compile
   - Run `cargo test` - should pass
   - Run `cargo clippy -- -D warnings` - should be clean
   - Run `cargo fmt --check` - should pass
   - Test `cargo run -- get <valid-uuid>` - should show task details
   - Test `cargo run -- get <invalid-uuid>` - should show "Task not found"

6. **Update sprint-status.yaml**
   - Mark story 03.4 as `complete`

### Skills to Read
- `./skills/rust-best-practices/SKILL.md` — For proper error handling and code style

### Dependencies
- None (the get task functionality already exists and works)

## Scope Boundaries

### This Story Includes
- Reverting out-of-scope changes to commands.rs, main.rs, cli.rs
- Verifying the get/show command still works correctly

### This Story Does NOT Include
- Implementing complete/reopen functionality (belongs to Epic 06)
- Any other changes to task, tag, or filter functionality

### Files in Scope
- `src/commands.rs` — modify (remove out-of-scope functions)
- `src/main.rs` — modify (remove imports and handlers)
- `src/cli.rs` — modify (remove command variants)

### Files NOT in Scope
- `src/repository.rs` — do NOT modify
- `src/models.rs` — do NOT modify
- `src/task.rs` — do NOT modify
- `src/tag.rs` — do NOT modify
- `src/filter.rs` — do NOT modify
- `src/config.rs` — do NOT modify
- `src/error.rs` — do NOT modify
