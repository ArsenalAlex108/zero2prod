use std::net::TcpListener;

use kust::ScopeFunctions;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use zero2prod::{
    configuration::get_configuration,
    domain::SubscriberEmail,
    email_client::EmailClient,
    hkt::{BoxHKT, RefHKT},
    startup,
    telemetry::{get_subscriber, init_subscriber},
};

pub const INFO: &str = "info";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    main_generic::<BoxHKT>().await
}

async fn main_generic<P: RefHKT>() -> std::io::Result<()>
where
    P::T<str>: Send + Sync,
    P::T<Client>: Send + Sync,
{
    let subscriber = get_subscriber(
        stringify!(zero2prod).into(),
        INFO.into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration::<P>()
        .expect("Failed to find configuration file.");

    let connection_pool = PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(
            configuration.database.with_db(),
        );

    TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    ))
    .and_then(|i| {
        startup::run::<P>(
            i,
            connection_pool,
            EmailClient::new(
                Box::<str>::from("").using(P::from_box),
                SubscriberEmail::try_from(
                    Box::<str>::from(
                        "ursula_le_guin@gmail.com",
                    )
                    .using(P::from_box),
                )
                .unwrap(),
            ),
        )
    })?
    .await
}
