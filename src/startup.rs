use std::net::TcpListener;

use actix_web::{
    App, HttpServer,
    dev::Server,
    web::{self},
};
use kust::ScopeFunctions;
use reqwest::Client;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::{
    email_client::EmailClient,
    hkt::{RcHKT, RefHKT},
    routes::{health_check, subscribe},
};

type DefaultSharedPointerHKT = RcHKT;

pub fn run<P: RefHKT + 'static>(
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
