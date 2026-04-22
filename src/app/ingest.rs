use std::path::Path;

use crate::ai::knowledge_base::KnowledgeBase;
use crate::ai::normalize::{Normalizer, normalize_or_fallback};
use crate::app::error::{AppError, AppResult};
use crate::app::events::ScrapeEvent;
use crate::app::state::ScrapeRun;
use crate::cache::filesystem::local_image_paths;
use crate::scraper::caching_decorator::CachingScraper;
use crate::scraper::registry::ScraperRegistry;
use crate::storage::db::persist_model;
use crate::storage::models::PersistOutcome;
use sqlx::SqlitePool;

/// Run a scrape operation for one manufacturer and query string.
pub async fn run_scrape(
    registry: &ScraperRegistry,
    manufacturer: &str,
    query: &str,
    normalizer: &dyn Normalizer,
    knowledge_base: &KnowledgeBase,
    pool: &SqlitePool,
    cache_root: &Path,
) -> AppResult<ScrapeRun> {
    let scraper = registry
        .get(manufacturer)
        .ok_or_else(|| AppError::Operation(format!("unknown scraper: {manufacturer}")))?;

    let caching_scraper = CachingScraper::new(scraper, cache_root);
    let candidates = scraper.search(query)?;
    let mut run = ScrapeRun::default();
    run.summary.discovered = candidates.len();

    for candidate in candidates {
        run.events.push(ScrapeEvent::Discovered {
            source_url: candidate.source_url.clone(),
        });

        match caching_scraper.fetch(&candidate) {
            Ok(fetched) => {
                let mut model =
                    normalize_or_fallback(normalizer, &fetched.page.extracted, knowledge_base);
                model.source_fingerprint = fetched.page.raw_fingerprint();
                model.local_image_paths =
                    local_image_paths(&fetched.image_dir, &fetched.page.extracted.image_urls);

                match persist_model(pool, &model).await? {
                    PersistOutcome::Inserted => run.summary.new_records += 1,
                    PersistOutcome::Updated => run.summary.updated_records += 1,
                    PersistOutcome::Unchanged => {}
                }

                run.summary.processed += 1;
                run.events.push(ScrapeEvent::Processed {
                    product_code: model.product_code,
                });
            }
            Err(err) => {
                run.summary.failed += 1;
                run.events.push(ScrapeEvent::Failed {
                    source_url: candidate.source_url,
                    reason: err.to_string(),
                });
            }
        }
    }

    Ok(run)
}
