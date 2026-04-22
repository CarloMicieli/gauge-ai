use gauge_ai::ai::health::{
    HealthCheckPolicy, HealthStatus, command_allowed, is_stale, should_run_periodic_check,
    validate_health_for,
};
use gauge_ai::ai::knowledge_base::OllamaHealthState;

#[test]
fn command_allowed_only_for_healthy_state() {
    assert!(command_allowed(&OllamaHealthState::Healthy));
    assert!(!command_allowed(&OllamaHealthState::Checking));
    assert!(!command_allowed(&OllamaHealthState::Disconnected));
    assert!(!command_allowed(&OllamaHealthState::ModelMissing));
}

#[test]
fn stale_and_periodic_policy_transitions_are_reported_correctly() {
    let health = HealthStatus {
        state: OllamaHealthState::Healthy,
        missing_models: vec![],
        last_error: None,
        last_checked_epoch_secs: 100,
    };
    let policy = HealthCheckPolicy {
        interval_secs: 30,
        stale_after_secs: 30,
    };

    assert!(!is_stale(&health, 130, policy.stale_after_secs));
    assert!(is_stale(&health, 131, policy.stale_after_secs));
    assert!(should_run_periodic_check(&health, 130, policy));
    assert!(!should_run_periodic_check(&health, 129, policy));
}

#[test]
fn validate_health_for_returns_human_readable_errors() {
    let disconnected = HealthStatus {
        state: OllamaHealthState::Disconnected,
        missing_models: vec![],
        last_error: Some("connection refused".to_string()),
        last_checked_epoch_secs: 0,
    };
    let error = validate_health_for("/query", &disconnected).expect_err("must fail");
    assert!(error.to_string().contains("Ollama unavailable for /query"));
    assert!(error.to_string().contains("connection refused"));

    let model_missing = HealthStatus {
        state: OllamaHealthState::ModelMissing,
        missing_models: vec!["llama3.1:8b".to_string()],
        last_error: None,
        last_checked_epoch_secs: 0,
    };
    let error = validate_health_for("/latest", &model_missing).expect_err("must fail");
    assert!(error.to_string().contains("missing models: llama3.1:8b"));
}
