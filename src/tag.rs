//! Tag entity module for TaskForge.
//!
//! This module defines the Tag struct and related types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::filter::TagFilter;
#[cfg(test)]
use crate::models::TagWithCount;
#[cfg(test)]
use crate::repository::{Repository, SqliteRepository};
#[cfg(test)]
use crate::task::Task;
#[cfg(test)]
use std::path::Path;

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

#[cfg(test)]
mod tag_with_count_tests {
    use super::*;
    use serde_json;

    #[test]
    fn tag_with_count_serialization() {
        let tag = Tag::new("work".to_string());
        let tag_with_count = TagWithCount {
            tag,
            usage_count: 5,
        };

        let json = serde_json::to_string(&tag_with_count).unwrap();
        assert!(json.contains("work"));
        assert!(json.contains("5"));
    }

    #[test]
    fn tag_with_count_deserialization() {
        let json = r#"{"tag":{"id":"test-id","name":"work","color":null,"created_at":"2024-01-01T00:00:00Z"},"usage_count":3}"#;

        let tag_with_count: TagWithCount = serde_json::from_str(json).unwrap();
        assert_eq!(tag_with_count.tag.name, "work");
        assert_eq!(tag_with_count.usage_count, 3);
    }

    #[test]
    fn tag_with_count_roundtrip() {
        let tag = Tag::new("important".to_string());
        let original = TagWithCount {
            tag: tag.clone(),
            usage_count: 10,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TagWithCount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tag.name, original.tag.name);
        assert_eq!(deserialized.usage_count, original.usage_count);
    }

    #[test]
    fn tag_with_count_zero_usage() {
        let json = r#"{"tag":{"id":"test-id","name":"unused","color":null,"created_at":"2024-01-01T00:00:00Z"},"usage_count":0}"#;

        let tag_with_count: TagWithCount = serde_json::from_str(json).unwrap();
        assert_eq!(tag_with_count.tag.name, "unused");
        assert_eq!(tag_with_count.usage_count, 0);
    }
}

#[cfg(test)]
mod list_tags_integration_tests {
    use super::*;

    fn create_test_repository() -> SqliteRepository {
        let repo = SqliteRepository::new(Path::new(":memory:")).unwrap();
        repo.initialize().unwrap();
        repo
    }

    #[test]
    fn list_tags_empty_returns_empty_vector() {
        let repo = create_test_repository();
        let tags = repo.list_tags(&TagFilter::new()).unwrap();
        assert!(tags.is_empty());
    }

    #[test]
    fn list_tags_returns_all_unique_tags() {
        let repo = create_test_repository();

        // Create multiple tags
        let tag1 = Tag::new("work".to_string());
        let tag2 = Tag::new("personal".to_string());
        let tag3 = Tag::new("urgent".to_string());

        repo.create_tag(&tag1).unwrap();
        repo.create_tag(&tag2).unwrap();
        repo.create_tag(&tag3).unwrap();

        let tags = repo.list_tags(&TagFilter::new()).unwrap();

        assert_eq!(tags.len(), 3);
        let tag_names: Vec<&str> = tags.iter().map(|t| t.tag.name.as_str()).collect();
        assert!(tag_names.contains(&"work"));
        assert!(tag_names.contains(&"personal"));
        assert!(tag_names.contains(&"urgent"));
    }

    #[test]
    fn list_tags_returns_correct_usage_counts() {
        let repo = create_test_repository();

        // Create a tag
        let tag = Tag::new("work".to_string());
        repo.create_tag(&tag).unwrap();

        // Create tasks and associate them with the tag
        let task1 = Task::new("Task 1".to_string());
        let task2 = Task::new("Task 2".to_string());
        let task3 = Task::new("Task 3".to_string());

        repo.create_task(&task1).unwrap();
        repo.create_task(&task2).unwrap();
        repo.create_task(&task3).unwrap();

        // Add tag to two tasks
        repo.add_tag_to_task(&task1.id, &tag.id).unwrap();
        repo.add_tag_to_task(&task2.id, &tag.id).unwrap();

        let tags = repo.list_tags(&TagFilter::new()).unwrap();

        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].tag.name, "work");
        assert_eq!(tags[0].usage_count, 2);
    }

    #[test]
    fn list_tags_tags_with_zero_usage_count() {
        let repo = create_test_repository();

        // Create a tag without any tasks
        let tag = Tag::new("unused".to_string());
        repo.create_tag(&tag).unwrap();

        let tags = repo.list_tags(&TagFilter::new()).unwrap();

        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].tag.name, "unused");
        assert_eq!(tags[0].usage_count, 0);
    }

    #[test]
    fn list_tags_multiple_tags_different_usage_counts() {
        let repo = create_test_repository();

        // Create tags
        let tag1 = Tag::new("work".to_string());
        let tag2 = Tag::new("personal".to_string());
        let tag3 = Tag::new("urgent".to_string());

        repo.create_tag(&tag1).unwrap();
        repo.create_tag(&tag2).unwrap();
        repo.create_tag(&tag3).unwrap();

        // Create tasks and associate tags with different usage counts
        let task1 = Task::new("Task 1".to_string());
        let task2 = Task::new("Task 2".to_string());
        let task3 = Task::new("Task 3".to_string());

        repo.create_task(&task1).unwrap();
        repo.create_task(&task2).unwrap();
        repo.create_task(&task3).unwrap();

        // work: 3 tasks
        repo.add_tag_to_task(&task1.id, &tag1.id).unwrap();
        repo.add_tag_to_task(&task2.id, &tag1.id).unwrap();
        repo.add_tag_to_task(&task3.id, &tag1.id).unwrap();

        // personal: 1 task
        repo.add_tag_to_task(&task1.id, &tag2.id).unwrap();

        // urgent: 0 tasks (no associations)

        let tags = repo.list_tags(&TagFilter::new()).unwrap();

        assert_eq!(tags.len(), 3);

        // Find each tag and verify its count
        let work_tag = tags.iter().find(|t| t.tag.name == "work").unwrap();
        let personal_tag = tags.iter().find(|t| t.tag.name == "personal").unwrap();
        let urgent_tag = tags.iter().find(|t| t.tag.name == "urgent").unwrap();

        assert_eq!(work_tag.usage_count, 3);
        assert_eq!(personal_tag.usage_count, 1);
        assert_eq!(urgent_tag.usage_count, 0);
    }
}
