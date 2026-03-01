# DEV_TODO1 — Development Agent 1

> Sprint: 8
> Focus Area: Tag Management Commands
> Last Updated: 2026-03-01T04:37:35Z
> Total Points: 5 pts

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `src/tag.rs` — Current tag implementation
- `src/commands.rs` — Command handlers

## Stories

- [ ] **{04.2}**: List Tags Command (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-04-2-list-tags-command.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- tags` shows all tags
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev1): [04.2] Add list tags command`

- [ ] **{04.4}**: Manage Tags on Existing Tasks (3 pts)
  - 📄 Story: `.switchboard/state/stories/story-04-4-manage-tags-on-existing-tasks.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- update <id> --add-tag work` works
  - 🔒 Risk: Medium
  - 📝 Commit: `feat(dev1): [04.4] Add manage tags on existing tasks`

## AGENT QA

Run full build and test suite:
```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

If all green, create `.switchboard/state/.dev_done_1` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.
