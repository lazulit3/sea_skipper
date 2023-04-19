use anyhow::{Context, Result};
use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub api: ApiSettings,
}

#[derive(serde::Deserialize)]
pub struct ApiSettings {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl ApiSettings {
    pub fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");

    let settings = config::Config::builder()
        .add_source(config::File::from(base_path.join("config.yml")))
        .add_source(
            config::Environment::with_prefix("SEA_SKIPPER_AXUM_EXAMPLE")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .context("Failed to build configuration")?;
    settings
        .try_deserialize::<Settings>()
        .context("Failed to deserialize configuration")
}
