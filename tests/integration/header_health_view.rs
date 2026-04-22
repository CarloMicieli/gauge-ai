use gauge_ai::ai::health::HealthStatus;
use gauge_ai::ai::knowledge_base::OllamaHealthState;
use gauge_ai::app::state::RuntimeState;
use gauge_ai::tui::widgets::render_header_status;

#[test]
fn header_renders_health_indicator_states() {
    let base = HealthStatus {
        state: OllamaHealthState::Checking,
        missing_models: vec![],
        last_error: None,
        last_checked_epoch_secs: 10,
    };

    let mut runtime = RuntimeState::new(base.clone());
    let checking = render_header_status(&runtime, &base, 12, 2, 120).join("\n");
    assert!(checking.contains("status: checking"));

    runtime.update_health(HealthStatus {
        state: OllamaHealthState::Healthy,
        missing_models: vec![],
        last_error: None,
        last_checked_epoch_secs: 11,
    });
    runtime.tick_logo();
    let healthy = render_header_status(&runtime, &runtime.health, 20, 2, 120).join("\n");
    assert!(healthy.contains("status: healthy"));

    runtime.update_health(HealthStatus {
        state: OllamaHealthState::Disconnected,
        missing_models: vec![],
        last_error: Some("connection refused".to_string()),
        last_checked_epoch_secs: 12,
    });
    let disconnected = render_header_status(&runtime, &runtime.health, 20, 2, 50).join("\n");
    assert!(disconnected.contains("status: disconnected"));
    assert!(disconnected.contains("Gauge.ai"));

    runtime.update_health(HealthStatus {
        state: OllamaHealthState::ModelMissing,
        missing_models: vec!["llama3.1:8b".to_string()],
        last_error: None,
        last_checked_epoch_secs: 13,
    });
    let model_missing = render_header_status(&runtime, &runtime.health, 20, 2, 120).join("\n");
    assert!(model_missing.contains("status: model-missing"));
    assert!(model_missing.contains("missing llama3.1:8b"));
}
