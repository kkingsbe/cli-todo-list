# Review Queue

## Sprint 7

### story-03.4: Get Task Details Command

- **Implemented by:** dev-2
- **Sprint:** 7
- **Commits:** 80b630b..c9f4ee3
- **Story file:** `.switchboard/state/stories/story-03-4-get-task-details-command.md`
- **Files changed:** src/cli.rs, src/commands.rs, src/main.rs
- **Status:** ❌ CHANGES_REQUESTED
- **Review date:** 2026-03-01T04:56:00Z
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

## Sprint 9

### 06.1: Delete Task Command

- **Implemented by:** dev-2
- **Sprint:** 9
- **Commits:** 4310cdc..f7c5ae7
- **Story file:** `.switchboard/state/stories/story-06-1-delete-task-command.md`
- **Files changed:** src/cli.rs, src/commands.rs, src/main.rs
- **Status:** ❌ CHANGES_REQUESTED
- **Acceptance Criteria:**
  - [x] Criterion 1 — verified by: Implementation prompts for confirmation when --force is not provided
  - [x] Criterion 2 — verified by: Implementation accepts --force flag to skip confirmation
  - [x] Criterion 3 — verified by: Repository.delete_task removes task from database
  - [x] Criterion 4 — verified by: Implementation returns AppError::NotFound for non-existent IDs
- **Notes:** Implemented delete command with confirmation prompt, --force flag, and proper error handling. All 187 tests pass.

**Must Fix Issues (MUST FIX - blocks approval):**

1. **Convention violation:** Uses `println!`/`eprintln!` instead of `tracing!` - violates project convention from project-context.md which states "Use `tracing` for logging — never `println!` or `eprintln!`"
   - File: src/main.rs:450 - "Deleted task: {id}" uses println!
   - File: src/main.rs:454 - "Task not found" uses eprintln!
2. **Missing tests:** No unit/integration tests for delete_task_with_dyn function - violates convention from project-context.md: "Tests colocated in #[cfg(test)] modules within the same file as the code they test"

**Should Fix Issues:**

- CLI test at cli.rs:233 doesn't test the --force flag
