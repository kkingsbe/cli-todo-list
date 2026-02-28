# DEV_TODO2 — Development Agent 2

> Sprint: 4
> Focus Area: Task Details & Tagging System
> Last Updated: 2026-02-28T21:05:00Z
> Total Points: 5

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `./skills/rust-best-practices/SKILL.md`

## Stories

- [ ] **{story-03.4}**: Get Task Details Command (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-03-4-get-task-details-command.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`
  - ⚡ Pre-check: Build + tests pass (`cargo build --release && cargo test`)
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev2): [03.4] implement get task details command`
  - **Dependency:** 03.3 List Tasks Command (for testing, but can work in parallel)

- [ ] **{story-04.1}**: Create Tag with Task (3 pts)
  - 📄 Story: `.switchboard/state/stories/story-04-1-create-tag-with-task.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev2): [04.1] implement create tag with task`
  - **Dependency:** 03.2 Create Task Command must be complete (uses task creation)

- [ ] **AGENT QA:** Run full build and test suite. If green, create `.switchboard/state/.dev_done_2` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.

## Implementation Notes

- Story 04.1 adds tagging to task creation (depends on 03.2 being complete, not done by same agent)
- Story 03.4 is independent - can start anytime
- Story 04.1 requires modifying cli.rs to add --tag flag
- Run `cargo clippy -- -D warnings` before marking complete
