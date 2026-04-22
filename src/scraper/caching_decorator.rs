use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::app::error::AppResult;
use crate::cache::filesystem::{cache_paths, ensure_image_cache_files};
use crate::scraper::traits::{ModelScraper, ScrapeCandidate, ScrapedPage};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedPage {
    page: ScrapedPage,
}

/// Fetch scraper pages with filesystem-backed cache reuse.
pub struct CachingScraper<'a> {
    inner: &'a dyn ModelScraper,
    cache_root: &'a Path,
}

impl<'a> CachingScraper<'a> {
    /// Create a caching wrapper around a scraper implementation.
    pub fn new(inner: &'a dyn ModelScraper, cache_root: &'a Path) -> Self {
        Self { inner, cache_root }
    }

    /// Load a page from cache when available, otherwise fetch and persist it.
    pub fn fetch(&self, candidate: &ScrapeCandidate) -> AppResult<CachedFetch> {
        let (metadata_path, image_dir) =
            cache_paths(self.cache_root, self.inner.name(), &candidate.source_url);
        if metadata_path.exists() {
            fs::create_dir_all(&image_dir)?;
            let raw = fs::read_to_string(&metadata_path)?;
            let cached: CachedPage = serde_json::from_str(&raw)?;
            let _ = ensure_image_cache_files(&image_dir, &cached.page.extracted.image_urls)?;
            return Ok(CachedFetch {
                page: cached.page,
                metadata_path,
                image_dir,
                cache_hit: true,
            });
        }

        if let Some(parent) = metadata_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::create_dir_all(&image_dir)?;

        let page = self.inner.fetch(candidate)?;
        let _ = ensure_image_cache_files(&image_dir, &page.extracted.image_urls)?;
        fs::write(
            &metadata_path,
            serde_json::to_string_pretty(&CachedPage { page: page.clone() })?,
        )?;

        Ok(CachedFetch {
            page,
            metadata_path,
            image_dir,
            cache_hit: false,
        })
    }
}

/// Returned value for cached or freshly fetched scraper pages.
#[derive(Debug, Clone)]
pub struct CachedFetch {
    pub page: ScrapedPage,
    pub metadata_path: PathBuf,
    pub image_dir: PathBuf,
    pub cache_hit: bool,
}
