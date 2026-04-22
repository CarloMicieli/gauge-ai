use std::path::Path;

use crate::app::error::AppResult;
use crate::storage::models::ModelData;

/// Write selected records as pretty JSON.
pub fn write_json(path: &Path, records: &[ModelData]) -> AppResult<()> {
    let content = serde_json::to_string_pretty(records)?;
    std::fs::write(path, content)?;
    Ok(())
}
