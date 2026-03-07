use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use agentic_evolve_core::cache::LruCache;
use agentic_evolve_core::matching::signature::SignatureMatcher;
use agentic_evolve_core::storage::store::PatternStore;
use agentic_evolve_core::types::match_result::MatchContext;
use agentic_evolve_core::types::pattern::{
    FunctionSignature, Language, ParamSignature, Pattern, PatternVariable, Visibility,
};

fn make_pattern(name: &str, domain: &str) -> Pattern {
    let sig = FunctionSignature::new(name, Language::Rust);
    Pattern::new(
        name,
        domain,
        Language::Rust,
        sig,
        "fn {{name}}() { todo!() }",
        vec![PatternVariable {
            name: "name".to_string(),
            var_type: "string".to_string(),
            pattern: None,
            default: Some("example".to_string()),
        }],
        0.85,
    )
}

fn bench_pattern_creation(c: &mut Criterion) {
    c.bench_function("pattern_creation", |b| {
        b.iter(|| {
            make_pattern("bench-pattern", "web");
        });
    });
}

fn bench_pattern_serialization(c: &mut Criterion) {
    let pattern = make_pattern("serialize-test", "web");
    c.bench_function("pattern_serialization", |b| {
        b.iter(|| {
            serde_json::to_string(&pattern).unwrap();
        });
    });
}

fn bench_pattern_deserialization(c: &mut Criterion) {
    let pattern = make_pattern("deserialize-test", "web");
    let json = serde_json::to_string(&pattern).unwrap();
    c.bench_function("pattern_deserialization", |b| {
        b.iter(|| {
            let _: Pattern = serde_json::from_str(&json).unwrap();
        });
    });
}

fn bench_signature_matching(c: &mut Criterion) {
    let matcher = SignatureMatcher::new();
    let query = FunctionSignature::new("authenticate", Language::Rust);
    let patterns: Vec<Pattern> = (0..100)
        .map(|i| make_pattern(&format!("pattern-{i}"), "web"))
        .collect();
    c.bench_function("signature_match_100", |b| {
        b.iter(|| {
            for p in &patterns {
                matcher.score_match(p, &query);
            }
        });
    });
}

fn bench_content_hash(c: &mut Criterion) {
    let template = "fn authenticate(req: Request) -> Result<User> { validate_token(req.header(\"Authorization\"))? }";
    c.bench_function("content_hash_blake3", |b| {
        b.iter(|| {
            blake3::hash(template.as_bytes());
        });
    });
}

fn bench_pattern_record_use(c: &mut Criterion) {
    let mut pattern = make_pattern("usage-test", "web");
    c.bench_function("pattern_record_use", |b| {
        b.iter(|| {
            pattern.record_use(true);
        });
    });
}

fn bench_language_parsing(c: &mut Criterion) {
    let languages = ["rust", "python", "typescript", "go", "java", "csharp", "cpp"];
    c.bench_function("language_from_name", |b| {
        b.iter(|| {
            for lang in &languages {
                Language::from_name(lang);
            }
        });
    });
}

fn bench_pattern_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_scaling");
    for size in [10, 100, 1000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let patterns: Vec<Pattern> = (0..size)
                    .map(|i| make_pattern(&format!("p-{i}"), "web"))
                    .collect();
                patterns.len()
            });
        });
    }
    group.finish();
}

fn bench_cache_insert(c: &mut Criterion) {
    let cache: LruCache<String, String> = LruCache::new(1024, Duration::from_secs(300));
    c.bench_function("cache_insert", |b| {
        let mut i = 0u64;
        b.iter(|| {
            cache.insert(format!("key-{i}"), format!("value-{i}"));
            i += 1;
        });
    });
}

fn bench_cache_get_hit(c: &mut Criterion) {
    let cache: LruCache<String, String> = LruCache::new(1024, Duration::from_secs(300));
    for i in 0..100 {
        cache.insert(format!("key-{i}"), format!("value-{i}"));
    }
    c.bench_function("cache_get_hit", |b| {
        let mut i = 0u64;
        b.iter(|| {
            cache.get(&format!("key-{}", i % 100));
            i += 1;
        });
    });
}

fn bench_cache_get_miss(c: &mut Criterion) {
    let cache: LruCache<String, String> = LruCache::new(1024, Duration::from_secs(300));
    c.bench_function("cache_get_miss", |b| {
        b.iter(|| {
            cache.get(&"nonexistent".to_string());
        });
    });
}

fn bench_signature_find_matches(c: &mut Criterion) {
    let matcher = SignatureMatcher::new();
    let query = FunctionSignature {
        name: "authenticate_user".to_string(),
        params: vec![
            ParamSignature { name: "token".to_string(), param_type: "String".to_string(), is_optional: false },
            ParamSignature { name: "scope".to_string(), param_type: "Scope".to_string(), is_optional: true },
        ],
        return_type: Some("Result<User>".to_string()),
        language: Language::Rust,
        is_async: true,
        visibility: Visibility::Public,
    };
    let patterns: Vec<Pattern> = (0..200)
        .map(|i| make_pattern(&format!("auth-pattern-{i}"), "web"))
        .collect();
    let refs: Vec<&Pattern> = patterns.iter().collect();
    let context = MatchContext::new().with_domain("web").with_max_results(10);

    c.bench_function("signature_find_matches_200", |b| {
        b.iter(|| {
            matcher.find_matches(&query, &refs, &context, 10).unwrap();
        });
    });
}

fn bench_store_save_and_get(c: &mut Criterion) {
    c.bench_function("store_save_and_get", |b| {
        b.iter(|| {
            let mut store = PatternStore::new();
            let pattern = make_pattern("store-bench", "web");
            let id = pattern.id.as_str().to_string();
            store.save(&pattern).unwrap();
            store.get(&id).unwrap();
        });
    });
}

fn bench_store_search(c: &mut Criterion) {
    let mut store = PatternStore::new();
    for i in 0..100 {
        let p = make_pattern(&format!("search-pattern-{i}"), if i % 2 == 0 { "web" } else { "cli" });
        store.save(&p).unwrap();
    }
    c.bench_function("store_search_100", |b| {
        b.iter(|| {
            store.search("search-pattern");
        });
    });
}

criterion_group!(
    benches,
    bench_pattern_creation,
    bench_pattern_serialization,
    bench_pattern_deserialization,
    bench_signature_matching,
    bench_signature_find_matches,
    bench_content_hash,
    bench_pattern_record_use,
    bench_language_parsing,
    bench_pattern_scaling,
    bench_cache_insert,
    bench_cache_get_hit,
    bench_cache_get_miss,
    bench_store_save_and_get,
    bench_store_search,
);
criterion_main!(benches);
