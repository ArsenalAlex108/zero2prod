use eyre::Context;
use secrecy::ExposeSecret;
use zero2prod::database::postgres::{
    PgPool, PgPoolDependencies,
};

use crate::common::test_dependency_injection::test_database::{get_subscriptions_repository::GetSubscriptionsRepository, insert_newsletter_writer_repository::InsertNewsletterWriterRepository, repository_suspender::RepositorySuspender};

use super::get_subscriptions_repository::SubscriptionStatus;

impl<D: PgPoolDependencies> GetSubscriptionsRepository
    for PgPool<D>
{
    async fn get_subscriptions(
        &self,
        username: &str,
    ) -> Result<
        super::get_subscriptions_repository::Subscription,
        eyre::Report,
    > {
        sqlx::query!(
            "SELECT email, name, status FROM subscriptions WHERE name = $1",
            username
        )
        .fetch_one(self.pool())
        .await
        .context("Expected to fetch subscription")
        .and_then(|row| {
            let status = SubscriptionStatus::try_from(row.status.as_str())
                .map_err(|_| eyre::eyre!("Expected valid subscription status"))?;
            Ok(super::get_subscriptions_repository::Subscription {
                email: row.email,
                name: row.name,
                status,
            })
        })
    }
}

impl<D: PgPoolDependencies> InsertNewsletterWriterRepository
    for PgPool<D>
{
    async fn insert(
        &self,
        user_id: uuid::Uuid,
        username: &str,
        password_hash: &secrecy::SecretString,
    ) -> Result<(), eyre::Report> {
        sqlx::query!("--sql
            INSERT INTO newsletter_writers (user_id, username, salted_password) 
            VALUES ($1, $2, $3)",
            user_id,
            username,
            password_hash.expose_secret(),
        )
        .execute(self.pool())
        .await
        .context("Expected to insert newsletter writer")?;

        Ok(())
    }
}

impl<D: PgPoolDependencies> RepositorySuspender
    for PgPool<D>
{
    async fn suspend(&self) -> Result<(), eyre::Report> {
        sqlx::query!(
            "--sql
            DROP SCHEMA public CASCADE"
        )
        .execute(self.pool())
        .await
        .context("Expected to suspend repositories.")?;

        Ok(())
    }
}
