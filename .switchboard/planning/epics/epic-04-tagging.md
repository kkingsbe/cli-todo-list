# Epic 04: Tagging System

> Priority: 4
> Depends on: 03
> Estimated stories: 4
> Status: not-started

## Description

Implement tag management functionality including creating tags, viewing tags, and associating tags with tasks.

## Stories

### Story 04.1: Create Tag with Task

- **Points:** 3
- **Depends on:** 03.2
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** to add tags when creating a task,
**So that** I can organize tasks by category.

**Acceptance Criteria:**

1. `task add "Task" --tag work --tag urgent` creates task with tags
   - Verification: Task shows in list with both tags

2. Tags are created automatically if they don't exist
   - Verification: New tag "project" is created when first used

3. Multiple tags work correctly
   - Verification: Task has all specified tags

---

### Story 04.2: List Tags Command

- **Points:** 2
- **Depends on:** 04.1
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** to view all tags with `task tags`,
**So that** I know what tags exist.

**Acceptance Criteria:**

1. `task tags` shows all unique tags
   - Verification: Output lists all tags in system

2. Shows usage count for each tag
   - Verification: Tag shows number of tasks using it

---

### Story 04.3: Delete Tag Command

- **Points:** 2
- **Depends on:** 04.2
- **Risk:** Medium
- **Type:** feature

**As a** user,
**I want** to delete a tag with `task tag delete <name>`,
**So that** I can remove unused tags.

**Acceptance Criteria:**

1. `task tag delete <name>` removes the tag
   - Verification: Tag no longer appears in `task tags`

2. Tag is removed from all associated tasks
   - Verification: Tasks no longer show the deleted tag

3. Error if tag doesn't exist
   - Verification: Deleting non-existent tag shows error

---

### Story 04.4: Manage Tags on Existing Tasks

- **Points:** 3
- **Depends on:** 04.1
- **Risk:** Medium
- **Type:** feature

**As a** user,
**I want** to add or remove tags from existing tasks,
**So that** I can reorganize tasks.

**Acceptance Criteria:**

1. Can add tags to existing task with `task update <id> --add-tag <name>`
   - Verification: Task shows new tag in list

2. Can remove tags from task with `task update <id> --remove-tag <name>`
   - Verification: Tag is removed from task

3. Tags case-insensitive for lookup but preserve display case
   - Verification: "Work" tag displays as "Work" but "work" finds it
