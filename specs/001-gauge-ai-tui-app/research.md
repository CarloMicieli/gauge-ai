# Research: Gauge.ai Implementation Plan

## Decision 1: Use existing crate set from Cargo.toml as the implementation baseline
- Decision: Keep the implementation constrained to the crates already present in Cargo.toml (`tokio`, `sqlx`, `ollama-rs`, `reqwest`, `scraper`, `ratatui`, `ratatui-image`, `zip`, `serde`, `config`, `directories`, `sha2`, `digest`, `throbber-widgets-tui`, `tui-input`, `crossterm`).
- Rationale: The feature request explicitly requires planning around the current dependency set. This reduces integration risk and avoids churn before execution.
- Alternatives considered: Adding `rusqlite` for direct SQLite integration. Rejected because `sqlx` is already present and sufficient for async + SQLite access.

## Decision 2: Use sqlx + SQLite for relational and vector-adjacent persistence
- Decision: Implement persistence via `sqlx` against a local SQLite file, including tables for current records and `model_versions` history.
- Rationale: `sqlx` is already available, async-friendly, and aligns with the Tokio runtime in this codebase.
- Alternatives considered: Synchronous DB access via `rusqlite`. Rejected for this phase because the dependency is not currently present and does not align as cleanly with async ingestion workflows.

## Decision 3: Treat sqlite-vec as a migration/runtime capability check
- Decision: Plan vector storage/search as a capability layered on top of SQLite setup, with startup migration checks and graceful failure reporting if vector extension support is unavailable.
- Rationale: The spec requires semantic retrieval; capability checks prevent hard crashes and keep the TUI usable when environment support is incomplete.
- Alternatives considered: Deferring vector search entirely. Rejected because `/query` is core P2 functionality.

## Decision 4: Model scraper architecture with optional latest capability
- Decision: Keep `ModelScraper` as the core trait and add optional latest-arrivals support (`supports_latest`, latest discovery method).
- Rationale: Not every manufacturer supports a latest feed; capability discovery preserves extensibility while enabling `/latest` global mode.
- Alternatives considered: Separate dedicated latest trait per scraper type. Rejected to reduce registry complexity in early implementation.

## Decision 5: Use URL/content hashing for cache and merge triggering
- Decision: Use SHA-256 hashes for metadata cache keys and source-fingerprint comparisons to decide whether to run Ollama merge.
- Rationale: `sha2`/`digest` are already present and produce stable, deterministic keys for dedupe and incremental sync.
- Alternatives considered: Timestamp-only freshness logic. Rejected because it is less accurate and causes unnecessary merge calls.

## Decision 6: Keep Ollama calls serialized through a bounded queue
- Decision: Funnel normalization and merge prompts through a single-consumer worker queue (`tokio::sync::mpsc`) while allowing scraper network I/O to remain concurrent.
- Rationale: Prevents local model overload and aligns with existing spec constraints around resource exhaustion.
- Alternatives considered: Fully parallel Ollama calls. Rejected for reliability and host resource limits.

## Decision 7: Use TUI command contract as the primary external interface
- Decision: Define command-level contracts for `/help`, `/list-scraper`, `/scrape`, `/latest`, `/query`, `/export`, `/setup`, and `/quit` (with `/exit` alias) plus standardized summary/error outputs.
- Rationale: The application is TUI-first; command semantics are the user-facing contract that downstream tests must validate.
- Alternatives considered: HTTP API as the primary contract. Rejected because no web-service interface is in scope.

## Decision 8: Add model lifecycle states for reliable pipeline behavior
- Decision: Track pipeline states across discovery, extraction, normalization, merge, persistence, and error/manual-review outcomes.
- Rationale: Explicit state handling improves observability and enables robust partial-failure behavior.
- Alternatives considered: Implicit state inferred from nullable fields. Rejected due to ambiguity and weaker diagnostics.

## Decision 9: Constitution gate handling
- Decision: Treat constitution checks as informational for this run because `.specify/memory/constitution.md` is still an uninitialized template with placeholders.
- Rationale: There are no enforceable project-specific rules to evaluate yet.
- Alternatives considered: Blocking planning on missing constitution definitions. Rejected to keep feature planning moving while clearly documenting this governance gap.
