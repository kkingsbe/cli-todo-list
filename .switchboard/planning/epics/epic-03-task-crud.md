# Epic 03: Task CRUD Operations

> Priority: 3
> Depends on: 02
> Estimated stories: 5
> Status: not-started

## Description

Implement the core task management functionality: creating tasks, viewing tasks, updating tasks, and deleting tasks through the CLI.

## Stories

### Story 03.1: Database Repository Setup

- **Points:** 3
- **Depends on:** 02.1
- **Risk:** Medium
- **Type:** infrastructure

**As a** developer,
**I want** a repository that handles database operations,
**So that** I can persist and retrieve tasks.

**Acceptance Criteria:**

1. SqliteRepository implements Repository trait
   - Verification: Can insert and retrieve a Task from SQLite

2. Database schema matches architecture.md (tasks, tags, task_tags tables)
   - Verification: Tables created on first run with correct indexes

3. Database stored at ~/.taskforge/tasks.db
   - Verification: Database file created in correct location

---

### Story 03.2: Create Task Command

- **Points:** 3
- **Depends on:** 03.1
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** to create a task with `task add "title"`,
**So that** I can track new work.

**Acceptance Criteria:**

1. `task add "Buy groceries"` creates a task with the given title
   - Verification: Task appears in list after creation

2. Task has auto-generated UUID and timestamps
   - Verification: Task has valid UUID, created_at, updated_at

3. Default status is "incomplete", default priority is from config (or 3)
   - Verification: New tasks show incomplete status

4. Output shows task ID after creation
   - Verification: Command output includes the new task ID

---

### Story 03.3: List Tasks Command

- **Points:** 3
- **Depends on:** 03.2
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** to view all tasks with `task list`,
**So that** I can see my current work.

**Acceptance Criteria:**

1. `task list` shows all tasks in table format
   - Verification: Output displays all task fields

2. Tasks sorted by created_at descending by default
   - Verification: Most recent tasks appear first

3. Pagination with --limit flag (default 50)
   - Verification: Can limit output to 10 tasks

---

### Story 03.4: Get Task Details Command

- **Points:** 2
- **Depends on:** 03.3
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** to view a single task with `task get <id>`,
**So that** I can see detailed information.

**Acceptance Criteria:**

1. `task get <uuid>` shows full task details
   - Verification: Output shows title, description, priority, status, dates

2. Shows 404 error for unknown task ID
   - Verification: Invalid ID shows "Task not found" error

---

### Story 03.5: Update Task Command

- **Points:** 5
- **Depends on:** 03.4
- **Risk:** Medium
- **Type:** feature

**As a** user,
**I want** to update a task with `task update <id> [options]`,
**So that** I can modify task details.

**Acceptance Criteria:**

1. Can update title with --title flag
   - Verification: `task update <id> --title "New title"` changes title

2. Can update description with --description flag
   - Verification: Description field updates correctly

3. Can update priority with --priority flag (1-4)
   - Verification: Priority changes as specified

4. Can mark complete/incomplete with --status flag
   - Verification: `task update <id> --status completed` changes status

5. Can update due date with --due flag
   - Verification: Due date updates correctly

6. updated_at timestamp updates on any change
   - Verification: updated_at is newer than created_at after update
