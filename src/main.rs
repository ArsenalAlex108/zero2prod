use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to find configuration file.");
    TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
    .and_then(startup::run)
    ?.await
}
