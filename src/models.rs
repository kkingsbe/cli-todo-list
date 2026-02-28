//! Models module for TaskForge.
//!
//! This module re-exports core types from domain modules and defines shared types.

pub use crate::tag::Tag;
pub use crate::task::{Priority, Status, Task};

/// Represents a unique identifier for entities.
#[allow(dead_code)]
pub type EntityId = String;
