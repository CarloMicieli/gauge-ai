# Tasks: Gauge.ai Local Knowledge Aggregator TUI

**Input**: Design documents from `/specs/001-gauge-ai-tui-app/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/, quickstart.md

**Tests**: Include integration/contract tests because the feature spec and plan define independent test criteria and command contracts.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the base module layout and configuration scaffolding used by all stories.

- [X] T001 Create module skeleton and `mod.rs` wiring in `src/app/mod.rs`, `src/tui/mod.rs`, `src/scraper/mod.rs`, `src/ai/mod.rs`, `src/storage/mod.rs`, `src/cache/mod.rs`, and `src/export/mod.rs`
- [X] T002 Configure crate features and build settings for async SQLite and terminal stack in `Cargo.toml`
- [X] T003 [P] Implement configuration loader and default paths in `src/app/config.rs` and initialize startup config flow in `src/main.rs`
- [X] T004 [P] Add shared error/result types and app logging surface in `src/app/error.rs` and `src/app/logging.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that must be complete before user stories.

**CRITICAL**: No user story work starts until this phase is done.

- [X] T005 Implement database bootstrap and migration runner in `src/storage/migrations.rs`, `src/storage/db.rs`, and `migrations/0001_init.sql`
- [X] T006 [P] Define core domain structs/enums (`ModelData`, `ModelVersion`, `KnowledgeBase`, `OllamaHealthState`) in `src/storage/models.rs` and `src/ai/knowledge_base.rs`
- [X] T007 [P] Implement slash-command parser and dispatch shell for `/help`, `/list-scraper`, `/scrape`, `/latest`, `/query`, `/export`, `/setup`, `/quit` in `src/app/commands.rs`
- [X] T008 [P] Implement Ollama client wrapper, health-check scheduler, and serialized worker queue scaffolding in `src/ai/client.rs`, `src/ai/health.rs`, and `src/ai/queue.rs`
- [X] T009 [P] Implement scraper trait/registry contracts with optional latest capability in `src/scraper/traits.rs` and `src/scraper/registry.rs`
- [X] T010 Implement cache path/hash helpers and content-addressable layout utilities in `src/cache/filesystem.rs`

**Checkpoint**: Foundation complete; stories can proceed independently.

---

## Phase 3: User Story 1 - Scrape & Cache Manufacturer Data (Priority: P1) 🎯 MVP

**Goal**: Ingest manufacturer pages via `/scrape`, normalize with Ollama, and persist/cache results with progress feedback.

**Independent Test**: Run `/scrape <manufacturer> <query>` and verify progress, persisted records, cached metadata/images, and non-fatal per-page error handling.

### Tests for User Story 1

- [X] T011 [P] [US1] Create integration test for end-to-end scrape ingestion in `tests/integration/scrape_flow.rs`
- [X] T012 [P] [US1] Create integration test for cache hit reuse (no refetch) in `tests/integration/scrape_cache_hit.rs`

### Implementation for User Story 1

- [X] T013 [P] [US1] Implement manufacturer discovery/extraction pipeline interfaces in `src/scraper/mod.rs` and `src/scraper/manufacturers/mod.rs`
- [X] T014 [US1] Implement caching decorator metadata/image persistence and cache-hit retrieval in `src/scraper/caching_decorator.rs` and `src/cache/filesystem.rs`
- [X] T015 [US1] Implement ingestion worker with per-item isolation and progress events in `src/app/state.rs` and `src/app/events.rs`
- [X] T016 [US1] Implement normalization with knowledge-base injection and `Unnormalized` fallback path in `src/ai/normalize.rs` and `src/ai/knowledge_base.rs`
- [X] T017 [US1] Persist current records and history snapshots for scrape updates in `src/storage/models.rs` and `src/storage/db.rs`
- [X] T018 [US1] Wire `/scrape` command execution and throbber/progress UI updates in `src/app/commands.rs` and `src/tui/widgets.rs`
- [X] T019 [US1] Implement image download + local path mapping for `local_image_paths` in `src/cache/filesystem.rs` and `src/scraper/caching_decorator.rs`

