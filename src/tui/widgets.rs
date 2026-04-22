use crate::app::events::ScrapeEvent;
use crate::app::state::ScrapeRun;

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
