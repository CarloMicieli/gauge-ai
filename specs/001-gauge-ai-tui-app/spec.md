# Feature Specification: Gauge.ai — Local Knowledge Aggregator TUI

**Feature Branch**: `001-gauge-ai-tui-app`  
**Created**: 2026-04-22  
**Status**: Draft  
**Input**: User description: "Gauge.ai is a high-performance Terminal User Interface (TUI) built in Rust. It serves as a local knowledge aggregator for model railway enthusiasts, combining multi-source web scraping, AI-driven data normalization via Ollama, and local semantic search using a vector-enabled SQLite database."

---

## Clarifications

### Session 2026-04-22

- Q: How should database startup failures be handled? -> A: Missing DB initializes via migrations; corrupted DB is quarantined as `.bak` and a fresh DB is created with a visible TUI notice.
- Q: How should cache writes behave under low-disk conditions? -> A: Image writes are skipped on disk exhaustion with a warning, while metadata JSON is still persisted.
- Q: How should scraper parse breakages and very large catalogues be handled? -> A: Per-page parse failures surface as non-fatal scraper errors, and discovery/processing must support paginated or streamed handling for large catalogues.
- Q: How should malformed Ollama JSON and missing configured models be handled? -> A: Normalization attempts JSON repair and fallback to raw text tagged `Unnormalized`; configured chat/embedding models are validated against Ollama tags with user remediation guidance.
- Q: How should terminal image capability negotiation work? -> A: Runtime capability detection attempts Sixel/Kitty first and falls back automatically to Unicode half-block rendering.
- Refinement: Duplicate product codes are reconciled via an Ollama merge task that synthesizes a new golden record from existing and newly scraped data, then archives the previous version.
- Refinement: Add a `/latest [scraper_name]` command that supports global latest sync across capable scrapers or targeted sync for one scraper, with merge-aware ingestion and summary output.
- Refinement: Add a branded ASCII-art locomotive header for Gauge.ai with orange-on-black styling and optional subtle wheel animation.
- Refinement: Add startup and periodic Ollama health checks with visible status states and guided remediation for missing models.

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Scrape & Cache Manufacturer Data (Priority: P1)

A model railway enthusiast opens Gauge.ai in their terminal and issues `/scrape Roco "BR 50"`. The application discovers all matching product pages from the Roco website, extracts structured model data from each page, normalises the raw HTML content via a local LLM, and persists the results to a local SQLite database and an on-disk cache. Downloaded product images are stored locally and displayed in the TUI using Sixel/Kitty image rendering. A real-time progress indicator (throbber + counter "3/15 models extracted") is shown throughout the operation.

**Why this priority**: This is the foundational data-ingestion flow. Without it, no local data exists and all other features (search, export) have nothing to operate on.

**Independent Test**: Issue `/scrape <manufacturer> <query>` against a live or mocked manufacturer website. Verify that structured records appear in the local database and image files are written to the cache directory. Delivers immediate standalone value: the user has an offline, structured catalogue of matching models.

**Acceptance Scenarios**:

1. **Given** Gauge.ai is running and Ollama is reachable, **When** the user issues `/scrape Roco "BR 50"`, **Then** the TUI displays a progress counter that increments as pages are processed and completes without error, leaving at least one record in the local database.
2. **Given** a product page was previously scraped, **When** `/scrape` targets the same URL again, **Then** the cached JSON is returned immediately without re-fetching the remote page or re-querying Ollama.
3. **Given** the scraper encounters a network error on one page, **When** extracting model data, **Then** the error is logged and the remaining pages are still processed.
4. **Given** a manufacturer has no registered scraper module, **When** `/scrape <unknown-manufacturer> <query>` is issued, **Then** the TUI displays an informative error and lists available manufacturers.

---

### User Story 2 — Natural Language Search Against Local Data (Priority: P2)

After data has been ingested, the user types `/query Which Roco BR 50s are available?`. The application converts the query to a vector embedding, retrieves the top 5 most relevant local records via SQLite vector search, sends those records to Ollama as context, and displays a conversational, structured answer in the TUI — all without any external network request.

