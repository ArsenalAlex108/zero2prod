use eyre::Context;
use secrecy::SecretString;
use std::{
    convert::identity,
    fmt::{Debug, Display},
};
use tokio::task::JoinError;

use reqwest::Client;
use zero2prod::{
    configuration::{
        ApplicationSettings, DatabaseSettings,
        EmailClientSettings, get_configuration,
    },
    hkt::SharedPointerHKT,
    issue_delivery_worker,
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
    P::T<DatabaseSettings<P>>: Send + Sync,
    P::T<ApplicationSettings<P>>: Send + Sync,
    P::T<EmailClientSettings<P>>: Send + Sync,
{
    let subscriber = get_subscriber(
        stringify!(zero2prod).into(),
        INFO.into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration::<P>()
        .expect("Failed to find configuration file.");

    let application = Application::build(&configuration)
        .await?
        .run_until_stopped()
        .pipe(tokio::spawn);
    // .await
    // .context("Application should run successfully.")

    let worker =
        issue_delivery_worker::run_worker_until_stopped(
            configuration,
        )
        .pipe(tokio::spawn);

    tokio::select! {
        i = application => report_exit("Application", i),
        i = worker => report_exit("Background Worker", i),
    };

    Ok(())
}

fn report_exit(
    task_name: &str,
    outcome: Result<
        Result<(), impl Debug + Display>,
        JoinError,
    >,
) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
            error.cause_chain = ?e,
            error.message = %e,
            "{} failed",
            task_name
            )
        }
        Err(e) => {
            tracing::error!(
            error.cause_chain = ?e,
            error.message = %e,
            "{}' task failed to complete",
            task_name
            )
        }
    }
}
