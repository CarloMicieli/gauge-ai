use std::path::Path;

use sqlx::SqlitePool;

use crate::ai::knowledge_base::KnowledgeBase;
use crate::ai::normalize::Normalizer;
use crate::app::error::{AppError, AppResult};
use crate::app::ingest::run_scrape;
use crate::app::state::ScrapeRun;
use crate::scraper::registry::ScraperRegistry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Help,
    ListScraper,
    Scrape { manufacturer: String, query: String },
    Latest { scraper: Option<String> },
    Query { text: String },
    Export { query: String },
    Setup,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    Empty,
    Unknown,
    MissingArgs(&'static str),
}

/// Dependencies required to execute supported commands.
pub struct CommandContext<'a> {
    pub registry: &'a ScraperRegistry,
    pub normalizer: &'a dyn Normalizer,
    pub knowledge_base: &'a KnowledgeBase,
    pub pool: &'a SqlitePool,
    pub cache_root: &'a Path,
}

/// Result of executing one command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandOutcome {
    Scrape(ScrapeRun),
    Message(String),
}

/// Parse slash commands into strongly typed app commands.
pub fn parse(input: &str) -> Result<Command, CommandError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CommandError::Empty);
    }

    let mut parts = trimmed.split_whitespace();
    let cmd = parts.next().ok_or(CommandError::Empty)?;

    match cmd {
        "/help" => Ok(Command::Help),
        "/list-scraper" => Ok(Command::ListScraper),
        "/scrape" => {
            let manufacturer = parts
                .next()
                .ok_or(CommandError::MissingArgs("manufacturer and query"))?
                .to_string();
            let query = parts.collect::<Vec<_>>().join(" ");
            if query.is_empty() {
                return Err(CommandError::MissingArgs("query"));
            }
            Ok(Command::Scrape {
                manufacturer,
                query,
            })
        }
        "/latest" => Ok(Command::Latest {
            scraper: parts.next().map(ToString::to_string),
        }),
        "/query" => {
            let text = parts.collect::<Vec<_>>().join(" ");
            if text.is_empty() {
                return Err(CommandError::MissingArgs("query text"));
            }
            Ok(Command::Query { text })
        }
        "/export" => {
            let query = parts.collect::<Vec<_>>().join(" ");
            if query.is_empty() {
                return Err(CommandError::MissingArgs("export query"));
            }
            Ok(Command::Export { query })
        }
        "/setup" => Ok(Command::Setup),
        "/quit" | "/exit" => Ok(Command::Quit),
        _ => Err(CommandError::Unknown),
    }
}

/// Execute a parsed command for the currently supported bootstrap flows.
pub async fn execute(command: Command, context: &CommandContext<'_>) -> AppResult<CommandOutcome> {
    match command {
        Command::Scrape {
            manufacturer,
            query,
        } => {
            let run = run_scrape(
                context.registry,
                &manufacturer,
                &query,
                context.normalizer,
                context.knowledge_base,
                context.pool,
                context.cache_root,
            )
            .await?;
            Ok(CommandOutcome::Scrape(run))
        }
        other => Err(AppError::Operation(format!(
            "command not implemented yet: {other:?}"
        ))),
    }
}