**Why this priority**: This is the primary consumption interface. Users collect data in order to search it; this story delivers the core productivity value of the application.

**Independent Test**: Insert at least three pre-seeded records into the database, issue a `/query` command, and verify that a coherent answer referencing the seeded records is displayed within the TUI.

**Acceptance Scenarios**:

1. **Given** local records exist, **When** the user issues `/query Which Roco BR 50s are available?`, **Then** the TUI displays a conversational answer mentioning relevant models drawn from local data.
2. **Given** no local records match the query semantically, **When** `/query` is issued, **Then** the TUI responds that no matching records were found locally and suggests running `/scrape`.
3. **Given** Ollama is unavailable, **When** a query is issued, **Then** the TUI displays a clear error message and does not crash.

---

### User Story 3 — Sync Latest Arrivals (Priority: P3)

The user types `/latest` to check newly listed models from all scraper modules that support latest-arrivals discovery, or `/latest roco` to target a single source. The system runs eligible latest-arrivals fetches concurrently, ingests found URLs, and applies merge reconciliation so duplicates become updates instead of new records. After completion, the TUI prints a concise summary (for example: "12 new, 3 updated").

**Why this priority**: This delivers a recurring "what changed" workflow and turns Gauge.ai into an update feed, while reusing existing scraping and merge pipelines.

**Independent Test**: Run `/latest` with at least two scraper modules where one supports latest-arrivals and one does not. Verify only capable modules run, results are ingested, and a new-vs-updated summary is shown.

**Acceptance Scenarios**:

1. **Given** at least one scraper supports latest-arrivals, **When** the user runs `/latest`, **Then** all supporting scrapers are executed and non-supporting scrapers are skipped.
2. **Given** the user runs `/latest roco`, **When** `roco` supports latest-arrivals, **Then** only that scraper is executed and results are merged into existing records where applicable.
3. **Given** the user runs `/latest acme`, **When** `acme` does not support latest-arrivals, **Then** the TUI returns a clear unsupported-command message for that scraper.
4. **Given** latest-arrivals ingestion completes, **When** the command finishes, **Then** the TUI reports counts of newly inserted versus merged/updated records.

---

### User Story 4 — Export Matching Records as an Archive (Priority: P4)

The user issues `/export "Roco BR 50"` to bundle all matching records and their associated cached images into a single portable archive (ZIP or directory) in JSON and/or CSV format. This allows the user to share a curated dataset with others or import it into external tools.

**Why this priority**: Export is a secondary productivity feature that adds shareability and interoperability, but the application remains fully functional without it.

**Independent Test**: Seed the database with known records, run `/export`, and verify that a ZIP/directory is created containing both a structured data file (JSON/CSV) and all referenced image files.

**Acceptance Scenarios**:

1. **Given** matching records with local images exist, **When** `/export "Roco BR 50"` is issued, **Then** a ZIP or directory is created in the working directory containing a JSON file with all matching records and a subfolder with associated images.
2. **Given** no records match the export query, **When** `/export` is issued, **Then** the TUI reports zero results and no archive is created.
3. **Given** a record's cached image files are missing from disk, **When** exporting, **Then** the record is still included in the data file and missing images are noted in a manifest rather than causing export failure.

---

### User Story 5 — Browse Scrapers and Navigate the TUI (Priority: P5)

A new user opens Gauge.ai and uses `/help` to discover available commands and `/list-scraper` to see which manufacturer modules are active. The TUI provides a responsive, keyboard-driven interface with visible slash-command input, scroll navigation for results, and in-terminal image previews of model photographs.

**Why this priority**: Discoverability and navigation complete the user experience but are not required for core data workflows.

**Independent Test**: Launch the application, type `/help` and `/list-scraper`, and verify that informative output is rendered without errors.

**Acceptance Scenarios**:

