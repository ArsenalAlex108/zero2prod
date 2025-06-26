use std::{borrow::Cow, net::TcpListener, ops::Deref};

use actix_web::{
    App, HttpServer,
    dev::Server,
    web::{self},
};
use kust::ScopeFunctions;
use reqwest::Client;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::{DatabaseSettings, Settings},
    email_client::EmailClient,
    hkt::{
        ArcHKT, HKT1Unsized, K1, RcHKT, RefHKT,
        SharedPointerHKT,
    },
    routes::{
        confirm_subscription_token, health_check, subscribe,
    },
    utils::Pipe,
};

pub type GlobalSharedPointerType = ArcHKT;

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl<P: HKT1Unsized>(
    pub K1<P, str>,
);

pub fn run<P: RefHKT>(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient<P>,
    // Cow<'_, str> can be alternative using into_owned() (clone at most once).
    // But String is more honest at that point
    // Arc-like can be shared accross threads without cloning at the cost of intial memory allocation.
    base_url: K1<P, str>,
) -> std::io::Result<Server>
where
    P::T<Client>: Send + Sync,
    P::T<str>: Send + Sync,
{
    let db_pool = db_pool.pipe(web::Data::new);
    let email_client = email_client.pipe(web::Data::new);
    let base_url = base_url
        .pipe(ApplicationBaseUrl)
        .pipe(web::Data::new);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route(
                "/health_check",
                web::get().to(health_check),
            )
            .route(
                "/subscriptions",
                web::post().to(subscribe),
            )
            .route(
                "/subscriptions/confirm",
                web::get().to(confirm_subscription_token),
            )
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

impl Application {
    pub fn build<P: SharedPointerHKT>(
        configuration: &Settings<P>,
    ) -> std::io::Result<Application>
    where
        P::T<Client>: Send + Sync,
        P::T<str>: Send + Sync,
    {
        let connection_pool =
            get_connection_pool(&configuration.database);

        let sender_email =
            configuration.email_client.sender().unwrap();

        let email_client = EmailClient::new(
            configuration.email_client.base_url.clone(),
            sender_email,
            configuration
                .email_client
                .authorization_token
                .clone(),
            configuration.email_client.timeout(),
        );

        let address = format!(
            "{}:{}",
            configuration.application.host,
            configuration.application.port
        );

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        run::<P>(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url.clone(),
        )
        .map(|server| Application { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(
        self,
    ) -> std::io::Result<()> {
        self.server.await
    }
}

pub fn get_connection_pool<P: RefHKT>(
    configuration: &DatabaseSettings<P>,
) -> PgPool {
    PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
