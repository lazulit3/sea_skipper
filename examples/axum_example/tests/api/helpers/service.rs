use std::net::TcpListener;

use axum::Server;
use sea_orm::{prelude::Uuid, DatabaseConnection};
use tower::make::Shared;

use sea_orm::{ConnectionTrait, Database, Statement};
use secrecy::ExposeSecret;

use axum_example::{api::router, configuration::get_configuration, db::migrate};

/// Details for connecting to API service and database for tests.
pub struct TestService {
    db_conn: DatabaseConnection,
    api_url: String,
}

impl TestService {
    /// Starts an API service connected to a unique database for use in tests.
    ///
    /// The API service is bound to http://127.0.0.1 on an unused port.
    /// A new database with a random name is created using the database service configured in `config.yml`.
    pub async fn new() -> TestService {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
        let service_addr = listener.local_addr().unwrap();
        let service_url = format!("http://{}", service_addr);
        println!("API service is listening on {}", service_url);

        let db_conn = unique_database().await;
        let router = router(db_conn.clone());

        tokio::spawn(async move {
            let server = Server::from_tcp(listener)
                .unwrap()
                .serve(Shared::new(router));
            server.await.expect("server error");
        });

        TestService {
            db_conn,
            api_url: service_url,
        }
    }

    /// Returns URL of the API service.
    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    /// Returns a `DatabaseConnection` to the unique database created for this test service.
    pub fn database_connection(&self) -> &DatabaseConnection {
        &self.db_conn
    }
}

/// Create a database, run migrations, and return a `DatabaseConnection` for isolated test runs.
pub async fn unique_database() -> DatabaseConnection {
    // Build datatabase connection URL with a random database name.
    let mut config = get_configuration()
        .expect("Failed to read configuration")
        .database;
    config.database_name = Uuid::new_v4().to_string();

    // Connect to database service without selecting a specific database name
    let db = Database::connect(config.connection_string_without_db().expose_secret())
        .await
        .expect("Failed to connect to database service");

    // Create the new database with the configured name
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE \"{}\";", config.database_name),
    ))
    .await
    .expect("Failed to create database");

    // Create a new database connection selecting the newly created database
    let db = Database::connect(config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to newly created database");

    // Run migrations against the newly created database
    migrate(&db).await.unwrap();

    db
}
