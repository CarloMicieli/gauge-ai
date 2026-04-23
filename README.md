# Gauge.ai

**Gauge.ai** is a high-performance Terminal User Interface (TUI) built in Rust. It serves as a local knowledge aggregator for model railway enthusiasts, combining multi-source web scraping, AI-driven data reconciliation via Ollama, and local semantic search using a vector-enabled SQLite database.

---

## 1. Core Objectives
* **Local-First:** All data, high-res images, and metadata are stored locally in the user's home directory.
* **Intelligent Normalization:** Leverage Ollama to transform "noisy" web HTML into structured technical datasets.
* **Semantic Search:** Natural language querying (RAG) powered by `sqlite-vec`.
* **Data Synthesis:** Use LLMs to merge conflicting data from multiple sources into a single "Golden Record."

---

## 2. Technical Stack
| Component | Technology |
| :--- | :--- |
| **Language** | Rust (Edition 2021) |
| **Async Runtime** | `tokio` |
| **TUI Framework** | `ratatui` with `crossterm` |
| **Database** | SQLite + `sqlite-vec` (Single-file storage) |
| **AI Inference** | Ollama (Local API) |
| **Hashing** | `sha2` (SHA-256) for CAS and cache keys |
| **Image Rendering** | `ratatui-image` (Sixel/Kitty support) |

---

## 3. Data Architecture

### 3.1 ModelData Schema
```rust
pub struct ModelData {
    pub manufacturer: String,
    pub product_code: String, // SKU
    pub name: String,
    pub description: String,
    pub details: String,
    pub scale: String,
    pub epoch: String,
    pub railway_company: String,
    pub local_image_paths: Vec<String>,
    pub image_urls: Vec<String>,
    pub specifications: HashMap<String, String>,
}
```

### 3.2 Storage Layout
* **Config:** `~/.config/gauge-ai/config.toml`
* **Database:** `~/.local/share/gauge-ai/trains.db`
* **Cache:** `~/.local/share/gauge-ai/cache/<scraper-name>/<url-hash>.json`
* **Images:** `~/.local/share/gauge-ai/cache/<scraper-name>/<url-hash>/<img-hash>.jpg`

---

## 4. Component Requirements

### 4.1 The `ModelScraper` Trait
Scrapers use a two-stage process to allow for TUI progress tracking.

```rust
#[async_trait]
pub trait ModelScraper: Send + Sync {
    fn name(&self) -> &str;
    fn supports_manufacturer(&self, mfr: &str) -> bool;
    fn supports_latest(&self) -> bool { false }
    
    async fn discover_product_pages(&self, criteria: ScrapeCriteria) -> Result<Vec<Url>, ScraperError>;
    async fn discover_latest(&self) -> Result<Vec<Url>, ScraperError>;
    async fn extract_model_info(&self, url: Url) -> Result<ModelData, ScraperError>;
}
```

### 4.2 The Caching Decorator (Mixin)
A wrapper that intercepts scraper calls to manage the local filesystem.
1.  **URL Hashing:** Uses SHA-256 of the URL as the primary cache key.
2.  **Asset Management:** Downloads images to a subdirectory named after the URL hash.
3.  **Persistence:** Saves the `ModelData` as JSON to allow for offline TUI browsing.

---

## 5. AI Pipeline & Normalization

### 5.1 Data Reconciliation (Merging)
When a duplicate SKU is found across different sources (e.g., Roco.cc vs. an E-shop), the system does not simply overwrite data.
1.  **Retrieval:** Pulls the existing "Golden Record" from SQLite.
2.  **Synthesis:** Ollama is prompted to merge the two records, prioritizing technical accuracy and preserving unique specs from both sources.
3.  **Versioning:** The previous state is archived in a `model_versions` table before the new merge is committed.

### 5.2 Retrieval-Augmented Generation (RAG)
1.  **Knowledge Injection:** Local `knowledge_base.toml` containing NEM standards and Epoch definitions is used to ground the LLM's normalization.
2.  **Semantic Search:** User queries are embedded via Ollama and matched in the `sqlite-vec` virtual table.

---

## 6. TUI & Slash Commands

| Command | Usage | Logic |
| :--- | :--- | :--- |
| `/help` | `/help` | Displays command overview. |
| `/list-scraper` | `/list-scraper` | Iterates and prints names of available scraper modules. |
| `/scrape` | `/scrape <mfr> <query>` | Triggers background task; sends MPSC message upon completion. |
| `/latest` | `/latest [mfr]` | Scans "New Arrivals" on supported sites. |
| `/query` | `/query <text>` | Semantic search via embeddings + vector database. |
| `/export` | `/export <query>` | Bundles JSON and image assets for matching records. |
| `/clear` | `/clear` | Clears in-memory chat history from the TUI console. |

### 6.1 Feedback Systems
* **Async Loader:** A throbber widget that alternates between bold and normal text with a "..." cycle to indicate background Ollama/Scrape activity.
* **Protocol Fallback:** Auto-detects terminal capabilities. Uses high-res rendering for Sixel/Kitty; falls back to Unicode Half-blocks for basic terminals.

---

## 7. Edge Case Handling
* **Database Corruption:** Automatic backup and re-initialization of `trains.db` on startup.
* **Scraper Brittleness:** Graceful error handling for HTML structure changes; reports "Update Required" instead of crashing.
* **Incompatible Embeddings:** Detects if the embedding model version has changed in `config.toml` and prompts for a vector re-index.
* **Disk Pressure:** Aborts image caching if disk space is low, preserving the lightweight text metadata.