# Implementation Plan: Gauge.ai Local Knowledge Aggregator TUI

**Branch**: `001-gauge-ai-tui-app` | **Date**: 2026-04-22 | **Spec**: `/specs/001-gauge-ai-tui-app/spec.md`
**Input**: Feature specification from `/specs/001-gauge-ai-tui-app/spec.md`

## Summary

Build a local-first Rust TUI application that ingests manufacturer catalogue data via modular scrapers, normalizes and reconciles data with Ollama, persists relational + vector-searchable records in local SQLite, and exposes operator workflows through slash commands (`/scrape`, `/latest`, `/query`, `/export`, `/setup`, `/quit`).

Implementation will use existing dependencies in `Cargo.toml`, with async orchestration via Tokio, HTML extraction via `scraper`, storage via `sqlx` + SQLite, TUI rendering via `ratatui`, and merge-aware dedupe using Ollama prompts.

## Technical Context

**Language/Version**: Rust edition 2024  
**Primary Dependencies**: `tokio`, `sqlx`, `ollama-rs`, `reqwest`, `scraper`, `ratatui`, `ratatui-image`, `tui-input`, `throbber-widgets-tui`, `serde`, `config`, `directories`, `sha2`, `digest`, `zip`, `crossterm`  
**Storage**: Local SQLite database (`trains.db`) + local filesystem cache for metadata/images  
**Configuration**: User TOML config must define Ollama base URL (connection string), chat model, embedding model, and preferred language; loaded from standard config directory via `config` + `directories`  
**Testing**: `cargo test` (unit/integration), plus command/contract verification for slash command flows  
**Target Platform**: Linux primary, macOS secondary (terminal environments supporting crossterm/ratatui)  
**Project Type**: Single binary desktop TUI application  
**Performance Goals**: `/query` response <= 5 seconds for up to 1,000 records; startup <= 3 seconds; responsive input while background jobs run  
**Constraints**: Local-first operation for query/export; serialized Ollama worker queue; graceful degradation on DB/caching/image-protocol failures  
**Scale/Scope**: Thousands of discovered pages per run, up to 5,000 scrape candidates in stress scenarios

Ollama Health Plan:
- Maintain app-level health state (`Checking`, `Healthy`, `Disconnected`, `ModelMissing`) updated on startup and periodic background checks.
- Render a small status indicator in the header area tied to current health state.
- Route missing-model remediation through `/setup` with explicit confirmation before pull actions.
- Enforce fast-fail user messaging for AI-dependent commands when health is not ready.

UI Branding Plan:
- Render a Gauge.ai ASCII locomotive header in the main/home pane using `ratatui::text::Line` + styled spans.
- Default palette: black background with orange primary text for logo elements; subdued gray for metadata/subtext.
- Provide two logo layouts: full-width and compact fallback for narrow terminal widths.
- Optional wheel-glyph animation runs on existing UI tick cadence and must not block input handling.
- Header may include compact grounded metrics (for example knowledge-base coverage counts and active scraper count) when width allows.

Requirement Traceability:
- Spec FR-019 requires loading Ollama URL and model selection from user config.

## Knowledge Base Build Plan

Goal: Build a deterministic, local knowledge base used to ground normalization and merge prompts so Ollama outputs remain schema-accurate and low-hallucination.

1. Authoring Format
- Store the canonical knowledge base as TOML (`knowledge_base.toml`) in the app config directory.
- Represent domain standards in normalized sections:
    - `scales`
    - `epochs`
    - `power_systems`
    - `interfaces`
    - `couplers`
    - `manufacturer_aliases`
    - `translation_glossary`
    - `railway_companies`
    - `liveries`
    - `lighting_features`
    - `function_mapping`
    - `prototypes`
    - `prototype_mappings`
- Use alias-rich value arrays for normalization lookups (canonical key + multilingual aliases/search terms).
- Parse/deserialise via existing `config` + `serde` stack backed by TOML (no new dependency required).

2. Bootstrap and Versioning
- On first launch, generate a default knowledge-base file from an embedded template with a `version` and `last_updated` field.
- If the file exists, load user-customized content without overwrite.
- If parsing fails, keep the app running with a warning and fall back to built-in minimal defaults.

