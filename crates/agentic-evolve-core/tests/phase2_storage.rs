//! Phase 2: Storage tests — PatternStore, PatternIndex, PatternVersioner, binary format.

use agentic_evolve_core::storage::format::{read_patterns, write_patterns, MAGIC};
use agentic_evolve_core::storage::index::PatternIndex;
use agentic_evolve_core::storage::store::PatternStore;
use agentic_evolve_core::storage::versioner::PatternVersioner;
use agentic_evolve_core::types::error::EvolveError;
use agentic_evolve_core::types::pattern::{FunctionSignature, Language, Pattern};

fn make_pattern(name: &str, domain: &str) -> Pattern {
    let sig = FunctionSignature::new(name, Language::Rust);
    Pattern::new(
        name,
        domain,
        Language::Rust,
        sig,
        "fn test() {}",
        vec![],
        0.8,
    )
}

fn make_pattern_with_tag(name: &str, domain: &str, tag: &str) -> Pattern {
    let mut p = make_pattern(name, domain);
    p.tags.push(tag.to_string());
    p
}

fn make_pattern_with_return(name: &str, domain: &str, ret: &str) -> Pattern {
    let mut sig = FunctionSignature::new(name, Language::Rust);
    sig.return_type = Some(ret.to_string());
    Pattern::new(
        name,
        domain,
        Language::Rust,
        sig,
        "fn test() -> T {}",
        vec![],
        0.8,
    )
}

// ===========================================================================
// PatternStore (in-memory)
// ===========================================================================

#[test]
fn store_new_is_empty() {
    let store = PatternStore::new();
    assert_eq!(store.count(), 0);
}

#[test]
fn store_save_and_get() {
    let mut store = PatternStore::new();
    let p = make_pattern("save_test", "web");
    let id = p.id.as_str().to_string();
    store.save(&p).unwrap();
    let got = store.get(&id).unwrap();
    assert_eq!(got.name, "save_test");
}

#[test]
fn store_get_not_found() {
    let store = PatternStore::new();
    let result = store.get("nonexistent");
    assert!(matches!(
        result.unwrap_err(),
        EvolveError::PatternNotFound(_)
    ));
}

#[test]
fn store_delete() {
    let mut store = PatternStore::new();
    let p = make_pattern("del_test", "web");
    let id = p.id.as_str().to_string();
    store.save(&p).unwrap();
    let removed = store.delete(&id).unwrap();
    assert_eq!(removed.name, "del_test");
    assert_eq!(store.count(), 0);
}

#[test]
fn store_delete_not_found() {
    let mut store = PatternStore::new();
    let result = store.delete("ghost");
    assert!(result.is_err());
}

#[test]
fn store_list() {
    let mut store = PatternStore::new();
    store.save(&make_pattern("a", "web")).unwrap();
    store.save(&make_pattern("b", "cli")).unwrap();
    assert_eq!(store.list().len(), 2);
}

#[test]
fn store_list_by_domain() {
    let mut store = PatternStore::new();
    store.save(&make_pattern("a", "web")).unwrap();
    store.save(&make_pattern("b", "cli")).unwrap();
    store.save(&make_pattern("c", "web")).unwrap();
    assert_eq!(store.list_by_domain("web").len(), 2);
    assert_eq!(store.list_by_domain("cli").len(), 1);
}

#[test]
fn store_list_by_language() {
    let mut store = PatternStore::new();
    store.save(&make_pattern("a", "web")).unwrap();
    assert_eq!(store.list_by_language("rust").len(), 1);
    assert_eq!(store.list_by_language("python").len(), 0);
}

#[test]
fn store_search_by_name() {
    let mut store = PatternStore::new();
    store.save(&make_pattern("parse_json", "data")).unwrap();
    store.save(&make_pattern("build_html", "web")).unwrap();
    let results = store.search("parse");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "parse_json");
}

#[test]
fn store_search_by_tag() {
    let mut store = PatternStore::new();
    store
        .save(&make_pattern_with_tag("t1", "web", "serde"))
        .unwrap();
    store.save(&make_pattern("t2", "cli")).unwrap();
    let results = store.search("serde");
    assert_eq!(results.len(), 1);
}

#[test]
fn store_count() {
    let mut store = PatternStore::new();
    assert_eq!(store.count(), 0);
    store.save(&make_pattern("x", "y")).unwrap();
    assert_eq!(store.count(), 1);
}

#[test]
fn store_contains() {
    let mut store = PatternStore::new();
    let p = make_pattern("exist", "web");
    let id = p.id.as_str().to_string();
    store.save(&p).unwrap();
    assert!(store.contains(&id));
    assert!(!store.contains("nope"));
}

