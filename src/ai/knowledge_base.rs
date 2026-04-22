use std::collections::BTreeMap;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OllamaHealthState {
    Checking,
    Healthy,
    Disconnected,
    ModelMissing,
}
