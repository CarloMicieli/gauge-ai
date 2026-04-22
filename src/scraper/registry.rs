use crate::scraper::traits::ModelScraper;

pub struct ScraperRegistry {
    scrapers: Vec<Box<dyn ModelScraper>>,
}

impl ScraperRegistry {
    pub fn new() -> Self {
        Self {
            scrapers: Vec::new(),
        }
    }

    pub fn register(&mut self, scraper: Box<dyn ModelScraper>) {
        self.scrapers.push(scraper);
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.scrapers.iter().map(|s| s.name()).collect()
    }

    pub fn latest_capable_names(&self) -> Vec<&'static str> {
        self.scrapers
            .iter()
            .filter(|s| s.supports_latest())
            .map(|s| s.name())
            .collect()
    }
}

impl Default for ScraperRegistry {
    fn default() -> Self {
        Self::new()
    }
}
