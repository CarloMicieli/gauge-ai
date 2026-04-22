use sqlx::{SqlitePool, migrate::Migrator};

use crate::app::error::AppResult;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Run all pending SQL migrations against the application database.
pub async fn run(pool: &SqlitePool) -> AppResult<()> {
    MIGRATOR.run(pool).await.map_err(Into::into)
}