#[test]
fn store_clear() {
    let mut store = PatternStore::new();
    store.save(&make_pattern("a", "b")).unwrap();
    store.save(&make_pattern("c", "d")).unwrap();
    store.clear().unwrap();
    assert_eq!(store.count(), 0);
}

// ===========================================================================
// PatternStore with_data_dir (disk persistence)
// ===========================================================================

#[test]
fn store_disk_save_persists() {
    let dir = tempfile::tempdir().unwrap();
    let p = make_pattern("disk_test", "io");
    let id = p.id.as_str().to_string();

    {
        let mut store = PatternStore::with_data_dir(dir.path()).unwrap();
        store.save(&p).unwrap();
        assert_eq!(store.count(), 1);
    }

    // Re-open: data should still be there.
    let store2 = PatternStore::with_data_dir(dir.path()).unwrap();
    assert_eq!(store2.count(), 1);
    let got = store2.get(&id).unwrap();
    assert_eq!(got.name, "disk_test");
}

#[test]
fn store_disk_load_all() {
    let dir = tempfile::tempdir().unwrap();
    {
        let mut store = PatternStore::with_data_dir(dir.path()).unwrap();
        store.save(&make_pattern("one", "a")).unwrap();
        store.save(&make_pattern("two", "b")).unwrap();
    }
    let store2 = PatternStore::with_data_dir(dir.path()).unwrap();
    assert_eq!(store2.count(), 2);
}

#[test]
fn store_disk_delete_removes_file() {
    let dir = tempfile::tempdir().unwrap();
    let p = make_pattern("del_disk", "x");
    let id = p.id.as_str().to_string();
    {
        let mut store = PatternStore::with_data_dir(dir.path()).unwrap();
        store.save(&p).unwrap();
        store.delete(&id).unwrap();
    }
    let store2 = PatternStore::with_data_dir(dir.path()).unwrap();
    assert_eq!(store2.count(), 0);
}

#[test]
fn store_disk_clear() {
    let dir = tempfile::tempdir().unwrap();
    {
        let mut store = PatternStore::with_data_dir(dir.path()).unwrap();
        store.save(&make_pattern("a", "b")).unwrap();
        store.save(&make_pattern("c", "d")).unwrap();
        store.clear().unwrap();
    }
    let store2 = PatternStore::with_data_dir(dir.path()).unwrap();
    assert_eq!(store2.count(), 0);
}

// ===========================================================================
// PatternIndex
// ===========================================================================

#[test]
fn index_add_and_find_by_name() {
    let mut idx = PatternIndex::new();
    let p = make_pattern("hello", "greet");
    idx.add(&p);
    let ids = idx.find_by_name("hello");
    assert_eq!(ids.len(), 1);
    assert_eq!(ids[0], p.id.as_str());
}

#[test]
fn index_find_by_domain() {
    let mut idx = PatternIndex::new();
    let p = make_pattern("f1", "network");
    idx.add(&p);
    assert_eq!(idx.find_by_domain("network").len(), 1);
    assert_eq!(idx.find_by_domain("other").len(), 0);
}

#[test]
fn index_find_by_language() {
    let mut idx = PatternIndex::new();
    let p = make_pattern("f1", "web");
    idx.add(&p);
    assert_eq!(idx.find_by_language("rust").len(), 1);
    assert_eq!(idx.find_by_language("python").len(), 0);
}

#[test]
fn index_find_by_tag() {
    let mut idx = PatternIndex::new();
    let p = make_pattern_with_tag("tagged", "web", "serde");
    idx.add(&p);
    assert_eq!(idx.find_by_tag("serde").len(), 1);
    assert_eq!(idx.find_by_tag("tokio").len(), 0);
}

#[test]
fn index_find_by_return_type() {
    let mut idx = PatternIndex::new();
    let p = make_pattern_with_return("getter", "web", "String");
    idx.add(&p);
    assert_eq!(idx.find_by_return_type("String").len(), 1);
    assert_eq!(idx.find_by_return_type("i32").len(), 0);
}

#[test]
fn index_remove() {
    let mut idx = PatternIndex::new();
    let p = make_pattern("removable", "web");
    idx.add(&p);
    assert_eq!(idx.total_indexed(), 1);
    idx.remove(&p);
    assert_eq!(idx.find_by_name("removable").len(), 0);
}

#[test]
fn index_search() {
    let mut idx = PatternIndex::new();
    idx.add(&make_pattern("parse_json", "data"));
    idx.add(&make_pattern("build_html", "web"));
    let results = idx.search("parse");
    assert_eq!(results.len(), 1);
}

