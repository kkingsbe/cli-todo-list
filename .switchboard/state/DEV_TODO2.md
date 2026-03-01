# DEV_TODO2 — Development Agent 2

> Sprint: 9
> Focus Area: Task Deletion
> Last Updated: 2026-03-01T07:40:00Z
> Total Points: 3 pts

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `src/cli.rs` — CLI command definitions
- `src/commands.rs` — Command handlers
- `src/task.rs` — Current task implementation
- `src/repository.rs` — Database operations

## Stories

- [ ] **{06.1}**: Delete Task Command (3 pts)
  - 📄 Story: `.switchboard/state/stories/story-06-1-delete-task-command.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- delete <id> --force` removes task
  - 🔒 Risk: Medium
  - 📝 Commit: `feat(dev2): [06.1] Add delete task command with confirmation`

## AGENT QA

Run full build and test suite:
```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

If all green, create `.switchboard/state/.dev_done_2` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.
