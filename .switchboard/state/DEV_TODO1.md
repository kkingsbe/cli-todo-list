# DEV_TODO1 — Development Agent 1

> Sprint: 5
> Focus Area: Filtering & Tags
> Last Updated: 2026-02-28T22:40:00Z
> Total Points: 3

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `./skills/rust-best-practices/SKILL.md`
- `./skills/test-driven-development/SKILL.md`

## Stories

- [ ] **{story-05.2}**: Filter by Priority and Tags (3 pts)
  - 📄 Story: `.switchboard/state/stories/story-05-2-filter-by-tags.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`, `./skills/test-driven-development/SKILL.md`
  - ⚡ Pre-check: Build + tests pass (`cargo build --release && cargo test`)
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev1): [05.2] implement filter by priority and tags`
  - **Dependency:** 05.1 Filter by Status (complete)

- [ ] **AGENT QA:** Run full build and test suite. If green, create `.switchboard/state/.dev_done_1` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.

## Implementation Notes

- This story adds `--tag` filter to the list command
- Multiple tags should use AND logic
- Repository already has filtering infrastructure - extend it for tags
- Run `cargo clippy -- -D warnings` before marking complete
