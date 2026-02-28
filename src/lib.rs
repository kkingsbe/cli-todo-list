//! TaskForge - CLI task management application
//!
//! This library provides the core functionality for the TaskForge CLI tool.

mod cli;
mod commands;
mod config;
mod error;
mod filter;
mod models;
mod repository;
mod tag;
mod task;

pub use cli::{Cli, Commands, TagCommands};
pub use commands::*;
pub use config::{load_config, set_database_path, Config};
pub use error::{AppError, SystemError, ValidationError};
pub use filter::{TagFilter, TaskFilter, TaskSort, TaskSortField};
pub use models::{Priority, Status, Tag, Task};
pub use repository::{Repository, RepositoryError};

/// Placeholder for library functionality
/// Module files will be added in subsequent stories
pub fn placeholder() -> &'static str {
    "TaskForge library"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert_eq!(placeholder(), "TaskForge library");
    }
}
