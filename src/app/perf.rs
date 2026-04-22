use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use directories::ProjectDirs;

use crate::app::error::AppResult;

const PERF_LOG_FILE: &str = "perf.log";

/// Resolve the local performance log path under the Gauge.ai data directory.
pub fn perf_log_path() -> AppResult<PathBuf> {
    let dirs = ProjectDirs::from("ai", "gauge", "gauge-ai").ok_or_else(|| {
        crate::app::error::AppError::Config("unable to resolve project directories".to_string())
    })?;
    Ok(dirs.data_dir().join(PERF_LOG_FILE))
}

/// Append one performance event line to the local perf log.
pub fn append_perf_event(event: &str, elapsed: Duration) -> AppResult<()> {
    let path = perf_log_path()?;
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    let epoch_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    writeln!(
        file,
        "{epoch_secs} event={event} elapsed_ms={}",
        elapsed.as_millis()
    )?;
    Ok(())
}

/// Log command-to-response timing for one command execution.
pub fn log_command_timing(command: &str, started_at: Instant) -> AppResult<()> {
    append_perf_event(&format!("command:{command}"), started_at.elapsed())
}

/// Log startup timing from process initialization to interactive-ready state.
pub fn log_startup_timing(started_at: Instant) -> AppResult<()> {
    append_perf_event("startup", started_at.elapsed())
}
