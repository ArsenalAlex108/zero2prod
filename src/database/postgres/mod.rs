use std::marker::PhantomData;

use secrecy::{ExposeSecret as _, SecretString};
use uuid::Uuid;

use crate::{
    database::transactional::{
        authentication::{
            AuthenticationRepository,
            GetHashedCredentialsError, HashedCredentials,
        },
        issue_delivery_queue::{
            DisableTaskError, EnqueueDeliveryTaskResult,
            FinalizeNewsletterTaskError,
            IssueDeliveryQueueRepository,
            ScheduleTaskRetryError,
        },
        newsletters::{
            GetNewsletterContentError, NewsletterContent,
            NewslettersRepository,
        },
        persistence::{
            GetSavedResponseBodyError, HeaderPairRecord,
            PersistenceRepository, SaveResponseBodyError,
            SavedResponseBody,
        },
        subscriptions::SubscriptionsRepository,
        subscriptions_confirm::{
            GetSubscriberIdOfConfirmationTokenError,
            SubscriptionsConfirmRepository,
            UpdateConfirmationStatusOfSubscriberIdError,
        },
        unit_of_work::{
            BeginError, BeginUnitOfWork, UnitOfWork,
            UnitOfWorkRepository,
        },
    },
    domain::NewSubscriber,
    hkt::{SendHKT, SharedPointerHKT, SyncHKT},
    idempotency::IdempotencyKey,
    issue_delivery_worker::IssueDeliveryRecord,
    services::{clock::Clock, uuid::UuidGenerator},
    startup::GlobalSharedPointer,
    utils::Pipe,
};

use super::transactional::{
    authentication::UpdatePasswordError,
    issue_delivery_queue::{
        AcquireNewsletterTaskError,
        AcquireNewsletterTaskFromIssueError,
        EnqueueDeliveryTaskError,
    },
    newsletters::InsertNewsletterIssueError,
    subscriptions::{
        InsertSubscriberError, StoreTokenError,
    },
    unit_of_work::CommitError,
};

pub struct PgPool<D: PgPoolDependencies>(
    sqlx::PgPool,
    PhantomData<D>,
);

impl<D: PgPoolDependencies> PgPool<D> {
    #[must_use]
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self(pool, PhantomData)
    }

    #[must_use]
    pub fn pool(&self) -> &sqlx::PgPool {
        &self.0
    }
}

pub trait PgPoolDependencies: Send + Sync {}

#[derive(
    Debug, derive_more::Deref, derive_more::DerefMut,
)]
pub struct PgTransaction(
    #[deref(forward)]
    #[deref_mut(forward)]
    sqlx::Transaction<'static, sqlx::Postgres>,
);

impl UnitOfWork for PgTransaction {
    async fn commit(self) -> Result<(), CommitError> {
        self.0.commit().await.map_err(eyre::Report::new)?;

        Ok(())
    }
}

pub struct PgRepository<D: PgRepositoryDependencies> {
    clock: GlobalSharedPointer<D::Clock>,
    uuid_generator: GlobalSharedPointer<D::UuidGenerator>,
}

impl<D: PgRepositoryDependencies> PgRepository<D> {
    pub fn new(
        clock: GlobalSharedPointer<D::Clock>,
        uuid_generator: GlobalSharedPointer<
            D::UuidGenerator,
        >,
    ) -> Self {
        Self {
            clock,
            uuid_generator,
        }
    }
}

pub trait PgRepositoryDependencies: Send + Sync {
    type Clock: Clock;
    type UuidGenerator: UuidGenerator;
}

impl<D: PgRepositoryDependencies> UnitOfWorkRepository
    for PgRepository<D>
{
    type UnitOfWork = PgTransaction;
}

impl<D: PgPoolDependencies> BeginUnitOfWork for PgPool<D> {
    type UnitOfWork = PgTransaction;
    async fn begin(
        &self,
    ) -> Result<PgTransaction, BeginError> {
        self.0
            .begin()
            .await
            .map(PgTransaction)
            .map_err(eyre::Report::new)?
            .pipe(Ok)
    }
}

