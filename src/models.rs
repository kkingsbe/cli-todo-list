//! Models module for TaskForge.
//!
//! This module re-exports core types from domain modules and defines shared types.

pub use crate::tag::Tag;
pub use crate::task::{Priority, Status, Task};

/// Represents a tag with its usage count (number of tasks using this tag).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TagWithCount {
    /// The tag data.
    pub tag: Tag,
    /// Number of tasks using this tag.
    pub usage_count: i64,
}

/// Represents a unique identifier for entities.
#[allow(dead_code)]
pub type EntityId = String;
