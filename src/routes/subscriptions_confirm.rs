use std::{borrow::Cow, str::FromStr};

use actix_web::{HttpResponse, http::StatusCode, web};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    database::transactional::subscriptions_confirm::{
        GetSubscriberIdOfConfirmationTokenError,
        SubscriptionsConfirmRepository,
        UpdateConfirmationStatusOfSubscriberIdError,
    },
    dependency_injection::app_state::Inject,
    hkt::traversable,
    utils::Pipe,
};

#[derive(serde::Deserialize)]
pub struct Parameters<'a> {
    subscription_token: Cow<'a, str>,
}

// Note: Status is usually only updated once for each subscriber, transactionalization is probably not needed.
#[tracing::instrument(
    name = "Confirm pending subscriber",
    skip(parameters, subscriptions_confirm_repository)
)]
pub async fn confirm_subscription_token<
    S: SubscriptionsConfirmRepository,
>(
    parameters: web::Query<Parameters<'_>>,
    subscriptions_confirm_repository: Inject<S>,
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
            subscriptions_confirm_repository.get_subscriber_id_of_confirmation_token(
                subscription_token,
            )
            .await
            .map_err(ConfirmSubscriptionTokenError::from)
        })
        .pipe(traversable::traverse_result_future_result)
        .await
        .map(async |subscriber_id| {
            subscriptions_confirm_repository.update_status_of_subscriber_id_to_confirmed(
                subscriber_id,
            )
            .await
            .map_err(ConfirmSubscriptionTokenError::from)
        })
        .pipe(traversable::traverse_result_future_result)
        .await
        .map(|()| HttpResponse::Ok().finish())
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
            GetSubscriberIdOfConfirmationTokenError::Unexpected(e) => e
            .pipe(ConfirmSubscriptionTokenError::from)
        }
    }
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
