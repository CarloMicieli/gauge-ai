use crate::ai::health::HealthStatus;
use crate::app::events::ScrapeEvent;
use crate::app::state::{ExportResultView, LatestRun, QueryResultView, RuntimeState, ScrapeRun};
use crate::scraper::registry::ScraperRegistry;
use crate::tui::layout::{HeaderMetrics, HeaderSections, render_header, render_header_sections};

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
        return vec![
            format!("query error: {error}"),
            "recovery: run /setup, then retry /query with clearer keywords".to_string(),
        ];
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
    if result.summary.failed_scrapers > 0 {
        lines.push("recovery: inspect scraper failures and rerun /latest <scraper>".to_string());
    }
    lines
}

/// Build human-readable lines for an export result.
pub fn render_export_result(result: &ExportResultView) -> Vec<String> {
    let mut lines = vec![
        format!(
            "export: records={}, images={}, missing_images={}",
            result.records, result.images, result.missing_images
        ),
        format!("output: {}", result.output_path),
    ];
    if result.missing_images > 0 {
        lines.push(
            "recovery: review missing-images.txt in the export bundle and rerun /scrape"
                .to_string(),
        );
    }
    lines
}

/// Render command help content.
pub fn render_help() -> Vec<String> {
    vec![
        "/help".to_string(),
        "/list-scraper".to_string(),
        "/scrape <manufacturer> <query>".to_string(),
        "/latest [scraper_name]".to_string(),
        "/query <text>".to_string(),
        "/export <query>".to_string(),
        "/setup".to_string(),
        "/clear".to_string(),
        "/quit (/exit)".to_string(),
    ]
}

/// Render registry summary used by /list-scraper.
pub fn render_scraper_list(registry: &ScraperRegistry) -> Vec<String> {
    let mut lines = vec!["scrapers:".to_string()];
    for scraper in registry.all() {
        let latest = if scraper.supports_latest() {
            "yes"
        } else {
            "no"
        };
        lines.push(format!("- {} (latest: {latest})", scraper.name()));
    }
    if lines.len() == 1 {
        lines.push("- none registered".to_string());
    }
    lines
}

/// Render status/header block for the home view.
pub fn render_header_status(
    runtime: &RuntimeState,
    health: &HealthStatus,
    records: usize,
    scrapers: usize,
    width: u16,
) -> Vec<String> {
    render_header(runtime, health, HeaderMetrics { records, scrapers }, width)
}

/// Render split-header content for the top-level nested layout.
pub fn render_split_header(
    runtime: &RuntimeState,
    health: &HealthStatus,
    records: usize,
    scrapers: usize,
    width: u16,
) -> HeaderSections {
    render_header_sections(runtime, health, HeaderMetrics { records, scrapers }, width)
}

/// Render a generic command failure with a command-specific recovery hint.
pub fn render_command_failure(command: &str, error: &str) -> Vec<String> {
    let hint = match command {
        "/scrape" => "recovery: run /setup and verify manufacturer spelling",
        "/latest" => "recovery: try /list-scraper and run /latest <scraper>",
        "/query" => "recovery: run /setup and try broader keywords",
        "/export" => "recovery: run /query first to confirm matching records",
        _ => "recovery: run /help for command usage",
    };

    vec![format!("{command} error: {error}"), hint.to_string()]
}
