use crate::app::events::ScrapeEvent;
use crate::app::state::{LatestRun, QueryResultView, ScrapeRun};

/// Build human-readable progress lines for the current scrape run.
pub fn render_scrape_progress(run: &ScrapeRun) -> Vec<String> {
    let mut lines = vec![format!(
        "scrape: discovered={}, processed={}, failed={}, new={}, updated={}",
        run.summary.discovered,
        run.summary.processed,
        run.summary.failed,
        run.summary.new_records,
        run.summary.updated_records
    )];

    lines.extend(run.events.iter().map(render_event));
    lines
}

fn render_event(event: &ScrapeEvent) -> String {
    match event {
        ScrapeEvent::Discovered { source_url } => format!("discovered {source_url}"),
        ScrapeEvent::Processed { product_code } => format!("processed {product_code}"),
        ScrapeEvent::Failed { source_url, reason } => format!("failed {source_url}: {reason}"),
    }
}

/// Build human-readable lines for a query result.
pub fn render_query_result(result: &QueryResultView) -> Vec<String> {
    if let Some(error) = &result.error {
        return vec![format!("query error: {error}")];
    }

    let mut lines = vec![format!(
        "query: results={}, latency={}ms",
        result.result_count, result.latency_ms
    )];
    lines.push(result.answer.clone());
    lines.extend(result.hints.iter().map(|hint| format!("hint: {hint}")));
    lines
}

/// Build human-readable lines for a latest-sync result.
pub fn render_latest_result(result: &LatestRun) -> Vec<String> {
    let mut lines = vec![format!(
        "latest: processed={}, inserted={}, updated={}, skipped_scrapers={}, failed_scrapers={}",
        result.summary.processed,
        result.summary.inserted,
        result.summary.updated,
        result.summary.skipped_scrapers,
        result.summary.failed_scrapers
    )];
    lines.extend(result.messages.iter().cloned());
    lines
}
