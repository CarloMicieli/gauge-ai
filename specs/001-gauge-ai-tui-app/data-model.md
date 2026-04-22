# Data Model: Gauge.ai

## Entity: ModelData (Current Golden Record)
- Purpose: Authoritative current model record used for display, search, and export.
- Key Fields:
  - manufacturer: String (required, immutable identity component)
  - product_code: String (required, immutable identity component)
  - name: String (required)
  - description: String (required)
  - details: String (optional, defaults empty)
  - scale: String (optional)
  - epoch: String (optional)
  - railway_company: String (optional)
  - local_image_paths: Vec<String> (cache paths)
  - image_urls: Vec<String> (source URLs)
  - specifications: Map<String, String> (normalized technical attributes)
  - normalization_status: Enum (`Normalized`, `Unnormalized`)
  - source_fingerprint: String (hash of raw/source payload used for merge gating)
  - last_scraped_at: DateTime (updated on scrape/latest touch)
  - created_at: DateTime
  - updated_at: DateTime
- Validation Rules:
  - `(manufacturer, product_code)` unique.
  - `manufacturer` and `product_code` cannot be changed by merge output.
  - `local_image_paths` entries must reside under cache root.

## Entity: ModelVersion
- Purpose: Immutable history snapshots for change tracking and rollback/audit.
- Key Fields:
  - id: UUID/int primary key
  - manufacturer: String
  - product_code: String
  - snapshot_json: JSON blob (or decomposed fields)
  - change_reason: Enum (`ScrapeUpdate`, `LatestSyncMerge`, `ManualCorrection`)
  - merged_by_model: String (optional; e.g. ollama model id)
  - created_at: DateTime
- Relationships:
  - Many `ModelVersion` rows per one current `ModelData` key.

## Entity: ScraperModuleRegistration
- Purpose: Runtime registry metadata for installed scraper implementations.
- Key Fields:
  - name: String (unique)
  - supported_manufacturers: Vec<String>
  - supports_latest: Bool
  - enabled: Bool
- Validation Rules:
  - Name unique in registry.

## Entity: CacheEntry
- Purpose: On-disk metadata cache for source pages and images.
- Key Fields:
  - scraper_name: String
  - source_url: String
  - url_hash: String (SHA-256)
  - metadata_path: String
  - image_dir_path: String
  - created_at: DateTime
- Validation Rules:
  - `metadata_path` exists for valid cache hits.

## Entity: MergeAudit
- Purpose: Record reconciliation attempts and outcomes.
- Key Fields:
  - manufacturer: String
  - product_code: String
  - outcome: Enum (`Applied`, `Rejected`, `ManualReview`)
  - rejection_reason: String (optional)
  - model_used: String
  - prompt_hash: String
  - created_at: DateTime

## Entity: QueryRun
- Purpose: Track semantic query executions and latency metrics.
- Key Fields:
  - id: UUID/int
  - query_text: String
  - top_k: Int
  - latency_ms: Int
  - result_count: Int
  - created_at: DateTime

## Entity: KnowledgeBase
- Purpose: Local domain standards source used to ground normalization and merge prompts.
- Key Fields:
  - version: String
  - last_updated: DateTime
  - scales: Map<String, Vec<String>>
  - epochs: Map<String, Vec<String>>
  - power_systems: Map<String, Vec<String>>
  - interfaces: Map<String, Vec<String>>
  - couplers: Map<String, Vec<String>>
  - manufacturer_aliases: Map<String, Vec<String>>
  - translation_glossary: Map<String, String>
  - railway_companies: Map<String, Vec<String>>
  - liveries: Map<String, Vec<String>>
  - lighting_features: Map<String, Vec<String>>
  - function_mapping: Map<String, Vec<String>>
  - prototypes: Map<String, Vec<String>>
  - prototype_mappings: Map<String, Vec<String>>
- Validation Rules:
  - `version`, `epochs`, and `scales` required.
  - Empty knowledge base is invalid unless fallback defaults are available.
  - Keys are normalized for lookup, while values preserve display/localized text.
  - Alias arrays are flattened into case-insensitive lookup indexes at load time.
  - `prototypes` keys represent canonical class/series names; alias arrays should include at least one nickname/operator/era token.
  - `liveries` keys represent canonical paint schemes; alias arrays should include at least one color-name alias and one operator/era hint token.

## Entity: OllamaHealthState
- Purpose: Runtime health snapshot used by UI rendering and command gating.
- Key Fields:
  - state: Enum (`Checking`, `Healthy`, `Disconnected`, `ModelMissing`)
  - missing_models: Vec<String> (empty unless `ModelMissing`)
  - last_checked_at: DateTime
  - last_error: String (optional)
- Validation Rules:
  - `missing_models` must be non-empty when state is `ModelMissing`.
  - `last_checked_at` is updated on startup check and each periodic check.

## Relationships Overview
- `ModelData` 1-to-many `ModelVersion` by `(manufacturer, product_code)`.
- `ModelData` 1-to-many `MergeAudit` by `(manufacturer, product_code)`.
- `ScraperModuleRegistration` governs command routing and `/latest` capability selection.
- `CacheEntry` references a source page and image assets used to build/update `ModelData`.
- `KnowledgeBase` provides prompt-grounding context to normalization and merge workers.
- `OllamaHealthState` gates AI-dependent commands and drives header health indicator rendering.

## State Transitions

### Record lifecycle
1. Discovered -> URL found by scraper.
2. Extracted -> raw fields produced by parser.
3. Normalized -> Ollama returns schema-valid `ModelData`.
4. Merged -> duplicate key reconciled with existing golden record.
5. Persisted -> current record updated and prior version archived.
6. Indexed -> vector embedding written/refreshed.
7. Exported -> optional, record included in export run.

### Failure paths
- Extracted -> ParseError (non-fatal, job continues).
- Normalized -> Unnormalized (repair failed, raw content persisted).
- Merged -> ManualReview (identity mutation or invalid merge output).
