# Story 05.3: Filter by Due Date

> Epic: epic-05 — Filtering & Sorting
> Points: 3
> Sprint: 8
> Type: feature
> Risk: low
> Created: 2026-03-01T04:37:35Z

## User Story

**As a** user,
**I want** to filter tasks by due date,
**So that** I can see overdue or upcoming tasks.

## Acceptance Criteria

1. `task list --due-before 2026-03-01` shows tasks due before date
   - **Test:** Run `cargo run -- list --due-before 2026-03-01` and verify all tasks have due_date < specified date

2. `task list --due-after 2026-03-01` shows tasks due after date
   - **Test:** Run `cargo run -- list --due-after 2026-03-01` and verify all tasks have due_date > specified date

3. `task list --due 2026-03-15` shows tasks due on specific date
   - **Test:** Run `cargo run -- list --due 2026-03-15` and verify all tasks have due_date = specified date

## Technical Context

### Architecture Reference
- Module: src/filter.rs — TaskFilter struct already has due_before and due_after fields
- Module: src/commands.rs — list_tasks function uses TaskFilter
- Module: src/cli.rs — CLI command definitions
- Module: src/repository.rs — Database operations with filter support
- Data model: Task entity with due_date field from architecture.md

### Project Conventions
Follow all rules from .switchboard/planning/project-context.md

### Existing Code Context
- src/filter.rs has TaskFilter struct with due_before field (line 36)
- src/filter.rs has TaskFilter struct with due_after field (line 38)
- src/models.rs has Task with due_date: Option<DateTime<Utc>> field
- Database schema has due_date column with index (architecture.md line 221)
- Repository likely has filter method but may need date-specific query

## Implementation Plan

1. **src/repository.rs** — Add WHERE clause for due dates in list_tasks query
   - Filter tasks by due_date column using <, >, = operators
   - Handle NULL due_date (tasks without due date)

2. **src/commands.rs** — Update `list_tasks` to pass due date filters to repository

3. **src/cli.rs** — Add --due-before, --due-after, and --due arguments to list command

4. **src/filter.rs** — Add tests for due date filtering

5. Run build + tests

### Skills to Read
- ./skills/rust-best-practices/SKILL.md
- ./skills/test-driven-development/SKILL.md

### Dependencies
Stories that must be complete: 05.2 (Filter by Priority and Tags) - note: this is already complete according to sprint-status.yaml

## Scope Boundaries

### This Story Includes
- Implement `task list --due-before <date>` filtering
- Implement `task list --due-after <date>` filtering
- Implement `task list --due <date>` filtering (exact match)
- Date format: YYYY-MM-DD

### This Story Does NOT Include
- Time component in due dates
- Relative date filtering (e.g., "due today", "due this week")

### Files in Scope
- src/filter.rs — modify (due date fields already exist)
- src/commands.rs — modify (implement due date filtering)
- src/cli.rs — modify (add due date arguments)
- src/repository.rs — modify (add due date WHERE clause)

### Files NOT in Scope
- src/task.rs — don't modify
- src/tag.rs — don't modify
- src/models.rs — don't modify
