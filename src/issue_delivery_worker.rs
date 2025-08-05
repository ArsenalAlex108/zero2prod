use std::{ops::ControlFlow, time::Duration};

use crate::hkt::{RefHKT as _, SendHKT, SyncHKT};
use eyre::Context;
use lazy_errors::{IntoEyreResult, OrStash};
use uuid::Uuid;

use crate::startup::GlobalSharedPointer;
use crate::{
    configuration::Settings,
    database::transactional::{
        issue_delivery_queue::IssueDeliveryQueueRepository,
        newsletters::{
            NewsletterContent, NewslettersRepository,
        },
        unit_of_work::{BeginUnitOfWork, UnitOfWork as _},
    },
    domain::SubscriberEmail,
    email_client::EmailClient,
    hkt::{
        K1, SharedPointerHKT,
        traversable::traverse_result_future,
    },
    utils::Pipe as _,
};

pub struct IssueDeliveryRecord {
    pub newsletter_issue_id: Uuid,
    pub subscriber_email: String,
}

pub trait IssueDeliveryWorkerDependencyAlias {
    type P: SharedPointerHKT + SendHKT + SyncHKT;
    type B: BeginUnitOfWork;
    type N: NewslettersRepository<UnitOfWork =
        <Self::B as BeginUnitOfWork>::UnitOfWork>;
    type I: IssueDeliveryQueueRepository<UnitOfWork =
        <Self::B as BeginUnitOfWork>::UnitOfWork>;
}

pub struct IssueDeliveryWorkerDependencies<'a, D>
where
    D: IssueDeliveryWorkerDependencyAlias,
{
    pub email_client: &'a EmailClient<D::P>,
    pub begin_unit_of_work: &'a D::B,
    pub issue_delivery_queue_repository: &'a D::I,
    pub newsletters_repository: &'a D::N,
}

impl<D: IssueDeliveryWorkerDependencyAlias> Clone
    for IssueDeliveryWorkerDependencies<'_, D>
{
    fn clone(&self) -> Self {
        Self {
            email_client: self.email_client,
            begin_unit_of_work: self.begin_unit_of_work,
            issue_delivery_queue_repository: self
                .issue_delivery_queue_repository,
            newsletters_repository: self
                .newsletters_repository,
        }
    }
}

pub async fn run_worker_until_stopped<
    D: IssueDeliveryWorkerDependencyAlias,
>(
    issue_delivery_queue_repository: GlobalSharedPointer<
        D::I,
    >,
    begin_unit_of_work: GlobalSharedPointer<D::B>,
    newsletters_repository: GlobalSharedPointer<D::N>,
    configuration: Settings<D::P>,
) -> Result<(), eyre::Report> {
    let sender_email = configuration
        .email_client
        .sender()
        .context("Invalid sender email address.")?;

    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration
            .email_client
            .base_url
            .pipe_ref(K1::clone),
        sender_email,
        configuration
            .email_client
            .authorization_token
            .pipe_ref(K1::clone),
        timeout,
    );

    let dependencies = IssueDeliveryWorkerDependencies::<D> {
        email_client: &email_client,
        begin_unit_of_work: &begin_unit_of_work,
        issue_delivery_queue_repository:
            &issue_delivery_queue_repository,
        newsletters_repository: &newsletters_repository,
    };

    let iterator = get_newsletter_sending_worker_iterator(
        &dependencies,
    )
    .await;

    for task in iterator {
        task.await;
    }

    Ok(())
}

#[tracing::instrument(
    name = "Gets infinite iterator handling result of sending newsletters to subscribers."
    skip_all
)]
pub async fn get_newsletter_sending_worker_iterator<
    'a,
    D: IssueDeliveryWorkerDependencyAlias,
>(
    dependencies: &'a IssueDeliveryWorkerDependencies<
        'a,
        D,
    >,
) -> impl Iterator<Item = impl Future> {
    std::iter::repeat_with(async || {
        let iterator = get_single_newsletter_picking_and_sending_iterator(
            dependencies
        );

        for task_result in iterator {
            use SingleNewsletterPickingAndSendingTaskResult as R;
            match task_result.await {
                R::Completed => (),
                R::NothingFound => {
                    tokio::time::sleep(
                        Duration::from_secs(10),
                    )
                    .await;
                }
                R::Error(_) => {
                    tokio::time::sleep(
                        Duration::from_secs(1),
                    )
                    .await;
                }
            }
        }
    })
}

pub enum SingleNewsletterPickingAndSendingTaskResult {
    Completed,
    NothingFound,
    Error(eyre::Report),
}

impl
    From<
        Result<
            SingleNewsletterPickingAndSendingTaskResult,
            eyre::Report,
        >,
    > for SingleNewsletterPickingAndSendingTaskResult
{
    fn from(
        value: Result<
            SingleNewsletterPickingAndSendingTaskResult,
            eyre::Report,
        >,
    ) -> Self {
        use SingleNewsletterPickingAndSendingTaskResult as R;
        match value {
            Ok(r) => r,
            Err(e) => R::Error(e),
        }
    }
}

#[tracing::instrument(
    name = "Gets infinite iterator picking a newsletter to send to all subscribers."
    skip_all
)]
pub fn get_single_newsletter_picking_and_sending_iterator<D: IssueDeliveryWorkerDependencyAlias>(
    dependencies: &IssueDeliveryWorkerDependencies<D>,
) -> impl Iterator<
    Item = impl Future<
        Output = SingleNewsletterPickingAndSendingTaskResult
        >
