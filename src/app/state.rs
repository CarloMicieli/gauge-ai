use crate::ai::health::{HealthCheckPolicy, HealthStatus, is_stale, should_run_periodic_check};
use crate::ai::knowledge_base::OllamaHealthState;

/// Summary counters for a completed scrape job.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScrapeSummary {
    pub discovered: usize,
    pub processed: usize,
    pub failed: usize,
    pub new_records: usize,
    pub updated_records: usize,
}

/// Final state returned from a scrape run.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScrapeRun {
    pub summary: ScrapeSummary,
    pub events: Vec<crate::app::events::ScrapeEvent>,
}

/// Query result view model for command output rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryResultView {
    pub answer: String,
    pub result_count: usize,
    pub latency_ms: i64,
    pub hints: Vec<String>,
    pub error: Option<String>,
}

/// Summary counters for latest-sync jobs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LatestSummary {
    pub processed: usize,
    pub inserted: usize,
    pub updated: usize,
    pub skipped_scrapers: usize,
    pub failed_scrapers: usize,
}

/// Final state returned from a latest-sync run.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LatestRun {
    pub summary: LatestSummary,
    pub messages: Vec<String>,
}

/// Export result view model for command output rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportResultView {
    pub output_path: String,
    pub records: usize,
    pub images: usize,
    pub missing_images: usize,
}

/// Header view model for current Ollama health state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderHealthView {
    pub label: String,
    pub details: String,
    pub stale: bool,
}

/// Runtime state used by periodic health checks and optional logo animation ticks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeState {
    pub health: HealthStatus,
    pub check_policy: HealthCheckPolicy,
    pub logo_tick: usize,
}

impl RuntimeState {
    /// Build default runtime state from initial health.
    pub fn new(health: HealthStatus) -> Self {
        Self {
            health,
            check_policy: HealthCheckPolicy::default(),
            logo_tick: 0,
        }
    }

    /// Determine if a periodic health check should execute at this moment.
    pub fn should_check_health(&self, now_epoch_secs: u64) -> bool {
        should_run_periodic_check(&self.health, now_epoch_secs, self.check_policy)
    }

    /// Replace health status from the latest checker probe.
    pub fn update_health(&mut self, health: HealthStatus) {
        self.health = health;
    }

    /// Advance optional logo animation wheel by one frame.
    pub fn tick_logo(&mut self) {
        self.logo_tick = self.logo_tick.wrapping_add(1);
    }

    /// Build a human-friendly header snapshot from current health.
    pub fn header_health_view(&self, now_epoch_secs: u64) -> HeaderHealthView {
        let stale = is_stale(
            &self.health,
            now_epoch_secs,
            self.check_policy.stale_after_secs,
        );
        let label = match self.health.state {
            OllamaHealthState::Checking => "checking",
            OllamaHealthState::Healthy => "healthy",
            OllamaHealthState::Disconnected => "disconnected",
            OllamaHealthState::ModelMissing => "model-missing",
        }
        .to_string();

        let details = match self.health.state {
            OllamaHealthState::ModelMissing if !self.health.missing_models.is_empty() => {
                format!("missing {}", self.health.missing_models.join(", "))
            }
            OllamaHealthState::Disconnected => self
                .health
                .last_error
                .clone()
                .unwrap_or_else(|| "connection unavailable".to_string()),
            _ => "ready".to_string(),
        };

        HeaderHealthView {
            label,
            details,
            stale,
        }
    }
}

impl QueryResultView {
    /// Construct a successful query result.
    pub fn success(answer: String, result_count: usize, latency_ms: i64) -> Self {
        Self {
            answer,
            result_count,
            latency_ms,
            hints: Vec::new(),
            error: None,
        }
    }

    /// Construct a no-match query result with guidance hints.
    pub fn no_match(answer: String, latency_ms: i64, hints: Vec<String>) -> Self {
        Self {
            answer,
            result_count: 0,
            latency_ms,
            hints,
            error: None,
        }
    }

    /// Construct an error query result.
    pub fn error(message: String) -> Self {
        Self {
            answer: String::new(),
            result_count: 0,
            latency_ms: 0,
            hints: Vec::new(),
            error: Some(message),
        }
    }
}