impl<D: PgRepositoryDependencies>
    IssueDeliveryQueueRepository for PgRepository<D>
{
    async fn schedule_task_retry(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        record: &IssueDeliveryRecord,
    ) -> Result<(), ScheduleTaskRetryError> {
        sqlx::query!(
            "--sql
            UPDATE issue_delivery_queue
            SET n_retries = n_retries + 1,
                execute_after = execute_after * 1.5
            WHERE newsletter_issue_id = $1
            ",
            &record.newsletter_issue_id
        )
        .execute(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)
        .map_err(ScheduleTaskRetryError::from)?;

        Ok(())
    }

    async fn disable_task(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        record: &IssueDeliveryRecord,
    ) -> Result<(), DisableTaskError> {
        sqlx::query!(
            "--sql
            UPDATE issue_delivery_queue
            SET enabled = false
            WHERE newsletter_issue_id = $1
            AND subscriber_email = $2",
            &record.newsletter_issue_id,
            &record.subscriber_email
        )
        .execute(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)?;

        Ok(())
    }

    #[tracing::instrument(
        name = "Get and uniquely lock a task in the issue delivery queue from a specific issue.",
        skip_all
    )]
    async fn acquire_newsletter_task_from_issue(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        newsletter_issue_id: uuid::Uuid,
    ) -> Result<
        Option<IssueDeliveryRecord>,
        AcquireNewsletterTaskFromIssueError,
    > {
        sqlx::query_as!(
            IssueDeliveryRecord,
            r#"--sql
            SELECT newsletter_issue_id as "newsletter_issue_id!",
                subscriber_email as "subscriber_email!"
            FROM get_available_issue_delivery_queue($2)
            WHERE newsletter_issue_id = $1
            FOR UPDATE
            SKIP LOCKED
            LIMIT 1
        "#,
            newsletter_issue_id,
            self.clock.now()
        )
        .fetch_optional(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)?
        .pipe(Ok)
    }

    #[tracing::instrument(
        name = "Get and uniquely lock a task in the issue delivery queue.",
        skip_all
    )]
    async fn acquire_newsletter_task(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
    ) -> Result<Option<Uuid>, AcquireNewsletterTaskError>
    {
        sqlx::query!(
            "--sql
            SELECT newsletter_issue_id as \"newsletter_issue_id!\"
            FROM get_available_issue_delivery_queue($1)
            FOR UPDATE
            SKIP LOCKED
            LIMIT 1
        ",
        self.clock.now(),
        )
        .fetch_optional(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)?
        .map(|i| i.newsletter_issue_id)
        .pipe(Ok)
    }

    #[tracing::instrument(
        name = "Delete a task in the issue delivery queue after completion.",
        skip_all
    )]
    async fn finalize_newsletter_task(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        record: IssueDeliveryRecord,
    ) -> Result<(), FinalizeNewsletterTaskError> {
        sqlx::query!(
            "--sql
            DELETE FROM issue_delivery_queue
            WHERE newsletter_issue_id = $1
            AND subscriber_email = $2
            ",
            record.newsletter_issue_id,
            record.subscriber_email
        )
        .execute(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn enqueue_delivery_tasks(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        newsletter_issue_id: Uuid,
    ) -> Result<
        EnqueueDeliveryTaskResult,
        EnqueueDeliveryTaskError,
    > {
        let result = sqlx::query!(
            "--sql
            INSERT INTO issue_delivery_queue (
                newsletter_issue_id,
                subscriber_email
            )
            SELECT $1, email
            FROM subscriptions
            WHERE status = 'confirmed'
            ",
            newsletter_issue_id,
        )
        .execute(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)?;

        if result.rows_affected() == 0 {
            return Ok(
                EnqueueDeliveryTaskResult::Unchanged,
            );
        }

        Ok(EnqueueDeliveryTaskResult::Enqueued)
    }
}

impl<D: PgPoolDependencies> AuthenticationRepository
    for PgPool<D>
{
    async fn get_hashed_credentials_from_username(
        &self,
        username: &str,
    ) -> Result<
        Option<HashedCredentials>,
        GetHashedCredentialsError,
    > {
        sqlx::query_as!(
            HashedCredentials,
            "--sql
            SELECT user_id, username, salted_password
            FROM newsletter_writers
            WHERE username = $1",
            &username
        )
        .fetch_optional(&self.0)
        .await
        .map_err(|e| {
            GetHashedCredentialsError::Unexpected(
                e.pipe(eyre::Report::new),
            )
        })
    }

    async fn get_hashed_credentials_from_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<
        Option<HashedCredentials>,
        GetHashedCredentialsError,
    > {
        sqlx::query_as!(
            HashedCredentials,
            "--sql
            SELECT user_id, username, salted_password
            FROM newsletter_writers
            WHERE user_id = $1",
            &user_id
        )
        .fetch_optional(&self.0)
        .await
        .map_err(|e| {
            GetHashedCredentialsError::Unexpected(
                e.pipe(eyre::Report::new),
            )
        })?
        .pipe(Ok)
    }

    async fn update_password(
        &self,
        user_id: Uuid,
        new_salted_password: &SecretString,
    ) -> Result<(), UpdatePasswordError> {
        sqlx::query!(
            "--sql
        UPDATE newsletter_writers SET
        salted_password = $1
        WHERE user_id = $2
        ",
            &new_salted_password.expose_secret(),
            &user_id
        )
        .execute(&self.0)
        .await
        .map_err(|e| {
            UpdatePasswordError::Unexpected(
                e.pipe(eyre::Report::new),
            )
        })?;

        Ok(())
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
pub struct SqlxHeaderPairRecord {
    name: String,
    value: Vec<u8>,
}

impl<D: PgRepositoryDependencies> PersistenceRepository
    for PgRepository<D>
{
    async fn get_saved_response_body<'a>(
        &self,
        unit_of_work: &'a mut Self::UnitOfWork,
        idempotency_key: &'a IdempotencyKey<'a>,
        user_id: Uuid,
    ) -> Result<
        Option<SavedResponseBody>,
        GetSavedResponseBodyError<'a>,
    > {
        sqlx::query!(
            r#"--sql
            SELECT
            response_status_code,
            response_headers as "response_headers: Vec<SqlxHeaderPairRecord>",
            response_body
            FROM idempotency
            WHERE
            user_id = $1 AND
            idempotency_key = $2
            "#,
            user_id,
            idempotency_key.as_ref()
        )
        .fetch_optional(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)?
        .map(|r| SavedResponseBody {
            response_status_code: r.response_status_code.pipe(u16::try_from)
                .unwrap(),
            response_headers: r.response_headers.into_iter()
                .map(|h| HeaderPairRecord {
                    name: h.name,
                    value: h.value,
                })
                .collect(),
            response_body: r.response_body,
        })
        .pipe(Ok)
    }

    async fn save_response_body(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        user_id: Uuid,
        idempotency_key: &IdempotencyKey<'_>,
        status_code: u16,
        headers: Vec<HeaderPairRecord>,
        body: &[u8],
    ) -> Result<(), SaveResponseBodyError> {
        sqlx::query_unchecked!(
            "--sql
            INSERT INTO idempotency (user_id, idempotency_key, response_status_code, response_headers, response_body, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ",
            user_id,
            idempotency_key.as_ref(),
            status_code.pipe(i16::try_from)
                .unwrap(),
            headers.into_iter()
                .map(|h| SqlxHeaderPairRecord {
                    name: h.name,
                    value: h.value,
                })
                .collect::<Vec<_>>(),
            body,
            self.clock.now()
        )
        .execute(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)?;

        Ok(())
    }
}

impl<D: PgPoolDependencies> SubscriptionsConfirmRepository
    for PgPool<D>
{
    async fn update_status_of_subscriber_id_to_confirmed(
        &self,
        subscriber_id: Uuid,
    ) -> Result<
        (),
        UpdateConfirmationStatusOfSubscriberIdError,
    > {
        sqlx::query!(
            "--sql
            UPDATE subscriptions
            SET status = 'confirmed'
            WHERE id = $1",
            &subscriber_id
        )
        .execute(&self.0)
        .await
        .pipe(|i| {
            use UpdateConfirmationStatusOfSubscriberIdError as E;
            match i {
                Ok(result) => {
                    let row_count = result.rows_affected();
                    if row_count == 1 { Ok(()) }
                    else { Err(E::AbnormalUpdatedRowCount(row_count.pipe(usize::try_from).unwrap())) }
                }
                Err(e) => {
                    E::Unexpected(e.into())
                    .pipe(Err)
                }
            }
        })
    }

    async fn get_subscriber_id_of_confirmation_token(
        &self,
        subscription_token: Uuid,
    ) -> Result<Uuid, GetSubscriberIdOfConfirmationTokenError>
    {
        sqlx::query!(
            "--sql
            SELECT subscriber_id
            FROM subscription_tokens
            WHERE id = $1",
            &subscription_token
        )
        .fetch_one(&self.0)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => GetSubscriberIdOfConfirmationTokenError::TokenNotFound { subscription_token },
            _ => GetSubscriberIdOfConfirmationTokenError::Unexpected(e.pipe(eyre::Report::new)),
        })
        .map(|r| r.subscriber_id)
    }
}

impl<D: PgRepositoryDependencies> NewslettersRepository
    for PgRepository<D>
{
    async fn get_newsletter_content(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        uuid: uuid::Uuid,
    ) -> Result<NewsletterContent, GetNewsletterContentError>
    {
        sqlx::query_as!(
            NewsletterContent,
            "--sql
            SELECT title, text_content, html_content
            FROM newsletter_issues
            WHERE newsletter_issue_id = $1
            ",
            &uuid
        )
        .fetch_one(&mut **unit_of_work)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                GetNewsletterContentError::NotFound(uuid)
            }
            _ => eyre::Report::new(e).pipe(
                GetNewsletterContentError::Unexpected,
            ),
        })
    }

    async fn insert_newsletter_issue(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        title: &str,
        text_content: &str,
        html_content: &str,
    ) -> Result<Uuid, InsertNewsletterIssueError> {
        let newsletter_issue_id =
            self.uuid_generator.generate_uuid();
        sqlx::query!(
            "--sql
            INSERT INTO newsletter_issues (
                newsletter_issue_id,
                title,
                text_content,
                html_content,
                published_at
            )
            VALUES ($1, $2, $3, $4, $5)
            ",
            newsletter_issue_id,
            title,
            text_content,
            html_content,
            self.clock.now(),
        )
        .execute(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)?;

        Ok(newsletter_issue_id)
    }
}

