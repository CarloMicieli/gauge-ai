/// Full pixel-locomotive logo for wider terminal headers.
pub fn full_logo_lines(tick: usize) -> Vec<String> {
    let wheel_left = if tick.is_multiple_of(2) {
        "▀▀  ▀▀"
    } else {
        " ▀▀  ▀▀"
    };
    let wheel_right = if tick.is_multiple_of(2) {
        "▀▀        ▀▀"
    } else {
        " ▀▀      ▀▀"
    };

    vec![
        "       ██████   ".to_string(),
        "      ████████  _________________________".to_string(),
        "      ██    ██ |                         |".to_string(),
        "    ██████████ |        GAUGE.AI         |".to_string(),
        "    ██████████ |_________________________|".to_string(),
        "    █████████████   ██            ██".to_string(),
        "     ███████████    ██            ██".to_string(),
        "      █████████      ████      ████".to_string(),
        format!("       {wheel_left}         {wheel_right}"),
    ]
}

/// Compact Gauge.ai logo for narrow terminal widths.
pub fn compact_logo_lines() -> Vec<String> {
    vec!["Gauge.ai".to_string(), "pixel-loco mode".to_string()]
}
