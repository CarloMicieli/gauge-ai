use std::path::Path;

use sqlx::SqlitePool;

use crate::app::error::{AppError, AppResult};
use crate::storage::db::list_models;
use crate::storage::models::ModelData;

pub mod archive;
pub mod csv;
pub mod json;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportBundleResult {
    pub output_path: String,
    pub records: usize,
    pub images: usize,
    pub missing_images: usize,
}

/// Select records that match an export query.
pub async fn select_records(pool: &SqlitePool, query: &str) -> AppResult<Vec<ModelData>> {
    let normalized_query = query.to_lowercase();
    let models = list_models(pool).await?;

    let selected = models
        .into_iter()
        .filter(|model| {
            [
                model.manufacturer.as_str(),
                model.product_code.as_str(),
                model.name.as_str(),
                model.description.as_str(),
                model.details.as_str(),
            ]
            .iter()
            .any(|field| field.to_lowercase().contains(&normalized_query))
        })
        .collect::<Vec<_>>();

    if selected.is_empty() {
        return Err(AppError::Operation("NO_MATCHING_RECORDS".to_string()));
    }

    Ok(selected)
}

/// Export selected records into JSON/CSV and bundled image artifacts.
pub async fn export_records(
    pool: &SqlitePool,
    query: &str,
    output_dir: &Path,
) -> AppResult<ExportBundleResult> {
    let records = select_records(pool, query).await?;
    std::fs::create_dir_all(output_dir)?;

    let json_path = output_dir.join("models.json");
    let csv_path = output_dir.join("models.csv");
    json::write_json(&json_path, &records)?;
    csv::write_csv(&csv_path, &records)?;

    let archive_result = archive::write_assets(output_dir, &records)?;

    Ok(ExportBundleResult {
        output_path: output_dir.display().to_string(),
        records: records.len(),
        images: archive_result.images,
        missing_images: archive_result.missing_images,
    })
}