**Checkpoint**: US1 delivers a fully working ingest-and-cache MVP.

---

## Phase 4: User Story 2 - Natural Language Search Against Local Data (Priority: P2)

**Goal**: Execute `/query` with embedding retrieval, grounded response generation, and health-aware fail-fast behavior.

**Independent Test**: Seed records and run `/query` to verify grounded answers, no-match behavior, and friendly health errors.

### Tests for User Story 2

- [X] T020 [P] [US2] Create integration test for semantic query happy path in `tests/integration/query_flow.rs`
- [X] T021 [P] [US2] Create integration test for disconnected/missing-model fail-fast query behavior in `tests/integration/query_health_guard.rs`

### Implementation for User Story 2

- [X] T022 [US2] Implement embedding generation and vector retrieval adapter in `src/storage/vector.rs` and `src/ai/client.rs`
- [X] T023 [US2] Implement query augmentation and grounded answer generation in `src/ai/query.rs` and `src/app/commands.rs`
- [X] T024 [US2] Implement knowledge-base prototype/livery alias query expansion in `src/ai/knowledge_base.rs` and `src/ai/query.rs`
- [X] T025 [US2] Persist query telemetry (`QueryRun`) in `src/storage/models.rs` and `src/storage/db.rs`
- [X] T026 [US2] Render query results, no-match hints, and error states in `src/tui/widgets.rs` and `src/app/state.rs`

**Checkpoint**: US2 works independently on top of local data.

---

## Phase 5: User Story 3 - Sync Latest Arrivals (Priority: P3)

**Goal**: Support `/latest [scraper]` with global/targeted latest discovery, merge-aware ingestion, and summary reporting.

**Independent Test**: Run global and targeted `/latest` and verify capable scraper selection, merge behavior, and summary counts.

### Tests for User Story 3

- [ ] T027 [P] [US3] Create integration test for global `/latest` with skipped-scraper counts in `tests/integration/latest_flow.rs`
- [ ] T028 [P] [US3] Create integration test for targeted `/latest` unsupported/unknown handling in `tests/integration/latest_targeted_errors.rs`
- [ ] T029 [P] [US3] Create integration test for `/latest` fail-fast when Ollama is disconnected in `tests/integration/latest_health_guard.rs`

### Implementation for User Story 3

- [ ] T030 [US3] Implement latest discovery API on scrapers and registry filtering in `src/scraper/traits.rs` and `src/scraper/registry.rs`
- [ ] T031 [US3] Implement concurrent latest job orchestration in `src/app/jobs.rs` and `src/app/commands.rs`
- [ ] T032 [US3] Route latest URLs through existing dedupe/merge pipeline in `src/app/ingest.rs` and `src/ai/merge.rs`
- [ ] T033 [US3] Implement latest completion summaries (inserted/updated/skipped/failed) in `src/app/state.rs` and `src/tui/widgets.rs`
- [ ] T034 [US3] Update `last_scraped_at` semantics for latest-sync touches in `src/storage/models.rs`

**Checkpoint**: US3 provides recurring update-feed behavior.

---

## Phase 6: User Story 4 - Export Matching Records as an Archive (Priority: P4)

**Goal**: Support `/export` producing JSON/CSV plus related image assets in archive/directory output.

**Independent Test**: Export known records and verify JSON/CSV content, image inclusion, and missing-image manifest behavior.

### Tests for User Story 4

- [ ] T035 [P] [US4] Create integration test for JSON/CSV + image export bundle in `tests/integration/export_flow.rs`
- [ ] T036 [P] [US4] Create integration test for missing-image manifest handling in `tests/integration/export_missing_images.rs`

### Implementation for User Story 4

