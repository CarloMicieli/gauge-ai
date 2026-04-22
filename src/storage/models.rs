use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelData {
    pub manufacturer: String,
    pub product_code: String,
    pub name: String,
    pub description: String,
    pub details: String,
    pub scale: Option<String>,
    pub epoch: Option<String>,
    pub railway_company: Option<String>,
    pub image_urls: Vec<String>,
    pub local_image_paths: Vec<String>,
    pub specifications: BTreeMap<String, String>,
    pub normalization_status: NormalizationStatus,
    pub source_fingerprint: String,
    pub last_scraped_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelVersion {
    pub id: Option<i64>,
    pub manufacturer: String,
    pub product_code: String,
    pub snapshot_json: String,
    pub change_reason: ChangeReason,
    pub merged_by_model: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeReason {
    ScrapeUpdate,
    LatestSyncMerge,
    ManualCorrection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizationStatus {
    Normalized,
    Unnormalized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistOutcome {
    Inserted,
    Updated,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryRun {
    pub id: Option<i64>,
    pub query_text: String,
    pub top_k: i64,
    pub latency_ms: i64,
    pub result_count: i64,
    pub created_at: String,
}
