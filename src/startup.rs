use std::net::TcpListener;

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
        RcHKT, RefHKT,
        SharedPointerHKT,
    },
    routes::{health_check, subscribe},
};

type DefaultSharedPointerHKT = RcHKT;

pub struct Application {
    port: u16,
    server: Server,
}

pub fn run<P: RefHKT>(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient<P>,
) -> std::io::Result<Server>
where
    P::T<Client>: Send + Sync,
    P::T<str>: Send + Sync,
{
    let db_pool = db_pool.using(web::Data::new);
    let email_client = email_client.using(web::Data::new);
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
            // .route(
            //     "/subscriptions/confirm",
            //     web::get().to(confirm_subscription_token::<DefaultSharedPointerHKT>)
            // )
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
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

        run::<P>(listener, connection_pool, email_client)
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
