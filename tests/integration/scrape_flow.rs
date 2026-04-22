use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::{SystemTime, UNIX_EPOCH};

use gauge_ai::ai::knowledge_base::KnowledgeBase;
use gauge_ai::ai::normalize::PassThroughNormalizer;
use gauge_ai::app::ingest::run_scrape;
use gauge_ai::scraper::registry::ScraperRegistry;
use gauge_ai::scraper::traits::{ExtractedModel, ModelScraper, ScrapeCandidate, ScrapedPage};
use gauge_ai::storage::db::{initialize, list_models};

struct FakeScraper {
    fetch_count: Arc<AtomicUsize>,
}

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
        self.fetch_count.fetch_add(1, Ordering::SeqCst);

        Ok(ScrapedPage {
            source_url: candidate.source_url.clone(),
            raw_content: "<html>BR 50</html>".to_string(),
            extracted: ExtractedModel {
                manufacturer: "roco".to_string(),
                product_code: "BR50-001".to_string(),
                name: "BR 50".to_string(),
                description: "Freight steam locomotive".to_string(),
                details: "Digital sound".to_string(),
                scale: Some("H0".to_string()),
                epoch: Some("III".to_string()),
                railway_company: Some("DB".to_string()),
                image_urls: vec!["https://example.test/images/br50.jpg".to_string()],
                specifications: BTreeMap::from([("decoder".to_string(), "Sound".to_string())]),
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
async fn scrape_flow_persists_records_and_cache() {
    let root = unique_temp_dir("scrape-flow");
    let cache_root = root.join("cache");
    std::fs::create_dir_all(&cache_root).expect("failed to create cache dir");

    let pool = initialize(&root.join("trains.db"))
        .await
        .expect("db init failed");
    let fetch_count = Arc::new(AtomicUsize::new(0));
    let mut registry = ScraperRegistry::new();
    registry.register(Box::new(FakeScraper {
        fetch_count: fetch_count.clone(),
    }));

    let run = run_scrape(
        &registry,
        "roco",
        "BR 50",
        &PassThroughNormalizer,
        &KnowledgeBase::default(),
        &pool,
        &cache_root,
    )
    .await
    .expect("scrape run failed");

    assert_eq!(run.summary.discovered, 1);
    assert_eq!(run.summary.processed, 1);
    assert_eq!(run.summary.failed, 0);
    assert_eq!(run.summary.new_records, 1);
    assert_eq!(run.summary.updated_records, 0);
    assert_eq!(fetch_count.load(Ordering::SeqCst), 1);

    let models = list_models(&pool).await.expect("list models failed");
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].manufacturer, "roco");
    assert_eq!(models[0].product_code, "BR50-001");
    assert_eq!(models[0].local_image_paths.len(), 1);

    let metadata_files = std::fs::read_dir(cache_root.join("roco"))
        .expect("scraper cache dir missing")
        .count();
    assert_eq!(metadata_files, 1);
}
