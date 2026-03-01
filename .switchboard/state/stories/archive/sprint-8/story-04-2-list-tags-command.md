# Story 04.2: List Tags Command

> Epic: epic-04 — Tagging System
> Points: 2
> Sprint: 8
> Type: feature
> Risk: low
> Created: 2026-03-01T04:37:35Z

## User Story

**As a** user,
**I want** to view all tags with `task tags`,
**So that** I know what tags exist.

## Acceptance Criteria

1. `task tags` shows all unique tags
   - **Test:** Run `cargo run -- tags` and verify output lists all tags in system

2. Shows usage count for each tag
   - **Test:** Run `cargo run -- tags` and verify each tag shows number of tasks using it

## Technical Context

### Architecture Reference
- Module: src/tag.rs — Tag entity and TagManager
- Module: src/commands.rs — Command handlers (list_tags function exists but returns empty)
- Module: src/cli.rs — CLI command definitions
- Module: src/repository.rs — Database operations
- Data model: Tag, Task entities from architecture.md

### Project Conventions
Follow all rules from .switchboard/planning/project-context.md

### Existing Code Context
- src/tag.rs already defines Tag struct with id, name, color, created_at
- src/commands.rs has `list_tags` function at line 381-388 but returns empty Vec
- src/filter.rs has TagFilter struct for filtering
- Repository trait has list_tags method (needs verification)

## Implementation Plan

1. **src/repository.rs** — Add `list_tags()` method to Repository trait and SqliteRepository implementation
   - Query all tags from tags table
   - Optionally join with task_tags to get count

2. **src/commands.rs** — Update `list_tags` function to call repository and return actual tags

3. **src/cli.rs** — Add `tags` subcommand to CLI (if not already present)

4. **src/tag.rs** — Add tests for tag listing functionality

5. Run build + tests

### Skills to Read
- ./skills/rust-best-practices/SKILL.md
- ./skills/test-driven-development/SKILL.md

### Dependencies
Stories that must be complete: 04.1 (Create Tag with Task)

## Scope Boundaries

### This Story Includes
- Implement `task tags` command showing all tags
- Show usage count per tag
- Add repository method to list tags

### This Story Does NOT Include
- Tag deletion (Story 04.3)
- Tag colors display customization

### Files in Scope
- src/tag.rs — modify (add list functionality)
- src/commands.rs — modify (implement list_tags)
- src/cli.rs — modify (add tags command)
- src/repository.rs — modify (add list_tags method)

### Files NOT in Scope
- src/filter.rs — don't modify
- src/task.rs — don't modify
