# Story 06.2: Output Format Support

> Epic: epic-06 — Delete Task and Output Formats
> Points: 2
> Sprint: 7
> Type: feature
> Risk: low
> Created: 2026-03-01T02:43:00Z

## User Story

**As a** user,
**I want** different output formats,
**So that** I can use the data elsewhere.

## Acceptance Criteria

1. `task list --format table` shows table output (default)
   - **Test:** Run `cargo build --release`, then `cargo run -- list --format table` - should show table formatting

2. `task list --format plain` shows plain text
   - **Test:** `cargo run -- list --format plain` - should show simple line-by-line output

3. `task list --format json` shows JSON output
   - **Test:** `cargo run -- list --format json` - should output valid JSON array

## Technical Context

### Architecture Reference

From [`architecture.md`](.switchboard/planning/architecture.md):
- **Module:** cli.rs - Define CLI commands and arguments using clap
- **Output Format enum:** OutputFormat (Table, Plain, Json) defined in cli.rs

### Project Conventions

From [`project-context.md`](.switchboard/planning/project-context.md):
- **Build:** `cargo build --release`
- **Test:** `cargo test`
- **Lint:** `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --check`
- **Never use unwrap()** - Return Result types properly

### Existing Code Context

**Current OutputFormat enum in** [`src/cli.rs`](src/cli.rs:9):
```rust
/// Output format for list command.
#[derive(Clone, Debug, Default, PartialEq, clap::ValueEnum)]
pub enum OutputFormat {
    #[default]
    Table,
    Plain,
    Json,
}
```

**Current List command in** [`src/cli.rs`](src/cli.rs:44):
```rust
/// List tasks.
List {
    /// Filter by status (complete/incomplete).
    #[arg(long)]
    status: Option<String>,

    /// Filter by priority.
    #[arg(short, long)]
    priority: Option<u8>,

    /// Search term.
    #[arg(long)]
    search: Option<String>,

    /// Sort field (created, priority, due, title).
    #[arg(short, long, default_value = "created")]
    sort_by: String,

    /// Sort order (asc/desc).
    #[arg(short, long, default_value = "asc")]
    order: String,
},
```

**Notice:** The `--format` argument is NOT currently in the List command - this is what needs to be added.

## Implementation Plan

1. **Modify** `src/cli.rs` — Add `--format` argument to List command:
   ```rust
   /// Output format (table, plain, json).
   #[arg(long, value_enum, default_value = "table")]
   format: OutputFormat,
   ```

2. **Modify** `src/main.rs` — Pass format to list handler and implement output:
   - Table: Use existing table formatting
   - Plain: Simple println for each task field
   - Json: Use serde to serialize to JSON

3. **Write tests** for format handling

4. **Run build + tests** — Verify:
   - `cargo build --release` passes
   - `cargo test` passes
   - `cargo clippy -- -D warnings` passes

### Skills to Read

- `./skills/rust-best-practices/SKILL.md` — Serialization patterns

### Dependencies

Stories that must be complete. "None" if this is independent.
- Depends on: 05.1 (Filter by Status) - not-started BUT this story can be done in parallel

## Scope Boundaries

### This Story Includes
- Adding `--format` argument to list command
- Implementing Table, Plain, JSON output formats

### This Story Does NOT Include
- Format support for other commands (get, show, etc.)
- Modifications to the data model

### Files in Scope
- `src/cli.rs` — add format argument to List command
- `src/main.rs` — implement output formatting logic

### Files NOT in Scope
- `src/commands.rs` — don't modify
- `src/models.rs` — don't modify