>{
    use SingleNewsletterPickingAndSendingTaskResult as R;

    let IssueDeliveryWorkerDependencies {
        email_client: _,
        begin_unit_of_work,
        issue_delivery_queue_repository,
        newsletters_repository,
    } = dependencies.clone();

    std::iter::repeat_with(async || {
        let mut unit_of_work = match begin_unit_of_work
            .begin()
            .await
            .map_err(eyre::Report::new)
        {
            Ok(u) => u,
            Err(e) => return R::Error(e),
        };

        match issue_delivery_queue_repository.acquire_newsletter_task(&mut unit_of_work).await {
            Ok(Some(id)) => {
                newsletters_repository.get_newsletter_content(&mut unit_of_work, id)
                .await
                .map_err(eyre::Report::new)
                .map(async |i| {
                    let NewsletterContent {
                        title,
                        text_content,
                        html_content,
                    } = i;

                    let subject = D::P::from_string(title);
                    let html_content = D::P::from_string(html_content);
                    let text_content = D::P::from_string(text_content);

                    let iterator = get_sending_to_subscribers_of_single_newsletter_issue_iterator(
                        &subject,
                        &html_content,
                        &text_content,
                        &id,
                        dependencies
                    );

                    for task in iterator {
                        match task.await {
                            ControlFlow::Continue(()) => {},
                            ControlFlow::Break(Ok(())) => return R::Completed,
                            ControlFlow::Break(Err(e)) => return R::Error(e)
                        }
                    };

                    R::Completed
                })
                .pipe(traverse_result_future)
                .await
                .pipe(R::from)
            },
            Ok(None) => R::NothingFound,
            Err(e) => R::Error(e.into()),
        }
    })
}

#[tracing::instrument(
    name = "Gets infinite iterator sending a newsletter to a subscriber."
    skip_all
)]
fn get_sending_to_subscribers_of_single_newsletter_issue_iterator<
    D: IssueDeliveryWorkerDependencyAlias,
>(
    subject: &K1<D::P, str>,
    html_content: &K1<D::P, str>,
    text_content: &K1<D::P, str>,
    newsletter_issue_id: &Uuid,
    dependencies: &IssueDeliveryWorkerDependencies<D>,
) -> impl Iterator<
    Item = impl Future<
        Output = ControlFlow<Result<(), eyre::Report>, ()>,
    >,
> {
    let IssueDeliveryWorkerDependencies {
        email_client,
        begin_unit_of_work,
        issue_delivery_queue_repository,
        newsletters_repository: _,
    } = dependencies.clone();
    std::iter::repeat_with(async || {
        let mut unit_of_work = match begin_unit_of_work
            .begin()
            .await
            .map_err(eyre::Report::new)
        {
            Ok(u) => u,
            Err(e) => return ControlFlow::Break(Err(e)),
        };

        let result = match issue_delivery_queue_repository.acquire_newsletter_task_from_issue(
            &mut unit_of_work,
            *newsletter_issue_id,
        ).await
        {
            Ok(Some(record)) => {
                match SubscriberEmail::try_from(record.subscriber_email.to_string()) {
                    Ok(subscriber_email) => {
                        match email_client
                                .send_email(
                                    subscriber_email.pipe_ref(SubscriberEmail::clone),
                                    subject.pipe_ref(K1::clone),
                                    html_content.pipe_ref(K1::clone),
                                    text_content.pipe_ref(K1::clone),
                                ).await
                                {
                                    Ok(()) => {
                                        match issue_delivery_queue_repository.finalize_newsletter_task(
                                            &mut unit_of_work,
                                            record
                                        ).await {
                                            Ok(()) => ControlFlow::Continue(()),
                                            Err(e) => e
                                                .pipe(eyre::Report::new)
                                                .wrap_err(format!("Failed to finalize newsletter task to: '{}'", subscriber_email))
                                                .pipe(Err)
                                                .pipe(ControlFlow::Break),
                                        }
                                    },
                                    Err(e) => {

                                        let mut error_stash = lazy_errors::ErrorStash::<_, _,eyre::Report>::new(|| format!("Failed to send newsletter to: '{}'", subscriber_email));

                                        issue_delivery_queue_repository.schedule_task_retry(
                                            &mut unit_of_work,
                                            &record,
                                        ).await
                                        .context("Failed to schedule task retry.").or_stash(&mut error_stash);

                                        error_stash.push(eyre::Report::new(e));

                                        error_stash
                                        .into_eyre_result()
                                        .pipe(ControlFlow::Break)
                                    },
                                }
                    }
                    Err(e) => {
                        tracing::warn!("Found subscriber with invalid email while attempting to send a newsletter to them: '{0}'\n
                Error: '{e}'", &record.subscriber_email);

                        let _ = issue_delivery_queue_repository.disable_task(
                            &mut unit_of_work,
                            &record,
                        ).await;

                        ControlFlow::Continue(())
                    },
                }
            },
            Ok(None) => ControlFlow::Break(Ok(())),
            Err(e) => ControlFlow::Break(Err(e.into()))
        };

        match unit_of_work.commit().await {
            Ok(()) => result,
            Err(e) => e
                .pipe(eyre::Report::new)
                .wrap_err("Failed to commit unit of work.")
                .pipe(Err)
                .pipe(ControlFlow::Break),
        }
    })
}
