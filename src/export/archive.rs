use std::path::{Path, PathBuf};

use crate::app::error::AppResult;
use crate::storage::models::ModelData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssetArchiveResult {
    pub images: usize,
    pub missing_images: usize,
}

/// Copy image assets into an export bundle and emit missing image manifest.
pub fn write_assets(output_dir: &Path, records: &[ModelData]) -> AppResult<AssetArchiveResult> {
    let image_dir = output_dir.join("images");
    std::fs::create_dir_all(&image_dir)?;

    let mut copied = 0usize;
    let mut missing = Vec::<String>::new();

    for record in records {
        for source in &record.local_image_paths {
            let source_path = PathBuf::from(source);
            if source_path.exists() {
                let file_name = source_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("image.bin");
                let target_name = format!(
                    "{}-{}-{}",
                    record.manufacturer, record.product_code, file_name
                );
                std::fs::copy(&source_path, image_dir.join(target_name))?;
                copied += 1;
            } else {
                missing.push(source.clone());
            }
        }
    }

    if !missing.is_empty() {
        std::fs::write(output_dir.join("missing_images.txt"), missing.join("\n"))?;
    }

    Ok(AssetArchiveResult {
        images: copied,
        missing_images: missing.len(),
    })
}
