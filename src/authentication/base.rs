use std::borrow::Cow;

use crate::{
    authentication, telemetry,
    utils::{self, Pipe},
};
use actix_web::web;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier, password_hash,
};
use base64::Engine;
use eyre::eyre;
use eyre::{ContextCompat, WrapErr};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BasicAuthCredentials<'a> {
    pub username: Cow<'a, str>,
    // Implying a secret cannot be created from a reference instead of ownership.
    pub raw_password: Cow<'a, SecretString>,
}

impl BasicAuthCredentials<'_> {
    pub fn into_owned<'b>(
        self,
    ) -> BasicAuthCredentials<'b> {
        BasicAuthCredentials {
            username: self.username.into_owned().into(),
            raw_password: self
                .raw_password
                .into_owned()
                .pipe(Cow::Owned),
        }
    }

    pub fn from_strings(
        username: &str,
        raw_password: String,
    ) -> BasicAuthCredentials<'_> {
        BasicAuthCredentials {
            username: username.into(),
            raw_password: raw_password
                .pipe(Box::<str>::from)
                .pipe(SecretString::new)
                .pipe(Cow::Owned),
        }
    }
}

pub fn basic_authentication(
    headers: &actix_web::http::header::HeaderMap,
) -> Result<BasicAuthCredentials<'_>, eyre::Report> {
    // Get header value
    let authorization_value = headers.get("Authorization")
    .context("The 'Authorization' header was missing.")?
    .to_str()
    .context("Value of 'Authorization' header is not valid UTF-8")?;

    // Confirm authorization scheme is basic
    let encoded_credentials = authorization_value
        .strip_prefix("Basic ")
        .context("Authorization Scheme was not 'Basic'")?;

    // Get & decode base64
    let decoded_credentials = encoded_credentials
    .pipe(|i| base64::engine::general_purpose::STANDARD.decode(i))
    .context("Failed to base64 decode credentials bytes.")?
    .pipe(String::from_utf8)
    .context("Decoded credentials bytes were not valid utf8.")?;
    let (username, password, ..) =
        decoded_credentials.split_once(':').context(
            "Both username and password must be provided",
        )?;

    // Salt password

    BasicAuthCredentials {
        username: Cow::<str>::from(username.to_string()),
        raw_password: Cow::Owned(SecretString::from(
            password.to_string(),
        )),
    }
    .pipe(Ok)
}

pub fn validate_password(
    password: &SecretString,
    salted_password: &SecretString,
) -> Result<bool, eyre::Report> {
    match Argon2::default().verify_password(
        password.expose_secret().as_bytes(),
        PasswordHash::new(salted_password.expose_secret())
            .context("Failed to parse password hash")?
            .ref_cast(),
    ) {
        Ok(_) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(e) => e
            .pipe(eyre::Report::new)
            .wrap_err("Failed to validate password.")
            .pipe(Err),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NewsletterWritersAuthenticationError {
    #[error("Authentication failed.")]
    Authentication(
        #[source] lazy_errors::Error<eyre::Report>,
    ),
    #[error(transparent)]
    Unexpected(#[from] lazy_errors::Error<eyre::Report>),
}

impl From<NewsletterWritersAuthenticationError>
    for lazy_errors::Error<eyre::Report>
{
    fn from(
        val: NewsletterWritersAuthenticationError,
    ) -> Self {
        val.pipe(eyre::Report::new)
            .pipe(lazy_errors::Error::wrap)
    }
}

#[tracing::instrument(
    name = "Authenticating Newsletter Writer",
    skip(pool, credentials)
)]
pub async fn authenticate_newsletter_writer(
    pool: &PgPool,
    credentials: BasicAuthCredentials<'_>,
) -> Result<Uuid, NewsletterWritersAuthenticationError> {
    let query_result = sqlx::query!("--sql
        SELECT user_id, salted_password
        FROM newsletter_writers
        WHERE username = $1",
        &credentials.username)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => NewsletterWritersAuthenticationError::Authentication(eyre!("Username not found.").pipe(lazy_errors::Error::wrap)),
        e => NewsletterWritersAuthenticationError::Unexpected(e
            .pipe(eyre::Report::new)
            .pipe(lazy_errors::Error::wrap)
        )
    });

    let credentials = credentials.into_owned();
    let raw_password = credentials.raw_password;

    let (record_result, query_error) = query_result
        .pipe(utils::unpack_result_to_result_tuple);

    let (salted_password, user_id) = match record_result {
        Ok(t) => (Some(t.salted_password), Some(t.user_id)),
        Err(_) => (None, None),
    };

    // Hides whether user exists.
    let salted_password = salted_password
    .unwrap_or_else(|| "$argon2id$v=19$m=15000,t=2,p=1$\
                            gZiV/M1gPc22ElAH/Jh1Hw$\
                            CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno".to_string());

    let validation_result =
        telemetry::spawn_blocking_with_tracing(move || {
            authentication::validate_password(
                &raw_password,
                &salted_password
                    .pipe(Box::<str>::from)
                    .pipe(SecretString::new),
            )
        })
        .await;

    query_error?;

    validation_result
    .context("Failed to spawn a new thread to validate password.")
    .map_err(lazy_errors::Error::wrap)?
    .context("Error occurred trying to validate password.")
    .map_err(lazy_errors::Error::wrap)?
    .pipe(|i|
        if i { user_id
            .expect("query_error was validated, so query was successful.") 
            .pipe(Ok) }
        else { NewsletterWritersAuthenticationError::Authentication(
                eyre!("Incorrect password.")
                .pipe(lazy_errors::Error::wrap)
            ).pipe(Err)
        }
    )
}
