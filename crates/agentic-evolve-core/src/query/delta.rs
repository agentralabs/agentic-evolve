//! Delta queries — track state versions and return only changes since a given version.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Tracks versioned state for delta queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedState<T> {
    /// The current data.
    data: Vec<T>,
    /// Monotonically increasing version counter.
    version: u64,
    /// Timestamp of the last modification.
    last_modified: DateTime<Utc>,
    /// History of changes: (version, change_type, index_or_id).
    #[serde(skip)]
    change_log: Vec<ChangeEntry>,
}

/// A single change entry in the version history.
#[derive(Debug, Clone)]
struct ChangeEntry {
    version: u64,
    change_type: ChangeType,
    index: usize,
}

/// The type of change recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeType {
    /// A new item was added.
    Created,
    /// An existing item was modified.
    Updated,
    /// An item was removed.
    Deleted,
}

/// The result of a delta query: either unchanged or a set of changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeltaResult<T> {
    /// No changes since the requested version.
    Unchanged {
        /// The current version.
        version: u64,
    },
    /// Changes that occurred since the requested version.
    Changed {
        /// Items that were created or updated.
        items: Vec<T>,
        /// Number of deletions.
        deletions: usize,
        /// The old version the client had.
        from_version: u64,
        /// The current version after changes.
        to_version: u64,
    },
}

impl<T: Clone> VersionedState<T> {
    /// Create a new versioned state with no data.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            version: 0,
            last_modified: Utc::now(),
            change_log: Vec::new(),
        }
    }

    /// Create from initial data at version 1.
    pub fn from_data(data: Vec<T>) -> Self {
        let len = data.len();
        let mut state = Self {
            data,
            version: 1,
            last_modified: Utc::now(),
            change_log: Vec::new(),
        };
        for i in 0..len {
            state.change_log.push(ChangeEntry {
                version: 1,
                change_type: ChangeType::Created,
                index: i,
            });
        }
        state
    }

    /// Current version number.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Timestamp of last modification.
    pub fn last_modified(&self) -> DateTime<Utc> {
        self.last_modified
    }

    /// Read the full data slice.
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Add a new item, incrementing the version.
    pub fn add(&mut self, item: T) {
        self.version += 1;
        self.last_modified = Utc::now();
        self.data.push(item);
        self.change_log.push(ChangeEntry {
            version: self.version,
            change_type: ChangeType::Created,
            index: self.data.len() - 1,
        });
    }

    /// Update an item at the given index.
    pub fn update(&mut self, index: usize, item: T) {
        if index < self.data.len() {
            self.version += 1;
            self.last_modified = Utc::now();
            self.data[index] = item;
            self.change_log.push(ChangeEntry {
                version: self.version,
                change_type: ChangeType::Updated,
                index,
            });
        }
    }

    /// Mark an item as deleted (removes from data vec).
    pub fn delete(&mut self, index: usize) {
        if index < self.data.len() {
            self.version += 1;
            self.last_modified = Utc::now();
            self.data.remove(index);
            self.change_log.push(ChangeEntry {
                version: self.version,
                change_type: ChangeType::Deleted,
                index,
            });
        }
    }

    /// Query changes since a given version.
    ///
    /// Returns `DeltaResult::Unchanged` if there have been no changes, or
    /// `DeltaResult::Changed` with the affected items.
    pub fn changes_since_version(&self, since_version: u64) -> DeltaResult<T> {
        if since_version >= self.version {
            return DeltaResult::Unchanged {
                version: self.version,
            };
        }

        let relevant: Vec<&ChangeEntry> = self
            .change_log
            .iter()
            .filter(|e| e.version > since_version)
            .collect();

        if relevant.is_empty() {
            return DeltaResult::Unchanged {
                version: self.version,
            };
        }

        let mut items = Vec::new();
        let mut deletions = 0usize;

        for entry in &relevant {
            match entry.change_type {
                ChangeType::Created | ChangeType::Updated => {
                    if entry.index < self.data.len() {
                        items.push(self.data[entry.index].clone());
                    }
                }
                ChangeType::Deleted => {
                    deletions += 1;
                }
            }
        }

        DeltaResult::Changed {
            items,
            deletions,
            from_version: since_version,
            to_version: self.version,
        }
    }

    /// Check whether the state is unchanged since a given version.
    pub fn is_unchanged_since(&self, since_version: u64) -> bool {
        since_version >= self.version
    }
}

impl<T: Clone> Default for VersionedState<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state_version_zero() {
        let state = VersionedState::<String>::new();
        assert_eq!(state.version(), 0);
        assert!(state.data().is_empty());
    }

    #[test]
    fn from_data_version_one() {
        let state = VersionedState::from_data(vec![1, 2, 3]);
        assert_eq!(state.version(), 1);
        assert_eq!(state.data().len(), 3);
    }

    #[test]
    fn add_increments_version() {
        let mut state = VersionedState::new();
        state.add("hello".to_string());
        assert_eq!(state.version(), 1);
        state.add("world".to_string());
        assert_eq!(state.version(), 2);
    }

    #[test]
    fn changes_since_returns_unchanged_when_current() {
        let state = VersionedState::from_data(vec![1, 2, 3]);
        let result = state.changes_since_version(1);
        assert!(matches!(result, DeltaResult::Unchanged { .. }));
    }

    #[test]
    fn changes_since_returns_changed_items() {
        let mut state = VersionedState::from_data(vec![1, 2, 3]);
        state.add(4);
        let result = state.changes_since_version(1);
        match result {
            DeltaResult::Changed { items, to_version, .. } => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0], 4);
                assert_eq!(to_version, 2);
            }
            _ => panic!("Expected Changed"),
        }
    }

    #[test]
    fn is_unchanged_since_current_version() {
        let state = VersionedState::from_data(vec![1]);
        assert!(state.is_unchanged_since(1));
        assert!(state.is_unchanged_since(99));
        assert!(!state.is_unchanged_since(0));
    }

    #[test]
    fn delete_increments_version() {
        let mut state = VersionedState::from_data(vec![1, 2, 3]);
        state.delete(0);
        assert_eq!(state.version(), 2);
        assert_eq!(state.data().len(), 2);
    }

    #[test]
    fn update_tracks_change() {
        let mut state = VersionedState::from_data(vec!["a".to_string()]);
        state.update(0, "b".to_string());
        assert_eq!(state.version(), 2);
        assert_eq!(state.data()[0], "b");
    }

    #[test]
    fn delta_proportional_to_changes() {
        let mut state = VersionedState::from_data(vec![1, 2, 3, 4, 5]);
        let baseline = state.version();
        state.add(6);
        state.add(7);
        let result = state.changes_since_version(baseline);
        match result {
            DeltaResult::Changed { items, .. } => {
                // Only the 2 new items, not the full 7.
                assert_eq!(items.len(), 2);
            }
            _ => panic!("Expected Changed"),
        }
    }
}
