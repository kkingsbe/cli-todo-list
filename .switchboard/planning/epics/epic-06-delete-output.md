# Epic 06: Delete Task and Output Formats

> Priority: 6
> Depends on: 05
> Estimated stories: 3
> Status: not-started

## Description

Implement task deletion with confirmation and support for multiple output formats.

## Stories

### Story 06.1: Delete Task Command

- **Points:** 3
- **Depends on:** 05.1
- **Risk:** Medium
- **Type:** feature

**As a** user,
**I want** to delete a task with `task delete <id>`,
**So that** I can remove unwanted tasks.

**Acceptance Criteria:**

1. `task delete <id>` prompts for confirmation
   - Verification: User must confirm before deletion

2. `task delete <id> --force` deletes without confirmation
   - Verification: Task is deleted immediately

3. Deleted task no longer appears in list
   - Verification: Task is completely removed

4. Returns error for non-existent task ID
   - Verification: Deleting unknown ID shows error message

---

### Story 06.2: Output Format Support

- **Points:** 2
- **Depends on:** 05.1
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** different output formats,
**So that** I can use the data elsewhere.

**Acceptance Criteria:**

1. `task list --format table` shows table output (default)
   - Verification: Output uses table formatting

2. `task list --format plain` shows plain text
   - Verification: Output is simple line-by-line

3. `task list --format json` shows JSON output
   - Verification: Output is valid JSON array

---

### Story 06.3: Complete and Toggle Status

- **Points:** 2
- **Depends on:** 05.1
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** quick commands to mark tasks complete,
**So that** I can track progress efficiently.

**Acceptance Criteria:**

1. `task complete <id>` marks task as completed
   - Verification: Task status changes to completed

2. `task reopen <id>` marks task as incomplete
   - Verification: Task status changes to incomplete

3. Output confirms the status change
   - Verification: Success message shown
