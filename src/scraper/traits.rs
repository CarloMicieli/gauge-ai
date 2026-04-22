use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::app::error::AppResult;

/// Search result pointing to one scraper page that can be fetched and extracted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrapeCandidate {
    pub manufacturer: String,
    pub query: String,
    pub source_url: String,
}

/// Structured data extracted from a scraper page before normalization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedModel {
    pub manufacturer: String,
    pub product_code: String,
    pub name: String,
    pub description: String,
    pub details: String,
    pub scale: Option<String>,
    pub epoch: Option<String>,
    pub railway_company: Option<String>,
    pub image_urls: Vec<String>,
    pub specifications: BTreeMap<String, String>,
}

/// One fetched page with raw content and extracted structured fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrapedPage {
    pub source_url: String,
    pub raw_content: String,
    pub extracted: ExtractedModel,
}

impl ScrapedPage {
    /// Compute a stable fingerprint for merge and idempotency decisions.
    pub fn raw_fingerprint(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.raw_content.as_bytes());
        hasher
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }
}

/// Core scraper contract for manufacturer-specific integrations.
pub trait ModelScraper: Send + Sync {
    fn name(&self) -> &'static str;
    fn search(&self, query: &str) -> AppResult<Vec<ScrapeCandidate>>;
    fn fetch(&self, candidate: &ScrapeCandidate) -> AppResult<ScrapedPage>;
    fn supports_latest(&self) -> bool {
        false
    }
}
