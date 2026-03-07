//! Cursor-based pagination for token-efficient result delivery.

use serde::{Deserialize, Serialize};

/// A page of results using cursor-based pagination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPage<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Cursor for the next page, if any.
    pub next_cursor: Option<String>,
    /// Whether more results are available beyond this page.
    pub has_more: bool,
    /// Total number of items across all pages (if known).
    pub total: Option<usize>,
}

impl<T: Clone> CursorPage<T> {
    /// Create a page from a full slice, given a cursor (starting index) and limit.
    ///
    /// The cursor is a stringified index. If `None`, starts from 0.
    pub fn from_slice(data: &[T], cursor: Option<&str>, limit: usize) -> Self {
        let start = cursor.and_then(|c| c.parse::<usize>().ok()).unwrap_or(0);

        let clamped_start = start.min(data.len());
        let end = (clamped_start + limit).min(data.len());
        let items = data[clamped_start..end].to_vec();
        let has_more = end < data.len();
        let next_cursor = if has_more {
            Some(end.to_string())
        } else {
            None
        };

        Self {
            items,
            next_cursor,
            has_more,
            total: Some(data.len()),
        }
    }

    /// Number of items in this page.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Whether this page is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Map items to another type.
    pub fn map<U: Clone, F: Fn(&T) -> U>(&self, f: F) -> CursorPage<U> {
        CursorPage {
            items: self.items.iter().map(f).collect(),
            next_cursor: self.next_cursor.clone(),
            has_more: self.has_more,
            total: self.total,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_slice_first_page() {
        let data: Vec<i32> = (0..10).collect();
        let page = CursorPage::from_slice(&data, None, 3);
        assert_eq!(page.items, vec![0, 1, 2]);
        assert!(page.has_more);
        assert_eq!(page.next_cursor, Some("3".to_string()));
        assert_eq!(page.total, Some(10));
    }

    #[test]
    fn from_slice_second_page() {
        let data: Vec<i32> = (0..10).collect();
        let page = CursorPage::from_slice(&data, Some("3"), 3);
        assert_eq!(page.items, vec![3, 4, 5]);
        assert!(page.has_more);
        assert_eq!(page.next_cursor, Some("6".to_string()));
    }

    #[test]
    fn from_slice_last_page() {
        let data: Vec<i32> = (0..10).collect();
        let page = CursorPage::from_slice(&data, Some("8"), 5);
        assert_eq!(page.items, vec![8, 9]);
        assert!(!page.has_more);
        assert_eq!(page.next_cursor, None);
    }

    #[test]
    fn from_slice_exact_fit() {
        let data = vec![1, 2, 3];
        let page = CursorPage::from_slice(&data, None, 3);
        assert_eq!(page.items, vec![1, 2, 3]);
        assert!(!page.has_more);
    }

    #[test]
    fn from_slice_empty() {
        let data: Vec<i32> = vec![];
        let page = CursorPage::from_slice(&data, None, 10);
        assert!(page.is_empty());
        assert!(!page.has_more);
    }

    #[test]
    fn from_slice_cursor_beyond_end() {
        let data = vec![1, 2, 3];
        let page = CursorPage::from_slice(&data, Some("100"), 5);
        assert!(page.is_empty());
        assert!(!page.has_more);
    }

    #[test]
    fn map_transforms_items() {
        let data = vec![1, 2, 3];
        let page = CursorPage::from_slice(&data, None, 10);
        let mapped = page.map(|x| x * 10);
        assert_eq!(mapped.items, vec![10, 20, 30]);
    }

    #[test]
    fn count_and_is_empty() {
        let data = vec![1, 2];
        let page = CursorPage::from_slice(&data, None, 10);
        assert_eq!(page.count(), 2);
        assert!(!page.is_empty());
    }
}
