use std::{borrow::Cow, str::FromStr};

use actix_web::{HttpResponse, http::StatusCode, web};
use eyre::{Context, eyre};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{hkt::traversable, utils::Pipe};

#[derive(serde::Deserialize)]
pub struct Parameters<'a> {
    subscription_token: Cow<'a, str>,
}

// TODO: Refactor database queryies into methods with bespoke error types.
#[tracing::instrument(
    name = "Confirm pending subscriber",
    skip(parameters, pg_pool)
)]
pub async fn confirm_subscription_token<'a>(
    parameters: web::Query<Parameters<'a>>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ConfirmSubscriptionTokenError> {
    parameters.subscription_token.ref_cast()
    .as_ref()
    .pipe(Uuid::from_str)
    .map_err(|_| ConfirmSubscriptionTokenError::Unauthorized)
    .map(async |subscription_token|
        sqlx::query!(
            "--sql
            SELECT subscriber_id
            FROM subscription_tokens
            WHERE id = $1",
            subscription_token
        )
        .fetch_one(pg_pool.as_ref())
        .await
        .map_err(|e| {
            use ConfirmSubscriptionTokenError as E;
            match e {
                sqlx::Error::RowNotFound => E::Unauthorized,
                e => eyre::Report::new(e)
                .wrap_err("Subscription token query from the database failed.")
                .pipe(E::from)
            }
        })
    )
    .pipe(traversable::traverse_result_future_result)
    .await
    .map(|i| i.subscriber_id)
    .map(async |subscriber_id| {
        sqlx::query!(
            "--sql
            UPDATE subscriptions
            SET status = 'confirmed'
            WHERE id = $1",
            &subscriber_id
        )
        .execute(pg_pool.as_ref())
        .await
        .context("Failed to update subscription status to confirmed in database.")
        .map_err(ConfirmSubscriptionTokenError::from)
    })
    .pipe(traversable::traverse_result_future_result)
    .await
    .and_then(|i|
        if i.rows_affected() == 1 {
            HttpResponse::Ok().finish().pipe(Ok)
        } else {
            eyre!(
                format!(
                    "Abnormal number of rows affected: {}",
                    i.rows_affected()
                )
            )
            .pipe(ConfirmSubscriptionTokenError::from)
            .pipe(Err)
        }
    )
}

#[derive(Debug, thiserror::Error)]
pub enum ConfirmSubscriptionTokenError {
    #[error("Invalid subscription token.")]
    Unauthorized,
    #[error("Unexpected: {0}")]
    Unexpected(#[from] eyre::Report),
}

impl actix_web::ResponseError
    for ConfirmSubscriptionTokenError
{
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ConfirmSubscriptionTokenError::Unauthorized => {
                StatusCode::UNAUTHORIZED
            }
            ConfirmSubscriptionTokenError::Unexpected(
                _,
            ) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