1. **Given** Gauge.ai is running, **When** the user types `/help`, **Then** a formatted list of all supported slash commands with brief descriptions is displayed in the TUI.
2. **Given** one or more scraper modules are registered, **When** the user types `/list-scraper`, **Then** each module's name and the manufacturers it supports are listed.
3. **Given** a model record with a local image path exists, **When** the user navigates to that record, **Then** the image is rendered inline using Sixel or Kitty protocol if the terminal supports it, falling back to Unicode half-block characters otherwise.
4. **Given** the app starts and enters the main TUI, **When** the header area is rendered, **Then** a centered ASCII-art Gauge.ai locomotive logo is displayed with the configured primary theme color.
5. **Given** the app starts or a periodic health interval elapses, **When** Ollama health is checked, **Then** the header shows current status (`Checking`, `Healthy`, `Disconnected`, or `ModelMissing`) with a clear visual indicator.

---

### Edge Cases

- If the SQLite file is missing at startup, the app initialises a new database by running migrations before accepting commands.
- If the SQLite file is detected as corrupted, the app moves it to a timestamped `.bak` file, creates a fresh database, and surfaces a recovery message in the TUI.
- If a scraper cannot find mandatory selectors (such as product code/SKU), the current page returns a parse error and processing continues with remaining pages.
- If available disk space is insufficient during image caching, image download for that item is skipped, metadata is still saved, and a warning is shown in the status area.
- If Ollama returns malformed JSON, the pipeline performs one repair attempt; if still invalid, raw text is saved and the record is tagged `Unnormalized` for review.
- If configured embedding/chat models are not available in Ollama, the app blocks AI-dependent actions and guides the user to pull/select valid models.
- For very large catalogues (thousands of pages), discovery and extraction use paginated or streamed processing to avoid UI lockups and excessive memory growth.
- If Sixel/Kitty is unsupported, the image renderer automatically falls back to Unicode half-block rendering without user intervention.
- If duplicate product codes are re-scraped, the app runs an Ollama merge workflow (existing golden record + new scrape) and stores the merged result as current while archiving the previous version.
- If merge output mutates immutable identity fields (manufacturer or SKU), the merge is rejected and the record is flagged for manual review in the TUI.
- If `/latest` runs globally, scrapers that do not support latest-arrivals are skipped and listed as skipped in the final summary.
- If `/latest <scraper_name>` references an unknown scraper, the command fails fast with available scraper names.
- If terminal rendering width is too small for the full ASCII logo, the app uses a compact header variant without layout breakage.
- If Ollama health checks time out, the status transitions to `Disconnected` and command handlers provide user-friendly remediation text.
- If required models are missing, the status transitions to `ModelMissing` and the user is prompted to run setup/pull actions.

---

## Requirements *(mandatory)*

### Functional Requirements

**Scraping & Ingestion**

- **FR-001**: The system MUST provide a trait-based scraper interface so that manufacturer-specific scraping logic can be added independently without modifying core application code.
- **FR-002**: The scraper pipeline MUST separate product-page discovery from data extraction, enabling real-time progress reporting ("N/M models extracted") in the TUI.
- **FR-003**: The system MUST cache scraped metadata as JSON files on disk, keyed by a hash of the source URL, so that repeated scrapes do not re-fetch remote pages.
- **FR-004**: The system MUST download and store product images locally in a content-addressable structure, keyed by hashes of the image URL.
- **FR-005**: The system MUST route `/scrape` commands to the correct scraper module based on the manufacturer name supplied by the user.
- **FR-006**: The system MUST run scraping tasks as background async jobs so that the TUI remains responsive during ingestion.
- **FR-007**: The scraper interface MUST expose optional latest-arrivals capability so each scraper can declare whether latest sync is supported.
- **FR-008**: The system MUST provide a `/latest [scraper_name]` command where no argument triggers global latest sync across all capable scrapers and an argument triggers targeted latest sync for one scraper.
- **FR-009**: The `/latest` command MUST execute selected latest-arrivals jobs concurrently and keep the TUI responsive during execution.
- **FR-010**: Each latest-arrivals URL discovered by `/latest` MUST flow through the same dedupe and Ollama merge reconciliation pipeline used for standard scrape ingestion.
- **FR-011**: After `/latest` completes, the TUI MUST display a summary of inserted records, merged/updated records, skipped scrapers, and failed scraper jobs.

