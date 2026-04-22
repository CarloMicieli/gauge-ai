use std::path::Path;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use crate::app::error::AppResult;

/// Open a SQLite connection pool for the provided database path.
pub async fn connect(db_path: &Path) -> AppResult<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .foreign_keys(true);

    SqlitePool::connect_with(options).await.map_err(Into::into)
}
