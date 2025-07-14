use eyre::Context;
use secrecy::SecretString;
use std::convert::identity;

use reqwest::Client;
use zero2prod::{
    configuration::get_configuration,
    hkt::SharedPointerHKT,
    startup::{self, Application},
    telemetry::{get_subscriber, init_subscriber},
    utils::Pipe,
};

pub const INFO: &str = "info";

#[actix_web::main]
async fn main() -> Result<(), eyre::Report> {
    main_generic::<startup::GlobalSharedPointerType>().await
}

async fn main_generic<P: SharedPointerHKT>()
-> Result<(), eyre::Report>
where
    P::T<str>: Send + Sync,
    P::T<Client>: Send + Sync,
    P::T<SecretString>: Send + Sync,
{
    let subscriber = get_subscriber(
        stringify!(zero2prod).into(),
        INFO.into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration::<P>()
        .expect("Failed to find configuration file.");

    Application::build(&configuration)
        .await?
        .pipe(|i| {
            use rodio::{
                Decoder, OutputStream, source::Source,
            };
            use std::fs::File;
            use std::io::BufReader;

            let (_stream, stream_handle) =
                OutputStream::try_default().expect(
                    "rodio default OutputStream success.",
                );

            File::open("audio/startup_finished.mp3")
                .ok()
                .map(BufReader::new)
                .map(Decoder::new)
                .map(Result::ok)
                .and_then(identity)
                .map(|source| {
                    stream_handle
                        .play_raw(source.convert_samples())
                });

            dbg!("Starting...");

            i
        })
        .run_until_stopped()
        .await
        .context("Application should run successfully.")
}
