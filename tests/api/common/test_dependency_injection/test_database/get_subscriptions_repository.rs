pub struct Subscription {
    pub email: String,
    pub name: String,
    pub status: SubscriptionStatus,
}

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
pub enum SubscriptionStatus {
    #[display("pending_confirmation")]
    PendingConfirmation,
    #[display("confirmed")]
    Confirmed,
}

impl TryFrom<&str> for SubscriptionStatus {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "pending_confirmation" => {
                Ok(SubscriptionStatus::PendingConfirmation)
            }
            "confirmed" => {
                Ok(SubscriptionStatus::Confirmed)
            }
            _ => Err(()),
        }
    }
}

pub trait GetSubscriptionsRepository {
    fn get_subscriptions(
        &self,
        username: &str,
    ) -> impl Future<
        Output = Result<Subscription, eyre::Report>,
    > + Send;
}
