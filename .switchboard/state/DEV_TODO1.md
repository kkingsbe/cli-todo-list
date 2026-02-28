# DEV_TODO1 — Development Agent 1

> Sprint: 4
> Focus Area: Task CRUD Operations - Create & List
> Last Updated: 2026-02-28T21:05:00Z
> Total Points: 6

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `./skills/rust-best-practices/SKILL.md`
- `./skills/test-driven-development/SKILL.md`

## Stories

- [ ] **{story-03.2}**: Create Task Command (3 pts)
  - 📄 Story: `.switchboard/state/stories/story-03-2-create-task-command.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass (`cargo build --release && cargo test`)
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev1): [03.2] implement create task command`
  - **Pre-requisite:** 03.1 Database Repository Setup (complete)

- [ ] **{story-03.3}**: List Tasks Command (3 pts)
  - 📄 Story: `.switchboard/state/stories/story-03-3-list-tasks-command.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev1): [03.3] implement list tasks command`
  - **Dependency:** 03.2 must be complete first (dependency chain)

- [ ] **AGENT QA:** Run full build and test suite. If green, create `.switchboard/state/.dev_done_1` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.

## Implementation Notes

- Story 03.3 depends on 03.2 (the list command needs tasks to list)
- Complete 03.2 first, then 03.3
- Both stories touch `main.rs`, `commands.rs`, and `repository.rs`
- Run `cargo clippy -- -D warnings` before marking complete
