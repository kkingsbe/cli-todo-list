# Story 04.4: Manage Tags on Existing Tasks

> Epic: epic-04 — Tagging System
> Points: 3
> Sprint: 8
> Type: feature
> Risk: medium
> Created: 2026-03-01T04:37:35Z

## User Story

**As a** user,
**I want** to add or remove tags from existing tasks,
**So that** I can reorganize tasks.

## Acceptance Criteria

1. Can add tags to existing task with `task update <id> --add-tag <name>`
   - **Test:** Run `cargo run -- update <task-id> --add-tag work` and verify task shows new tag in list

2. Can remove tags from task with `task update <id> --remove-tag <name>`
   - **Test:** Run `cargo run -- update <task-id> --remove-tag work` and verify tag is removed from task

3. Tags case-insensitive for lookup but preserve display case
   - **Test:** Create tag "Work", then use "work" to find it - should display as "Work"

## Technical Context

### Architecture Reference
- Module: src/task.rs — Task entity
- Module: src/tag.rs — Tag entity
- Module: src/commands.rs — Command handlers (add_tag_to_task, remove_tag_from_task exist but return empty)
- Module: src/cli.rs — CLI command definitions
- Module: src/repository.rs — Database operations
- Data model: Task, Tag, TaskTag (relationship) entities from architecture.md

### Project Conventions
Follow all rules from .switchboard/planning/project-context.md

### Existing Code Context
- src/commands.rs has `add_tag_to_task` function at line 398-406 but returns Ok(())
- src/commands.rs has `remove_tag_from_task` function at line 409-417 but returns Ok(())
- Repository trait has task_tag operations (needs verification)
- Task model has tags field (needs verification)

## Implementation Plan

1. **src/repository.rs** — Add methods for tag-task relationship management:
   - `add_tag_to_task(task_id, tag_id)` — insert into task_tags table
   - `remove_tag_from_task(task_id, tag_id)` — delete from task_tags table
   - `get_or_create_tag(name)` — get existing tag or create new one

2. **src/commands.rs** — Update `update_task` or create new function to handle --add-tag and --remove-tag flags

3. **src/cli.rs** — Add --add-tag and --remove-tag arguments to update command

4. **src/task.rs** — May need to add tags field to Task struct or create separate query

5. Run build + tests

### Skills to Read
- ./skills/rust-best-practices/SKILL.md
- ./skills/test-driven-development/SKILL.md

### Dependencies
Stories that must be complete: 04.1 (Create Tag with Task)

## Scope Boundaries

### This Story Includes
- Implement `task update <id> --add-tag <name>` functionality
- Implement `task update <id> --remove-tag <name>` functionality
- Auto-create tags if they don't exist when adding
- Case-insensitive tag lookup

### This Story Does NOT Include
- Tag deletion command (Story 04.3)
- Bulk tag operations

### Files in Scope
- src/task.rs — modify (may need tags field)
- src/tag.rs — modify (may need get_or_create method)
- src/commands.rs — modify (implement tag management)
- src/cli.rs — modify (add tag arguments)
- src/repository.rs — modify (add tag-task relationship methods)

### Files NOT in Scope
- src/filter.rs — don't modify
- src/models.rs — don't modify (unless needed for new types)
