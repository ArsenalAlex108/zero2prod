use uuid::Uuid;

use crate::dependency_injection::app_state::SendSyncStatic;

pub trait SubscriptionsConfirmRepository:
    SendSyncStatic
{
    fn get_subscriber_id_of_confirmation_token(
        &self,
        subscription_token: Uuid,
    ) -> impl std::future::Future<
        Output = Result<
            Uuid,
            GetSubscriberIdOfConfirmationTokenError,
        >,
    > + Send;

    fn update_status_of_subscriber_id_to_confirmed(
        &self,
        subscriber_id: Uuid,
    ) -> impl std::future::Future<
        Output = Result<
            (),
            UpdateConfirmationStatusOfSubscriberIdError,
        >,
    > + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum GetSubscriberIdOfConfirmationTokenError {
    #[error("Subscription token not found.")]
    TokenNotFound { subscription_token: Uuid },
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateConfirmationStatusOfSubscriberIdError {
    #[error("Updated row count is not one: {0}")]
    AbnormalUpdatedRowCount(usize),
    #[error(
        "Database error occured trying to update confirmation status of subscriber id."
    )]
    Unexpected(#[from] eyre::Report),
}
