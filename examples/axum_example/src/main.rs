//! example of `sea_skipper` usage with an HTTP API built with [`axum`].

use std::net::SocketAddr;

use anyhow::{Context, Result};
use sea_orm::Database;
use secrecy::ExposeSecret;

use axum_example::{api::router, configuration::get_configuration, db::migrate};

#[tokio::main]
pub async fn main() -> Result<()> {
    let config = get_configuration().context("Failed to load config file")?;
    let api_addr: SocketAddr = format!("{}:{}", config.api.host, config.api.port)
        .parse()
        .context("Failed to parse API service host and port configs as socket address")?;
    let db = Database::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Database connection failed");

    // Run database migrations.
    migrate(&db).await?;

    let app = router(db);

    axum::Server::bind(&api_addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
