# Review Queue

## Sprint 7

### story-03.4: Get Task Details Command

- **Implemented by:** dev-2
- **Sprint:** 7
- **Commits:** 80b630b..c9f4ee3
- **Story file:** `.switchboard/state/stories/story-03-4-get-task-details-command.md`
- **Files changed:** src/cli.rs, src/commands.rs, src/main.rs
- **Status:** ✅ PENDING_REVIEW
- **Review date:** 2026-03-01T23:25:25Z
- **Acceptance Criteria:**
  - [x] Criterion 1 — `task get <uuid>` shows full task details — MET
  - [x] Criterion 2 — Shows 404 error for unknown task ID — MET
- **Must Fix:**
  1. **Scope Violation** — Changes to `src/commands.rs` and `src/main.rs` implementing complete/reopen functionality are outside the story's scope
     - Files in Scope per story: `src/main.rs` (wire up Show), `src/cli.rs` (Show command), `src/commands.rs` (get_task already exists)
     - Actual changes: Added `complete_task`, `reopen_task`, `complete_task_with_dyn`, `reopen_task_with_dyn` in commands.rs; Implemented `Commands::Complete` and `Commands::Reopen` in main.rs
     - Expected: Only add `#[command(alias = "get")]` to Show command in cli.rs (the Show command was already wired in previous stories)
  2. Revert all changes to `src/commands.rs` except what was already there for get_task
  3. Revert all changes to `src/main.rs` that implement Commands::Complete and Commands::Reopen
- **Fix applied:** Commit d75bd0b — Reverted out-of-scope complete/reopen code from commands.rs, cli.rs, and main.rs
- **Requeue Instructions:** Revert the out-of-scope changes. The core "get" alias functionality already works correctly.
- **Summary:** The "get" alias works perfectly and both acceptance criteria are verified. However, scope was exceeded by implementing complete/reopen commands which belong to a different story.

## Sprint 8

### story-05.3: Filter by Due Date

- **Implemented by:** dev-2
- **Sprint:** 8
- **Story file:** `.switchboard/state/stories/story-05-3-filter-by-due-date.md`
- **Files in Scope:**
  - src/filter.rs — modify (due date fields already exist)
  - src/commands.rs — modify (implement due date filtering)
  - src/cli.rs — modify (add due date arguments)
  - src/repository.rs — modify (add due date WHERE clause)
- **Files NOT in Scope:**
  - src/task.rs
  - src/tag.rs
  - src/models.rs
- **Acceptance Criteria:**
  1. `task list --due-before 2026-03-01` shows tasks due before date
     - Test: Run `cargo run -- list --due-before 2026-03-01` and verify all tasks have due_date < specified date
  2. `task list --due-after 2026-03-01` shows tasks due after date
     - Test: Run `cargo run -- list --due-after 2026-03-01` and verify all tasks have due_date > specified date
  3. `task list --due 2026-03-15` shows tasks due on specific date
     - Test: Run `cargo run -- list --due 2026-03-15` and verify all tasks have due_date = specified date
- **Technical Context:**
  - Module: src/filter.rs — TaskFilter struct already has due_before and due_after fields (lines 36, 38)
  - Module: src/commands.rs — list_tasks function uses TaskFilter
  - Module: src/cli.rs — CLI command definitions
  - Module: src/repository.rs — Database operations with filter support
  - Data model: Task entity with due_date: Option<DateTime<Utc>> field
  - Date format: YYYY-MM-DD
- **Implementation Details:**
  - Add WHERE clause for due dates in list_tasks query
  - Filter tasks by due_date column using <, >, = operators
  - Handle NULL due_date (tasks without due date)
- **Status:** ✅ APPROVED
- **Fix applied:** Commit 7ab5f1d — fixes have been applied
- **Review status:** ✅ APPROVED

## Sprint 9

### 06.1: Delete Task Command

- **Implemented by:** dev-2
- **Sprint:** 9
- **Commits:** 4310cdc..0a99cc6
- **Story file:** `.switchboard/state/stories/archive/sprint-9/story-06-1-delete-task-command.md`
- **Files changed:** src/cli.rs, src/commands.rs, src/main.rs
- **Status:** ✅ APPROVED
- **Reviewed by:** code-reviewer
- **Review date:** 2026-03-01
- **Acceptance Criteria:**
  - [x] `task delete <id>` prompts for confirmation — MET
  - [x] `task delete <id> --force` deletes without confirmation — MET
  - [x] Deleted task no longer appears in list — MET
  - [x] Returns error for non-existent task ID — MET
- **Must Fix:**
  1. Unused imports in src/main.rs:11
     - Current: `use tracing::{error, info, warn};`
     - Expected: `use tracing::info;` (remove unused `error` and `warn`)
     - Why: Clippy fails with `-D warnings` flag, introduced by story-06.1
- **Should Fix:** None
- **Requeue Instructions:** 
  - Fix the unused imports in src/main.rs:11
  - Re-run cargo clippy -- -D warnings to verify
  - Re-queue for review


### story-06.1: Delete Task Command

- **Status:** ✅ APPROVED
- **Reviewed by:** code-reviewer
- **Review date:** 2026-03-01T10:50:03Z
- **Acceptance Criteria:**
  - [x] Criterion 1 — MET: `task delete <id>` prompts for confirmation
  - [x] Criterion 2 — MET: `task delete <id> --force` deletes without confirmation
  - [x] Criterion 3 — MET: Deleted task no longer appears in list
  - [x] Criterion 4 — MET: Returns error for non-existent task ID
- **Must Fix:**
  1. ~~Formatting issue in `src/commands.rs:995`~~ - ✅ FIXED via `cargo fmt` (commit ead2785)
- **Should Fix:**
  1. Clippy warning in `src/main.rs:11` — unused imports (pre-existing, not from this story)
- **Requeue Instructions:** Run `cargo fmt` to fix the formatting issue, then re-queue for review
- **Rework completed:** 2026-03-01T11:55:00Z
- **Rework completed:** 2026-03-01T11:55:00Z

## Sprint 10

### 03.4: Get Task Details Command - Scope Fix

- **Implemented by:** dev-1
- **Sprint:** 10
- **Commits:** 80b630b..c9f4ee3 (fix-up: d75bd0b)
- **Story file:** `.switchboard/state/stories/story-03-4-get-task-details-fix.md`
- **Files changed:** src/commands.rs, src/main.rs, src/cli.rs
- **Status:** PENDING_REVIEW
- **Acceptance Criteria:**
  - [x] `task get <uuid>` shows full task details (title, description, priority, status, dates) — verified by: cargo run -- get <uuid>
  - [x] Shows 404 error for unknown task ID — verified by: cargo run -- get <invalid-uuid>
  - [x] Complete/reopen commands removed from scope — verified by: cargo run -- complete/reopen returns "unrecognized subcommand"
- **Notes:** Removed out-of-scope complete/reopen commands that were added in error. The 'get' command was already working correctly.