#[test]
fn index_total_indexed() {
    let mut idx = PatternIndex::new();
    assert_eq!(idx.total_indexed(), 0);
    idx.add(&make_pattern("a", "x"));
    idx.add(&make_pattern("b", "y"));
    assert_eq!(idx.total_indexed(), 2);
}

#[test]
fn index_clear() {
    let mut idx = PatternIndex::new();
    idx.add(&make_pattern("a", "x"));
    idx.clear();
    assert_eq!(idx.total_indexed(), 0);
}

// ===========================================================================
// PatternVersioner
// ===========================================================================

#[test]
fn versioner_record_version() {
    let mut v = PatternVersioner::new();
    let p = make_pattern("versioned", "web");
    let ver = v.record_version(&p, "initial").unwrap();
    assert_eq!(ver, 1);
}

#[test]
fn versioner_get_history() {
    let mut v = PatternVersioner::new();
    let p = make_pattern("hist", "web");
    v.record_version(&p, "v1").unwrap();
    let history = v.get_history(p.id.as_str());
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].change_description, "v1");
}

#[test]
fn versioner_get_version() {
    let mut v = PatternVersioner::new();
    let p = make_pattern("getv", "web");
    v.record_version(&p, "first").unwrap();
    let entry = v.get_version(p.id.as_str(), 1).unwrap();
    assert_eq!(entry.version, 1);
}

#[test]
fn versioner_get_version_not_found() {
    let mut v = PatternVersioner::new();
    let p = make_pattern("noversion", "web");
    v.record_version(&p, "v1").unwrap();
    let result = v.get_version(p.id.as_str(), 99);
    assert!(result.is_err());
}

#[test]
fn versioner_latest_version() {
    let mut v = PatternVersioner::new();
    let mut p = make_pattern("latest", "web");
    v.record_version(&p, "v1").unwrap();
    p.version = 2;
    v.record_version(&p, "v2").unwrap();
    assert_eq!(v.latest_version(p.id.as_str()), Some(2));
}

#[test]
fn versioner_latest_version_none() {
    let v = PatternVersioner::new();
    assert_eq!(v.latest_version("nonexistent"), None);
}

#[test]
fn versioner_total_versions() {
    let mut v = PatternVersioner::new();
    let p1 = make_pattern("a", "web");
    let p2 = make_pattern("b", "cli");
    v.record_version(&p1, "v1").unwrap();
    v.record_version(&p2, "v1").unwrap();
    assert_eq!(v.total_versions(), 2);
}

#[test]
fn versioner_clear() {
    let mut v = PatternVersioner::new();
    v.record_version(&make_pattern("a", "b"), "v1").unwrap();
    v.clear();
    assert_eq!(v.total_versions(), 0);
}

// ===========================================================================
// Binary format
// ===========================================================================

#[test]
fn binary_format_roundtrip() {
    let p1 = make_pattern("bin1", "data");
    let p2 = make_pattern("bin2", "web");
    let patterns: Vec<&Pattern> = vec![&p1, &p2];
    let mut buf = Vec::new();
    write_patterns(&patterns, &mut buf).unwrap();
    let loaded = read_patterns(&buf).unwrap();
    assert_eq!(loaded.len(), 2);
    assert_eq!(loaded[0].name, "bin1");
    assert_eq!(loaded[1].name, "bin2");
}

#[test]
fn binary_format_invalid_magic() {
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00];
    let result = read_patterns(&data);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Invalid magic"));
}

#[test]
fn binary_format_too_small() {
    let data = vec![0x41, 0x45, 0x56, 0x4C]; // just AEVL, no version/count
    let result = read_patterns(&data);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("too small"));
}

#[test]
fn binary_format_empty_patterns() {
    let patterns: Vec<&Pattern> = vec![];
    let mut buf = Vec::new();
    write_patterns(&patterns, &mut buf).unwrap();
    assert_eq!(&buf[0..4], MAGIC);
    let loaded = read_patterns(&buf).unwrap();
    assert_eq!(loaded.len(), 0);
}

#[test]
fn binary_format_preserves_fields() {
    let mut p = make_pattern("preserve", "data");
    p.tags = vec!["tag1".to_string(), "tag2".to_string()];
    p.usage_count = 42;
    let patterns: Vec<&Pattern> = vec![&p];
    let mut buf = Vec::new();
    write_patterns(&patterns, &mut buf).unwrap();
    let loaded = read_patterns(&buf).unwrap();
    assert_eq!(loaded[0].tags, vec!["tag1", "tag2"]);
    assert_eq!(loaded[0].usage_count, 42);
}
