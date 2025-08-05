use uuid::Uuid;

use crate::{
    database::transactional::unit_of_work::UnitOfWorkRepository,
    domain::NewSubscriber,
    hkt::{SendHKT, SharedPointerHKT, SyncHKT},
};

pub trait SubscriptionsRepository:
    UnitOfWorkRepository
{
    fn insert_subscriber<
        P: SharedPointerHKT + SendHKT + SyncHKT,
    >(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        form: &NewSubscriber<P>,
    ) -> impl std::future::Future<
        Output = Result<Uuid, InsertSubscriberError>,
    > + Send;

    fn store_token<P: SharedPointerHKT>(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        subscriber_id: &Uuid,
    ) -> impl std::future::Future<
        Output = Result<Uuid, StoreTokenError>,
    > + Send;
}

#[derive(Debug, thiserror::Error)]
#[error(
    "Database error occurred trying to insert subscriber."
)]
pub enum InsertSubscriberError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
#[error(
    "Database error occurred trying to store subscription token."
)]
pub enum StoreTokenError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}