**AI Normalisation**

- **FR-012**: The system MUST use a locally-running Ollama instance to normalise raw scraped text into the structured `ModelData` format.
- **FR-013**: The normalisation pipeline MUST inject domain knowledge (NEM standards, epoch definitions) via a knowledge base file to prevent AI hallucination.
- **FR-014**: The system MUST use Ollama to translate technical specifications from foreign languages (German, Italian, etc.) into the user's configured preferred language.
- **FR-015**: The system MUST queue Ollama requests sequentially to prevent resource exhaustion, while still allowing background scraping to proceed concurrently.

**Persistence**

- **FR-016**: The system MUST persist structured model records to a local SQLite database, including a versioned history table that tracks changes to price, stock, and description over time.
- **FR-017**: The system MUST store vector embeddings of normalised model descriptions in a virtual table within the same SQLite file to enable semantic search.
- **FR-018**: The system MUST store the database and cache under standard platform-appropriate directories (e.g., `~/.local/share/gauge-ai/` on Linux).
- **FR-019**: The system MUST load configuration (Ollama URL, model name, preferred language) from a TOML file in the user's config directory.
- **FR-020**: On startup, if the SQLite database is missing, the system MUST initialize a new database schema via migrations automatically.
- **FR-021**: On startup, if the SQLite database is corrupted, the system MUST quarantine the corrupted file as a backup and create a fresh database without crashing.
- **FR-022**: The system MUST track a last-scraped timestamp for each current model record and update it whenever a scrape or latest-sync ingestion event touches the record.

**Search**

- **FR-023**: The system MUST convert a natural language `/query` into a vector embedding and retrieve the top 5 most semantically similar local records.
- **FR-024**: The system MUST pass the retrieved records as context to Ollama to generate a conversational answer grounded strictly in local data.
- **FR-025**: The system MUST present query results entirely from local data without requiring network access at query time.

**Export**

- **FR-026**: The system MUST bundle records matching an export query, along with their associated cached images, into a portable ZIP archive or directory.
- **FR-027**: The export MUST include a structured data file in both JSON and CSV formats.

**User Interface**

- **FR-028**: The TUI MUST accept slash commands (`/help`, `/list-scraper`, `/scrape`, `/latest`, `/query`, `/export`, `/setup`, `/quit`) via a text input widget.
- **FR-029**: The TUI MUST display a real-time progress indicator (throbber animation + activity counter) while background tasks are running.
- **FR-030**: The TUI MUST render product images inline using Sixel or Kitty terminal protocols where supported, with a Unicode half-block fallback.
- **FR-031**: The TUI MUST remain responsive and scrollable while background scraping or AI tasks are in progress.
- **FR-047**: The TUI MUST render a branded ASCII-art Gauge.ai header/logo in the startup or home pane, with color theming that supports orange-on-black defaults.
- **FR-048**: The header renderer MUST provide a compact fallback variant for narrow terminal widths to avoid clipping or overlap.
- **FR-049**: The UI MAY animate logo wheel glyphs with a low-frequency tick effect, and this animation MUST not impact command-input responsiveness.
- **FR-050**: The TUI MUST support graceful termination via `/quit` with `/exit` as an alias, closing background workers cleanly before process exit.
- **FR-051**: The app MUST perform an Ollama health check on startup and on a periodic background interval while the TUI is running.
- **FR-052**: The app MUST model health state as at least `Checking`, `Healthy`, `Disconnected`, and `ModelMissing(<models>)` so UI and commands can react deterministically.
- **FR-053**: The header area MUST display a real-time Ollama status indicator with distinct styling for healthy, disconnected, and missing-model states.
- **FR-054**: The app MUST provide guided remediation for missing models via `/setup`, including user confirmation before pull operations.
- **FR-055**: AI-dependent commands (`/scrape`, `/latest`, `/query`) MUST fail fast with actionable errors when Ollama is disconnected or required models are unavailable.

