use crate::scraper::traits::ModelScraper;

pub struct ScraperRegistry {
    scrapers: Vec<Box<dyn ModelScraper>>,
}

impl ScraperRegistry {
    /// Create an empty scraper registry.
    pub fn new() -> Self {
        Self {
            scrapers: Vec::new(),
        }
    }

    /// Register a scraper implementation.
    pub fn register(&mut self, scraper: Box<dyn ModelScraper>) {
        self.scrapers.push(scraper);
    }

    /// Return registered scraper names.
    pub fn names(&self) -> Vec<&'static str> {
        self.scrapers.iter().map(|s| s.name()).collect()
    }

    /// Return scraper names that support latest-arrivals sync.
    pub fn latest_capable_names(&self) -> Vec<&'static str> {
        self.scrapers
            .iter()
            .filter(|s| s.supports_latest())
            .map(|s| s.name())
            .collect()
    }

    /// Retrieve one registered scraper by case-insensitive name.
    pub fn get(&self, name: &str) -> Option<&dyn ModelScraper> {
        self.scrapers
            .iter()
            .find(|scraper| scraper.name().eq_ignore_ascii_case(name))
            .map(|scraper| scraper.as_ref())
    }
}

impl Default for ScraperRegistry {
    fn default() -> Self {
        Self::new()
    }
}
