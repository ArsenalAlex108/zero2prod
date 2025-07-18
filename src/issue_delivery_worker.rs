use std::{
    ops::{ControlFlow, DerefMut},
    time::Duration,
};

use eyre::Context;
use uuid::Uuid;

use crate::{
    configuration::Settings,
    domain::SubscriberEmail,
    email_client::EmailClient,
    hkt::{
        K1, SharedPointerHKT,
        traversable::{
            traverse_result_future,
            traverse_result_future_result,
        },
    },
    startup::get_connection_pool,
    utils::{Pipe as _, SyncMutCell},
};

pub struct IssueDeliveryRecord {
    pub newsletter_issue_id: Uuid,
    pub subscriber_email: String,
}

pub async fn run_worker_until_stopped<
    P: SharedPointerHKT,
>(
    configuration: Settings<P>,
) -> Result<(), eyre::Report> {
    let connection_pool =
        get_connection_pool(&configuration.database);
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

    let mut transaction =
        connection_pool.begin().await.context(
            "Transaction should begin successfully.",
        )?;

    let connection_ptr =
        SyncMutCell::from(transaction.deref_mut());

    let iterator = get_newsletter_sending_worker_iterator(
        &email_client,
        &connection_ptr,
    )
    .await;

    for task in iterator {
        task.await;
    }

    Ok(())
}

#[tracing::instrument(
    name = "Get and uniquely lock a task in the issue delivery queue.",
    skip(connection, email_client)
)]
async fn send_newsletter<P: SharedPointerHKT>(
    title: String,
    html: String,
    text: String,
    newsletter_issue_id: Uuid,
    email_client: &EmailClient<P>,
    connection: &mut sqlx::PgConnection,
) -> Result<(), eyre::Report> {
    let subject = P::from_string(title);
    let html_content = P::from_string(html);
    let text_content = P::from_string(text);

    //let connection_ptr = RefCell::new(connection);
    let connection_ptr = SyncMutCell::from(connection);

    let iterator = get_sending_to_subscribers_of_single_newsletter_issue_iterator(
        &subject,
        &html_content,
        &text_content,
        &newsletter_issue_id,
        email_client,
        &connection_ptr
    );

    for task in iterator {
        match task.await {
            ControlFlow::Continue(()) => continue,
            ControlFlow::Break(Ok(())) => return Ok(()),
            ControlFlow::Break(Err(e)) => return Err(e),
        }
    }

    Ok(())
}

#[tracing::instrument(
    name = "Gets infinite iterator handling result of sending newsletters to subscribers."
    skip_all
)]
pub async fn get_newsletter_sending_worker_iterator<
    P: SharedPointerHKT,
