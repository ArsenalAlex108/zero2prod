use config::Config;
use const_format::formatcp;
use naan::apply::Apply;
use naan::fun::F2Once;
use naan::semigroup::Semigroup;
use nameof::name_of;
use serde::de;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::convert::TryFrom;
use std::marker::PhantomData;
use tracing_log::log;

use crate::domain::{
    SubscriberEmail, SubscriberEmailParseError,
};
use crate::hkt::{
    K1, RefHKT, SharedPointerHKT, Validation, ValidationHKT,
};
use crate::serde::DeserializeError;
use crate::utils::{self, Pipe};

pub mod serde_impl;

const APP_ENVIRONMENT: &str = name_of!(APP_ENVIRONMENT);

//#[derive(serde::Deserialize)]
#[derive(derive_more::Constructor)]
pub struct Settings<P: RefHKT> {
    pub database: DatabaseSettings<P>,
    pub application: ApplicationSettings<P>,
    pub email_client: EmailClientSettings<P>,
}

#[derive(derive_more::Constructor)]
pub struct DatabaseSettings<P: RefHKT> {
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

#[derive(derive_more::Constructor)]
pub struct ApplicationSettings<P: RefHKT> {
    // #[serde(
    //     deserialize_with = "deserialize_number_from_string"
    // )]
    pub port: u16,
    pub host: K1<P, str>,
}

#[derive(derive_more::Constructor)]
pub struct EmailClientSettings<P: RefHKT> {
    pub base_url: K1<P, str>,
    pub sender_email: K1<P, str>,
}

impl<P: SharedPointerHKT> EmailClientSettings<P> {
    pub fn sender(
        &self,
    ) -> Result<SubscriberEmail<P>, SubscriberEmailParseError>
    {
        SubscriberEmail::try_from(self.sender_email.clone())
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

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use nameof::name_of;

    use crate::{
        configuration::EmailClientSettings,
        hkt::{BoxHKT, RcHKT},
    };

    #[test]
    fn email_client_settings_deserialize() {
        use std::env::current_dir;
        let configuration_directory = current_dir()
            .unwrap_or_else(|_| {
                panic!(
                    "'{}()' failed",
                    name_of!(current_dir)
                )
            })
            .join("tests/resources");

        let value = config::Config::builder()
            .add_source(
                config::File::from(
                    configuration_directory
                        .join("email_client_settings"),
                )
                .required(true),
            )
            .build()
            .and_then(
                config::Config::try_deserialize::<
                    EmailClientSettings<RcHKT>,
                >,
            )
            .unwrap();
        print!("{}, {}", value.base_url.deref(), value.sender_email.deref())
    }
}
