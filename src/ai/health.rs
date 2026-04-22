use crate::ai::knowledge_base::OllamaHealthState;
use crate::app::error::{AppError, AppResult};

pub const DEFAULT_HEALTH_STALE_SECS: u64 = 30;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthStatus {
    pub state: OllamaHealthState,
    pub missing_models: Vec<String>,
    pub last_error: Option<String>,
    pub last_checked_epoch_secs: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HealthCheckPolicy {
    pub interval_secs: u64,
    pub stale_after_secs: u64,
}

impl Default for HealthCheckPolicy {
    fn default() -> Self {
        Self {
            interval_secs: 30,
            stale_after_secs: DEFAULT_HEALTH_STALE_SECS,
        }
    }
}

/// Determine whether a command should fail fast for the current health state.
pub fn command_allowed(state: &OllamaHealthState) -> bool {
    matches!(state, OllamaHealthState::Healthy)
}

/// Determine whether the cached health snapshot is stale.
pub fn is_stale(health: &HealthStatus, now_epoch_secs: u64, stale_after_secs: u64) -> bool {
    now_epoch_secs.saturating_sub(health.last_checked_epoch_secs) > stale_after_secs
}

/// Determine whether a periodic health check should run now.
pub fn should_run_periodic_check(
    health: &HealthStatus,
    now_epoch_secs: u64,
    policy: HealthCheckPolicy,
) -> bool {
    now_epoch_secs.saturating_sub(health.last_checked_epoch_secs) >= policy.interval_secs
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
