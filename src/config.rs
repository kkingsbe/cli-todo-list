//! Configuration module for TaskForge.
//!
//! This module handles loading and managing application configuration.

use std::fs;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::task::Priority;

/// Application configuration settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Path to the SQLite database file.
    pub database_path: String,
    /// Application name for display purposes.
    pub app_name: String,
    /// Default priority for new tasks.
    pub default_priority: Priority,
    /// Date format string for displaying dates.
    pub date_format: String,
    /// Output format (plain, json, etc.).
    pub output_format: String,
    /// Editor to use for editing tasks.
    pub editor: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_path: default_database_path(),
            app_name: "TaskForge".to_string(),
            default_priority: Priority::P3,
            date_format: "%Y-%m-%d".to_string(),
            output_format: "plain".to_string(),
            editor: "vim".to_string(),
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

/// Returns the default config directory path.
fn config_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".taskforge"))
}

/// Returns the default config file path.
fn config_file_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("config.toml"))
}

/// Loads the application configuration.
///
/// If the config file exists at ~/.taskforge/config.toml, it will be loaded.
/// If it doesn't exist, a default configuration will be created and saved.
pub fn load_config() -> Config {
    if let Some(config_path) = config_file_path() {
        if config_path.exists() {
            // Try to load the config file
            match fs::read_to_string(&config_path) {
                Ok(contents) => {
                    match toml::from_str(&contents) {
                        Ok(config) => return config,
                        Err(e) => {
                            eprintln!("Warning: Failed to parse config file: {}. Using defaults.", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to read config file: {}. Using defaults.", e);
                }
            }
        }
    
        // Config file doesn't exist or failed to load - create it with defaults
        let config = Config::default();
        if let Err(e) = save_config(&config) {
            eprintln!("Warning: Failed to create config file: {}. Using defaults.", e);
        }
        return config;
    }
    
    // No home directory - return defaults
    Config::default()
}

/// Saves the configuration to the config file.
fn save_config(config: &Config) -> io::Result<()> {
    if let Some(config_dir) = config_dir() {
        fs::create_dir_all(&config_dir)?;
    }
    
    if let Some(config_path) = config_file_path() {
        let toml_string = toml::to_string_pretty(config)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&config_path, toml_string)?;
    }
    
    Ok(())
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
        assert_eq!(config.default_priority, Priority::P3);
        assert_eq!(config.date_format, "%Y-%m-%d");
        assert_eq!(config.output_format, "plain");
        assert_eq!(config.editor, "vim");
    }

    #[test]
    fn load_config_returns_default() {
        let config = load_config();
        assert_eq!(config.app_name, "TaskForge");
        assert_eq!(config.default_priority, Priority::P3);
        assert_eq!(config.date_format, "%Y-%m-%d");
        assert_eq!(config.output_format, "plain");
        assert_eq!(config.editor, "vim");
    }

    #[test]
    fn set_database_path_updates_path() {
        let mut config = Config::default();
        set_database_path(&mut config, "/custom/path.db".to_string());
        assert_eq!(config.database_path, "/custom/path.db");
    }

    #[test]
    fn config_serialization_deserialization() {
        let config = Config::default();
        let serialized = toml::to_string_pretty(&config).expect("Failed to serialize config");
        let deserialized: Config = toml::from_str(&serialized).expect("Failed to deserialize config");
        
        assert_eq!(config.app_name, deserialized.app_name);
        assert_eq!(config.database_path, deserialized.database_path);
        assert_eq!(config.default_priority, deserialized.default_priority);
        assert_eq!(config.date_format, deserialized.date_format);
        assert_eq!(config.output_format, deserialized.output_format);
        assert_eq!(config.editor, deserialized.editor);
    }
}
