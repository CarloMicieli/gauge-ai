/// Steam-locomotive art for wider terminal headers.
pub fn full_logo_lines(tick: usize) -> Vec<String> {
    let wheels = if tick.is_multiple_of(2) {
        "(o)====(o)"
    } else {
        "(O)====(O)"
    };

    vec![
        "        ~~      ~~~".to_string(),
        "       (__)    (___)".to_string(),
        "   _____||______".to_string(),
        " _| RED STEAM  |___  ____".to_string(),
        "|_|[] [] [] []|___ ||____\\____".to_string(),
        "  O==========O      ||  __    \\\\>>>".to_string(),
        format!("     {wheels}      ||_|  |____/"),
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
