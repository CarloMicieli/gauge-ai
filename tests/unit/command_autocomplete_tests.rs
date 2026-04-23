use gauge_ai::app::commands::{AVAILABLE_COMMANDS, top_command_suggestion};

#[test]
fn top_suggestion_returns_none_for_empty_input() {
    assert_eq!(top_command_suggestion(""), None);
}

#[test]
fn top_suggestion_returns_exact_match() {
    assert_eq!(top_command_suggestion("/help"), Some("/help"));
}

#[test]
fn top_suggestion_returns_first_prefix_match() {
    assert_eq!(top_command_suggestion("/ex"), Some("/export"));
}

#[test]
fn top_suggestion_returns_none_for_unknown_prefix() {
    assert_eq!(top_command_suggestion("/does-not-exist"), None);
}

#[test]
fn command_catalog_contains_quit_and_exit_aliases() {
    assert!(AVAILABLE_COMMANDS.contains(&"/quit"));
    assert!(AVAILABLE_COMMANDS.contains(&"/exit"));
}
