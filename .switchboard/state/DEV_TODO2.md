# DEV_TODO2 — Development Agent 2

> Sprint: 2
> Focus Area: CLI Entry Point, Tag Entity, and Configuration
> Last Updated: 2026-02-28T18:50:00Z
> Total Points: 6

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`

## Stories

- [ ] **01.3** (REWORK): Setup Logging and Error Handling (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-01-3-setup-logging-error-handling.md`
  - 🔍 Review: See REVIEW_QUEUE.md — CHANGES_REQUESTED
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Address ALL "Must Fix" items:
    1. Add UserError variant to AppError in src/error.rs
    2. Initialize tracing in src/main.rs
    3. Replace println! with tracing in src/main.rs
    4. Revert changes to src/commands.rs, src/lib.rs, src/models.rs, src/task.rs
  - 📝 Commit: `fix(dev2): [01.3] address review feedback`

- [ ] **01.4** (REWORK): Basic CLI Entry Point (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-01-4-basic-cli-entry-point.md`
  - 🔍 Review: See REVIEW_QUEUE.md — CHANGES_REQUESTED
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Address scope violations (revert changes to files outside scope), replace println! with tracing
  - 📝 Commit: `fix(dev2): [01.4] address review feedback`

- [ ] **01.4**: Basic CLI Entry Point (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-01-4-basic-cli-entry-point.md`
  - 📚 Skills: `./skills/skills/rust-best-practices/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev2): [01.4] basic CLI entry point`

- [ ] **02.2**: Define Tag Entity (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-02-2-define-tag-entity.md`
  - 📚 Skills: `./skills/skills/rust-best-practices/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev2): [02.2] define tag entity`

- [ ] **02.4**: Implement Configuration Management (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-02-4-implement-configuration-management.md`
  - 📚 Skills: `./skills/skills/rust-best-practices/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev2): [02.4] implement configuration management`

- [ ] AGENT QA: Run full build and test suite (`cargo build --release && cargo test && cargo clippy -- -D warnings && cargo fmt --check`). If green, create `.switchboard/state/.dev_done_2` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.
