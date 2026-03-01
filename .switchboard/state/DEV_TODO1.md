# DEV_TODO1 — Development Agent 1

> Sprint: 9
> Focus Area: Tag Management & Sorting
> Last Updated: 2026-03-01T07:40:00Z
> Total Points: 4 pts

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `src/tag.rs` — Current tag implementation
- `src/filter.rs` — Current filter/sort implementation
- `src/repository.rs` — Database operations
- `src/commands.rs` — Command handlers

## Stories

- [ ] **{04.3}**: Delete Tag Command (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-04-3-delete-tag-command.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- tag delete work` removes tag
  - 🔒 Risk: Medium
  - 📝 Commit: `feat(dev1): [04.3] Add delete tag command`

- [ ] **{05.4}**: Sorting (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-05-4-sorting.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- list --sort priority` sorts correctly
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev1): [05.4] Add task sorting functionality`

## AGENT QA

Run full build and test suite:
```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

If all green, create `.switchboard/state/.dev_done_1` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.
