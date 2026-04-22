use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

const PROMPT_CONTEXT_CACHE_LIMIT: usize = 128;
static PROMPT_CONTEXT_CACHE: OnceLock<Mutex<BTreeMap<String, String>>> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeBase {
    pub version: String,
    pub scales: BTreeMap<String, Vec<String>>,
    pub epochs: BTreeMap<String, Vec<String>>,
    pub power_systems: BTreeMap<String, Vec<String>>,
    pub interfaces: BTreeMap<String, Vec<String>>,
    pub couplers: BTreeMap<String, Vec<String>>,
    pub manufacturer_aliases: BTreeMap<String, Vec<String>>,
    pub translation_glossary: BTreeMap<String, String>,
    pub prototypes: BTreeMap<String, Vec<String>>,
    pub prototype_mappings: BTreeMap<String, Vec<String>>,
    pub liveries: BTreeMap<String, Vec<String>>,
}

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self {
            version: "1".to_string(),
            scales: BTreeMap::new(),
            epochs: BTreeMap::new(),
            power_systems: BTreeMap::new(),
            interfaces: BTreeMap::new(),
            couplers: BTreeMap::new(),
            manufacturer_aliases: BTreeMap::new(),
            translation_glossary: BTreeMap::new(),
            prototypes: BTreeMap::new(),
            prototype_mappings: BTreeMap::new(),
            liveries: BTreeMap::new(),
        }
    }
}

impl KnowledgeBase {
    /// Canonicalize free-text query terms with glossary aliases for better matching.
    pub fn canonicalize_query(&self, text: &str) -> String {
        text.split_whitespace()
            .map(|token| {
                let lowered = token.to_lowercase();
                self.translation_glossary
                    .iter()
                    .find(|(alias, _)| alias.eq_ignore_ascii_case(&lowered))
                    .map(|(_, canonical)| canonical.to_lowercase())
                    .unwrap_or(lowered)
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Build a bounded prompt context payload for query/normalization prompts.
    pub fn filtered_prompt_context(&self, text: &str, max_entries: usize) -> String {
        let key = format!(
            "{}|{}|{}",
            self.version,
            max_entries,
            self.canonicalize_query(text)
        );

        if let Some(cached) = read_prompt_context_cache(&key) {
            return cached;
        }

        let prototype_hits = self.matching_prototypes(text);
        let livery_hits = self.matching_liveries(text);
        let manufacturer_hits = self.matching_manufacturers(text);

        let mut lines = Vec::new();
        if !prototype_hits.is_empty() {
            lines.push(format!(
                "prototypes: {}",
                prototype_hits
                    .into_iter()
                    .take(max_entries)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !livery_hits.is_empty() {
            lines.push(format!(
                "liveries: {}",
                livery_hits
                    .into_iter()
                    .take(max_entries)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !manufacturer_hits.is_empty() {
            lines.push(format!(
                "manufacturers: {}",
                manufacturer_hits
                    .into_iter()
                    .take(max_entries)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        if lines.is_empty() {
            lines.push("no contextual aliases matched".to_string());
        }

        let context = lines.join("\n");
        write_prompt_context_cache(&key, &context);
        context
    }

    /// Return canonical prototype names that match aliases in the query text.
    pub fn matching_prototypes(&self, text: &str) -> Vec<String> {
        let lowered = self.canonicalize_query(text);
        self.prototypes
            .iter()
            .filter(|(_, aliases)| {
                aliases
                    .iter()
                    .any(|alias| lowered.contains(&alias.to_lowercase()))
            })
            .map(|(canonical, _)| canonical.clone())
            .collect()
    }

    /// Return canonical livery names that match aliases in the query text.
    pub fn matching_liveries(&self, text: &str) -> Vec<String> {
        let lowered = self.canonicalize_query(text);
        self.liveries
            .iter()
            .filter(|(_, aliases)| {
                aliases
                    .iter()
                    .any(|alias| lowered.contains(&alias.to_lowercase()))
            })
            .map(|(canonical, _)| canonical.clone())
            .collect()
    }

    fn matching_manufacturers(&self, text: &str) -> Vec<String> {
        let lowered = self.canonicalize_query(text);
        self.manufacturer_aliases
            .iter()
            .filter(|(_, aliases)| {
                aliases
                    .iter()
                    .any(|alias| lowered.contains(&alias.to_lowercase()))
            })
            .map(|(canonical, _)| canonical.clone())
            .collect()
    }
}

fn read_prompt_context_cache(key: &str) -> Option<String> {
    let cache = PROMPT_CONTEXT_CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    match cache.lock() {
        Ok(guard) => guard.get(key).cloned(),
        Err(_) => None,
    }
}

fn write_prompt_context_cache(key: &str, context: &str) {
    let cache = PROMPT_CONTEXT_CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    if let Ok(mut guard) = cache.lock() {
        guard.insert(key.to_string(), context.to_string());
        while guard.len() > PROMPT_CONTEXT_CACHE_LIMIT {
            if let Some(first_key) = guard.keys().next().cloned() {
                guard.remove(&first_key);
            } else {
                break;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OllamaHealthState {
    Checking,
    Healthy,
    Disconnected,
    ModelMissing,
}
