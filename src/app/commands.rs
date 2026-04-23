use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::SqlitePool;

use crate::ai::client::EmbeddingClient;
use crate::ai::client::run_setup;
use crate::ai::health::{HealthStatus, validate_health_for};
use crate::ai::knowledge_base::KnowledgeBase;
use crate::ai::normalize::Normalizer;
use crate::ai::query::execute_query;
use crate::app::error::AppResult;
use crate::app::ingest::run_scrape;
use crate::app::jobs::run_latest;
use crate::app::perf::log_command_timing;
use crate::app::state::{ExportResultView, LatestRun, QueryResultView, ScrapeRun};
use crate::export::export_records;
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
    Clear,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    Empty,
    Unknown,
    MissingArgs(&'static str),
}

/// Supported slash commands for command-line autocomplete in the TUI.
pub const AVAILABLE_COMMANDS: [&str; 10] = [
    "/help",
    "/list-scraper",
    "/scrape",
    "/latest",
    "/query",
    "/export",
    "/setup",
    "/clear",
    "/quit",
    "/exit",
];

/// Convert command parser failures into user-readable messages with recovery hints.
pub fn command_error_message(error: &CommandError) -> String {
    match error {
        CommandError::Empty => {
            "No command entered. Type /help to see available commands.".to_string()
        }
        CommandError::Unknown => {
            "Unknown command. Type /help to see valid slash commands.".to_string()
        }
        CommandError::MissingArgs(missing) => {
            format!("Missing arguments ({missing}). Example: /scrape roco BR 50")
        }
    }
}

/// Dependencies required to execute supported commands.
pub struct CommandContext<'a> {
    pub registry: &'a ScraperRegistry,
    pub normalizer: &'a dyn Normalizer,
    pub knowledge_base: &'a KnowledgeBase,
    pub pool: &'a SqlitePool,
    pub cache_root: &'a Path,
    pub health: &'a HealthStatus,
    pub embedding_client: &'a dyn EmbeddingClient,
}

/// Result of executing one command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandOutcome {
    Scrape(ScrapeRun),
    Latest(LatestRun),
    Query(QueryResultView),
    Export(ExportResultView),
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
        "/clear" => Ok(Command::Clear),
        "/quit" | "/exit" => Ok(Command::Quit),
        _ => Err(CommandError::Unknown),
    }
}

/// Return the first command that starts with the current input text.
pub fn top_command_suggestion(input: &str) -> Option<&'static str> {
    if input.is_empty() {
        return None;
    }

    AVAILABLE_COMMANDS
        .iter()
        .copied()
        .find(|command| command.starts_with(input))
}

/// Execute a parsed command for the currently supported bootstrap flows.
pub async fn execute(command: Command, context: &CommandContext<'_>) -> AppResult<CommandOutcome> {
    let command_name = command_name(&command);
    let started_at = std::time::Instant::now();

    let result = match command {
        Command::Scrape {
            manufacturer,
            query,
        } => {
            validate_health_for("/scrape", context.health)?;
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
        Command::Query { text } => {
            validate_health_for("/query", context.health)?;
            let query_result = execute_query(
                context.pool,
                context.knowledge_base,
                context.embedding_client,
                &text,
                5,
            )
            .await?;
            Ok(CommandOutcome::Query(query_result))
        }
        Command::Latest { scraper } => {
            validate_health_for("/latest", context.health)?;
            let latest = run_latest(
                context.registry,
                scraper.as_deref(),
                context.normalizer,
                context.knowledge_base,
                context.pool,
                context.cache_root,
            )
            .await?;
            Ok(CommandOutcome::Latest(latest))
        }
        Command::Help => Ok(CommandOutcome::Message(help_message())),
        Command::ListScraper => Ok(CommandOutcome::Message(list_scrapers_message(context))),
        Command::Export { query } => {
            let export_dir = build_export_dir(context.cache_root)?;
            let exported = export_records(context.pool, &query, &export_dir).await?;
            Ok(CommandOutcome::Export(ExportResultView {
                output_path: exported.output_path,
                records: exported.records,
                images: exported.images,
                missing_images: exported.missing_images,
            }))
        }
        Command::Setup => {
            let setup = run_setup(context.health, true)?;
            if setup.missing_models.is_empty() {
                Ok(CommandOutcome::Message(
                    "setup: Ollama healthy, all required models are already available; continue with /scrape, /latest, or /query"
                        .to_string(),
                ))
            } else {
                Ok(CommandOutcome::Message(format!(
                    "setup: missing models [{}]; confirmation accepted; pulled [{}]; retry your last AI command",
                    setup.missing_models.join(", "),
                    setup.pulled_models.join(", ")
                )))
            }
        }
        Command::Clear => Ok(CommandOutcome::Message(
            "clear: chat history buffer reset".to_string(),
        )),
        Command::Quit => Ok(CommandOutcome::Message(
            "shutdown: graceful quit requested; background work will be stopped safely".to_string(),
        )),
    };

    let _ = log_command_timing(command_name, started_at);
    result
}

fn command_name(command: &Command) -> &'static str {
    match command {
        Command::Help => "/help",
        Command::ListScraper => "/list-scraper",
        Command::Scrape { .. } => "/scrape",
        Command::Latest { .. } => "/latest",
        Command::Query { .. } => "/query",
        Command::Export { .. } => "/export",
        Command::Setup => "/setup",
        Command::Clear => "/clear",
        Command::Quit => "/quit",
    }
}

fn help_message() -> String {
    [
        "/help",
        "/list-scraper",
        "/scrape <manufacturer> <query>",
        "/latest [scraper_name]",
        "/query <text>",
        "/export <query>",
        "/setup",
        "/clear",
        "/quit (/exit)",
    ]
    .join("\n")
}

fn list_scrapers_message(context: &CommandContext<'_>) -> String {
    let mut lines = vec!["scrapers:".to_string()];
    for scraper in context.registry.all() {
        let latest = if scraper.supports_latest() {
            "yes"
        } else {
            "no"
        };
        lines.push(format!("- {} (latest: {latest})", scraper.name()));
    }
    if lines.len() == 1 {
        lines.push("- none registered".to_string());
        lines
            .push("hint: verify scraper modules are enabled, then retry /list-scraper".to_string());
    }
    lines.join("\n")
}

fn build_export_dir(cache_root: &Path) -> AppResult<PathBuf> {
    let base = cache_root
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| cache_root.to_path_buf());
    let epoch_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let export_dir = base.join("exports").join(format!("export-{epoch_secs}"));
    std::fs::create_dir_all(&export_dir)?;
    Ok(export_dir)
}
