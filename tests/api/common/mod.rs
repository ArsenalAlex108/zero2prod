use core::str;
use eyre::Context;
use once_cell::sync::Lazy;
use reqwest::Url;
use secrecy::ExposeSecret;
use secrecy::SecretString;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::{borrow::Cow, ops::Deref};
use uuid::Uuid;
use zero2prod::dependency_injection::app_state::AppState;
use zero2prod::dependency_injection::app_state::AppStateFactory;
use zero2prod::dependency_injection::app_state::AppStateTypes;
use zero2prod::dependency_injection::app_state::DefaultAppStateFactory;
use zero2prod::dependency_injection::app_state::DefaultAppStateTypes;
use zero2prod::dependency_injection::app_state::IssueDeliveryWorkerTypes;
use zero2prod::email_client::EmailClient;
use zero2prod::hkt::SendHKT;
use zero2prod::hkt::SyncHKT;
use zero2prod::issue_delivery_worker::IssueDeliveryWorkerDependencies;
use zero2prod::issue_delivery_worker::SingleNewsletterPickingAndSendingTaskResult;
use zero2prod::issue_delivery_worker::get_single_newsletter_picking_and_sending_iterator;
use zero2prod::{
    authentication::BasicAuthCredentials,
    configuration::{
        DatabaseSettings, Settings, get_configuration,
    },
    hkt::{RefHKT, SharedPointerHKT},
    startup::{self, Application},
    utils::Pipe,
};

use argon2::{
    Argon2, PasswordHash, PasswordHasher,
    password_hash::SaltString,
};

use anyhow::anyhow;

use crate::common::test_dependency_injection::test_app_state::TestAppState;
use crate::common::test_dependency_injection::test_app_state::TestAppStateFactory;
use crate::common::test_dependency_injection::test_app_state::TestAppStateFactoryImpl;
use crate::common::test_dependency_injection::test_app_state::TestAppStateTypes;
use crate::common::test_dependency_injection::test_app_state::TestAppTypesImpl;
use crate::common::test_dependency_injection::test_database::insert_newsletter_writer_repository::InsertNewsletterWriterRepository as _;

pub mod email_server;
pub mod test_dependency_injection;

pub struct TestApp<
    'a,
    P: RefHKT = startup::GlobalSharedPointerType,
    A: AppStateTypes = DefaultAppStateTypes,
    TA: TestAppStateTypes = TestAppTypesImpl,
> {
    pub address: Cow<'a, str>,
    pub email_server: wiremock::MockServer,
    pub port: u16,
    pub http_client: reqwest::Client,
    pub email_client: EmailClient<P>,
    pub app_state: AppState<A>,
    pub test_app_state: TestAppState<TA>,
}

