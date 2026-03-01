# DEV_TODO1 — Development Agent 1

> Sprint: 7
> Focus Area: Tagging and Output Features
> Last Updated: 2026-03-01T02:44:00Z
> Total Points: 5 pts (3 + 2)

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `src/cli.rs` — Current CLI structure
- `src/commands.rs` — Current command handlers

## Stories

- [ ] **{04.1}**: Create Tag with Task (3 pts)
  - 📄 Story: `.switchboard/state/stories/story-04-1-create-tag-with-task.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- add "Test" --tag work` works
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev1): [04.1] Create Tag with Task`

- [ ] **{06.2}**: Output Format Support (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-06-2-output-format.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- list --format json` outputs valid JSON
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev1): [06.2] Output Format Support`

## AGENT QA

Run full build and test suite:
```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

If all green, create `.dev_done_1` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.

> ⚠️ Rebalanced by Sprint Planner on 2026-03-01
