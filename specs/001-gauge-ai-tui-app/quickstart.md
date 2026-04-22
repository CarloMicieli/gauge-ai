# Quickstart: Gauge.ai Feature Implementation

## 1. Prerequisites
- Rust toolchain with edition 2024 support.
- Local Ollama instance running (default `http://localhost:11434`).
- SQLite environment capable of required schema and vector setup.

## 2. Validate Environment
From project root:

```bash
cargo --version
```

## 3. Build and Run
```bash
cargo check
cargo run
```

Expected startup behavior:
- Config loaded from user config directory.
- Database initialized (or recovered) under local share directory.
- TUI launches with command input.

## 4. Exercise Core Commands
```text
/help
/list-scraper
/scrape roco BR 50
/latest
/latest roco
/query Which Roco BR 50s are available?
/export Roco BR 50
/setup
/quit
```

## 5. Verify Resilience Paths
- Simulate missing DB file -> startup migration creates a fresh DB.
- Simulate corrupted DB -> file quarantined as `.bak`, app starts with new DB.
- Force malformed Ollama JSON -> record stored with `Unnormalized` status.
- Re-scrape duplicate product code with changed fingerprint -> merge path updates golden record and archives version.

## 6. Developer Verification Loop
```bash
cargo fmt
cargo check
cargo clippy -- -D warnings
cargo test
```

## 7. Artifacts to Review During Implementation
- `specs/001-gauge-ai-tui-app/spec.md`
- `specs/001-gauge-ai-tui-app/plan.md`
- `specs/001-gauge-ai-tui-app/research.md`
- `specs/001-gauge-ai-tui-app/data-model.md`
- `specs/001-gauge-ai-tui-app/contracts/command-contract.md`
- `specs/001-gauge-ai-tui-app/contracts/merge-contract.md`

## 8. Run Notes (2026-04-22)
- `cargo --version` -> `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- `cargo check` -> pass
- `cargo run -- /quit` -> pass (`Gauge.ai shutdown completed.`)
- `cargo fmt` -> pass
- `cargo clippy -- -D warnings` -> pass
- `cargo test` -> pass (contract, integration, and unit suites green)
