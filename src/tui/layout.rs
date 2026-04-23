use crate::ai::health::HealthStatus;
use crate::app::state::RuntimeState;
use crate::tui::logo::{compact_logo_lines, full_logo_lines, gauge_banner_lines};

/// Header metrics shown next to status for a grounded-at-a-glance view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderMetrics {
    pub records: usize,
    pub scrapers: usize,
}

/// Pre-formatted content for the split Gauge.ai header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderSections {
    pub left_ascii: Vec<String>,
    pub right_banner: Vec<String>,
    pub checking_line: String,
    pub grounded_line: String,
}

/// Render structured header sections for nested top-block layout.
pub fn render_header_sections(
    runtime: &RuntimeState,
    health: &HealthStatus,
    metrics: HeaderMetrics,
    width: u16,
) -> HeaderSections {
    let left_ascii: Vec<String> = if width >= 72 {
        full_logo_lines(runtime.logo_tick)
    } else {
        compact_logo_lines()
    };

    let right_banner = if width >= 72 {
        gauge_banner_lines()
    } else {
        vec!["GAUGE.AI".to_string()]
    };

    let now_epoch_secs = health.last_checked_epoch_secs.saturating_add(1);
    let view = runtime.header_health_view(now_epoch_secs);
    let stale_tag = if view.stale { "stale" } else { "fresh" };

    HeaderSections {
        left_ascii,
        right_banner,
        checking_line: format!("{} ({}) [{}]", view.label, view.details, stale_tag),
        grounded_line: format!(
            "grounded: records={}, scrapers={}",
            metrics.records, metrics.scrapers
        ),
    }
}

/// Render a plain-text header with branding, health, and grounded metrics.
pub fn render_header(
    runtime: &RuntimeState,
    health: &HealthStatus,
    metrics: HeaderMetrics,
    width: u16,
) -> Vec<String> {
    let sections = render_header_sections(runtime, health, metrics, width);
    let mut lines = sections.left_ascii;
    lines.extend(sections.right_banner);
    lines.push(format!("✦ {}", sections.checking_line));
    lines.push(format!("▦ {}", sections.grounded_line));
    lines
}
