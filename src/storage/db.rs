use std::path::Path;

use sqlx::Row;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use crate::app::error::AppResult;
use crate::storage::migrations;
use crate::storage::models::{
    ChangeReason, ModelData, ModelVersion, NormalizationStatus, PersistOutcome, QueryRun,
};

/// Open a SQLite connection pool for the provided database path.
pub async fn connect(db_path: &Path) -> AppResult<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .foreign_keys(true);

    SqlitePool::connect_with(options).await.map_err(Into::into)
}

/// Open a database and run migrations before use.
pub async fn initialize(db_path: &Path) -> AppResult<SqlitePool> {
    let pool = connect(db_path).await?;
    migrations::run(&pool).await?;
    Ok(pool)
}

/// Persist one model, archiving history when the source fingerprint changes.
pub async fn persist_model(pool: &SqlitePool, model: &ModelData) -> AppResult<PersistOutcome> {
    let existing = sqlx::query(
        "SELECT manufacturer, product_code, name, description, details, scale, epoch, railway_company, image_urls, local_image_paths, specifications, normalization_status, source_fingerprint FROM model_data WHERE manufacturer = ?1 AND product_code = ?2",
    )
    .bind(&model.manufacturer)
    .bind(&model.product_code)
    .fetch_optional(pool)
    .await?;

    let now = timestamp_now();
    let image_urls_json = serde_json::to_string(&model.image_urls)?;
    let local_image_paths_json = serde_json::to_string(&model.local_image_paths)?;
    let specifications_json = serde_json::to_string(&model.specifications)?;
    let normalization_status = match model.normalization_status {
        NormalizationStatus::Normalized => "Normalized",
        NormalizationStatus::Unnormalized => "Unnormalized",
    };

    if let Some(row) = existing {
        let existing_fingerprint: String = row.get("source_fingerprint");
        if existing_fingerprint == model.source_fingerprint {
            return Ok(PersistOutcome::Unchanged);
        }

        let previous = ModelVersion {
            id: None,
            manufacturer: row.get("manufacturer"),
            product_code: row.get("product_code"),
            snapshot_json: serde_json::to_string(&ModelData {
                manufacturer: row.get("manufacturer"),
                product_code: row.get("product_code"),
                name: row.get("name"),
                description: row.get("description"),
                details: row.get("details"),
                scale: row.get("scale"),
                epoch: row.get("epoch"),
                railway_company: row.get("railway_company"),
                image_urls: serde_json::from_str::<Vec<String>>(
                    &row.get::<String, _>("image_urls"),
                )?,
                local_image_paths: serde_json::from_str::<Vec<String>>(
                    &row.get::<String, _>("local_image_paths"),
                )?,
                specifications: serde_json::from_str(&row.get::<String, _>("specifications"))?,
                normalization_status: parse_normalization_status(
                    &row.get::<String, _>("normalization_status"),
                ),
                source_fingerprint: existing_fingerprint,
            })?,
            change_reason: ChangeReason::ScrapeUpdate,
            merged_by_model: None,
        };

        sqlx::query(
            "INSERT INTO model_versions (manufacturer, product_code, snapshot_json, change_reason, merged_by_model, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )
        .bind(&previous.manufacturer)
        .bind(&previous.product_code)
        .bind(&previous.snapshot_json)
        .bind("ScrapeUpdate")
        .bind(&previous.merged_by_model)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "UPDATE model_data SET name = ?3, description = ?4, details = ?5, scale = ?6, epoch = ?7, railway_company = ?8, image_urls = ?9, local_image_paths = ?10, specifications = ?11, normalization_status = ?12, source_fingerprint = ?13, last_scraped_at = ?14, updated_at = ?15 WHERE manufacturer = ?1 AND product_code = ?2",
        )
        .bind(&model.manufacturer)
        .bind(&model.product_code)
        .bind(&model.name)
        .bind(&model.description)
        .bind(&model.details)
        .bind(&model.scale)
        .bind(&model.epoch)
        .bind(&model.railway_company)
        .bind(&image_urls_json)
        .bind(&local_image_paths_json)
        .bind(&specifications_json)
        .bind(normalization_status)
        .bind(&model.source_fingerprint)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        return Ok(PersistOutcome::Updated);
    }

    sqlx::query(
        "INSERT INTO model_data (manufacturer, product_code, name, description, details, scale, epoch, railway_company, image_urls, local_image_paths, specifications, normalization_status, source_fingerprint, last_scraped_at, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
    )
    .bind(&model.manufacturer)
    .bind(&model.product_code)
    .bind(&model.name)
    .bind(&model.description)
    .bind(&model.details)
    .bind(&model.scale)
    .bind(&model.epoch)
    .bind(&model.railway_company)
    .bind(&image_urls_json)
    .bind(&local_image_paths_json)
    .bind(&specifications_json)
    .bind(normalization_status)
    .bind(&model.source_fingerprint)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(PersistOutcome::Inserted)
}

/// Load all persisted models for validation and export workflows.
pub async fn list_models(pool: &SqlitePool) -> AppResult<Vec<ModelData>> {
    let rows = sqlx::query(
        "SELECT manufacturer, product_code, name, description, details, scale, epoch, railway_company, image_urls, local_image_paths, specifications, normalization_status, source_fingerprint FROM model_data ORDER BY manufacturer, product_code",
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(row_to_model).collect()
}

fn row_to_model(row: sqlx::sqlite::SqliteRow) -> AppResult<ModelData> {
    Ok(ModelData {
        manufacturer: row.get("manufacturer"),
        product_code: row.get("product_code"),
        name: row.get("name"),
        description: row.get("description"),
        details: row.get("details"),
        scale: row.get("scale"),
        epoch: row.get("epoch"),
        railway_company: row.get("railway_company"),
        image_urls: serde_json::from_str(&row.get::<String, _>("image_urls"))?,
        local_image_paths: serde_json::from_str(&row.get::<String, _>("local_image_paths"))?,
        specifications: serde_json::from_str(&row.get::<String, _>("specifications"))?,
        normalization_status: parse_normalization_status(
            &row.get::<String, _>("normalization_status"),
        ),
        source_fingerprint: row.get("source_fingerprint"),
    })
}

fn parse_normalization_status(value: &str) -> NormalizationStatus {
    match value {
        "Unnormalized" => NormalizationStatus::Unnormalized,
        _ => NormalizationStatus::Normalized,
    }
}

fn timestamp_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().to_string(),
        Err(_) => String::from("0"),
    }
}

/// Persist telemetry for one `/query` execution.
pub async fn insert_query_run(pool: &SqlitePool, query_run: &QueryRun) -> AppResult<()> {
    let created_at = timestamp_now();

    sqlx::query(
        "INSERT INTO query_run (query_text, top_k, latency_ms, result_count, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(&query_run.query_text)
    .bind(query_run.top_k)
    .bind(query_run.latency_ms)
    .bind(query_run.result_count)
    .bind(&created_at)
    .execute(pool)
    .await?;

    Ok(())
}
