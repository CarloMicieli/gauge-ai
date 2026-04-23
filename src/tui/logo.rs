use ratatui::style::Style;
use ratatui::text::{Line, Span};

/// Steam-locomotive ASCII art for wider terminal headers.
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

/// Styled pixel sprite logo used in the left header panel.
pub fn pixel_sprite_lines(red: Style, white: Style) -> Vec<Line<'static>> {
    vec![
        Line::from(Span::styled("   ███████", red)),
        Line::from(Span::styled(" ███████████", red)),
        Line::from(vec![
            Span::styled(" █  ", red),
            Span::styled("●   ●", white),
            Span::styled("  █", red),
        ]),
        Line::from(Span::styled("█████████████", red)),
        Line::from(vec![
            Span::styled(" █  ", red),
            Span::styled("▄▄▄▄▄", red),
            Span::styled("  █", red),
        ]),
        Line::from(Span::styled("  ██     ██", red)),
    ]
}

/// ASCII brand banner shown in the right-side status panel.
pub fn gauge_banner_lines() -> Vec<String> {
    vec![
        "  ____    _    _   _  ____ _____   _    ___ ".to_string(),
        " / ___|  / \\  | | | |/ ___| ____| / \\  |_ _|".to_string(),
        "| |  _  / _ \\ | | | | |  _|  _|  / _ \\  | | ".to_string(),
        "| |_| |/ ___ \\| |_| | |_| | |___/ ___ \\ | | ".to_string(),
        " \\____/_/   \\_\\\\___/ \\____|_____/_/   \\_\\___|".to_string(),
    ]
}

/// Compact Gauge.ai logo for narrow terminal widths.
pub fn compact_logo_lines() -> Vec<String> {
    vec!["Gauge.ai".to_string(), "pixel-loco mode".to_string()]
}
