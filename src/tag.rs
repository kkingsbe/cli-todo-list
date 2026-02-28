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
            name: name.to_lowercase(),
            color: None,
            created_at: Utc::now(),
        }
    }

    /// Creates a tag with all fields specified.
    pub fn with_color(name: String, color: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_lowercase(),
            color: Some(color),
            created_at: Utc::now(),
        }
    }

    /// Updates the tag name.
    pub fn rename(&mut self, name: String) {
        self.name = name.to_lowercase();
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

    #[test]
    fn tag_name_is_normalized_to_lowercase() {
        // Verify that tag names are normalized to lowercase
        let tag_work = Tag::new("Work".to_string());
        let tag_work_upper = Tag::new("WORK".to_string());
        let tag_work_mixed = Tag::new("WoRk".to_string());

        assert_eq!(tag_work.name, "work");
        assert_eq!(tag_work_upper.name, "work");
        assert_eq!(tag_work_mixed.name, "work");

        // Also test with_color method
        let tag_color = Tag::with_color("Important".to_string(), "#FF0000".to_string());
        assert_eq!(tag_color.name, "important");

        // Test rename also normalizes
        let mut tag = Tag::new("test".to_string());
        tag.rename("NEW_NAME".to_string());
        assert_eq!(tag.name, "new_name");
    }
}
