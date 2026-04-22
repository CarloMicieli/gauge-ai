use std::time::Instant;

use sqlx::SqlitePool;

use crate::ai::client::EmbeddingClient;
use crate::ai::knowledge_base::KnowledgeBase;
use crate::app::error::AppResult;
use crate::app::state::QueryResultView;
use crate::storage::db::insert_query_run;
use crate::storage::models::QueryRun;
use crate::storage::vector::search_by_embedding;

/// Execute a grounded query against locally persisted model data.
pub async fn execute_query(
    pool: &SqlitePool,
    knowledge_base: &KnowledgeBase,
    embedding_client: &dyn EmbeddingClient,
    text: &str,
    top_k: usize,
) -> AppResult<QueryResultView> {
    let started = Instant::now();
    let expanded = expand_query(text, knowledge_base);
    let embedding = embedding_client.embed(&expanded);
    let results = search_by_embedding(pool, &embedding, top_k).await?;

    let latency_ms = started.elapsed().as_millis() as i64;
    let answer = if results.is_empty() {
        "No matching local records found. Try manufacturer, prototype, or livery aliases."
            .to_string()
    } else {
        let lines = results
            .iter()
            .map(|model| {
                format!(
                    "{} {} ({})",
                    model.manufacturer, model.name, model.product_code
                )
            })
            .collect::<Vec<_>>()
            .join("; ");
        format!("Grounded matches: {lines}")
    };

    insert_query_run(
        pool,
        &QueryRun {
            id: None,
            query_text: text.to_string(),
            top_k: top_k as i64,
            latency_ms,
            result_count: results.len() as i64,
            created_at: String::new(),
        },
    )
    .await?;

    if results.is_empty() {
        Ok(QueryResultView::no_match(
            answer,
            latency_ms,
            vec![
                "Try adding manufacturer aliases".to_string(),
                "Try a broader prototype term".to_string(),
            ],
        ))
    } else {
        Ok(QueryResultView::success(answer, results.len(), latency_ms))
    }
}

/// Expand a query using knowledge base prototype and livery aliases.
pub fn expand_query(query: &str, knowledge_base: &KnowledgeBase) -> String {
    let mut expanded = query.to_string();

    for canonical in knowledge_base.matching_prototypes(query) {
        expanded.push(' ');
        expanded.push_str(&canonical);
    }

    for canonical in knowledge_base.matching_liveries(query) {
        expanded.push(' ');
        expanded.push_str(&canonical);
    }

    expanded
}
