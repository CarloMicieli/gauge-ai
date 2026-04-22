use std::path::Path;

use crate::app::error::AppResult;
use crate::storage::models::ModelData;

/// Write selected records as CSV with stable columns.
pub fn write_csv(path: &Path, records: &[ModelData]) -> AppResult<()> {
    let mut lines = vec![
        "manufacturer,product_code,name,description,details,scale,epoch,railway_company"
            .to_string(),
    ];

    for model in records {
        lines.push(format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
            escape_csv(&model.manufacturer),
            escape_csv(&model.product_code),
            escape_csv(&model.name),
            escape_csv(&model.description),
            escape_csv(&model.details),
            escape_csv(model.scale.as_deref().unwrap_or("")),
            escape_csv(model.epoch.as_deref().unwrap_or("")),
            escape_csv(model.railway_company.as_deref().unwrap_or("")),
        ));
    }

    std::fs::write(path, lines.join("\n"))?;
    Ok(())
}

fn escape_csv(value: &str) -> String {
    value.replace('"', "\"\"")
}
