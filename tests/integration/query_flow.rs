use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use gauge_ai::ai::client::LocalEmbeddingClient;
use gauge_ai::ai::health::HealthStatus;
use gauge_ai::ai::knowledge_base::{KnowledgeBase, OllamaHealthState};
use gauge_ai::ai::normalize::PassThroughNormalizer;
use gauge_ai::app::commands::{Command, CommandContext, CommandOutcome, execute};
use gauge_ai::app::ingest::run_scrape;
use gauge_ai::scraper::registry::ScraperRegistry;
use gauge_ai::scraper::traits::{ExtractedModel, ModelScraper, ScrapeCandidate, ScrapedPage};
use gauge_ai::storage::db::initialize;

struct FakeScraper;

impl ModelScraper for FakeScraper {
    fn name(&self) -> &'static str {
        "roco"
    }

    fn search(&self, query: &str) -> gauge_ai::app::error::AppResult<Vec<ScrapeCandidate>> {
        Ok(vec![ScrapeCandidate {
            manufacturer: "roco".to_string(),
            query: query.to_string(),
            source_url: "https://example.test/roco/br50".to_string(),
        }])
    }

    fn fetch(&self, candidate: &ScrapeCandidate) -> gauge_ai::app::error::AppResult<ScrapedPage> {
        Ok(ScrapedPage {
            source_url: candidate.source_url.clone(),
            raw_content: "<html>BR 50</html>".to_string(),
            extracted: ExtractedModel {
                manufacturer: "roco".to_string(),
                product_code: "BR50-001".to_string(),
                name: "BR 50".to_string(),
                description: "Freight steam locomotive with sound".to_string(),
                details: "Digital sound decoder".to_string(),
                scale: Some("H0".to_string()),
                epoch: Some("III".to_string()),
                railway_company: Some("DB".to_string()),
                image_urls: vec![],
                specifications: BTreeMap::new(),
            },
        })
    }
}

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
async fn query_flow_returns_grounded_answer() {
    let root = unique_temp_dir("query-flow");
    let cache_root = root.join("cache");
    std::fs::create_dir_all(&cache_root).expect("failed to create cache dir");

    let pool = initialize(&root.join("trains.db"))
        .await
        .expect("db init failed");

    let mut registry = ScraperRegistry::new();
    registry.register(Box::new(FakeScraper));

    run_scrape(
        &registry,
        "roco",
        "BR 50",
        &PassThroughNormalizer,
        &KnowledgeBase::default(),
        &pool,
        &cache_root,
    )
    .await
    .expect("scrape failed");

    let health = HealthStatus {
        state: OllamaHealthState::Healthy,
        missing_models: vec![],
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

    let outcome = execute(
        Command::Query {
            text: "Which BR 50 has sound?".to_string(),
        },
        &context,
    )
    .await
    .expect("query command failed");

    match outcome {
        CommandOutcome::Query(result) => {
            assert!(result.result_count >= 1);
            assert!(result.answer.contains("BR 50"));
        }
        other => panic!("unexpected outcome: {other:?}"),
    }
}
