//! PatternIndex — fast lookup by signature, domain, tags.

use std::collections::{HashMap, HashSet};

use crate::types::pattern::Pattern;

/// In-memory index for fast pattern lookup.
#[derive(Debug, Default)]
pub struct PatternIndex {
    by_name: HashMap<String, HashSet<String>>,
    by_domain: HashMap<String, HashSet<String>>,
    by_language: HashMap<String, HashSet<String>>,
    by_tag: HashMap<String, HashSet<String>>,
    by_return_type: HashMap<String, HashSet<String>>,
}

impl PatternIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, pattern: &Pattern) {
        let id = pattern.id.as_str().to_string();

        self.by_name
            .entry(pattern.name.to_lowercase())
            .or_default()
            .insert(id.clone());

        self.by_domain
            .entry(pattern.domain.to_lowercase())
            .or_default()
            .insert(id.clone());

        self.by_language
            .entry(pattern.language.as_str().to_string())
            .or_default()
            .insert(id.clone());

        for tag in &pattern.tags {
            self.by_tag
                .entry(tag.to_lowercase())
                .or_default()
                .insert(id.clone());
        }

        if let Some(ret) = &pattern.signature.return_type {
            self.by_return_type
                .entry(ret.to_lowercase())
                .or_default()
                .insert(id.clone());
        }
    }

    pub fn remove(&mut self, pattern: &Pattern) {
        let id = pattern.id.as_str();
        self.by_name.values_mut().for_each(|s| {
            s.remove(id);
        });
        self.by_domain.values_mut().for_each(|s| {
            s.remove(id);
        });
        self.by_language.values_mut().for_each(|s| {
            s.remove(id);
        });
        self.by_tag.values_mut().for_each(|s| {
            s.remove(id);
        });
        self.by_return_type.values_mut().for_each(|s| {
            s.remove(id);
        });
    }

    pub fn find_by_name(&self, name: &str) -> Vec<String> {
        self.by_name
            .get(&name.to_lowercase())
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn find_by_domain(&self, domain: &str) -> Vec<String> {
        self.by_domain
            .get(&domain.to_lowercase())
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn find_by_language(&self, language: &str) -> Vec<String> {
        self.by_language
            .get(language)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn find_by_tag(&self, tag: &str) -> Vec<String> {
        self.by_tag
            .get(&tag.to_lowercase())
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn find_by_return_type(&self, return_type: &str) -> Vec<String> {
        self.by_return_type
            .get(&return_type.to_lowercase())
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn search(&self, query: &str) -> HashSet<String> {
        let query_lower = query.to_lowercase();
        let mut results = HashSet::new();

        for (key, ids) in &self.by_name {
            if key.contains(&query_lower) {
                results.extend(ids.iter().cloned());
            }
        }
        for (key, ids) in &self.by_domain {
            if key.contains(&query_lower) {
                results.extend(ids.iter().cloned());
            }
        }
        for (key, ids) in &self.by_tag {
            if key.contains(&query_lower) {
                results.extend(ids.iter().cloned());
            }
        }
        results
    }

    pub fn total_indexed(&self) -> usize {
        let mut all = HashSet::new();
        for ids in self.by_name.values() {
            all.extend(ids.iter().cloned());
        }
        all.len()
    }

    pub fn clear(&mut self) {
        self.by_name.clear();
        self.by_domain.clear();
        self.by_language.clear();
        self.by_tag.clear();
        self.by_return_type.clear();
    }
}
