/// Progress events emitted while processing a scrape job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrapeEvent {
    Discovered { source_url: String },
    Processed { product_code: String },
    Failed { source_url: String, reason: String },
}
