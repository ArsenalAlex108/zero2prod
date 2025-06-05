use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup, telemetry::{get_subscriber, init_subscriber}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber(
        stringify!(zero2prod).into(), "info".into()
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to find configuration file.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect(""); 
    TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .and_then(|i| startup::run(i, connection_pool))?
        .await
}