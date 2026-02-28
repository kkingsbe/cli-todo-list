# DEV_TODO1 — Development Agent 1

> Sprint: 3
> Focus Area: Core Data Models (Filter/Sort)
> Last Updated: 2026-02-28T20:10:00Z
> ⚠️ Rebalanced by Sprint Planner on 2026-02-28
> Total Points: 6 (3 original + 3 rebalanced)

## Rework Queue

- **01.3 (REWORK)**: Setup Logging and Error Handling — ✅ queued for review
- **02.3 (REWORK)**: Define Filter and Sort Structures
  - 📄 Story: `.switchboard/state/stories/story-02-3-define-filter-sort.md`
  - 🔍 Review: See REVIEW_QUEUE.md — CHANGES_REQUESTED
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Fix SortOrder default to Descending, update test, ensure build/test/clippy pass
  - 📝 Commit: `fix(dev1): [02.3] address review feedback - sort default`
- **03.1 (REWORK)**: Database Repository Setup
  - 📄 Story: `.switchboard/state/stories/story-03-1-database-repository.md`
  - 🔍 Review: See REVIEW_QUEUE.md — CHANGES_REQUESTED
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Fix schema to match architecture (INTEGER→TEXT for priority, timestamps), add indexes per architecture, fix unwrap() usage, ensure build/test/clippy pass
  - 📝 Commit: `fix(dev1): [03.1] address review feedback - schema and indexes`

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md` (Focus on §5 filter.rs and §6 Data Model)

## Stories

- [x] **02.3**: Define Filter and Sort Structures (3 pts) — ✅ queued for review
  - 📄 Story: `.switchboard/state/stories/story-02-3-define-filter-sort.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass (`cargo build && cargo test`)
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev1): [02.3] define filter and sort structures`

- [x] **03.1**: Database Repository Setup (3 pts) — ✅ queued for review
  - 📄 Story: `.switchboard/state/stories/story-03-1-database-repository.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass (`cargo build && cargo test`)
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Medium
  - 📝 Commit: `feat(dev1): [03.1] database repository setup`

- [x] AGENT QA: Run full build and test suite. If green, create
  `.switchboard/state/.dev_done_1` with date. If ALL `.dev_done_*`
  files exist for all agents with work, also create
  `.switchboard/state/.sprint_complete`.
