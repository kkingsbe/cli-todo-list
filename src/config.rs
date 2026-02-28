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
