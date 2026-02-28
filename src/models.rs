//! Models module for TaskForge.
//!
//! This module re-exports core types from domain modules and defines shared types.

pub use crate::task::{Priority, Status, Task};
pub use crate::tag::Tag;

/// Represents a unique identifier for entities.
pub type EntityId = String;
