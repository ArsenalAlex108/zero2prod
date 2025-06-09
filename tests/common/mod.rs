use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configuration::{DatabaseSettings, get_configuration},
    startup,
};

#[allow(dead_code, reason = "Used by tests.")]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub const TEST: &str = "test";
pub const DEBUG: &str = "debug";
pub const TEST_LOG: &str = "TEST_LOG";

// TODO: Use trait impl instead.
macro_rules! init_subscriber_from_env {
    ($n: path) => {{
        let subscriber = zero2prod::telemetry::get_subscriber(TEST.into(), DEBUG.into(), $n);
        zero2prod::telemetry::init_subscriber(subscriber);
    }};
}

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var(TEST_LOG).is_ok() {
        init_subscriber_from_env!(std::io::stdout)
    } else {
        init_subscriber_from_env!(std::io::sink)
    }
});

/// Spinup an instance of our application
/// and returns its address(i.e.http://localhost:XXXX)
pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    startup::run(listener, connection_pool.clone())
        .map(tokio::spawn)
        .expect("Failed to start server.");

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
