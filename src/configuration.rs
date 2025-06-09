use config::Config;
use kust::ScopeFunctions;
use nameof::name_of;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::convert::TryFrom;
use tracing_log::log;
const APP_ENVIRONMENT: &str = name_of!(APP_ENVIRONMENT);

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
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

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    use std::env::current_dir;
    let configuration_directory = current_dir()
        .unwrap_or_else(|_| panic!("'{}()' failed", name_of!(current_dir)))
        .join("configuration");

    config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base")).required(true))
        .add_source(
            config::File::from(
                configuration_directory.join(
                    std::env::var(APP_ENVIRONMENT)
                        .unwrap_or_else(|_| "local".to_string())
                        .using(Environment::try_from)
                        .expect("Parse env var failed.")
                        .as_str(),
                ),
            )
            .required(true),
        )
        .add_source(config::Environment::with_prefix("app").separator("__"))
        .build()
        .and_then(Config::try_deserialize::<Settings>)
}

impl DatabaseSettings {
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
