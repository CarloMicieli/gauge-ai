/// Initialize application logging.
///
/// This currently uses stderr output only and is intentionally lightweight until
/// structured logging sinks are introduced in later tasks.
pub fn init_logging() {
    eprintln!("[gauge-ai] startup logging initialized");
}
