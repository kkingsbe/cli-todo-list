//! Filter module for TaskForge.
//!
//! This module defines filter structs for querying tasks and tags.

use crate::models::{Priority, Status};

/// Sort order for queries.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SortOrder {
    Ascending,
    #[default]
    Descending,
}

/// Field to sort tasks by.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum TaskSortField {
    #[default]
    CreatedAt,
    UpdatedAt,
    Priority,
    DueDate,
    Title,
}

/// Filter criteria for querying tasks.
#[derive(Debug, Clone, Default)]
pub struct TaskFilter {
    /// Filter by task status.
    pub status: Option<Status>,
    /// Filter by priority.
    pub priority: Option<Priority>,
    /// Filter by tag IDs.
    pub tags: Option<Vec<String>>,
    /// Filter by due date (tasks due before this date).
    pub due_before: Option<chrono::DateTime<chrono::Utc>>,
    /// Filter by due date (tasks due after this date).
    pub due_after: Option<chrono::DateTime<chrono::Utc>>,
    /// Search term in title or description.
    pub search: Option<String>,
}

impl TaskFilter {
    /// Creates a new empty filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the status filter.
    pub fn with_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the priority filter.
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Sets the search filter.
    pub fn with_search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }

    /// Sets the tags filter.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}

/// Sort options for tasks.
#[derive(Debug, Clone, Default)]
pub struct TaskSort {
    /// Field to sort by.
    pub field: TaskSortField,
    /// Sort order.
    pub order: SortOrder,
}

impl TaskSort {
    /// Creates a new sort with the given field and order.
    pub fn new(field: TaskSortField, order: SortOrder) -> Self {
        Self { field, order }
    }

    /// Creates a default sort by created_at ascending.
    pub fn default_sort() -> Self {
        Self::default()
    }
}

/// Filter criteria for querying tags.
#[derive(Debug, Clone, Default)]
pub struct TagFilter {
    /// Search term in tag name.
    pub search: Option<String>,
}

impl TagFilter {
    /// Creates a new empty filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the search filter.
    pub fn with_search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_filter_new_is_empty() {
        let filter = TaskFilter::new();
        assert!(filter.status.is_none());
        assert!(filter.priority.is_none());
        assert!(filter.tags.is_none());
        assert!(filter.search.is_none());
    }

    #[test]
    fn task_filter_with_status() {
        let filter = TaskFilter::new().with_status(Status::Completed);
        assert_eq!(filter.status, Some(Status::Completed));
    }

    #[test]
    fn task_filter_with_priority() {
        let filter = TaskFilter::new().with_priority(Priority::P1);
        assert_eq!(filter.priority, Some(Priority::P1));
    }

    #[test]
    fn task_filter_with_search() {
        let filter = TaskFilter::new().with_search("test".to_string());
        assert_eq!(filter.search, Some("test".to_string()));
    }

    #[test]
    fn task_sort_new() {
        let sort = TaskSort::new(TaskSortField::Priority, SortOrder::Descending);
        assert_eq!(sort.field, TaskSortField::Priority);
        assert_eq!(sort.order, SortOrder::Descending);
    }

    #[test]
    fn task_sort_default() {
        let sort = TaskSort::default_sort();
        assert_eq!(sort.field, TaskSortField::CreatedAt);
        assert_eq!(sort.order, SortOrder::Descending);
    }

    #[test]
    fn tag_filter_new_is_empty() {
        let filter = TagFilter::new();
        assert!(filter.search.is_none());
    }

    #[test]
    fn tag_filter_with_search() {
        let filter = TagFilter::new().with_search("work".to_string());
        assert_eq!(filter.search, Some("work".to_string()));
    }
}
