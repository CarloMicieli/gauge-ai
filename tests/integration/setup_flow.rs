use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use gauge_ai::ai::client::LocalEmbeddingClient;
use gauge_ai::ai::health::HealthStatus;
use gauge_ai::ai::knowledge_base::{KnowledgeBase, OllamaHealthState};
use gauge_ai::ai::normalize::PassThroughNormalizer;
use gauge_ai::app::commands::{Command, CommandContext, CommandOutcome, execute};
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
async fn setup_reports_missing_models_and_pull_summary() {
    let root = unique_temp_dir("setup-flow");
    let cache_root = root.join("cache");
    std::fs::create_dir_all(&cache_root).expect("failed to create cache dir");

    let pool = initialize(&root.join("trains.db"))
        .await
        .expect("db init failed");

    let registry = gauge_ai::scraper::registry::ScraperRegistry::new();
    let health = HealthStatus {
        state: OllamaHealthState::ModelMissing,
        missing_models: vec!["llama3.1:8b".to_string(), "nomic-embed-text".to_string()],
        last_error: None,
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

    let outcome = execute(Command::Setup, &context)
        .await
        .expect("setup should succeed");

    let message = match outcome {
        CommandOutcome::Message(msg) => msg,
        other => panic!("unexpected outcome: {other:?}"),
    };

    assert!(message.contains("missing models"));
    assert!(message.contains("confirmation accepted"));
    assert!(message.contains("pulled"));
}