**Resilience & Failure Handling**

- **FR-032**: The caching pipeline MUST check writable disk availability before image persistence and degrade gracefully by storing metadata even when image writes fail.
- **FR-033**: Scraper parse failures for individual pages MUST be represented as non-fatal errors and must not terminate the overall scrape job.
- **FR-034**: Scraper discovery/extraction MUST support paginated or streamed handling for large catalogues to limit peak memory usage.
- **FR-035**: The normalisation pipeline MUST attempt structured-output repair for malformed Ollama responses before declaring failure.
- **FR-036**: If normalised JSON remains invalid after repair, the system MUST persist raw extracted text and mark the record with normalization status `Unnormalized`.
- **FR-037**: The system MUST validate configured embedding/chat models against Ollama's available tags before AI-dependent operations.
- **FR-038**: When configured Ollama models are unavailable, the system MUST provide explicit remediation guidance (e.g., pull/select model) in the TUI.
- **FR-039**: Terminal image rendering MUST perform runtime capability negotiation and automatically fall back to Unicode half-block rendering when advanced protocols are unavailable.

**Data Reconciliation & Versioning**

- **FR-040**: When a scraped record matches an existing `(manufacturer, product_code)`, the system MUST execute a merge task instead of blind overwrite.
- **FR-041**: The merge task MUST provide Ollama both the existing golden record and the new scraped payload, and require schema-conformant JSON output.
- **FR-042**: Merge conflict policy MUST prioritize technical specifications over marketing text and preserve non-conflicting unique fields from both sources.
- **FR-043**: After a successful merge, the system MUST atomically persist the new golden record and archive the prior record in `model_versions`.
- **FR-044**: The system MUST reject merge output that changes immutable identity fields (`manufacturer`, `product_code`) and mark the item for manual review.
- **FR-045**: The system MUST trigger merge only when source content fingerprint differs from the previously stored fingerprint for the same product key.
- **FR-046**: After merged content is committed, the system MUST regenerate and update the vector embedding for semantic search consistency.

### Key Entities

