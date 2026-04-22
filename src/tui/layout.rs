use crate::ai::health::HealthStatus;
use crate::app::state::RuntimeState;
use crate::tui::logo::{compact_logo_lines, full_logo_lines};

/// Header metrics shown next to status for a grounded-at-a-glance view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderMetrics {
    pub records: usize,
    pub scrapers: usize,
}

/// Render a plain-text header with branding, health, and grounded metrics.
pub fn render_header(
    runtime: &RuntimeState,
    health: &HealthStatus,
    metrics: HeaderMetrics,
    width: u16,
) -> Vec<String> {
    let mut lines: Vec<String> = if width >= 72 {
        full_logo_lines(runtime.logo_tick)
    } else {
        compact_logo_lines()
    };

    lines.insert(0, "Welcome back.".to_string());

    let now_epoch_secs = health.last_checked_epoch_secs.saturating_add(1);
    let view = runtime.header_health_view(now_epoch_secs);
    let stale_tag = if view.stale { "stale" } else { "fresh" };
    lines.push(format!(
        "status: {} ({}) [{}]",
        view.label, view.details, stale_tag
    ));
    lines.push(format!(
        "grounded: records={}, scrapers={}",
        metrics.records, metrics.scrapers
    ));
    lines
}
