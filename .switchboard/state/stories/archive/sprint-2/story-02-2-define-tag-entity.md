# Story 02.2: Define Tag Entity

> Epic: epic-02 — Core Data Models
> Points: 2
> Sprint: 2
> Type: feature
> Risk: Low
> Created: 2026-02-28T18:35:03Z

## User Story

As a developer,  
I want a Tag struct,  
So that I can represent tags in the system.

## Acceptance Criteria

1. `Tag` struct in `src/models.rs` (or `src/tag.rs`) has fields: id (UUID), name, created_at  
   - **Test:** Code compiles

2. Tag name is case-insensitive (normalized to lowercase for storage)  
   - **Test:** Creating tags "Work" and "WORK" should conflict

## Technical Context

### Architecture Reference
- ADR-004: UUID v4 for IDs
- Data Model: name, created_at Tag with id,

### Existing Code Context

The file `src/models.rs` already exists. Current content:

```rust
//! Models module for TaskForge.
//!
//! This module re-exports core types from domain modules and defines shared types.

pub use crate::tag::Tag;
pub use crate::task::{Priority, Status, Task};

/// Represents a unique identifier for entities.
#[allow(dead_code)]
pub type EntityId = String;
```

The file `src/tag.rs` already exists. Current content:

```rust
//! Tag entity module for TaskForge.
//!
//! This module defines the Tag struct and related types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A tag for categorizing tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// Unique identifier (UUID v4).
    pub id: String,
    /// Tag name (unique, case-insensitive, 1-50 chars).
    pub name: String,
    /// Optional color for display (hex code).
    pub color: Option<String>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

impl Tag {
    /// Creates a new tag with the given name.
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            color: None,
            created_at: Utc::now(),
        }
    }

    /// Creates a tag with all fields specified.
    pub fn with_color(name: String, color: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            color: Some(color),
            created_at: Utc::now(),
        }
    }

    /// Updates the tag name.
    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    /// Updates the tag color.
    pub fn set_color(&mut self, color: Option<String>) {
        self.color = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_new_creates_tag_with_defaults() {
        let tag = Tag::new("work".to_string());
        assert!(!tag.id.is_empty());
        assert_eq!(tag.name, "work");
        assert!(tag.color.is_none());
    }

    #[test]
    fn tag_with_color_creates_tag() {
        let tag = Tag::with_color("work".to_string(), "#FF0000".to_string());
        assert!(!tag.id.is_empty());
        assert_eq!(tag.name, "work");
        assert_eq!(tag.color, Some("#FF0000".to_string()));
    }

    #[test]
    fn tag_rename_changes_name() {
        let mut tag = Tag::new("old".to_string());
        tag.rename("new".to_string());
        assert_eq!(tag.name, "new");
    }

    #[test]
    fn tag_set_color_changes_color() {
        let mut tag = Tag::new("test".to_string());
        tag.set_color(Some("#00FF00".to_string()));
        assert_eq!(tag.color, Some("#00FF00".to_string()));
    }

    #[test]
    fn tag_set_color_to_none() {
        let mut tag = Tag::with_color("test".to_string(), "#FF0000".to_string());
        tag.set_color(None);
        assert!(tag.color.is_none());
    }
}
```

## Implementation Plan

1. **Review existing `src/models.rs` and `src/tag.rs`**
2. **Ensure Tag struct has required fields**
3. **Ensure tag name normalization (lowercase)**
4. **Run `cargo test`**
5. **Run `cargo clippy -- -D warnings`**

### Dependencies
- Story 01.2 (Create Module Structure)

## Scope Boundaries

### This Story Includes
- Tag struct with id, name, created_at

### This Story Does NOT Include
- Database persistence
- Tag CRUD operations

### Files in Scope
- `src/models.rs` — modify
- `src/tag.rs` — modify
