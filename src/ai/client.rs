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
