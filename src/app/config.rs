use std::path::PathBuf;

use config::{Config, Environment, File};
use directories::ProjectDirs;
use serde::Deserialize;

use crate::app::error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub ollama_base_url: String,
    pub chat_model: String,
    pub embedding_model: String,
    pub preferred_language: String,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub db_path: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct ConfigFile {
    ollama_base_url: Option<String>,
    chat_model: Option<String>,
    embedding_model: Option<String>,
    preferred_language: Option<String>,
}

impl AppConfig {
    /// Load app configuration from defaults, optional TOML file, and env overrides.
    pub fn load() -> AppResult<Self> {
        let dirs = ProjectDirs::from("ai", "gauge", "gauge-ai")
            .ok_or_else(|| AppError::Config("unable to resolve project directories".to_string()))?;

        let config_file = dirs.config_dir().join("config.toml");
        let settings = Config::builder()
            .add_source(File::from(config_file).required(false))
            .add_source(Environment::with_prefix("GAUGE_AI").separator("__"))
            .build()?;

        let parsed = settings
            .try_deserialize::<ConfigFile>()
            .unwrap_or(ConfigFile {
                ollama_base_url: None,
                chat_model: None,
                embedding_model: None,
                preferred_language: None,
            });

        let data_dir = dirs.data_dir().to_path_buf();
        let cache_dir = data_dir.join("cache");
        let db_path = data_dir.join("trains.db");

        Ok(Self {
            ollama_base_url: parsed
                .ollama_base_url
                .unwrap_or_else(|| "http://localhost:11434".to_string()),
            chat_model: parsed
                .chat_model
                .unwrap_or_else(|| "llama3.1:8b".to_string()),
            embedding_model: parsed
                .embedding_model
                .unwrap_or_else(|| "nomic-embed-text".to_string()),
            preferred_language: parsed
                .preferred_language
                .unwrap_or_else(|| "en".to_string()),
            data_dir,
            cache_dir,
            db_path,
        })
    }
}
