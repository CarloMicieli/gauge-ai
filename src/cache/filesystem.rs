use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::app::error::AppResult;

/// Compute a stable SHA-256 hex hash used for cache keys.
pub fn hash_url(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let bytes = hasher.finalize();
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Return metadata and image paths for a given scraper and URL.
pub fn cache_paths(cache_root: &Path, scraper: &str, url: &str) -> (PathBuf, PathBuf) {
    let key = hash_url(url);
    let base = cache_root.join(scraper).join(key);
    (base.join("metadata.json"), base.join("images"))
}

/// Map remote image URLs to deterministic local cache file paths.
pub fn local_image_paths(image_dir: &Path, image_urls: &[String]) -> Vec<String> {
    image_urls
        .iter()
        .enumerate()
        .map(|(index, url)| {
            let extension = Path::new(url)
                .extension()
                .and_then(|value| value.to_str())
                .filter(|value| !value.is_empty())
                .unwrap_or("img");
            image_dir
                .join(format!("image-{index}.{extension}"))
                .display()
                .to_string()
        })
        .collect()
}

/// Ensure local image cache files exist for the provided source URLs.
pub fn ensure_image_cache_files(image_dir: &Path, image_urls: &[String]) -> AppResult<Vec<String>> {
    fs::create_dir_all(image_dir)?;
    let paths = local_image_paths(image_dir, image_urls);
    for path in &paths {
        if !Path::new(path).exists() {
            fs::write(path, [])?;
        }
    }
    Ok(paths)
}