>(
    email_client: &EmailClient<P>,
    connection_ptr: &SyncMutCell<&mut sqlx::PgConnection>,
) -> impl Iterator<Item = impl Future> {
    [()]
    .into_iter()
    .cycle()
    .map(async |_|{

        let iterator = get_single_newsletter_picking_and_sending_iterator(
            email_client,
            connection_ptr,
        );

        for task_result in iterator {
            use SingleNewsletterPickingAndSendingTaskResult as R;
            match task_result.await {
                R::Completed => (),
                R::NothingFound => tokio::time::sleep(
                    Duration::from_secs(10)
                ).await,
                R::Error(_) => tokio::time::sleep(
                    Duration::from_secs(1)
                ).await,
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
pub fn get_single_newsletter_picking_and_sending_iterator<P: SharedPointerHKT>(
    email_client: &EmailClient<P>,
    connection_ptr: &SyncMutCell<&mut sqlx::PgConnection>,
) -> impl Iterator<
    Item = impl Future<
        Output = SingleNewsletterPickingAndSendingTaskResult
        >
>{
    use SingleNewsletterPickingAndSendingTaskResult as R;

    [async |connection: &mut sqlx::PgConnection|
        acquire_newsletter_task(connection).await
    ]
    .into_iter()
    .cycle()
    .map(async |acquire_task| {
        let mut connection_borrow = connection_ptr.borrow();

        let connection: &mut sqlx::PgConnection = connection_borrow.deref_mut();

        struct NewsletterContent {
            title: String,
            text_content: String,
            html_content: String,
        }

        match acquire_task(connection).await {
            Ok(Some(record)) => {
                sqlx::query_as!(NewsletterContent,
                    "--sql
                    SELECT title, text_content, html_content
                    FROM newsletter_issues
                    WHERE newsletter_issue_id = $1
                    ",
                    &record.newsletter_issue_id
                ).fetch_one(connection)
                .await
                .map_err(eyre::Report::new)
                .map(async |i| {
                    let NewsletterContent {
                        title,
                        text_content,
                        html_content,
                    } = i;

                    let subject = P::from_string(title);
                    let html_content = P::from_string(html_content);
                    let text_content = P::from_string(text_content);

                    // Danger
                    drop(connection_borrow);

                    let iterator = get_sending_to_subscribers_of_single_newsletter_issue_iterator(
                        &subject,
                        &html_content,
                        &text_content,
                        &record.newsletter_issue_id,
                        email_client,
                        connection_ptr
                    );

                    for task in iterator {
                        match task.await {
                            ControlFlow::Continue(()) => continue,
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
            Err(e) => R::Error(e),
        }
    })
}

#[tracing::instrument(
    name = "Gets infinite iterator sending a newsletter to a subscriber."
    skip_all
)]
fn get_sending_to_subscribers_of_single_newsletter_issue_iterator<
    P: SharedPointerHKT,
>(
    subject: &K1<P, str>,
    html_content: &K1<P, str>,
    text_content: &K1<P, str>,
    newsletter_issue_id: &Uuid,
    email_client: &EmailClient<P>,
    connection_ptr: &SyncMutCell<&mut sqlx::PgConnection>,
) -> impl Iterator<
    Item = impl Future<
        Output = ControlFlow<Result<(), eyre::Report>, ()>,
    >,
> {
    [async |connection: &mut sqlx::PgConnection| {
        acquire_newsletter_task_from_issue(
            *newsletter_issue_id,
            connection
        ).await
    }]
    .into_iter()
    .cycle()
    .map(async |acquire_task| {
        let mut connection = connection_ptr.borrow();

        match acquire_task(&mut connection).await
        {
            Ok(Some(record)) => {
                if let Ok(subscriber_email) =
                SubscriberEmail::try_from(record.subscriber_email.to_string())
                .inspect_err(|e| tracing::warn!("Found subscriber with invalid email while attempting to send a newsletter to them: '{0}'\n
                Error: '{e}'", &record.subscriber_email)) {
                    match email_client
                    .send_email(
                        subscriber_email.pipe_ref(SubscriberEmail::clone),
                        subject.pipe_ref(K1::clone),
                        html_content.pipe_ref(K1::clone),
                        text_content.pipe_ref(K1::clone),
                    ).await
                    {
                        Ok(()) => {
                            match finalize_newsletter_task(
                                &mut connection,
                                record
                            ).await {
                                Ok(()) => ControlFlow::Continue(()),
                                Err(e) => e
                                    .wrap_err(format!("Failed to finalize newsletter task to: '{}'", subscriber_email))
                                    .pipe(Err)
                                    .pipe(ControlFlow::Break),
                            }
                        },
                        Err(e) => e
                        .pipe(eyre::Report::new)
                        .wrap_err(format!("Failed to send newsletter to: '{}'", subscriber_email))
                        .pipe(Err)
                        .pipe(ControlFlow::Break),
                    }
                } else {
                    ControlFlow::Continue(())
                }
            },
            Ok(None) => ControlFlow::Break(Ok(())),
            Err(e) => ControlFlow::Break(Err(e))
        }
    })
}

#[tracing::instrument(
    name = "Get and uniquely lock a task in the issue delivery queue from a specific issue.",
    skip_all
)]
pub async fn acquire_newsletter_task_from_issue(
    newsletter_issue_id: Uuid,
    connection: &mut sqlx::PgConnection,
) -> Result<Option<IssueDeliveryRecord>, eyre::Report> {
    sqlx::query_as!(
        IssueDeliveryRecord,
        "--sql
        SELECT newsletter_issue_id, subscriber_email
        FROM issue_delivery_queue
        WHERE newsletter_issue_id = $1
        FOR UPDATE
        SKIP LOCKED
        LIMIT 1
    ",
        newsletter_issue_id
    )
    .fetch_optional(connection)
    .await?
    .pipe(Ok)
}

#[tracing::instrument(
    name = "Get and uniquely lock a task in the issue delivery queue.",
    skip_all
)]
pub async fn acquire_newsletter_task(
    connection: &mut sqlx::PgConnection,
) -> Result<Option<IssueDeliveryRecord>, eyre::Report> {
    sqlx::query_as!(
        IssueDeliveryRecord,
        "--sql
        SELECT newsletter_issue_id, subscriber_email
        FROM issue_delivery_queue
        FOR UPDATE
        SKIP LOCKED
        LIMIT 1
    "
    )
    .fetch_optional(connection)
    .await?
    .pipe(Ok)
}

#[tracing::instrument(
    name = "Delete a task in the issue delivery queue after completion.",
    skip_all
)]
pub async fn finalize_newsletter_task(
    connection: &mut sqlx::PgConnection,
    record: IssueDeliveryRecord,
) -> Result<(), eyre::Report> {
    sqlx::query!(
        "--sql
        DELETE FROM issue_delivery_queue
        WHERE newsletter_issue_id = $1
        AND subscriber_email = $2
        ",
        record.newsletter_issue_id,
        record.subscriber_email
    )
    .execute(connection)
    .await?;

    Ok(())
}
