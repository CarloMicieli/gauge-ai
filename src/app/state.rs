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
