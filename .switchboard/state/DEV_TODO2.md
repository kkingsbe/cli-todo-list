# DEV_TODO2 — Development Agent 2

> Sprint: 9
> Focus Area: Task Deletion, Tag Management & Sorting
> Last Updated: 2026-03-01T08:08:54Z
> Total Points: 8 pts

> ⚠️ Rebalanced by Sprint Planner on 2026-03-01

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `src/cli.rs` — CLI command definitions
- `src/commands.rs` — Command handlers
- `src/task.rs` — Current task implementation
- `src/repository.rs` — Database operations
- `src/tag.rs` — Current tag implementation
- `src/filter.rs` — Current filter/sort implementation

## Stories

- [x] **{06.1}**: Delete Task Command (3 pts) ✅ queued for review
  - 📄 Story: `.switchboard/state/stories/story-06-1-delete-task-command.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- delete <id> --force` removes task
  - 🔒 Risk: Medium
  - 📝 Commit: `feat(dev2): [06.1] Add delete task command with confirmation`

- [ ] **{04.4}** (REWORK): Manage Tags on Existing Tasks
  - 📄 Story: `.switchboard/state/stories/archive/sprint-8/story-04-4-manage-tags-on-existing-tasks.md`
  - 🔍 Review: See `.switchboard/state/review/REVIEW_QUEUE.md` — CHANGES_REQUESTED
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Replace eprintln!/println! with tracing::error!/tracing::info! in src/main.rs
  - 📝 Commit: `fix(dev2): [04.4] Replace eprintln!/println! with tracing`

- [ ] **{04.3}**: Delete Tag Command (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-04-3-delete-tag-command.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- tag delete work` removes tag
  - 🔒 Risk: Medium
  - 📝 Commit: `feat(dev2): [04.3] Add delete tag command`

- [ ] **{05.4}**: Sorting (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-05-4-sorting.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- list --sort priority` sorts correctly
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev2): [05.4] Add task sorting functionality`

## AGENT QA

Run full build and test suite:
```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

If all green, create `.switchboard/state/.dev_done_2` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.