impl<P: RefHKT> TestApp<'_, P> {
    pub async fn post_subscriptions(
        &self,
        body: impl Into<reqwest::Body>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.http_client
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

    pub async fn post_newsletter(
        &self,
        body: &impl serde::Serialize,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.http_client
            .post(format!(
                "{}/admin/newsletters",
                self.address.as_ref()
            ))
            .form(body)
            .send()
            .await
    }

    pub async fn get_newsletter_form(
        &self,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.http_client
            .get(format!(
                "{}/admin/newsletters",
                self.address.as_ref()
            ))
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

    pub async fn post_login<Body: serde::Serialize>(
        &self,
        body: &Body,
    ) -> Result<reqwest::Response, eyre::Report> {
        self.http_client
            .post(format!("{}/login", &self.address))
            .form(body)
            .send()
            .await
            .context("Login HTTP Request should succeed.")
    }

    pub async fn post_login_with_default(
        &self,
    ) -> Result<reqwest::Response, eyre::Report> {
        let test_user = get_test_newsletter_writer();

        self.post_login(&serde_json::json!({
            "username": test_user.username.as_ref(),
            "password": test_user.raw_password.as_ref().expose_secret(),
        })).await
    }

    pub async fn get_login_html(
        &self,
    ) -> Result<String, eyre::Report> {
        self.http_client
            .get(format!("{}/login", &self.address))
            .send()
            .await
            .context("Login HTTP Request should succeed.")?
            .text()
            .await
            .context("Failed to get response body as text.")
    }

    pub async fn get_admin_dashboard(
        &self,
    ) -> Result<reqwest::Response, eyre::Report> {
        self.http_client
        .get(format!("{}/admin/dashboard", &self.address))
        .send()
        .await
        .context("Admin Dashboard HTTP Request should succeed.")
    }

    pub async fn get_admin_dashboard_html(
        &self,
    ) -> Result<String, eyre::Report> {
        self.http_client
        .get(format!("{}/admin/dashboard", &self.address))
        .send()
        .await
        .context("Admin Dashboard HTTP Request should succeed.")?
        .text()
        .await
        .context("Failed to get response body as text.")
    }

    pub async fn get_reset_password_form(
        &self,
    ) -> Result<reqwest::Response, eyre::Report> {
        self.http_client
        .get(format!("{}/admin/reset_password", &self.address))
        .send()
        .await
        .context("Accessing admin reset password should always receive a response.")
    }

    pub async fn post_reset_password(
        &self,
        body: &impl serde::Serialize,
    ) -> Result<reqwest::Response, eyre::Report> {
        self.http_client
        .post(format!("{}/admin/reset_password", &self.address))
        .form(body)
        .send()
        .await
        .context("Request password reset should always return response.")
    }

    pub async fn post_logout(
        &self,
    ) -> Result<reqwest::Response, eyre::Report> {
        self.http_client
            .post(format!("{}/admin/logout", &self.address))
            .send()
            .await
            .context(
                "Logout should always return response.",
            )
    }
}

impl<
    P: SharedPointerHKT + SendHKT + SyncHKT,
    A: AppStateTypes,
> TestApp<'_, P, A>
{
    pub async fn dispatch_all_pending_emails(&self) {
        let dependencies =
            IssueDeliveryWorkerDependencies::<
                IssueDeliveryWorkerTypes<P, A>,
            > {
                issue_delivery_queue_repository: &self
                    .app_state
                    .issue_delivery_queue_repository,
                begin_unit_of_work: &self
                    .app_state
                    .begin_unit_of_work,
                newsletters_repository: &self
                    .app_state
                    .newsletters_repository,
                email_client: &self.email_client,
            };

        let iterator = get_single_newsletter_picking_and_sending_iterator(
            &dependencies,
        );

        for task in iterator {
            if let SingleNewsletterPickingAndSendingTaskResult::NothingFound = task.await {
                break;
            }
        }
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
pub async fn spawn_app<'a>()
-> TestApp<'a, startup::GlobalSharedPointerType> {
    spawn_app_generic::<
        startup::GlobalSharedPointerType,
        DefaultAppStateFactory,
        TestAppStateFactoryImpl,
    >()
    .await
}

pub async fn spawn_app_generic<
    'a,
    P: SharedPointerHKT + SendHKT + SyncHKT,
    A: AppStateFactory,
    TA: TestAppStateFactory<AppStateTypes = A::AppStateTypes>,
>()
-> TestApp<'a, P, A::AppStateTypes, TA::TestAppStateTypes> {
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

    let app_state = A::build(&configuration);

    let application = Application::build_with(
        &configuration,
        app_state.clone(),
    )
    .await
    .expect("Application should build successfully.");
    // Get the port before spawning the application
    let address =
        format!("http://127.0.0.1:{}", application.port());

    let port = application.port();

    application
        .pipe(Application::run_until_stopped)
        .pipe(tokio::spawn);

    let http_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .expect("reqwest ClientBuilder is valid.");

    let test_app_state = TA::build(&app_state);

    TestApp {
        address: address.into(),
        email_server,
        port,
        http_client,
        email_client: configuration
            .email_client
            .as_ref()
            .clone()
            .client(),
        app_state,
        test_app_state,
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

pub fn get_test_newsletter_writer<'a>()
-> BasicAuthCredentials<'a> {
    BasicAuthCredentials {
        username: "test_user".pipe(Cow::Borrowed),
        raw_password: "supersecret"
            .pipe(Box::<str>::from)
            .pipe(SecretString::new)
            .pipe(Cow::Owned),
    }
}

pub async fn create_test_newsletter_writer(
    app: &TestApp<'_>,
) {
    let test_newsletter_writer =
        get_test_newsletter_writer();
    let salt =
        SaltString::generate(&mut rand::thread_rng());
    let hash = hash_password(
        &test_newsletter_writer.raw_password,
        &salt,
    );
    let hash = hash.serialize();
    let hash = hash.as_str();
    let hash = hash.pipe(SecretString::from);

    let user_id = Uuid::new_v4();

    app.test_app_state
        .insert_newsletter_writer_repository
        .insert(
            user_id,
            test_newsletter_writer.username.as_ref(),
            &hash,
        )
        .await
        .expect("Inserting test user should succeed.");
}

pub fn hash_password<'a>(
    password: &SecretString,
    salt: &'a SaltString,
) -> PasswordHash<'a> {
    let hasher = Argon2::default();
    let hash = hasher
        .hash_password(
            password.expose_secret().as_bytes(),
            salt,
        )
        .unwrap();

    hash
}

pub fn assert_is_redirect_to(
    response: &reqwest::Response,
    uri_path: &str,
) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        uri_path
    );
}
