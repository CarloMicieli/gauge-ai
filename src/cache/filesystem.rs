use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

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