- [ ] T037 [US4] Implement export selection/query projection in `src/export/mod.rs` and `src/storage/models.rs`
- [ ] T038 [US4] Implement JSON and CSV serializers in `src/export/json.rs` and `src/export/csv.rs`
- [ ] T039 [US4] Implement zip/directory bundling and manifest generation in `src/export/archive.rs`
- [ ] T040 [US4] Wire `/export` command responses and output path messaging in `src/app/commands.rs` and `src/tui/widgets.rs`

**Checkpoint**: US4 can be demonstrated and used independently.

---

## Phase 7: User Story 5 - Browse Scrapers, Health Awareness, and TUI Navigation (Priority: P5)

**Goal**: Deliver interactive TUI polish: help/list commands, branding header, health indicator, setup remediation, and graceful quit.

**Independent Test**: Launch app and validate `/help`, `/list-scraper`, header rendering, health transitions, `/setup`, and `/quit` flows.

### Tests for User Story 5

- [ ] T041 [P] [US5] Create contract test for command grammar and aliases in `tests/contract/command_contract.rs`
- [ ] T042 [P] [US5] Create integration test for logo/header + health indicator states in `tests/integration/header_health_view.rs`
- [ ] T043 [P] [US5] Create integration test for `/setup` confirmation and pull workflow in `tests/integration/setup_flow.rs`
- [ ] T044 [P] [US5] Create integration test for `/scrape` fail-fast when Ollama is disconnected in `tests/integration/scrape_health_guard.rs`

### Implementation for User Story 5

- [ ] T045 [US5] Implement `/help` and `/list-scraper` presentation in `src/app/commands.rs` and `src/tui/widgets.rs`
- [ ] T046 [US5] Implement ASCII logo renderer (full/compact variants) and theme styles in `src/tui/logo.rs` and `src/tui/layout.rs`
- [ ] T047 [US5] Implement health-state enum updates and periodic checker task in `src/app/state.rs` and `src/ai/health.rs`
- [ ] T048 [US5] Implement centralized `validate_health_for(command)` guard and apply it to `/scrape`, `/latest`, and `/query` in `src/app/state.rs` and `src/app/commands.rs`
- [ ] T049 [US5] Render Ollama status indicator and grounded header metrics in `src/tui/widgets.rs` and `src/tui/layout.rs`
- [ ] T050 [US5] Implement `/setup` diagnostics, missing-model prompts, and pull orchestration in `src/app/commands.rs` and `src/ai/client.rs`
- [ ] T051 [US5] Implement `/quit` and `/exit` graceful shutdown sequence in `src/app/commands.rs` and `src/main.rs`
- [ ] T052 [US5] Implement optional wheel animation tick with no input blocking in `src/tui/logo.rs` and `src/app/state.rs`

**Checkpoint**: All user stories are independently functional and integrated.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Hardening, cleanup, and full-system validation across stories.

