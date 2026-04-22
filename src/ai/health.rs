use crate::ai::knowledge_base::OllamaHealthState;
use crate::app::error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthStatus {
    pub state: OllamaHealthState,
    pub missing_models: Vec<String>,
    pub last_error: Option<String>,
    pub last_checked_epoch_secs: u64,
}

/// Determine whether a command should fail fast for the current health state.
pub fn command_allowed(state: &OllamaHealthState) -> bool {
    matches!(state, OllamaHealthState::Healthy)
}

/// Validate health state before executing an AI-dependent command.
pub fn validate_health_for(command: &str, health: &HealthStatus) -> AppResult<()> {
    if command_allowed(&health.state) {
        return Ok(());
    }

    let details = match (&health.state, &health.last_error) {
        (OllamaHealthState::ModelMissing, _) if !health.missing_models.is_empty() => {
            format!("missing models: {}", health.missing_models.join(", "))
        }
        (_, Some(error)) => error.clone(),
        _ => "health check failed".to_string(),
    };

    Err(AppError::Operation(format!(
        "Ollama unavailable for {command}: {details}"
    )))
}
