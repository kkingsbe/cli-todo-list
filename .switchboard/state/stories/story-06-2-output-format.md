# Story 06.2: Output Format Support

> Epic: epic-06 — Delete Task and Output Formats
> Points: 2
> Sprint: 5
> Type: feature
> Risk: low
> Created: 2026-02-28T22:40:00Z

## User Story

**As a** user,
**I want** different output formats,
**So that** I can use the data elsewhere.

## Acceptance Criteria

1. `task list --format table` shows table output (default)
   - **Test:** `cargo run -- list --format table` should show table format

2. `task list --format plain` shows plain text
   - **Test:** `cargo run -- list --format plain` should show simple line-by-line output

3. `task list --format json` shows JSON output
   - **Test:** `cargo run -- list --format json` should output valid JSON array

## Technical Context

### Architecture Reference

From architecture.md §5 - cli.rs module:
- **Purpose:** Define CLI commands and arguments using clap

From architecture.md §6 - Data Model:
- Task struct with fields: id, title, description, priority, status, created_at, updated_at, due_date
- Serialization via serde

### Project Conventions

From project-context.md:
- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Never use unwrap()** - Return Result types properly
- **Use tracing for logging** - Never use println!
- **Serialization:** serde 1.x

### Existing Code Context

Current list command output is table format (using tabulate or similar). Need to add format option.

Current src/cli.rs ListArgs structure (similar to):
```rust
#[derive(Parser, Debug)]
pub struct ListArgs {
    /// Filter by status
    #[arg(long)]
    pub status: Option<String>,
    
    /// Max tasks to return
    #[arg(short, long)]
    pub limit: Option<usize>,
    
    // ... other filters
}
```

Current output uses table format by default. Need to add format option and support JSON/plain output.

## Implementation Plan

1. **Add --format argument to ListArgs in src/cli.rs**
   - Add: `#[arg(long, value_enum)] pub format: Option<OutputFormat>`
   - Create OutputFormat enum: Table, Plain, Json

2. **Update commands.rs run_list function**
   - Accept format parameter
   - Match on format to choose output style

3. **Implement JSON output**
   - Use serde_json::to_string for JSON serialization

4. **Implement plain output**
   - Simple println! for each task field

5. **Run build + tests**
   - `cargo build --release`
   - `cargo test`
   - `cargo clippy -- -D warnings`

6. **Manual testing**
   - Test table format
   - Test plain format
   - Test JSON format is valid

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Error handling and serialization
- `./skills/test-driven-development/SKILL.md` — Writing tests first

### Dependencies

- 05.1: Filter by Status (complete)

## Scope Boundaries

### This Story Includes
- Adding --format option to list command
- Table format (default, keep existing)
- Plain format output
- JSON format output

### This Story Does NOT Include
- Format options for other commands (show, tag list, etc.)
- Changes to filtering (stories 05.x)
- Any changes to delete command

### Files in Scope
- `src/cli.rs` — Add --format argument and OutputFormat enum
- `src/commands.rs` — Implement format selection in run_list

### Files NOT in Scope
- `src/main.rs` — No changes needed
- `src/repository.rs` — No changes needed
- Other commands (show, tag, etc.)
