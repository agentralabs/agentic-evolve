//! Extraction intent — controls how much data a query returns to conserve tokens.

use serde::{Deserialize, Serialize};

/// The level of detail a query should return.
///
/// Defaults to `IdsOnly` to be token-conservative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExtractionIntent {
    /// Just check existence — returns a boolean.
    Exists,
    /// Return only identifiers.
    IdsOnly,
    /// Return a compact summary (name, id, key metadata).
    Summary,
    /// Return specific fields.
    Fields,
    /// Return the full object.
    Full,
}

impl Default for ExtractionIntent {
    fn default() -> Self {
        Self::IdsOnly
    }
}

impl ExtractionIntent {
    /// Estimated token cost multiplier relative to `Full`.
    ///
    /// These are approximate multipliers:
    /// - `Exists`: ~1 token
    /// - `IdsOnly`: ~5 tokens per item
    /// - `Summary`: ~20 tokens per item
    /// - `Fields`: ~50 tokens per item
    /// - `Full`: ~100 tokens per item
    pub fn estimated_tokens(&self) -> u64 {
        match self {
            Self::Exists => 1,
            Self::IdsOnly => 5,
            Self::Summary => 20,
            Self::Fields => 50,
            Self::Full => 100,
        }
    }

    /// Whether this intent requests the full object.
    pub fn is_full(&self) -> bool {
        matches!(self, Self::Full)
    }

    /// Whether this is the minimal intent.
    pub fn is_minimal(&self) -> bool {
        matches!(self, Self::Exists | Self::IdsOnly)
    }
}

/// Trait for types that can be scoped to different extraction intents.
pub trait Scopeable {
    /// The identifier type for this object.
    type Id: Clone + Serialize + for<'de> Deserialize<'de>;

    /// Return just the identifier.
    fn id(&self) -> Self::Id;

    /// Return a summary representation (as JSON-friendly string).
    fn summary(&self) -> String;

    /// Estimated token count for the full representation.
    fn full_token_estimate(&self) -> u64;
}

/// Apply an extraction intent to a collection of scopeable items.
pub fn apply_intent<T: Scopeable + Clone + Serialize>(
    items: &[T],
    intent: ExtractionIntent,
) -> ScopedResult<T> {
    match intent {
        ExtractionIntent::Exists => ScopedResult::Exists(!items.is_empty()),
        ExtractionIntent::IdsOnly => {
            ScopedResult::Ids(items.iter().map(|i| i.id()).collect())
        }
        ExtractionIntent::Summary => {
            ScopedResult::Summaries(items.iter().map(|i| i.summary()).collect())
        }
        ExtractionIntent::Fields | ExtractionIntent::Full => {
            ScopedResult::Full(items.to_vec())
        }
    }
}

/// The result of a scoped query, carrying only the requested level of detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ScopedResult<T: Scopeable> {
    /// Just an existence check.
    Exists(bool),
    /// Only identifiers.
    Ids(Vec<T::Id>),
    /// Compact summaries.
    Summaries(Vec<String>),
    /// Full objects.
    Full(Vec<T>),
}

impl<T: Scopeable> ScopedResult<T> {
    /// Estimated token cost of this result.
    pub fn estimated_tokens(&self) -> u64 {
        match self {
            Self::Exists(_) => 1,
            Self::Ids(ids) => ids.len() as u64 * 5,
            Self::Summaries(s) => s.len() as u64 * 20,
            Self::Full(items) => items.iter().map(|i| i.full_token_estimate()).sum(),
        }
    }

    /// Number of items in the result.
    pub fn count(&self) -> usize {
        match self {
            Self::Exists(_) => 1,
            Self::Ids(ids) => ids.len(),
            Self::Summaries(s) => s.len(),
            Self::Full(items) => items.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestItem {
        id: String,
        name: String,
        data: String,
    }

    impl Scopeable for TestItem {
        type Id = String;

        fn id(&self) -> String {
            self.id.clone()
        }

        fn summary(&self) -> String {
            format!("{}: {}", self.id, self.name)
        }

        fn full_token_estimate(&self) -> u64 {
            100
        }
    }

    fn sample_items() -> Vec<TestItem> {
        vec![
            TestItem {
                id: "1".into(),
                name: "alpha".into(),
                data: "x".repeat(100),
            },
            TestItem {
                id: "2".into(),
                name: "beta".into(),
                data: "y".repeat(100),
            },
        ]
    }

    #[test]
    fn default_intent_is_ids_only() {
        assert_eq!(ExtractionIntent::default(), ExtractionIntent::IdsOnly);
    }

    #[test]
    fn estimated_tokens_ordering() {
        assert!(ExtractionIntent::Exists.estimated_tokens() < ExtractionIntent::IdsOnly.estimated_tokens());
        assert!(ExtractionIntent::IdsOnly.estimated_tokens() < ExtractionIntent::Summary.estimated_tokens());
        assert!(ExtractionIntent::Summary.estimated_tokens() < ExtractionIntent::Fields.estimated_tokens());
        assert!(ExtractionIntent::Fields.estimated_tokens() < ExtractionIntent::Full.estimated_tokens());
    }

    #[test]
    fn is_full_only_for_full() {
        assert!(!ExtractionIntent::IdsOnly.is_full());
        assert!(ExtractionIntent::Full.is_full());
    }

    #[test]
    fn is_minimal_for_exists_and_ids() {
        assert!(ExtractionIntent::Exists.is_minimal());
        assert!(ExtractionIntent::IdsOnly.is_minimal());
        assert!(!ExtractionIntent::Summary.is_minimal());
        assert!(!ExtractionIntent::Full.is_minimal());
    }

    #[test]
    fn apply_intent_exists() {
        let items = sample_items();
        let result = apply_intent(&items, ExtractionIntent::Exists);
        match result {
            ScopedResult::Exists(b) => assert!(b),
            _ => panic!("Expected Exists variant"),
        }
    }

    #[test]
    fn apply_intent_ids_only() {
        let items = sample_items();
        let result = apply_intent(&items, ExtractionIntent::IdsOnly);
        match result {
            ScopedResult::Ids(ids) => {
                assert_eq!(ids.len(), 2);
                assert_eq!(ids[0], "1");
            }
            _ => panic!("Expected Ids variant"),
        }
    }

    #[test]
    fn apply_intent_full() {
        let items = sample_items();
        let result = apply_intent(&items, ExtractionIntent::Full);
        assert_eq!(result.estimated_tokens(), 200);
    }

    #[test]
    fn scoped_result_count() {
        let items = sample_items();
        let result = apply_intent(&items, ExtractionIntent::IdsOnly);
        assert_eq!(result.count(), 2);
    }

    #[test]
    fn ids_cheaper_than_full() {
        let items = sample_items();
        let ids_result = apply_intent(&items, ExtractionIntent::IdsOnly);
        let full_result = apply_intent(&items, ExtractionIntent::Full);
        assert!(ids_result.estimated_tokens() < full_result.estimated_tokens());
    }
}
