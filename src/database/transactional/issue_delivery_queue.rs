use uuid::Uuid;

use crate::{
    database::transactional::unit_of_work::UnitOfWorkRepository,
    issue_delivery_worker::IssueDeliveryRecord,
};

pub trait IssueDeliveryQueueRepository:
    UnitOfWorkRepository
{
    fn enqueue_delivery_tasks(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        newsletter_issue_id: Uuid,
    ) -> impl Future<
        Output = Result<
            EnqueueDeliveryTaskResult,
            EnqueueDeliveryTaskError,
        >,
    > + Send;

    fn schedule_task_retry(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        record: &IssueDeliveryRecord,
    ) -> impl Future<
        Output = Result<(), ScheduleTaskRetryError>,
    > + Send;

    fn disable_task(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        record: &IssueDeliveryRecord,
    ) -> impl Future<Output = Result<(), DisableTaskError>> + Send;

    fn acquire_newsletter_task_from_issue(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        newsletter_issue_id: Uuid,
    ) -> impl Future<
        Output = Result<
            Option<IssueDeliveryRecord>,
            AcquireNewsletterTaskFromIssueError,
        >,
    > + Send;

    fn acquire_newsletter_task(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
    ) -> impl Future<
        Output = Result<
            Option<Uuid>,
            AcquireNewsletterTaskError,
        >,
    > + Send;

    fn finalize_newsletter_task(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        record: IssueDeliveryRecord,
    ) -> impl Future<
        Output = Result<(), FinalizeNewsletterTaskError>,
    > + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum ScheduleTaskRetryError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum DisableTaskError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum AcquireNewsletterTaskFromIssueError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum AcquireNewsletterTaskError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum FinalizeNewsletterTaskError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, derive_more::Display)]
pub enum EnqueueDeliveryTaskResult {
    Enqueued,
    Unchanged,
}

#[derive(Debug, thiserror::Error)]
pub enum EnqueueDeliveryTaskError {
    #[error(
        "No tasks enqueued for newsletter issue with uuid '{0}. Either there are no confirmed subscribers or the issue is not found."
    )]
    NotFound(Uuid),
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}
