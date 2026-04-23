use gauge_ai::app::commands::{Command, CommandError, parse};

#[test]
fn parses_command_grammar_and_aliases() {
    assert_eq!(parse("/help"), Ok(Command::Help));
    assert_eq!(parse("/list-scraper"), Ok(Command::ListScraper));
    assert_eq!(
        parse("/scrape roco BR 50"),
        Ok(Command::Scrape {
            manufacturer: "roco".to_string(),
            query: "BR 50".to_string(),
        })
    );
    assert_eq!(
        parse("/latest roco"),
        Ok(Command::Latest {
            scraper: Some("roco".to_string())
        })
    );
    assert_eq!(
        parse("/query find krokodil"),
        Ok(Command::Query {
            text: "find krokodil".to_string(),
        })
    );
    assert_eq!(
        parse("/export db cargo"),
        Ok(Command::Export {
            query: "db cargo".to_string(),
        })
    );
    assert_eq!(parse("/setup"), Ok(Command::Setup));
    assert_eq!(parse("/clear"), Ok(Command::Clear));
    assert_eq!(parse("/quit"), Ok(Command::Quit));
    assert_eq!(parse("/exit"), Ok(Command::Quit));
}

#[test]
fn rejects_unknown_or_missing_arguments() {
    assert_eq!(parse(""), Err(CommandError::Empty));
    assert_eq!(parse("/unknown"), Err(CommandError::Unknown));
    assert_eq!(
        parse("/scrape roco"),
        Err(CommandError::MissingArgs("query"))
    );
    assert_eq!(
        parse("/query"),
        Err(CommandError::MissingArgs("query text"))
    );
    assert_eq!(
        parse("/export"),
        Err(CommandError::MissingArgs("export query"))
    );
}
