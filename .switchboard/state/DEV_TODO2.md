# DEV_TODO2 — Development Agent 2

> Sprint: 5
> Focus Area: Output Formats
> Last Updated: 2026-02-28T22:40:00Z
> Total Points: 2

## Orientation

Before starting any stories, read these files:

- `.switchboard/planning/project-context.md`
- `.switchboard/planning/architecture.md`
- `./skills/rust-best-practices/SKILL.md`

## Stories

- [ ] **{story-06.2}**: Output Format Support (2 pts)
  - 📄 Story: `.switchboard/state/stories/story-06-2-output-format.md`
  - 📚 Skills: `./skills/rust-best-practices/SKILL.md`
  - ⚡ Pre-check: Build + tests pass (`cargo build --release && cargo test`)
  - ✅ Post-check: Build + tests pass, acceptance criteria met
  - 🔒 Risk: Low
  - 📝 Commit: `feat(dev2): [06.2] implement output format support`
  - **Dependency:** 05.1 Filter by Status (complete)

- [ ] **AGENT QA:** Run full build and test suite. If green, create `.switchboard/state/.dev_done_2` with date. If ALL `.dev_done_*` files exist for all agents with work, also create `.switchboard/state/.sprint_complete`.

## Implementation Notes

- This story adds `--format` option to list command (table, plain, json)
- Table is default (keep existing behavior)
- JSON output should be valid JSON array
- Use serde_json for JSON serialization
- Run `cargo clippy -- -D warnings` before marking complete