- **ModelData**: The core exchange format representing a single model railway product. Attributes: manufacturer, product code (SKU), name, description, details, scale, epoch, railway company, local image paths, original image URLs, and a flexible map of technical specifications.
- **ScrapeCriteria**: The user-supplied search parameters passed to a scraper (manufacturer name, free-text query).
- **ModelVersion**: A historical snapshot of a `ModelData` record at a point in time, capturing mutable fields (price, stock level, description) to enable change tracking.
- **VectorEmbedding**: A high-dimensional numerical representation of a model's normalised description, stored alongside the relational record for semantic retrieval.
- **KnowledgeBase**: A curated domain reference (NEM standards, epoch definitions, scale conventions) loaded at startup and injected into Ollama prompts.
- **ScraperModule**: A registered implementation of the scraper trait for a specific manufacturer, identified by name and the set of manufacturer names it can handle.
- **CacheEntry**: An on-disk artefact (JSON metadata file + image folder) keyed by URL hash, representing a point-in-time snapshot of a scraped product page.
- **NormalizationStatus**: A processing state for AI extraction outcomes (`Normalized`, `Unnormalized`) used to identify records needing manual review.
- **RecoveryNotice**: A user-visible operational event raised in the TUI for resilience events (database recovery, disk-pressure image skip, model-unavailable warnings).
- **GoldenRecord**: The authoritative current representation of a model identified by `(manufacturer, product_code)`.
- **SourceFingerprint**: A hash of source content used to determine whether merge processing is required.
- **MergeAudit**: Reconciliation metadata describing merge timestamp, merge engine/model, and review outcome (`Applied`, `Rejected`, `ManualReview`).
- **LatestSyncCapability**: A scraper capability flag indicating whether the scraper can discover new-arrivals/latest pages.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can complete the full journey from issuing `/scrape` to viewing normalised results in the TUI in under 60 seconds for a catalogue of up to 20 product pages (excluding first-time model download time for Ollama).
- **SC-002**: Re-scraping a previously fetched manufacturer page returns cached results with no remote network request, measured by zero outbound HTTP calls to the manufacturer's domain.
- **SC-003**: A `/query` operation against a database of up to 1,000 records returns a displayed result within 5 seconds on standard consumer hardware.
- **SC-004**: The TUI remains interactive (input latency under 100 ms) while a background scraping task is in progress.
- **SC-005**: 100% of scraped product images are retrievable from local storage after a scrape completes, without requiring internet access.
- **SC-006**: An export archive contains all matched records and their images, with zero data loss for records whose images are present in the local cache.
- **SC-007**: Normalised descriptions produced by the AI pipeline are in the user's configured language for at least 95% of records scraped from non-English sources.
- **SC-008**: The application starts and reaches an interactive TUI state within 3 seconds on standard consumer hardware.
- **SC-009**: In a fault-injection run with at least one parse error and one malformed Ollama response, ingestion continues processing remaining items with no full-job abort.
- **SC-010**: When image writes fail due to low disk space, at least 99% of associated metadata records are still persisted successfully.
- **SC-011**: On startup with a corrupted database file, the app restores to an interactive state with a fresh schema in under 5 seconds and displays a recovery notice.
- **SC-012**: For a scrape of 5,000 discovered pages, TUI input remains responsive (under 150 ms median input latency) throughout processing.
- **SC-013**: For duplicate detections with changed source fingerprints, at least 95% of merges complete without manual intervention while preserving immutable identity fields.
- **SC-014**: For unchanged duplicate payloads, no merge call is issued and ingestion skips directly to completion for that record.
- **SC-015**: A global `/latest` run across all capable scrapers completes and reports per-run totals for new records, updated records, skipped scrapers, and failures.
- **SC-016**: For `/latest <scraper_name>`, the command targets only the named scraper and returns a clear unsupported or unknown-scraper message within 1 second when applicable.
- **SC-017**: On terminals wider than the configured minimum header width, the branded Gauge.ai ASCII header renders without clipping in 100% of startup runs.
- **SC-018**: If header animation is enabled, median command-input latency remains within existing responsiveness targets during animation ticks.
- **SC-019**: Ollama health state is detected and reflected in the UI within 3 seconds of startup and within one periodic check interval after runtime outages.
- **SC-020**: In disconnected or missing-model states, 100% of AI-dependent command failures present human-readable remediation guidance instead of raw transport errors.

---

## Assumptions

- Users have a working Ollama instance running locally on the same machine (default: `http://localhost:11434`). Remote Ollama URLs are supported via configuration.
- Users are comfortable with a keyboard-driven, terminal-based interface; no graphical desktop UI is in scope.
- Initial scraper implementations will target a small set of manufacturers (e.g., Roco, Märklin, PIKO); the architecture must support adding further scrapers without core changes.
- The user's terminal emulator supports at least 256 colours; Sixel/Kitty image support is treated as a progressive enhancement.
- Internet access is required only during `/scrape` operations; all other operations (`/query`, `/export`, navigation) are fully offline.
- The knowledge base (`knowledge_base.toml`) ships with the application and is updated manually by the user or via a future update mechanism (out of scope for v1).
- Multi-language translation targets German and Italian as the primary foreign-language sources, based on the dominant languages of European model railway manufacturers.
- Mobile and Windows support are out of scope; the primary target is Linux with macOS as a secondary target.
- The application is a single-user desktop tool; no multi-user, authentication, or networked sharing features are in scope.
- Performance targets assume a modern consumer machine with at least 8 GB RAM and an SSD.
- A backup/quarantine of corrupted databases is retained locally until manually deleted by the user.
