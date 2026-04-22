use crate::ai::knowledge_base::OllamaHealthState;

/// Determine whether a command should fail fast for the current health state.
pub fn command_allowed(state: &OllamaHealthState) -> bool {
    matches!(state, OllamaHealthState::Healthy)
}
