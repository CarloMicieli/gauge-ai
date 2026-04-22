CREATE TABLE IF NOT EXISTS model_data (
    manufacturer TEXT NOT NULL,
    product_code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    details TEXT NOT NULL DEFAULT '',
    scale TEXT,
    epoch TEXT,
    railway_company TEXT,
    normalization_status TEXT NOT NULL,
    source_fingerprint TEXT NOT NULL,
    last_scraped_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (manufacturer, product_code)
);

CREATE TABLE IF NOT EXISTS model_versions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    manufacturer TEXT NOT NULL,
    product_code TEXT NOT NULL,
    snapshot_json TEXT NOT NULL,
    change_reason TEXT NOT NULL,
    merged_by_model TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS merge_audit (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    manufacturer TEXT NOT NULL,
    product_code TEXT NOT NULL,
    outcome TEXT NOT NULL,
    rejection_reason TEXT,
    model_used TEXT NOT NULL,
    prompt_hash TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS query_run (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    query_text TEXT NOT NULL,
    top_k INTEGER NOT NULL,
    latency_ms INTEGER NOT NULL,
    result_count INTEGER NOT NULL,
    created_at TEXT NOT NULL
);
