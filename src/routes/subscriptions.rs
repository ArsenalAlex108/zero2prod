use std::convert::identity;

use crate::{
    domain::{
        NewSubscriber, NewSubscriberParseError,
        SubscriberEmail,
        SubscriberEmailParseError,
        SubscriberName, SubscriberNameParseError,
    },
    hkt::{RcHKT, SharedPointerHKT},
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
use std::ops::Deref;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    pub email: String,
    pub name: String,
}

const SUBSCRIBE_INSTRUMENT_NAME: &str = formatcp!(
    "Enter Route '{}':",
    name_of!(subscribe)
);

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
    subscribe_with_shared_pointer::<RcHKT>(
        form, pool,
    )
    .await
}

#[tracing::instrument(
    name = SUBSCRIBE_INSTRUMENT_NAME,
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe_with_shared_pointer<
    P: SharedPointerHKT,
>(
    form: web::Form<SubscribeFormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let subscriber_parse_result =
        NewSubscriber::try_from(form.0)
            .map_err(SubscribeError::from);

    subscriber_parse_result
        .map(async |subscriber| {
            insert_subscriber::<P>(&subscriber, &pool)
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
                Err(E::NewSubscriberParseError(_)) => {
                    HttpResponse::BadRequest().finish()
                }
                Err(E::SqlxError(_)) => HttpResponse::InternalServerError().finish(),
            }
        })
}

#[derive(Debug, thiserror::Error)]
enum SubscribeError {
    #[error("{name}: {0}", name = name_of!(type NewSubscriberParseError))]
    NewSubscriberParseError(
        #[from] NewSubscriberParseError,
    ),
    #[error("Sqlx Error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

const INSERT_SUBSCRIBER_INSTRUMENT_NAME: &str = formatcp!(
    "Enter Route '{}':",
    stringify!(insert_subscriber)
);

#[tracing::instrument(
    name = INSERT_SUBSCRIBER_INSTRUMENT_NAME,
    skip(form, pool)
)]
pub async fn insert_subscriber<
    P: SharedPointerHKT,
>(
    form: &NewSubscriber<P>,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email.deref(),
        form.name.deref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .inspect_err(|e| tracing::error!("Query error in {}: {:?}", stringify!(insert_subscriber), e))?;
    Ok(())
}
