# Contract: TUI Slash Commands

## Scope
Defines command syntax, argument rules, and expected response semantics for the Gauge.ai TUI command interface.

## Command Grammar

```text
/help
/list-scraper
/scrape <manufacturer> <query>
/latest [scraper_name]
/query <text>
/export <query>
/setup
/clear
/quit
/exit
```

## Command Contracts

### /help
- Input: none
- Output: list of available commands with usage examples.
- Errors: none (always succeeds when UI is active).

### /list-scraper
- Input: none
- Output: registered scrapers with manufacturer support and latest capability flag.
- Errors: if registry unavailable, emit `SCRAPER_REGISTRY_UNAVAILABLE`.

### /scrape <manufacturer> <query>
- Input:
  - manufacturer: required, case-insensitive name mapped to registered scraper.
  - query: required free text.
- Output:
  - async progress updates (`discovered`, `processed`, `failed`).
  - completion summary (`new`, `updated`, `failed`).
- Errors:
  - `SCRAPER_NOT_FOUND`
  - `NETWORK_ERROR`
  - `PARSE_ERROR` (non-fatal per item)

### /latest [scraper_name]
- Input:
  - scraper_name: optional; if omitted, run across all scrapers where `supports_latest=true`.
- Output:
  - progress by scraper.
  - completion summary: `inserted`, `updated`, `skipped_scrapers`, `failed_scrapers`.
- Errors:
  - `SCRAPER_NOT_FOUND` (named scraper not registered)
  - `LATEST_UNSUPPORTED` (named scraper exists but does not support latest)

### /query <text>
- Input:
  - text: required natural-language query.
- Output:
  - conversational answer grounded in top-K local records.
  - optional metadata (latency, result count).
- Errors:
  - `EMPTY_QUERY`
  - `EMBEDDING_MODEL_UNAVAILABLE`
  - `QUERY_BACKEND_ERROR`

### /export <query>
- Input:
  - query: required filter text.
- Output:
  - archive path (zip or directory)
  - export summary (`records`, `images`, `missing_images`)
- Errors:
  - `NO_MATCHING_RECORDS`
  - `EXPORT_IO_ERROR`

### /setup
- Input: none
- Output:
  - runs Ollama health diagnostics
  - lists missing required models
  - offers guided pull workflow with user confirmation
  - updates health state indicator on completion
- Errors:
  - `OLLAMA_DISCONNECTED`
  - `SETUP_PULL_FAILED`

### /clear
- Input: none
- Output:
  - clears in-memory TUI console history for the current session.
- Errors: none (always succeeds when UI is active).

### /quit and /exit
- Input: none
- Output:
  - initiates graceful shutdown flow
  - flushes/finishes in-flight command updates where possible
  - exits process with success status
- Errors:
  - `SHUTDOWN_IN_PROGRESS` if repeated while shutdown has already started

## Response Style Rules
- Commands must not block UI input loop.
- Errors should be user-readable and include remediation hints when possible.
- Long-running commands must publish periodic progress and a terminal summary line.
- AI-dependent commands must map connection/model failures to friendly messages rather than raw connection exceptions.
