//! Cache invalidation with dependency tracking and cascade support.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

/// Tracks dependencies between cache keys and supports cascade invalidation.
///
/// When key A depends on key B, invalidating B also invalidates A.
pub struct CacheInvalidator<K> {
    /// Map from a key to the set of keys that depend on it.
    dependents: RwLock<HashMap<K, HashSet<K>>>,
}

/// A record of which keys were invalidated in a cascade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidationResult<K> {
    /// The key that was directly invalidated.
    pub root: K,
    /// All keys that were cascade-invalidated (including root).
    pub invalidated: Vec<K>,
}

impl<K> CacheInvalidator<K>
where
    K: Eq + Hash + Clone,
{
    /// Create a new invalidator with no dependencies.
    pub fn new() -> Self {
        Self {
            dependents: RwLock::new(HashMap::new()),
        }
    }

    /// Register that `dependent` depends on `dependency`.
    ///
    /// When `dependency` is invalidated, `dependent` will also be invalidated.
    pub fn add_dependency(&self, dependency: K, dependent: K) {
        let mut map = self.dependents.write().unwrap();
        map.entry(dependency).or_default().insert(dependent);
    }

    /// Remove a specific dependency relationship.
    pub fn remove_dependency(&self, dependency: &K, dependent: &K) {
        let mut map = self.dependents.write().unwrap();
        if let Some(deps) = map.get_mut(dependency) {
            deps.remove(dependent);
            if deps.is_empty() {
                map.remove(dependency);
            }
        }
    }

    /// Compute the full set of keys that should be invalidated when `root` is
    /// invalidated, following the dependency graph transitively.
    pub fn cascade(&self, root: &K) -> InvalidationResult<K> {
        let map = self.dependents.read().unwrap();
        let mut visited = HashSet::new();
        let mut stack = vec![root.clone()];

        while let Some(key) = stack.pop() {
            if visited.contains(&key) {
                continue;
            }
            visited.insert(key.clone());
            if let Some(deps) = map.get(&key) {
                for dep in deps {
                    if !visited.contains(dep) {
                        stack.push(dep.clone());
                    }
                }
            }
        }

        InvalidationResult {
            root: root.clone(),
            invalidated: visited.into_iter().collect(),
        }
    }

    /// Clear all dependency tracking.
    pub fn clear(&self) {
        self.dependents.write().unwrap().clear();
    }

    /// Number of keys that have dependents registered.
    pub fn dependency_count(&self) -> usize {
        self.dependents.read().unwrap().len()
    }

    /// Check if a key has any dependents.
    pub fn has_dependents(&self, key: &K) -> bool {
        self.dependents
            .read()
            .unwrap()
            .get(key)
            .is_some_and(|deps| !deps.is_empty())
    }
}

impl<K: Eq + Hash + Clone> Default for CacheInvalidator<K> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cascade_single_key() {
        let inv = CacheInvalidator::<String>::new();
        let result = inv.cascade(&"root".to_string());
        assert_eq!(result.invalidated.len(), 1);
        assert!(result.invalidated.contains(&"root".to_string()));
    }

    #[test]
    fn cascade_follows_dependencies() {
        let inv = CacheInvalidator::new();
        inv.add_dependency("a".to_string(), "b".to_string());
        inv.add_dependency("b".to_string(), "c".to_string());
        let result = inv.cascade(&"a".to_string());
        assert_eq!(result.invalidated.len(), 3);
        assert!(result.invalidated.contains(&"a".to_string()));
        assert!(result.invalidated.contains(&"b".to_string()));
        assert!(result.invalidated.contains(&"c".to_string()));
    }

    #[test]
    fn cascade_handles_cycles() {
        let inv = CacheInvalidator::new();
        inv.add_dependency("x".to_string(), "y".to_string());
        inv.add_dependency("y".to_string(), "x".to_string());
        let result = inv.cascade(&"x".to_string());
        assert_eq!(result.invalidated.len(), 2);
    }

    #[test]
    fn remove_dependency_works() {
        let inv = CacheInvalidator::new();
        inv.add_dependency("a".to_string(), "b".to_string());
        assert!(inv.has_dependents(&"a".to_string()));
        inv.remove_dependency(&"a".to_string(), &"b".to_string());
        assert!(!inv.has_dependents(&"a".to_string()));
    }

    #[test]
    fn clear_removes_all() {
        let inv = CacheInvalidator::new();
        inv.add_dependency("a".to_string(), "b".to_string());
        inv.add_dependency("c".to_string(), "d".to_string());
        inv.clear();
        assert_eq!(inv.dependency_count(), 0);
    }

    #[test]
    fn default_is_empty() {
        let inv = CacheInvalidator::<String>::default();
        assert_eq!(inv.dependency_count(), 0);
    }
}
