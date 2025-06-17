use config::Config;
use nameof::name_of;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::convert::TryFrom;
use tracing_log::log;

use crate::domain::{
    SubscriberEmail, SubscriberEmailParseError,
};
use crate::hkt::{
    HKT1Unsized, K1, RefHKT, SharedPointerHKT,
};
use crate::utils::Pipe;

pub mod generated;

const APP_ENVIRONMENT: &str = name_of!(APP_ENVIRONMENT);

//#[derive(serde::Deserialize)]
#[derive(derive_more::Constructor)]
pub struct Settings<P: HKT1Unsized> {
    pub database: K1<P, DatabaseSettings<P>>,
    pub application: K1<P, ApplicationSettings<P>>,
    pub email_client: K1<P, EmailClientSettings<P>>,
}

impl<P: SharedPointerHKT> Clone for Settings<P> {
    fn clone(&self) -> Self {
        Self {
            database: self.database.clone(),
            application: self.application.clone(),
            email_client: self.email_client.clone(),
        }
    }
}

#[derive(derive_more::Constructor)]
pub struct DatabaseSettings<P: HKT1Unsized> {
    pub username: K1<P, str>,
    pub password: K1<P, str>,
    // #[serde(
    //     deserialize_with = "deserialize_number_from_string"
    // )]
    pub port: u16,
    pub host: K1<P, str>,
    pub database_name: K1<P, str>,
    pub require_ssl: bool,
}

impl<P: SharedPointerHKT> Clone for DatabaseSettings<P> {
    fn clone(&self) -> Self {
        Self {
            username: self.username.clone(),
            password: self.password.clone(),
            port: self.port.clone(),
            host: self.host.clone(),
            database_name: self.database_name.clone(),
            require_ssl: self.require_ssl.clone(),
        }
    }
}

#[derive(derive_more::Constructor)]
pub struct ApplicationSettings<P: HKT1Unsized> {
    // #[serde(
    //     deserialize_with = "deserialize_number_from_string"
    // )]
    pub port: u16,
    pub host: K1<P, str>,
}

impl<P: SharedPointerHKT> Clone for ApplicationSettings<P> {
    fn clone(&self) -> Self {
        Self {
            port: self.port,
            host: self.host.clone(),
        }
    }
}

//#[derive(serde::Deserialize)]
#[derive(derive_more::Constructor)]
pub struct EmailClientSettings<P: HKT1Unsized> {
    pub base_url: K1<P, str>,
    pub sender_email: K1<P, str>,
    pub authorization_token: K1<P, str>,
    pub timeout_milliseconds: u64,
}

impl<P: HKT1Unsized> EmailClientSettings<P> {
    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(
            self.timeout_milliseconds,
        )
    }
}

impl<P: SharedPointerHKT> EmailClientSettings<P> {
    pub fn sender(
        &self,
    ) -> Result<SubscriberEmail<P>, SubscriberEmailParseError>
    {
        SubscriberEmail::try_from(self.sender_email.clone())
    }
}

impl<P: SharedPointerHKT> Clone for EmailClientSettings<P> {
    fn clone(&self) -> Self {
        Self {
            base_url: self.base_url.clone(),
            sender_email: self.sender_email.clone(),
            authorization_token: self
                .authorization_token
                .clone(),
            timeout_milliseconds: self.timeout_milliseconds,
        }
    }
}

pub enum Environment {
    Local,
    Production,
}
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub fn get_configuration<P: RefHKT>()
-> Result<Settings<P>, config::ConfigError> {
    use std::env::current_dir;
    let configuration_directory = current_dir()
        .unwrap_or_else(|_| {
            panic!("'{}()' failed", name_of!(current_dir))
        })
        .join("configuration");

    config::Config::builder()
        .add_source(
            config::File::from(
                configuration_directory.join("base"),
            )
            .required(true),
        )
        .add_source(
            config::File::from(
                configuration_directory.join(
                    std::env::var(APP_ENVIRONMENT)
                        .unwrap_or_else(|_| {
                            "local".to_string()
                        })
                        .pipe(Environment::try_from)
                        .expect("Parse env var failed.")
                        .as_str(),
                ),
            )
            .required(true),
        )
        .add_source(
            config::Environment::with_prefix("app")
                .separator("__"),
        )
        .build()
        .and_then(Config::try_deserialize::<Settings<P>>)
}

impl<P: RefHKT> DatabaseSettings<P> {
    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(if self.require_ssl {
                PgSslMode::Require
            } else {
                PgSslMode::Prefer
            })
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.database_name)
            .log_statements(log::LevelFilter::Trace)
    }
}
