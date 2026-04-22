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
