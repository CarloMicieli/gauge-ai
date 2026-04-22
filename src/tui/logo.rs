/// Full Gauge.ai ASCII logo for wider terminal headers.
pub fn full_logo_lines() -> Vec<&'static str> {
    vec![
        "   ____                     ___    _ ",
        "  / ___| __ _ _   _  __ _  / _ | _(_)",
        " | |  _ / _` | | | |/ _` || | | || |",
        " | |_| | (_| | |_| | (_| || |_| || |",
        "  |____|__,_|__,_|__,_| |___(_)_|",
    ]
}

/// Compact Gauge.ai logo for narrow terminal widths.
pub fn compact_logo_lines() -> Vec<&'static str> {
    vec!["Gauge.ai", "local rail intelligence"]
}

/// Optional spinner frame for wheel animation without blocking input.
pub fn wheel_frame(tick: usize) -> &'static str {
    const FRAMES: [&str; 4] = ["|", "/", "-", "\\"];
    FRAMES[tick % FRAMES.len()]
}
