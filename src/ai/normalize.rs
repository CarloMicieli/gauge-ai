use crate::ai::knowledge_base::KnowledgeBase;
use crate::app::error::AppResult;
use crate::scraper::traits::ExtractedModel;
use crate::storage::models::{ModelData, NormalizationStatus};

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
    match normalizer.normalize(extracted, knowledge_base) {
        Ok(model) => model,
        Err(_) => ModelData {
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
        },
    }
}
