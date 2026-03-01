# DEV_TODO2 — Development Agent 2

> Sprint: 8
> Focus Area: Filtering Commands
> Last Updated: 2026-03-01T04:37:35Z
> Total Points: 5 pts

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `src/filter.rs` — Current filter implementation
- `src/commands.rs` — Command handlers

## Stories

- [ ] **{05.1}**: Filter by Status (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-05-1-filter-by-status.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- list --status incomplete` works
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev2): [05.1] Add filter by status`

- [ ] **{05.3}**: Filter by Due Date (3 pts)
  - 📄 Story: `.switchboard/state/stories/story-05-3-filter-by-due-date.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass
  - ✅ Post-check: Build + tests pass, `cargo run -- list --due-before 2026-03-01` works
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev2): [05.3] Add filter by due date`

## AGENT QA

Run full build and test suite:
```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

If all green, create `.switchboard/state/.dev_done_2` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.
