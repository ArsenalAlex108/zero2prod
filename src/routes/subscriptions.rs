use crate::{
    domain::{
        NewSubscriber, NewSubscriberParseError,
        SubscriberEmail,
    },
    email_client::EmailClient,
    hkt::{
        RefHKT, SharedPointerHKT,
        traversable::traverse_result_future_result,
    },
    startup::{self, ApplicationBaseUrl},
    utils::Pipe,
};
use actix_web::{
    HttpResponse, Responder,
    http::StatusCode,
    web::{self},
};
use chrono::Utc;
use const_format::formatcp;
use eyre::WrapErr;
use nameof::name_of;
use sqlx::PgPool;
use std::ops::Deref;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    pub email: String,
    pub name: String,
}

const SUBSCRIBE_INSTRUMENT_NAME: &str =
    formatcp!("Enter Route '{}':", name_of!(subscribe));

#[tracing::instrument(
    name = SUBSCRIBE_INSTRUMENT_NAME,
    skip(form, pool, email_client, base_url),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<SubscribeFormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<
        EmailClient<startup::GlobalSharedPointerType>,
    >,
    base_url: web::Data<
        ApplicationBaseUrl<
            startup::GlobalSharedPointerType,
        >,
    >,
) -> impl Responder {
    subscribe_with_shared_pointer(
        form,
        pool,
        email_client,
        base_url,
    )
    .pipe(Box::pin)
    .await
}

#[tracing::instrument(
    name = SUBSCRIBE_INSTRUMENT_NAME,
    skip(form, pool, email_client, base_url),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe_with_shared_pointer<
    P: SharedPointerHKT,
    DataP: RefHKT,
>(
    form: web::Form<SubscribeFormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient<P>>,
    base_url: web::Data<ApplicationBaseUrl<DataP>>,
) -> Result<HttpResponse, SubscribeError> {
    pool.begin()
        .await
        .context("Failed to acquire Postgres connection from pool.")
        .map_err(SubscribeError::from)
        .map(async |mut transaction| {
            NewSubscriber::try_from(form.0)
                .map_err(SubscribeError::from)
                .map(async |subscriber| {
                    insert_subscriber::<P>(
                        &subscriber,
                        &mut *transaction,
                    )
                    .await
                    .context("Failed to insert new subscriber to database.")
                    .map_err(SubscribeError::from)
                    .map(async |subscriber_id| {
                        store_token::<P>(
                            &mut *transaction,
                            &subscriber_id,
                        )
                        .await
                        .context("Failed to store confirmation token of new subscriber to database.")
                        .map_err(SubscribeError::from)
                    })
                    .pipe(traverse_result_future_result)
                    .await
                    .map(async |token| {
                        send_confirmation_email(
                            &email_client,
                            &subscriber,
                            base_url
                                .deref()
                                .deref()
                                .0
                                .ref_cast(),
                            token.to_string().as_str(),
                        )
                        .await
                        .context("Failed to send confirmation email to new subscriber's email.")
                        .map_err(SubscribeError::from)
                    })
                    .pipe(traverse_result_future_result)
                    .await
                })
                .pipe(traverse_result_future_result)
                .await
                .map(async |()| {
                    transaction
                        .commit()
                        .await
                        .context("Failed to commit new subscription transaction.")
                        .map_err(SubscribeError::from)
                })
                .pipe(traverse_result_future_result)
                .await
        })
        .pipe(traverse_result_future_result)
        .await
        .map(|()| HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Send confirmation email to new subscriber",
    skip(
        email_client,
        new_subscriber,
        confirmation_link,
        subscription_token
    )
)]
pub async fn send_confirmation_email<
    P: SharedPointerHKT,
>(
    email_client: &EmailClient<P>,
    new_subscriber: &NewSubscriber<P>,
    confirmation_link: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        confirmation_link, subscription_token
    );

    email_client
        .send_email(
            new_subscriber.email.pipe_ref(SubscriberEmail::clone),
            "Welcome!"
                .pipe(P::from_static_str),
            format!(
                "Welcome to our newsletter!<br />\
                Click <a href=\"{}\">here</a> to confirm your subscription.",
                confirmation_link
            )
                .pipe(P::from_string),
            format!(
                "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
                confirmation_link
            )
                .pipe(P::from_string),
        ).await
}

#[derive(Debug, thiserror::Error)]
pub enum SubscribeError {
    #[error("Parsing new subscriber failed: {0}")]
    Validation(#[from] NewSubscriberParseError),
    #[error("{0}")]
    Unexpected(#[from] eyre::Report), // #[error("Database Error occured while attempting to store subscription token.")]
                                      // StoreTokenError(#[from] StoreTokenError),
                                      // #[error("Database Error occured while attempting to insert subscriber.")]
                                      // InsertSubscriberError(#[from] InsertSubscriberError),

                                      // #[error("Database Error occurred.")]
                                      // Sqlx(#[source] sqlx::Error),
                                      // #[error("Http Request failed.")]
                                      // Reqwest(#[from] reqwest::Error),
}

impl actix_web::ResponseError for SubscribeError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SubscribeError::Validation(_) => {
                StatusCode::BAD_REQUEST
            }
            SubscribeError::Unexpected(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

const INSERT_SUBSCRIBER_INSTRUMENT_NAME: &str = formatcp!(
    "Enter Route '{}':",
    stringify!(insert_subscriber)
);

#[derive(Debug, thiserror::Error)]
#[error(
    "Database error occurred trying to insert subscriber."
)]
pub struct InsertSubscriberError(#[from] sqlx::Error);

#[tracing::instrument(
    name = INSERT_SUBSCRIBER_INSTRUMENT_NAME,
    skip(form, connection)
)]
pub async fn insert_subscriber<P: SharedPointerHKT>(
    form: &NewSubscriber<P>,
    connection: &mut sqlx::PgConnection,
) -> Result<Uuid, InsertSubscriberError> {
    let subscriber_id = Uuid::new_v4();
    sqlx::query!(
        r#"--sql
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        subscriber_id,
        &*form.email,
        &*form.name,
        Utc::now()
    )
    .execute(connection)
    .await?;
    Ok(subscriber_id)
}

#[derive(Debug, thiserror::Error)]
#[error(
    "Database error occurred trying to store subscription token."
)]
pub struct StoreTokenError(#[from] sqlx::Error);

// There are two instances of Uuid here, possible mix up.
#[tracing::instrument(
    name = "Storing subscription confirmation token.",
    skip(connection, subscriber_id)
)]
pub async fn store_token<P: SharedPointerHKT>(
    connection: &mut sqlx::PgConnection,
    subscriber_id: &Uuid,
) -> Result<Uuid, StoreTokenError> {
    let token = Uuid::new_v4();
    sqlx::query!(
        r#"--sql
        INSERT INTO subscription_tokens (id, subscriber_id)
        VALUES ($1, $2)
        "#,
        token,
        subscriber_id,
    )
    .execute(connection)
    .await?;
    Ok(token)
}
