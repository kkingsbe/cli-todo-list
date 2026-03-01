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