3. Validation Pipeline
- Validate schema and required keys at startup (for example: epoch code map and known scales must exist).
- Normalize keys to stable forms used by prompts (trim, lowercase canonical keys, preserve display values).
- Canonicalize extracted values before persistence (for example, map `Modern` to epoch `VI` using alias lookup).
- Publish validation warnings to the TUI status area; do not crash unless both file and fallback are unavailable.

4. Runtime Prompt Injection
- Build a compact, deterministic "knowledge context" string from validated entries.
- Inject that context into:
    - normalization prompts (`raw scrape -> ModelData`)
    - merge prompts (`existing + new -> golden record`)
- Keep injection bounded (topical subset by task) to control token usage and latency.
- Build section-filtered context by scraper/source profile (for example: German source -> prioritize `translation_glossary`, `epochs`, `interfaces`, `couplers`).
- Do not inject the entire TOML for every call; inject only relevant canonical/alias slices.
- Include livery aliases for color-to-fact normalization (color/paint-name -> canonical livery -> likely operator + epoch range).
- For natural-language query expansion, include prototype alias slices so nicknames (for example, "Krokodil") resolve to canonical classes (for example, `SBB Ce 6/8 II`) before vector retrieval.
- For natural-language query expansion, include livery alias slices so visual descriptions (for example, "blue and beige german") resolve to canonical schemes and then to likely era/operator filters.
- For extraction normalization, include prototype/operator/era context to improve class resolution from informal descriptions.
- Add a consistency check step that flags implausible prototype-era combinations before persistence.
- Add a consistency check step that flags implausible livery-era or livery-operator combinations before persistence.

5. Update Strategy
- Treat the file as user-editable source-of-truth; no network dependency required.
- Re-load knowledge base on startup and optionally on explicit command (future `/reload-config` extension).
- Preserve backward compatibility by supporting migration from older `version` schemas.

6. Testing Strategy for Knowledge Base
- Unit tests: parse + validate good/bad TOML cases.
- Integration tests: confirm injected context changes normalization outcomes for known multilingual samples.
- Regression tests: ensure invalid user entries degrade gracefully and preserve app interactivity.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Constitution source (`.specify/memory/constitution.md`) is still template-only and contains placeholder sections without enforceable project rules.
- Gate Result (pre-research): PASS (informational only; no active constraints to violate).
- Gate Result (post-design): PASS (no constitution-defined mandatory practices available to evaluate).

## Project Structure

### Documentation (this feature)

```text
specs/001-gauge-ai-tui-app/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── command-contract.md
│   └── merge-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── main.rs
├── app/
│   ├── mod.rs
│   ├── state.rs
│   └── commands.rs
├── tui/
│   ├── mod.rs
│   ├── layout.rs
│   ├── logo.rs
│   ├── widgets.rs
│   └── image.rs
├── scraper/
│   ├── mod.rs
│   ├── traits.rs
│   ├── registry.rs
│   ├── caching_decorator.rs
│   └── manufacturers/
├── ai/
│   ├── mod.rs
│   ├── normalize.rs
│   ├── merge.rs
│   └── queue.rs
├── storage/
│   ├── mod.rs
│   ├── db.rs
│   ├── migrations.rs
│   ├── models.rs
│   └── vector.rs
├── cache/
│   ├── mod.rs
│   └── filesystem.rs
└── export/
    ├── mod.rs
    ├── json.rs
    ├── csv.rs
    └── archive.rs

tests/
├── integration/
│   ├── scrape_flow.rs
│   ├── latest_flow.rs
│   ├── query_flow.rs
│   └── export_flow.rs
└── contract/
    ├── command_contract.rs
    └── merge_contract.rs
```

**Structure Decision**: Single Rust binary project with domain-oriented modules under `src/`. This preserves straightforward build/run ergonomics while keeping clear boundaries for scraper orchestration, AI reconciliation, persistence, cache, and TUI concerns.

## Complexity Tracking

No constitution violations require justification for this planning cycle.
