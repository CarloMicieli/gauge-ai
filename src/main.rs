mod ai;
mod app;
mod cache;
mod export;
mod scraper;
mod storage;
mod tui;

use std::process::ExitCode;

use app::config::AppConfig;
use app::logging::init_logging;

fn main() -> ExitCode {
    init_logging();

    match run_startup() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[gauge-ai] startup failed: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run_startup() -> app::error::AppResult<()> {
    let config = AppConfig::load()?;

    std::fs::create_dir_all(&config.data_dir)?;
    std::fs::create_dir_all(&config.cache_dir)?;

    println!(
        "Gauge.ai initialized (db: {}, chat model: {}, embedding model: {}, language: {}, ollama: {})",
        config.db_path.display(),
        config.chat_model,
        config.embedding_model,
        config.preferred_language,
        config.ollama_base_url
    );

    Ok(())
}
