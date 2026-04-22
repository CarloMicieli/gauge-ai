use std::collections::BTreeMap;
use std::time::Instant;

use gauge_ai::ai::knowledge_base::KnowledgeBase;
use gauge_ai::cache::filesystem::hash_url;

#[test]
fn benchmark_cache_hashing_hot_path() {
    let start = Instant::now();
    let mut last_hash = String::new();
    for index in 0..10_000 {
        let url = format!("https://example.test/model/{index}");
        last_hash = hash_url(&url);
    }

    assert_eq!(last_hash.len(), 64);
    let elapsed_ms = start.elapsed().as_millis();
    eprintln!("benchmark cache_hashing elapsed_ms={elapsed_ms}");
}

#[test]
fn benchmark_knowledge_lookup_hot_path() {
    let mut knowledge_base = KnowledgeBase::default();
    let mut prototypes = BTreeMap::new();
    for index in 0..500 {
        prototypes.insert(
            format!("Proto-{index}"),
            vec![format!("alias-{index}"), format!("series-{index}")],
        );
    }
    knowledge_base.prototypes = prototypes;

    let start = Instant::now();
    let mut matches = Vec::new();
    for index in 0..5_000 {
        matches = knowledge_base.matching_prototypes(&format!("need alias-{}", index % 500));
    }

    assert!(!matches.is_empty());
    let elapsed_ms = start.elapsed().as_millis();
    eprintln!("benchmark knowledge_lookup elapsed_ms={elapsed_ms}");
}
