use std::{ops::ControlFlow, time::Duration};

use eyre::Context;
use lazy_errors::{IntoEyreResult, OrStash};
use uuid::Uuid;

use crate::{
    configuration::Settings,
    domain::SubscriberEmail,
    email_client::EmailClient,
    hkt::{
        K1, SharedPointerHKT,
        traversable::traverse_result_future,
    },
    startup::get_connection_pool,
    utils::Pipe as _,
};

use tokio::sync::Mutex;

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

    let connection_ptr = Mutex::from(&mut *transaction);

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
    name = "Gets infinite iterator handling result of sending newsletters to subscribers."
    skip_all
)]
pub async fn get_newsletter_sending_worker_iterator<
    P: SharedPointerHKT,
>(
    email_client: &EmailClient<P>,
    connection_ptr: &Mutex<&mut sqlx::PgConnection>,
) -> impl Iterator<Item = impl Future> {
    [()]
    .into_iter()
    .cycle()
    .map(async |()|{

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
    connection_ptr: &Mutex<&mut sqlx::PgConnection>,
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
        let mut connection_borrow = connection_ptr.try_lock()
        .expect("Mutex must be free at this point.");

        let connection: &mut sqlx::PgConnection = *connection_borrow;

        struct NewsletterContent {
            title: String,
            text_content: String,
            html_content: String,
        }

        match acquire_task(connection).await {
            Ok(Some(id)) => {
                sqlx::query_as!(NewsletterContent,
                    "--sql
                    SELECT title, text_content, html_content
                    FROM newsletter_issues
                    WHERE newsletter_issue_id = $1
                    ",
                    &id
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

                    // Danger: Drop required before passing mutex ref.
                    drop(connection_borrow);

                    let iterator = get_sending_to_subscribers_of_single_newsletter_issue_iterator(
                        &subject,
                        &html_content,
                        &text_content,
                        &id,
                        email_client,
                        connection_ptr
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
    connection_ptr: &Mutex<&mut sqlx::PgConnection>,
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
        let mut connection = connection_ptr.try_lock()
        .expect("Mutex must be free at this point.");

        match acquire_task(&mut connection).await
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
                                    Err(e) => {

                                        let mut error_stash = lazy_errors::ErrorStash::<_, _,eyre::Report>::new(|| format!("Failed to send newsletter to: '{}'", subscriber_email));

                                        schedule_task_retry(
                                            &record,
                                            &mut connection
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

                        let _ = disable_task(
                            &record,
                            &mut connection,
                        ).await;

                        ControlFlow::Continue(())
                    },
                }
            },
            Ok(None) => ControlFlow::Break(Ok(())),
            Err(e) => ControlFlow::Break(Err(e))
        }
    })
}

async fn schedule_task_retry(
    record: &IssueDeliveryRecord,
    connection: &mut sqlx::PgConnection,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "--sql
        UPDATE issue_delivery_queue
        SET n_retries = n_retries + 1,
            execute_after = execute_after * 1.5
        WHERE newsletter_issue_id = $1
        ",
        &record.newsletter_issue_id
    )
    .execute(connection)
    .await
}
async fn disable_task(
    record: &IssueDeliveryRecord,
    connection: &mut sqlx::PgConnection,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "--sql
        UPDATE issue_delivery_queue
        SET enabled = false
        WHERE newsletter_issue_id = $1
        AND subscriber_email = $2",
        &record.newsletter_issue_id,
        &record.subscriber_email
    )
    .execute(connection)
    .await
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
        r#"--sql
        SELECT newsletter_issue_id as "newsletter_issue_id!",
            subscriber_email as "subscriber_email!"
        FROM get_available_issue_delivery_queue(now())
        WHERE newsletter_issue_id = $1
        FOR UPDATE
        SKIP LOCKED
        LIMIT 1
    "#,
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
) -> Result<Option<Uuid>, eyre::Report> {
    sqlx::query!(
        "--sql
        SELECT newsletter_issue_id as \"newsletter_issue_id!\"
        FROM get_available_issue_delivery_queue(now())
        FOR UPDATE
        SKIP LOCKED
        LIMIT 1
    ")
    .fetch_optional(connection)
    .await?
    .map(|i| i.newsletter_issue_id)
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
