//! PatternStore — in-memory + disk persistence for patterns.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::types::error::{EvolveError, EvolveResult};
use crate::types::pattern::Pattern;

/// Pattern storage engine.
#[derive(Debug)]
pub struct PatternStore {
    patterns: HashMap<String, Pattern>,
    data_dir: Option<PathBuf>,
}

impl PatternStore {
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            data_dir: None,
        }
    }

    pub fn with_data_dir(data_dir: &Path) -> EvolveResult<Self> {
        std::fs::create_dir_all(data_dir)?;
        let mut store = Self {
            patterns: HashMap::new(),
            data_dir: Some(data_dir.to_path_buf()),
        };
        store.load_all()?;
        Ok(store)
    }

    pub fn save(&mut self, pattern: &Pattern) -> EvolveResult<()> {
        self.patterns.insert(pattern.id.as_str().to_string(), pattern.clone());
        if let Some(dir) = &self.data_dir {
            let path = dir.join(format!("{}.json", pattern.id.as_str()));
            let json = serde_json::to_string_pretty(pattern)?;
            std::fs::write(path, json)?;
        }
        Ok(())
    }

    pub fn get(&self, id: &str) -> EvolveResult<&Pattern> {
        self.patterns
            .get(id)
            .ok_or_else(|| EvolveError::PatternNotFound(id.to_string()))
    }

    pub fn get_mut(&mut self, id: &str) -> EvolveResult<&mut Pattern> {
        self.patterns
            .get_mut(id)
            .ok_or_else(|| EvolveError::PatternNotFound(id.to_string()))
    }

    pub fn delete(&mut self, id: &str) -> EvolveResult<Pattern> {
        let pattern = self.patterns.remove(id)
            .ok_or_else(|| EvolveError::PatternNotFound(id.to_string()))?;
        if let Some(dir) = &self.data_dir {
            let path = dir.join(format!("{id}.json"));
            if path.exists() {
                std::fs::remove_file(path)?;
            }
        }
        Ok(pattern)
    }

    pub fn list(&self) -> Vec<&Pattern> {
        self.patterns.values().collect()
    }

    pub fn list_by_domain(&self, domain: &str) -> Vec<&Pattern> {
        self.patterns
            .values()
            .filter(|p| p.domain == domain)
            .collect()
    }

    pub fn list_by_language(&self, language: &str) -> Vec<&Pattern> {
        self.patterns
            .values()
            .filter(|p| p.language.as_str() == language)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Pattern> {
        let query_lower = query.to_lowercase();
        self.patterns
            .values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower)
                    || p.domain.to_lowercase().contains(&query_lower)
                    || p.template.to_lowercase().contains(&query_lower)
                    || p.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn count(&self) -> usize {
        self.patterns.len()
    }

    pub fn contains(&self, id: &str) -> bool {
        self.patterns.contains_key(id)
    }

    pub fn clear(&mut self) -> EvolveResult<()> {
        self.patterns.clear();
        if let Some(dir) = &self.data_dir {
            if dir.exists() {
                for entry in std::fs::read_dir(dir)? {
                    let entry = entry?;
                    if entry.path().extension().is_some_and(|ext| ext == "json") {
                        std::fs::remove_file(entry.path())?;
                    }
                }
            }
        }
        Ok(())
    }

    fn load_all(&mut self) -> EvolveResult<()> {
        let dir = match &self.data_dir {
            Some(d) => d.clone(),
            None => return Ok(()),
        };
        if !dir.exists() {
            return Ok(());
        }
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json") {
                let content = std::fs::read_to_string(&path)?;
                match serde_json::from_str::<Pattern>(&content) {
                    Ok(pattern) => {
                        self.patterns.insert(pattern.id.as_str().to_string(), pattern);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load pattern from {:?}: {}", path, e);
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for PatternStore {
    fn default() -> Self {
        Self::new()
    }
}
