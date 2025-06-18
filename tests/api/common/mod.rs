use once_cell::sync::Lazy;
use reqwest::Client;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::{borrow::Cow, ops::Deref};
use uuid::Uuid;
use zero2prod::{
    configuration::{
        DatabaseSettings, Settings, get_configuration,
    },
    hkt::{ArcHKT, RefHKT, SharedPointerHKT},
    startup::{self, Application},
    utils::Pipe,
};

pub struct TestApp<'a> {
    pub address: Cow<'a, str>,
    pub db_pool: PgPool,
}

impl TestApp<'_> {
    pub async fn post_subscriptions(
        &self,
        body: impl Into<reqwest::Body>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        reqwest::Client::new()
            .post(format!(
                "{}/subscriptions",
                &self.address
            ))
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded",
            )
            .body(body)
            .send()
            .await
    }
}

pub const TEST: &str = "test";
pub const DEBUG: &str = "debug";
pub const TEST_LOG: &str = "TEST_LOG";

// TODO: Use trait impl instead.
macro_rules! init_subscriber_from_env {
    ($n: path) => {{
        let subscriber =
            zero2prod::telemetry::get_subscriber(
                TEST.into(),
                DEBUG.into(),
                $n,
            );
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
pub async fn spawn_app<'a>() -> TestApp<'a> {
    spawn_app_generic::<ArcHKT>().await
}

pub async fn spawn_app_generic<'a, P: SharedPointerHKT>()
-> TestApp<'a>
where
    P::T<str>: Send + Sync,
    P::T<Client>: Send + Sync,
{
    Lazy::force(&TRACING);

    let configuration = get_configuration::<P>()
        .expect("Failed to read configuration");

    // Least efficient (2 copies!)
    let database = configuration
        .database
        .deref()
        .clone()
        .pipe(|i| DatabaseSettings {
            database_name: Uuid::new_v4()
                .to_string()
                .pipe(P::from_string),
            ..i
        })
        .pipe(P::new);

    // Slightly less efficient (unneeded copied fields)
    let application = {
        let mut application =
            configuration.application.deref().clone();
        application.port = 0;
        application.pipe(P::new)
    };

    // Most efficient (with no mutations)
    let configuration = Settings {
        database,
        application,
        email_client: configuration.email_client,
    };

    configure_database(&configuration.database).await;

    let application = Application::build(&configuration)
        .expect("Application should build successfully.");
    // Get the port before spawning the application
    let address =
        format!("http://127.0.0.1:{}", application.port());

    application
        .pipe(Application::run_until_stopped)
        .pipe(tokio::spawn);

    TestApp {
        address: address.into(),
        db_pool: startup::get_connection_pool(
            &configuration.database,
        ),
    }
}

pub async fn configure_database<P: RefHKT>(
    config: &DatabaseSettings<P>,
) -> PgPool {
    // Create database
    let mut connection =
        PgConnection::connect_with(&config.without_db())
            .await
            .expect("Failed to connect to Postgres");
    connection
        .execute(
            format!(
                r#"CREATE DATABASE "{}";"#,
                config.database_name.deref()
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool =
        PgPool::connect_with(config.with_db())
            .await
            .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
