use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use gauge_ai::ai::client::LocalEmbeddingClient;
use gauge_ai::ai::health::HealthStatus;
use gauge_ai::ai::knowledge_base::{KnowledgeBase, OllamaHealthState};
use gauge_ai::ai::normalize::PassThroughNormalizer;
use gauge_ai::app::commands::{Command, CommandContext, execute};
use gauge_ai::scraper::registry::ScraperRegistry;
use gauge_ai::storage::db::initialize;

fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("gauge-ai-{label}-{nanos}"));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[tokio::test]
async fn latest_fails_fast_when_ollama_is_disconnected() {
    let root = unique_temp_dir("latest-health-guard");
    let cache_root = root.join("cache");
    std::fs::create_dir_all(&cache_root).expect("failed to create cache dir");

    let pool = initialize(&root.join("trains.db"))
        .await
        .expect("db init failed");
    let registry = ScraperRegistry::new();

    let health = HealthStatus {
        state: OllamaHealthState::Disconnected,
        missing_models: vec![],
        last_error: Some("connection refused".to_string()),
        last_checked_epoch_secs: 1,
    };
    let embedding_client = LocalEmbeddingClient;
    let context = CommandContext {
        registry: &registry,
        normalizer: &PassThroughNormalizer,
        knowledge_base: &KnowledgeBase::default(),
        pool: &pool,
        cache_root: &cache_root,
        health: &health,
        embedding_client: &embedding_client,
    };

    let error = execute(Command::Latest { scraper: None }, &context)
        .await
        .expect_err("latest should fail fast");

    assert!(error.to_string().contains("Ollama unavailable for /latest"));
}
