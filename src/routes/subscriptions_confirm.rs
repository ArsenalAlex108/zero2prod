use std::{borrow::Cow, str::FromStr};

use actix_web::{HttpResponse, http::StatusCode, web};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{hkt::traversable, utils::Pipe};

#[derive(serde::Deserialize)]
pub struct Parameters<'a> {
    subscription_token: Cow<'a, str>,
}

// Note: Status is usually only updated once for each subscriber, transactionalization is probably not needed.
#[tracing::instrument(
    name = "Confirm pending subscriber",
    skip(parameters, pg_pool)
)]
pub async fn confirm_subscription_token(
    parameters: web::Query<Parameters<'_>>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ConfirmSubscriptionTokenError> {
    parameters
        .subscription_token
        .ref_cast()
        .as_ref()
        .pipe(Uuid::from_str)
        .map_err(|_| {
            ConfirmSubscriptionTokenError::Unauthorized
        })
        .map(async |subscription_token| {
            get_subscriber_id_of_confirmation_token(
                subscription_token,
                &pg_pool,
            )
            .await
            .map_err(ConfirmSubscriptionTokenError::from)
        })
        .pipe(traversable::traverse_result_future_result)
        .await
        .map(async |subscriber_id| {
            update_status_of_subscriber_id_to_confirmed(
                subscriber_id,
                &pg_pool,
            )
            .await
            .map_err(ConfirmSubscriptionTokenError::from)
        })
        .pipe(traversable::traverse_result_future_result)
        .await
        .map(|_| HttpResponse::Ok().finish())
}

#[derive(Debug, thiserror::Error)]
pub enum ConfirmSubscriptionTokenError {
    #[error("Invalid subscription token.")]
    Unauthorized,
    #[error("Unexpected: {0}")]
    Unexpected(#[from] eyre::Report),
}

impl From<GetSubscriberIdOfConfirmationTokenError>
    for ConfirmSubscriptionTokenError
{
    fn from(
        value: GetSubscriberIdOfConfirmationTokenError,
    ) -> Self {
        match value {
            GetSubscriberIdOfConfirmationTokenError::TokenNotFound {
                subscription_token: _
            } => ConfirmSubscriptionTokenError::Unauthorized,
            GetSubscriberIdOfConfirmationTokenError::Database(e) => e.pipe(eyre::Report::new)
            // .wrap_err("Database error occurred during confirming subscription token process.")
            .pipe(ConfirmSubscriptionTokenError::from)
        }
    }
}

#[tracing::instrument(
    name = "Get Subscriber Id associated with a Subscription Confirmation Token.",
    skip(subscription_token, pg_pool)
)]
async fn get_subscriber_id_of_confirmation_token(
    subscription_token: Uuid,
    pg_pool: &PgPool,
) -> Result<Uuid, GetSubscriberIdOfConfirmationTokenError> {
    sqlx::query!(
        "--sql
        SELECT subscriber_id
        FROM subscription_tokens
        WHERE id = $1",
        &subscription_token
    )
    .fetch_one(pg_pool)
    .await
    .map(|i| i.subscriber_id)
    .map_err(|e| {
        use GetSubscriberIdOfConfirmationTokenError as E;
        match e {
            sqlx::Error::RowNotFound => {
                E::TokenNotFound { subscription_token }
            }
            e => E::Database(e),
        }
    })
}

#[derive(Debug, thiserror::Error)]
pub enum GetSubscriberIdOfConfirmationTokenError {
    #[error("Subscription token not found.")]
    TokenNotFound { subscription_token: Uuid },
    #[error(
        "Query for subscriber id of confirmation token failed."
    )]
    Database(#[from] sqlx::Error),
}

#[tracing::instrument(
    name = "Updates confirmation status of a Subscriber of Id to 'confirmed'",
    skip(subscriber_id, pg_pool)
)]
async fn update_status_of_subscriber_id_to_confirmed(
    subscriber_id: Uuid,
    pg_pool: &PgPool,
) -> Result<(), UpdateConfirmationStatusOfSubscriberIdError>
{
    sqlx::query!(
        "--sql
        UPDATE subscriptions
        SET status = 'confirmed'
        WHERE id = $1",
        &subscriber_id
    )
    .execute(pg_pool)
    .await
    .pipe(|i| {
        use UpdateConfirmationStatusOfSubscriberIdError as E;
        match i {
            Ok(result) => {
                let row_count = result.rows_affected();
                if row_count == 1 { Ok(()) }
                else { Err(E::AbnormalUpdatedRowCount(row_count as usize)) }
            }
            Err(e) => {
                E::Database(e)
                .pipe(Err)
            }
        }
    })
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateConfirmationStatusOfSubscriberIdError {
    #[error("Updated row count is not one: {0}")]
    AbnormalUpdatedRowCount(usize),
    #[error(
        "Database error occured trying to update confirmation status of subscriber id."
    )]
    Database(#[from] sqlx::Error),
}

impl From<UpdateConfirmationStatusOfSubscriberIdError>
    for ConfirmSubscriptionTokenError
{
    fn from(
        value: UpdateConfirmationStatusOfSubscriberIdError,
    ) -> Self {
        value
            .pipe(eyre::Report::new)
            .pipe(ConfirmSubscriptionTokenError::from)
    }
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
