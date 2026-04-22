use crate::ai::health::HealthStatus;
use crate::ai::knowledge_base::OllamaHealthState;
use crate::app::error::{AppError, AppResult};

/// Lightweight Ollama client settings used by command handlers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OllamaClientConfig {
    pub base_url: String,
    pub chat_model: String,
    pub embedding_model: String,
}

impl OllamaClientConfig {
    pub fn new(base_url: String, chat_model: String, embedding_model: String) -> Self {
        Self {
            base_url,
            chat_model,
            embedding_model,
        }
    }
}

/// Embedding client abstraction used by semantic query workflows.
pub trait EmbeddingClient: Send + Sync {
    /// Create a lightweight token embedding representation for query/search matching.
    fn embed(&self, text: &str) -> Vec<String>;
}

/// Local deterministic embedding implementation for early feature stages.
pub struct LocalEmbeddingClient;

impl EmbeddingClient for LocalEmbeddingClient {
    fn embed(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|token| !token.is_empty())
            .map(ToString::to_string)
            .collect()
    }
}

/// Summary returned by setup diagnostics and pull orchestration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupSummary {
    pub missing_models: Vec<String>,
    pub pulled_models: Vec<String>,
    pub confirmation_required: bool,
}

impl SetupSummary {
    /// Returns true when setup found all required models ready.
    pub fn all_ready(&self) -> bool {
        self.missing_models.is_empty() || self.missing_models == self.pulled_models
    }
}

/// Determine setup actions from the latest health snapshot.
pub fn run_setup(health: &HealthStatus, auto_confirm_pull: bool) -> AppResult<SetupSummary> {
    match health.state {
        OllamaHealthState::Disconnected => {
            let details = health
                .last_error
                .clone()
                .unwrap_or_else(|| "connection failed".to_string());
            Err(AppError::Operation(format!(
                "Ollama unavailable for /setup: {details}"
            )))
        }
        OllamaHealthState::ModelMissing => {
            let missing_models = health.missing_models.clone();
            let pulled_models = if auto_confirm_pull {
                missing_models.clone()
            } else {
                Vec::new()
            };
            Ok(SetupSummary {
                missing_models,
                pulled_models,
                confirmation_required: !auto_confirm_pull,
            })
        }
        _ => Ok(SetupSummary {
            missing_models: Vec::new(),
            pulled_models: Vec::new(),
            confirmation_required: false,
        }),
    }
}
