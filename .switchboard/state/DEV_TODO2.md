# DEV_TODO2 — Development Agent 2

> Sprint: 7
> Focus Area: Task Details Command
> Last Updated: 2026-03-01T02:44:00Z
> Total Points: 5 pts

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `src/cli.rs` — Current CLI structure (Show command)
- `src/commands.rs` — get_task function
- `src/main.rs` — Command wiring

## Stories

- [x] **{04.1}**: Create Tag with Task (3 pts) ✅ queued for review
  - 📄 Story: `.switchboard/state/stories/story-04-1-create-tag-with-task.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- add "Test" --tag work` works
  - 🔒 Risk: Low
  - 📝 Commit: Already implemented by dev1 in commit 626a49b

- [x] **{03.4}**: Get Task Details Command (2 pts) ✅ queued for review
  - 📄 Story: `.switchboard/state/stories/story-03-4-get-task-details-command.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- get <task-id>` shows full details
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev2): [03.4] Get Task Details Command`

## AGENT QA

Run full build and test suite:
```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

If all green, create `.switchboard/state/.dev_done_2` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.

> ⚠️ Rebalanced by Sprint Planner on 2026-03-01
