use anyhow::{Context, Result};
use sea_orm::DatabaseConnection;

use migration::{Migrator, MigratorTrait};

/// Run database migrations
pub async fn migrate(db_connection: &DatabaseConnection) -> Result<()> {
    Migrator::up(db_connection, None)
        .await
        .context("Failed to run database migrations")
}
