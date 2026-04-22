use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use crate::ai::knowledge_base::KnowledgeBase;
use crate::app::error::AppResult;
use crate::scraper::traits::ExtractedModel;
use crate::storage::models::{ModelData, NormalizationStatus};

const NORMALIZATION_CACHE_LIMIT: usize = 256;
static NORMALIZATION_CACHE: OnceLock<Mutex<BTreeMap<String, ModelData>>> = OnceLock::new();

/// Normalization boundary used by scrape ingestion.
pub trait Normalizer: Send + Sync {
    /// Convert extracted scraper data into persisted model data.
    fn normalize(
        &self,
        extracted: &ExtractedModel,
        knowledge_base: &KnowledgeBase,
    ) -> AppResult<ModelData>;
}

/// Minimal normalizer used during early implementation and tests.
pub struct PassThroughNormalizer;

impl Normalizer for PassThroughNormalizer {
    fn normalize(
        &self,
        extracted: &ExtractedModel,
        _knowledge_base: &KnowledgeBase,
    ) -> AppResult<ModelData> {
        Ok(ModelData {
            manufacturer: extracted.manufacturer.clone(),
            product_code: extracted.product_code.clone(),
            name: extracted.name.clone(),
            description: extracted.description.clone(),
            details: extracted.details.clone(),
            scale: extracted.scale.clone(),
            epoch: extracted.epoch.clone(),
            railway_company: extracted.railway_company.clone(),
            image_urls: extracted.image_urls.clone(),
            local_image_paths: Vec::new(),
            specifications: extracted.specifications.clone(),
            normalization_status: NormalizationStatus::Normalized,
            source_fingerprint: String::new(),
            last_scraped_at: None,
        })
    }
}

/// Normalize one extracted page and fall back to an unnormalized record when normalization fails.
pub fn normalize_or_fallback(
    normalizer: &dyn Normalizer,
    extracted: &ExtractedModel,
    knowledge_base: &KnowledgeBase,
) -> ModelData {
    let cache_key = normalization_cache_key(extracted, knowledge_base);
    if let Some(cached) = read_normalization_cache(&cache_key) {
        return cached;
    }

    let _context = normalization_prompt_context(extracted, knowledge_base);

    let normalized = match normalizer.normalize(extracted, knowledge_base) {
        Ok(model) => model,
        Err(_) => fallback_model(extracted),
    };

    write_normalization_cache(&cache_key, &normalized);
    normalized
}

/// Build a compact normalization context string from extracted text and knowledge aliases.
pub fn normalization_prompt_context(
    extracted: &ExtractedModel,
    knowledge_base: &KnowledgeBase,
) -> String {
    let context_query = format!(
        "{} {} {}",
        extracted.name, extracted.description, extracted.details
    );
    knowledge_base.filtered_prompt_context(&context_query, 8)
}

fn fallback_model(extracted: &ExtractedModel) -> ModelData {
    ModelData {
        manufacturer: extracted.manufacturer.clone(),
        product_code: extracted.product_code.clone(),
        name: extracted.name.clone(),
        description: extracted.description.clone(),
        details: extracted.details.clone(),
        scale: extracted.scale.clone(),
        epoch: extracted.epoch.clone(),
        railway_company: extracted.railway_company.clone(),
        image_urls: extracted.image_urls.clone(),
        local_image_paths: Vec::new(),
        specifications: extracted.specifications.clone(),
        normalization_status: NormalizationStatus::Unnormalized,
        source_fingerprint: String::new(),
        last_scraped_at: None,
    }
}

fn normalization_cache_key(extracted: &ExtractedModel, knowledge_base: &KnowledgeBase) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        knowledge_base.version,
        extracted.manufacturer,
        extracted.product_code,
        extracted.name,
        extracted.description
    )
}

fn read_normalization_cache(key: &str) -> Option<ModelData> {
    let cache = NORMALIZATION_CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    match cache.lock() {
        Ok(guard) => guard.get(key).cloned(),
        Err(_) => None,
    }
}

fn write_normalization_cache(key: &str, model: &ModelData) {
    let cache = NORMALIZATION_CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    if let Ok(mut guard) = cache.lock() {
        guard.insert(key.to_string(), model.clone());
        while guard.len() > NORMALIZATION_CACHE_LIMIT {
            if let Some(first_key) = guard.keys().next().cloned() {
                guard.remove(&first_key);
            } else {
                break;
            }
        }
    }
}
