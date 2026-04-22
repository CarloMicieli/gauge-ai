/// Core scraper contract for manufacturer-specific integrations.
pub trait ModelScraper: Send + Sync {
    fn name(&self) -> &'static str;
    fn supports_latest(&self) -> bool {
        false
    }
}
