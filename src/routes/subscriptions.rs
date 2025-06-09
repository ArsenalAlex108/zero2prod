use std::convert::identity;

use crate::domain::{
    NewSubscriber, SubscriberEmail, SubscriberEmailParseError, SubscriberName,
    SubscriberNameParseError,
};
use actix_web::{
    HttpResponse, Responder,
    web::{self},
};
use chrono::Utc;
use const_format::formatcp;
use kust::ScopeFunctions;
use nameof::name_of;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    pub email: String,
    pub name: String,
}

const SUBSCRIBE_INSTRUMENT_NAME: &str = formatcp!("Enter Route '{}':", name_of!(subscribe));

#[tracing::instrument(
    name = SUBSCRIBE_INSTRUMENT_NAME,
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<SubscribeFormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let subscriber_parse_result = SubscriberName::parse(form.0.name)
        .map_err(SubscribeError::from)
        .and_then(|name| {
            SubscriberEmail::parse(form.0.email)
                .map(|email| NewSubscriber { email, name })
                .map_err(SubscribeError::from)
        });

    subscriber_parse_result
        .map(async |subscriber| {
            insert_subscriber(&subscriber, &pool)
                .await
                .map_err(SubscribeError::from)
        })
        // Traverse
        .using(async |i| i?.await.using(Ok))
        .await
        .and_then(identity)
        .using(|i| {
            use SubscribeError as E;
            match i {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(E::SubscriberName(_)) | Err(E::SubscriberEmail(_)) => {
                    HttpResponse::BadRequest().finish()
                }
                Err(E::Sqlx(_)) => HttpResponse::InternalServerError().finish(),
            }
        })
}

#[derive(Debug, thiserror::Error)]
enum SubscribeError {
    #[error("{name}: {0}", name = name_of!(type SubscriberNameParseError))]
    SubscriberName(#[from] SubscriberNameParseError),
    #[error("{name}: {0}", name = name_of!(type SubscriberEmailParseError))]
    SubscriberEmail(#[from] SubscriberEmailParseError),
    #[error("Sqlx Error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

const INSERT_SUBSCRIBER_INSTRUMENT_NAME: &str =
    formatcp!("Enter Route '{}':", name_of!(insert_subscriber));

#[tracing::instrument(
    name = INSERT_SUBSCRIBER_INSTRUMENT_NAME,
    skip(form, pool)
)]
pub async fn insert_subscriber(form: &NewSubscriber, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email.as_str(),
        form.name.as_str(),
        Utc::now()
    )
    .execute(pool)
    .await
    .inspect_err(|e| tracing::error!("Query error in {}: {:?}", name_of!(insert_subscriber), e))?;
    Ok(())
}
