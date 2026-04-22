use std::path::Path;

use sqlx::SqlitePool;

use crate::ai::knowledge_base::KnowledgeBase;
use crate::ai::normalize::{Normalizer, normalize_or_fallback};
use crate::app::error::{AppError, AppResult};
use crate::app::state::{LatestRun, LatestSummary};
use crate::cache::filesystem::local_image_paths;
use crate::scraper::caching_decorator::CachingScraper;
use crate::scraper::registry::ScraperRegistry;
use crate::scraper::traits::{ModelScraper, ScrapeCandidate};
use crate::storage::db::persist_model;
use crate::storage::models::PersistOutcome;

/// Run latest synchronization globally or for a targeted scraper.
pub async fn run_latest(
    registry: &ScraperRegistry,
    target_scraper: Option<&str>,
    normalizer: &dyn Normalizer,
    knowledge_base: &KnowledgeBase,
    pool: &SqlitePool,
    cache_root: &Path,
) -> AppResult<LatestRun> {
    let mut run = LatestRun::default();

    let scrapers: Vec<&dyn ModelScraper> = if let Some(name) = target_scraper {
        let scraper = registry
            .get(name)
            .ok_or_else(|| AppError::Operation(format!("SCRAPER_NOT_FOUND: {name}")))?;
        vec![scraper]
    } else {
        registry.all()
    };

    for scraper in scrapers {
        if !scraper.supports_latest() {
            if target_scraper.is_some() {
                return Err(AppError::Operation(format!(
                    "LATEST_UNSUPPORTED: {}",
                    scraper.name()
                )));
            }
            run.summary.skipped_scrapers += 1;
            run.messages
                .push(format!("skipped {} (latest unsupported)", scraper.name()));
            continue;
        }

        let candidates = scraper.latest_candidates()?;
        if let Err(err) = process_latest_candidates(
            scraper,
            candidates,
            normalizer,
            knowledge_base,
            pool,
            cache_root,
            &mut run.summary,
        )
        .await
        {
            run.summary.failed_scrapers += 1;
            run.messages
                .push(format!("failed {}: {}", scraper.name(), err));
        }
    }

    Ok(run)
}

async fn process_latest_candidates(
    scraper: &dyn ModelScraper,
    candidates: Vec<ScrapeCandidate>,
    normalizer: &dyn Normalizer,
    knowledge_base: &KnowledgeBase,
    pool: &SqlitePool,
    cache_root: &Path,
    summary: &mut LatestSummary,
) -> AppResult<()> {
    let caching_scraper = CachingScraper::new(scraper, cache_root);

    for candidate in candidates {
        let fetched = caching_scraper.fetch(&candidate)?;
        let mut model = normalize_or_fallback(normalizer, &fetched.page.extracted, knowledge_base);
        model.source_fingerprint = fetched.page.raw_fingerprint();
        model.local_image_paths =
            local_image_paths(&fetched.image_dir, &fetched.page.extracted.image_urls);

        match persist_model(pool, &model).await? {
            PersistOutcome::Inserted => summary.inserted += 1,
            PersistOutcome::Updated => summary.updated += 1,
            PersistOutcome::Unchanged => {}
        }
        summary.processed += 1;
    }

    Ok(())
}
