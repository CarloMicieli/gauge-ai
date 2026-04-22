use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use gauge_ai::ai::client::LocalEmbeddingClient;
use gauge_ai::ai::health::HealthStatus;
use gauge_ai::ai::knowledge_base::{KnowledgeBase, OllamaHealthState};
use gauge_ai::ai::normalize::PassThroughNormalizer;
use gauge_ai::app::commands::{Command, CommandContext, CommandOutcome, execute};
use gauge_ai::scraper::registry::ScraperRegistry;
use gauge_ai::scraper::traits::{ExtractedModel, ModelScraper, ScrapeCandidate, ScrapedPage};
use gauge_ai::storage::db::initialize;

struct LatestCapableScraper;
struct NonLatestScraper;

impl ModelScraper for LatestCapableScraper {
    fn name(&self) -> &'static str {
        "roco"
    }

    fn search(&self, _query: &str) -> gauge_ai::app::error::AppResult<Vec<ScrapeCandidate>> {
        Ok(vec![])
    }

    fn fetch(&self, candidate: &ScrapeCandidate) -> gauge_ai::app::error::AppResult<ScrapedPage> {
        Ok(ScrapedPage {
            source_url: candidate.source_url.clone(),
            raw_content: "<html>latest</html>".to_string(),
            extracted: ExtractedModel {
                manufacturer: "roco".to_string(),
                product_code: "NEW-001".to_string(),
                name: "Latest Item".to_string(),
                description: "Latest arrivals entry".to_string(),
                details: "DCC".to_string(),
                scale: Some("H0".to_string()),
                epoch: Some("VI".to_string()),
                railway_company: Some("DB".to_string()),
                image_urls: vec![],
                specifications: BTreeMap::new(),
            },
        })
    }

    fn supports_latest(&self) -> bool {
        true
    }

    fn latest_candidates(&self) -> gauge_ai::app::error::AppResult<Vec<ScrapeCandidate>> {
        Ok(vec![ScrapeCandidate {
            manufacturer: "roco".to_string(),
            query: "latest".to_string(),
            source_url: "https://example.test/roco/latest-1".to_string(),
        }])
    }
}

impl ModelScraper for NonLatestScraper {
    fn name(&self) -> &'static str {
        "marklin"
    }

    fn search(&self, _query: &str) -> gauge_ai::app::error::AppResult<Vec<ScrapeCandidate>> {
        Ok(vec![])
    }

    fn fetch(&self, candidate: &ScrapeCandidate) -> gauge_ai::app::error::AppResult<ScrapedPage> {
        Ok(ScrapedPage {
            source_url: candidate.source_url.clone(),
            raw_content: "<html>none</html>".to_string(),
            extracted: ExtractedModel {
                manufacturer: "marklin".to_string(),
                product_code: "NOPE".to_string(),
                name: "Unused".to_string(),
                description: "Unused".to_string(),
                details: "Unused".to_string(),
                scale: None,
                epoch: None,
                railway_company: None,
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
async fn latest_global_reports_skipped_scrapers() {
    let root = unique_temp_dir("latest-flow");
    let cache_root = root.join("cache");
    std::fs::create_dir_all(&cache_root).expect("failed to create cache dir");

    let pool = initialize(&root.join("trains.db"))
        .await
        .expect("db init failed");
    let mut registry = ScraperRegistry::new();
    registry.register(Box::new(LatestCapableScraper));
    registry.register(Box::new(NonLatestScraper));

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

    let outcome = execute(Command::Latest { scraper: None }, &context)
        .await
        .expect("latest command failed");

    match outcome {
        CommandOutcome::Latest(run) => {
            assert_eq!(run.summary.processed, 1);
            assert_eq!(run.summary.skipped_scrapers, 1);
            assert_eq!(run.summary.failed_scrapers, 0);
        }
        other => panic!("unexpected outcome: {other:?}"),
    }
}