impl<D: PgRepositoryDependencies> SubscriptionsRepository
    for PgRepository<D>
{
    async fn insert_subscriber<
        P: SharedPointerHKT + SendHKT + SyncHKT,
    >(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        form: &NewSubscriber<P>,
    ) -> Result<Uuid, InsertSubscriberError> {
        let subscriber_id =
            self.uuid_generator.generate_uuid();
        sqlx::query!(
            r#"--sql
            INSERT INTO subscriptions (id, email, name, subscribed_at, status)
            VALUES ($1, $2, $3, $4, 'pending_confirmation')
            "#,
            subscriber_id,
            &*form.email,
            &*form.name,
            self.clock.now()
        )
        .execute(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)
        .map(|_| subscriber_id)?
        .pipe(Ok)
    }

    async fn store_token<P: SharedPointerHKT>(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        subscriber_id: &Uuid,
    ) -> Result<Uuid, StoreTokenError> {
        let token = self.uuid_generator.generate_uuid();
        sqlx::query!(
            r#"--sql
            INSERT INTO subscription_tokens (id, subscriber_id)
            VALUES ($1, $2)
            "#,
            token,
            subscriber_id,
        )
        .execute(&mut **unit_of_work)
        .await
        .map_err(eyre::Report::new)?;

        Ok(token)
    }
}
