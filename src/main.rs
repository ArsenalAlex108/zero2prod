use reqwest::Client;
use zero2prod::{
    configuration::get_configuration,
    hkt::SharedPointerHKT,
    startup::{self, Application},
    telemetry::{get_subscriber, init_subscriber},
};

pub const INFO: &str = "info";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    main_generic::<startup::GlobalSharedPointerType>().await
}

async fn main_generic<P: SharedPointerHKT>()
-> std::io::Result<()>
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

    Application::build(&configuration)?
        .run_until_stopped()
        .await
}
