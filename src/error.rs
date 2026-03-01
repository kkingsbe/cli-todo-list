//! Error handling module for TaskForge.
//!
//! This module defines the application's error types using thiserror.

use thiserror::Error;

/// Main application error type that wraps all possible errors.
#[derive(Debug, Error)]
pub enum AppError {
    /// Error during validation of input data.
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// Error from the system layer (database, file I/O, etc.).
    #[error("System error: {0}")]
    System(#[from] SystemError),

    /// Entity not found error.
    #[error("Entity not found: {0}")]
    NotFound(String),

    /// User-facing error with a message.
    #[error("User error: {0}")]
    UserError(String),
}

/// Validation errors for user input.
#[derive(Debug, Clone, Error)]
pub enum ValidationError {
    /// Title is empty or exceeds max length.
    #[error("Title cannot be empty or exceed 500 characters")]
    InvalidTitle,

    /// Description exceeds max length.
    #[error("Description exceeds maximum length of 10000 characters")]
    InvalidDescription,

    /// Tag name is invalid.
    #[error("Tag name must be 1-50 characters and contain only alphanumeric or underscore")]
    InvalidTagName,

    /// Priority is out of valid range.
    #[error("Priority must be between 1 and 4")]
    InvalidPriority,

    /// Status is invalid.
    #[error("{0}")]
    InvalidStatus(String),

    /// Date format is invalid.
    #[error("{0}")]
    InvalidDate(String),

    /// Empty required field.
    #[error("Required field '{0}' cannot be empty")]
    EmptyField(String),
}

/// System-level errors (database, file I/O, etc.).
#[derive(Debug, Error)]
pub enum SystemError {
    /// Database operation failed.
    #[error("Database error: {0}")]
    Database(String),

    /// File I/O error.
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_error_validation_displays_message() {
        let err = AppError::Validation(ValidationError::InvalidTitle);
        assert_eq!(
            err.to_string(),
            "Validation error: Title cannot be empty or exceed 500 characters"
        );
    }

    #[test]
    fn app_error_not_found_displays_message() {
        let err = AppError::NotFound("task".to_string());
        assert_eq!(err.to_string(), "Entity not found: task");
    }

    #[test]
    fn validation_error_invalid_title_message() {
        let err = ValidationError::InvalidTitle;
        assert_eq!(
            err.to_string(),
            "Title cannot be empty or exceed 500 characters"
        );
    }

    #[test]
    fn validation_error_empty_field_message() {
        let err = ValidationError::EmptyField("title".to_string());
        assert_eq!(err.to_string(), "Required field 'title' cannot be empty");
    }

    #[test]
    fn system_error_database_message() {
        let err = SystemError::Database("connection failed".to_string());
        assert_eq!(err.to_string(), "Database error: connection failed");
    }
}
