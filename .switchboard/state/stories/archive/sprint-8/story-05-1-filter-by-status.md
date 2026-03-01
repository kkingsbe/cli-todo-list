# Story 05.1: Filter by Status

> Epic: epic-05 — Filtering & Sorting
> Points: 2
> Sprint: 8
> Type: feature
> Risk: low
> Created: 2026-03-01T04:37:35Z

## User Story

**As a** user,
**I want** to filter tasks by status,
**So that** I can see only incomplete tasks.

## Acceptance Criteria

1. `task list --status incomplete` shows only incomplete tasks
   - **Test:** Run `cargo run -- list --status incomplete` and verify output contains no completed tasks

2. `task list --status completed` shows only completed tasks
   - **Test:** Run `cargo run -- list --status completed` and verify output contains only completed tasks

## Technical Context

### Architecture Reference
- Module: src/filter.rs — TaskFilter struct already has `status` field
- Module: src/commands.rs — list_tasks function uses TaskFilter
- Module: src/cli.rs — CLI command definitions
- Module: src/repository.rs — Database operations with filter support
- Data model: Task, Status enum from architecture.md

### Project Conventions
Follow all rules from .switchboard/planning/project-context.md

### Existing Code Context
- src/filter.rs has TaskFilter struct with status field (line 30)
- src/filter.rs has TaskFilter::with_status method (line 50-53)
- src/models.rs has Status enum (needs verification)
- src/commands.rs has list_tasks function (line 58-66) but returns empty Vec
- Repository likely has filter method but may need status-specific query

## Implementation Plan

1. **src/repository.rs** — Add WHERE clause for status in list_tasks query
   - Filter tasks by status column in SQLite

2. **src/commands.rs** — Update `list_tasks` to pass filter to repository

3. **src/cli.rs** — Add --status argument to list command (if not already present)

4. **src/filter.rs** — Add tests for status filtering

5. Run build + tests

### Skills to Read
- ./skills/rust-best-practices/SKILL.md
- ./skills/test-driven-development/SKILL.md

### Dependencies
Stories that must be complete: 04.1 (Create Tag with Task)

## Scope Boundaries

### This Story Includes
- Implement `task list --status incomplete` filtering
- Implement `task list --status completed` filtering
- Status can be "incomplete" or "completed"

### This Story Does NOT Include
- Priority filtering (Story 05.2)
- Due date filtering (Story 05.3)
- Tag filtering (Story 05.2)

### Files in Scope
- src/filter.rs — modify (status filter already exists)
- src/commands.rs — modify (implement list_tasks with filter)
- src/cli.rs — modify (add --status argument)
- src/repository.rs — modify (add status WHERE clause)

### Files NOT in Scope
- src/task.rs — don't modify
- src/tag.rs — don't modify
- src/models.rs — don't modify (unless Status enum needs changes)
