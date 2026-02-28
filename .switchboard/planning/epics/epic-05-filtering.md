# Epic 05: Filtering & Sorting

> Priority: 5
> Depends on: 04
> Estimated stories: 4
> Status: not-started

## Description

Implement filtering and sorting capabilities so users can find specific tasks based on various criteria.

## Stories

### Story 05.1: Filter by Status

- **Points:** 2
- **Depends on:** 04.1
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** to filter tasks by status,
**So that** I can see only incomplete tasks.

**Acceptance Criteria:**

1. `task list --status incomplete` shows only incomplete tasks
   - Verification: Output contains no completed tasks

2. `task list --status completed` shows only completed tasks
   - Verification: Output contains only completed tasks

---

### Story 05.2: Filter by Priority and Tags

- **Points:** 3
- **Depends on:** 05.1
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** to filter tasks by priority and tags,
**So that** I can focus on important work.

**Acceptance Criteria:**

1. `task list --priority 1` shows only P1 tasks
   - Verification: All tasks have priority 1

2. `task list --tag work` shows tasks with "work" tag
   - Verification: All tasks have "work" tag

3. Multiple tags use AND logic
   - Verification: `task list --tag work --tag urgent` shows only tasks with both tags

---

### Story 05.3: Filter by Due Date

- **Points:** 3
- **Depends on:** 05.2
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** to filter tasks by due date,
**So that** I can see overdue or upcoming tasks.

**Acceptance Criteria:**

1. `task list --due-before 2026-03-01` shows tasks due before date
   - Verification: All tasks have due_date < specified date

2. `task list --due-after 2026-03-01` shows tasks due after date
   - Verification: All tasks have due_date > specified date

3. `task list --due 2026-03-15` shows tasks due on specific date
   - Verification: All tasks have due_date = specified date

---

### Story 05.4: Sorting

- **Points:** 2
- **Depends on:** 05.1
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** to sort tasks by different fields,
**So that** I can organize my view.

**Acceptance Criteria:**

1. `task list --sort priority` sorts by priority (1 first)
   - Verification: P1 tasks appear before P4 tasks

2. `task list --sort priority --desc` sorts descending
   - Verification: P4 tasks appear before P1 tasks

3. Can sort by created_at, due_date, title
   - Verification: Each sort field orders correctly
