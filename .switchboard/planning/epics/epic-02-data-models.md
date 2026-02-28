# Epic 02: Core Data Models

> Priority: 2
> Depends on: 01
> Estimated stories: 4
> Status: not-started

## Description

Implement the core domain entities (Task, Tag) and data structures (Priority, Status, TaskFilter) that form the foundation of the application.

## Stories

### Story 02.1: Define Task Entity

- **Points:** 3
- **Depends on:** 01.2
- **Risk:** Low
- **Type:** feature

**As a** developer,
**I want** a Task struct with all required fields,
**So that** I can represent tasks in the system.

**Acceptance Criteria:**

1. Task struct in models.rs has all fields: id (UUID), title, description, priority, status, created_at, updated_at, due_date
   - Verification: Code compiles, fields are accessible

2. Priority enum (P1, P2, P3, P4) is defined with FromStr implementation
   - Verification: Priority::from_str("1") returns Ok(Priority::P1)

3. Status enum (Incomplete, Completed) is defined with Serialize/Deserialize
   - Verification: Status serializes to "incomplete"/"completed"

---

### Story 02.2: Define Tag Entity

- **Points:** 2
- **Depends on:** 01.2
- **Risk:** Low
- **Type:** feature

**As a** developer,
**I want** a Tag struct,
**So that** I can represent tags in the system.

**Acceptance Criteria:**

1. Tag struct in models.rs has fields: id (UUID), name, created_at
   - Verification: Code compiles

2. Tag name is case-insensitive (normalized to lowercase for storage)
   - Verification: Creating tags "Work" and "WORK" should conflict

---

### Story 02.3: Define Filter and Sort Structures

- **Points:** 3
- **Depends on:** 02.1
- **Risk:** Low
- **Type:** feature

**As a** developer,
**I want** TaskFilter and TaskSort structs,
**So that** I can query tasks with filters and sorting.

**Acceptance Criteria:**

1. TaskFilter struct in filter.rs has optional fields: status, priority, tags, due_before, due_after, due_on, search
   - Verification: Code compiles

2. TaskSort struct with field and direction
   - Verification: Sort by priority, created_at, due_date, title works

3. Clap derive supports all filter options in CLI
   - Verification: `task list --status incomplete --priority 1` parses correctly

---

### Story 02.4: Implement Configuration Management

- **Points:** 2
- **Depends on:** 01.3
- **Risk:** Low
- **Type:** feature

**As a** user,
**I want** configuration stored in ~/.taskforge/config.toml,
**So that** I can customize the application.

**Acceptance Criteria:**

1. Config struct with fields: default_priority, date_format, output_format, editor
   - Verification: Config loads from default path

2. Configuration file created with defaults if not exists
   - Verification: First run creates config file

3. Config values are used by commands
   - Verification: New tasks use default_priority from config