- [ ] T053 [P] Add focused unit tests for knowledge canonicalization and health transitions in `tests/unit/knowledge_base_tests.rs` and `tests/unit/health_state_tests.rs`
- [ ] T054 [P] Add benchmark harness for cache hashing and knowledge lookup hot paths in `tests/unit/perf_benchmarks.rs`
- [ ] T055 Implement command-to-response and startup timing instrumentation with local perf log output in `src/app/perf.rs` and `src/main.rs`
- [ ] T056 Refine cross-command error messages and recovery notices in `src/app/commands.rs` and `src/tui/widgets.rs`
- [ ] T057 Optimize prompt-context filtering and cache reuse in `src/ai/knowledge_base.rs` and `src/ai/normalize.rs`
- [ ] T058 Run quickstart end-to-end validation and update run notes in `specs/001-gauge-ai-tui-app/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- Setup (Phase 1): starts immediately
- Foundational (Phase 2): depends on Setup and blocks all user stories
- User Stories (Phases 3-7): depend on Foundational completion
- Polish (Phase 8): depends on completion of all target user stories

### User Story Dependencies

- US1 (P1): starts after Phase 2; no dependency on other stories
- US2 (P2): starts after Phase 2; depends on US1 data availability for realistic validation
- US3 (P3): starts after Phase 2; reuses ingestion/merge infrastructure from US1
- US4 (P4): starts after Phase 2; consumes persisted data from US1+
- US5 (P5): starts after Phase 2; can run mostly in parallel with US2-US4

### Within Each User Story

- Tests first, then implementation
- Core models/utilities before command handlers
- Command handlers before final UI polish

---

## Parallel Opportunities

- Setup: T003 and T004 can run in parallel
- Foundational: T006, T007, T008, and T009 can run in parallel
- US1: T011 and T012 parallel; T013 and T014 parallelizable
- US2: T020 and T021 parallel; T022 and T024 parallelizable
- US3: T027, T028, and T029 parallel
- US4: T035 and T036 parallel; T038 and T039 parallelizable
- US5: T041, T042, T043, and T044 parallel; T046 and T047 parallelizable
- Polish: T053 and T054 can run in parallel with T056/T057

---

## Parallel Example: User Story 1

```bash
# Parallel tests
T011 [US1] tests/integration/scrape_flow.rs
T012 [US1] tests/integration/scrape_cache_hit.rs

# Parallel core implementation
T013 [US1] src/scraper/mod.rs + src/scraper/manufacturers/mod.rs
T014 [US1] src/scraper/caching_decorator.rs + src/cache/filesystem.rs
```

## Parallel Example: User Story 2

```bash
# Parallel tests
T020 [US2] tests/integration/query_flow.rs
T021 [US2] tests/integration/query_health_guard.rs

# Parallel implementation slices
T022 [US2] src/storage/vector.rs + src/ai/client.rs
T024 [US2] src/ai/knowledge_base.rs + src/ai/query.rs
```

## Parallel Example: User Story 3

```bash
# Parallel tests
T027 [US3] tests/integration/latest_flow.rs
T028 [US3] tests/integration/latest_targeted_errors.rs
```

## Parallel Example: User Story 4

```bash
# Parallel tests
T035 [US4] tests/integration/export_flow.rs
T036 [US4] tests/integration/export_missing_images.rs

# Parallel serializers/packaging
T038 [US4] src/export/json.rs + src/export/csv.rs
T039 [US4] src/export/archive.rs
```

## Parallel Example: User Story 5

```bash
# Parallel tests
T041 [US5] tests/contract/command_contract.rs
T042 [US5] tests/integration/header_health_view.rs
T043 [US5] tests/integration/setup_flow.rs
T044 [US5] tests/integration/scrape_health_guard.rs

# Parallel rendering/health work
T046 [US5] src/tui/logo.rs + src/tui/layout.rs
T047 [US5] src/app/state.rs + src/ai/health.rs
```

---

## Implementation Strategy

### MVP First (US1)

1. Complete Phase 1 and Phase 2
2. Complete Phase 3 (US1)
3. Validate scrape/cache/normalize pipeline independently
4. Demo MVP before expanding scope

### Incremental Delivery

1. Add US2 semantic query capabilities
2. Add US3 latest-sync workflows
3. Add US4 export packaging
4. Add US5 UX/health/setup/quit polish
5. Finish with Phase 8 hardening

### Team Parallelization

1. Team converges on Setup + Foundational first
2. After checkpoint, split by story tracks:
   - Engineer A: US1/US3 ingestion track
   - Engineer B: US2 query/vector track
   - Engineer C: US4 export track
   - Engineer D: US5 TUI/health/setup track

---

## Notes

- `[P]` tasks are safe to run concurrently when dependencies are satisfied.
- `[USx]` labels map directly to user stories in `spec.md`.
- Preserve graceful degradation behavior across DB, cache, image protocol, and Ollama outages.
- Keep knowledge-base canonicalization deterministic before persistence.
