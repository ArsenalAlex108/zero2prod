use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::{
    configuration::get_configuration,
    startup,
    telemetry::{
        get_subscriber, init_subscriber,
    },
};

pub const INFO: &str = "info";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        stringify!(zero2prod).into(),
        INFO.into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration()
        .expect(
            "Failed to find configuration file.",
        );

    let connection_pool = PgPoolOptions::new()
        .idle_timeout(
            std::time::Duration::from_secs(2),
        )
        .connect_lazy_with(
            configuration.database.with_db(),
        );

    TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    ))
    .and_then(|i| {
        startup::run(i, connection_pool)
    })?
    .await
}
