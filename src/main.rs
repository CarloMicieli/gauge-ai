use std::process::ExitCode;
use std::time::Instant;

use gauge_ai::app::config::AppConfig;
use gauge_ai::app::logging::init_logging;
use gauge_ai::app::perf::log_startup_timing;

fn main() -> ExitCode {
    init_logging();
    let startup_started_at = Instant::now();

    if is_graceful_quit_requested() {
        println!("Gauge.ai shutdown completed.");
        return ExitCode::SUCCESS;
    }

    match run_startup(startup_started_at) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[gauge-ai] startup failed: {err}");
            ExitCode::FAILURE
        }
    }
}

fn is_graceful_quit_requested() -> bool {
    matches!(std::env::args().nth(1).as_deref(), Some("/quit" | "/exit"))
}

fn run_startup(startup_started_at: Instant) -> gauge_ai::app::error::AppResult<()> {
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

    let _ = log_startup_timing(startup_started_at);

    Ok(())
}
