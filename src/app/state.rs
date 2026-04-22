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
