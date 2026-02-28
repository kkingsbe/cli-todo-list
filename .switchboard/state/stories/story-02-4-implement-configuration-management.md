# Story 02.4: Implement Configuration Management

> Epic: epic-02 — Core Data Models
> Points: 2
> Sprint: 2
> Type: feature
> Risk: Low
> Created: 2026-02-28T18:35:03Z

## User Story

As a user,  
I want configuration stored in ~/.taskforge/config.toml,  
So that I can customize the application.

## Acceptance Criteria

1. `Config` struct with fields: default_priority, date_format, output_format, editor  
   - **Test:** Config loads from default path

2. Configuration file created with defaults if not exists  
   - **Test:** First run creates config file

3. Config values are used by commands  
   - **Test:** New tasks use default_priority from config

## Technical Context

### Architecture Reference
- Config stored at `~/.taskforge/config.toml`

### Existing Code Context

The file `src/config.rs` already exists. Current content:

```rust
//! Configuration module for TaskForge.
//!
//! This module handles loading and managing application configuration.

use serde::{Deserialize, Serialize};

/// Application configuration settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Path to the SQLite database file.
    pub database_path: String,
    /// Application name for display purposes.
    pub app_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_path: default_database_path(),
            app_name: "TaskForge".to_string(),
        }
    }
}

/// Returns the default database path based on the operating system.
fn default_database_path() -> String {
    if let Some(home) = dirs::home_dir() {
        let taskforge_dir = home.join(".taskforge");
        taskforge_dir.join("tasks.db").to_string_lossy().to_string()
    } else {
        "tasks.db".to_string()
    }
}

/// Loads the application configuration.
///
/// Returns the default configuration if no config file exists.
pub fn load_config() -> Config {
    Config::default()
}

/// Sets the database path in the configuration.
pub fn set_database_path(config: &mut Config, path: String) {
    config.database_path = path;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default_has_default_values() {
        let config = Config::default();
        assert_eq!(config.app_name, "TaskForge");
        assert!(!config.database_path.is_empty());
    }

    #[test]
    fn load_config_returns_default() {
        let config = load_config();
        assert_eq!(config.app_name, "TaskForge");
    }

    #[test]
    fn set_database_path_updates_path() {
        let mut config = Config::default();
        set_database_path(&mut config, "/custom/path.db".to_string());
        assert_eq!(config.database_path, "/custom/path.db");
    }
}
```

## Implementation Plan

1. **Review existing `src/config.rs`**
2. **Verify Config has required fields**
3. **Verify config file creation**
4. **Run `cargo test`**
5. **Run `cargo clippy -- -D warnings`**

### Dependencies
- Story 01.3 (Setup Logging and Error Handling)

## Scope Boundaries

### This Story Includes
- Config struct with all fields
- Config file creation/loading

### This Story Does NOT Include
- Advanced config options

### Files in Scope
- `src/config.rs` — modify
