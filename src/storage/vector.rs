use crate::app::error::AppResult;
use crate::storage::db::list_models;
use crate::storage::models::ModelData;
use sqlx::SqlitePool;

/// Retrieve top-k models using a lexical score over embedding tokens.
pub async fn search_by_embedding(
    pool: &SqlitePool,
    embedding_tokens: &[String],
    top_k: usize,
) -> AppResult<Vec<ModelData>> {
    let models = list_models(pool).await?;
    let mut scored: Vec<(i32, ModelData)> = models
        .into_iter()
        .map(|model| {
            let haystack = format!(
                "{} {} {} {} {}",
                model.manufacturer,
                model.product_code,
                model.name,
                model.description,
                model.details
            )
            .to_lowercase();
            let score = embedding_tokens
                .iter()
                .filter(|token| haystack.contains(token.as_str()))
                .count() as i32;
            (score, model)
        })
        .filter(|(score, _)| *score > 0)
        .collect();

    scored.sort_by(|(score_a, _), (score_b, _)| score_b.cmp(score_a));
    Ok(scored
        .into_iter()
        .take(top_k)
        .map(|(_, model)| model)
        .collect())
}
