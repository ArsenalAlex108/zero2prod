use once_cell::sync::Lazy;
use reqwest::{Client, Url};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::{borrow::Cow, ops::Deref};
use uuid::Uuid;
use zero2prod::{
    configuration::{
        DatabaseSettings, Settings, get_configuration,
    },
    hkt::{RefHKT, SharedPointerHKT},
    startup::{self, Application},
    utils::Pipe,
};

use anyhow::{Context, anyhow};

pub struct TestApp<'a> {
    pub address: Cow<'a, str>,
    pub db_pool: PgPool,
    pub email_server: wiremock::MockServer,
    pub port: u16,
}

pub mod email_server;

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

    pub fn get_confirmation_links<'a>(
        &self,
        email_request: &wiremock::Request,
    ) -> anyhow::Result<ConfirmationLinks<'a>> {
        let body: serde_json::Value =
            email_request.body[..]
                .ref_cast()
                .pipe(serde_json::from_slice)?;

        fn url_parse<'a>(
            body: &serde_json::Value,
            field_name: &str,
        ) -> anyhow::Result<Cow<'a, reqwest::Url>> {
            get_link(body[field_name].as_str().ok_or_else(
                || {
                    anyhow!(format!(
                        "{} parse failed.",
                        field_name
                    ))
                },
            )?)
            .as_str()
            .pipe(Url::parse)?
            .pipe(Cow::<'_, reqwest::Url>::Owned)
            .pipe(Ok)
        }

        ConfirmationLinks {
            plain_text: url_parse(&body, "TextBody")?,
            html: url_parse(&body, "HtmlBody")?,
        }
        .pipe(Ok)
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
    spawn_app_generic::<startup::GlobalSharedPointerType>()
        .await
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

    let email_server = wiremock::MockServer::start().await;

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

    let email_client = {
        let mut email_client =
            configuration.email_client.deref().clone();
        email_client.base_url =
            email_server.uri().pipe(P::from_string);
        email_client.pipe(P::new)
    };

    // Most efficient (with no mutations)
    let configuration = Settings {
        database,
        application,
        email_client,
    };

    configure_database(&configuration.database).await;

    let application = Application::build(&configuration)
        .expect("Application should build successfully.");
    // Get the port before spawning the application
    let address =
        format!("http://127.0.0.1:{}", application.port());

    let port = application.port();

    application
        .pipe(Application::run_until_stopped)
        .pipe(tokio::spawn);

    TestApp {
        address: address.into(),
        db_pool: startup::get_connection_pool(
            &configuration.database,
        ),
        email_server,
        port,
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

pub struct ConfirmationLinks<'a> {
    pub html: Cow<'a, reqwest::Url>,
    pub plain_text: Cow<'a, reqwest::Url>,
}
pub fn get_link(s: &str) -> String {
    let links: Vec<_> = linkify::LinkFinder::new()
        .links(s)
        .filter(|l| *l.kind() == linkify::LinkKind::Url)
        .collect();
    assert_eq!(links.len(), 1);
    links[0].as_str().to_owned()
}
