use std::borrow::Cow;

use crate::{
    domain::SubscriberEmail,
    email_client::EmailClient,
    hkt::{K1, SharedPointerHKT},
    startup,
    utils::Pipe,
};
use actix_web::{HttpResponse, web};
use eyre::Context;
use lazy_errors::{ErrorStash, StashErr};
use sqlx::PgPool;

#[derive(Debug, serde::Deserialize)]
pub struct BodyData<'a> {
    title: Cow<'a, str>,
    content: Content<'a>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Content<'a> {
    html: Cow<'a, str>,
    text: Cow<'a, str>,
}

#[derive(Debug, thiserror::Error)]
pub enum PublishNewsletterError {
    #[error(transparent)]
    Unexpected(#[from] lazy_errors::Error<eyre::Report>),
}

impl actix_web::ResponseError for PublishNewsletterError {}

#[tracing::instrument(
    name = "Publishing Newsletter To Confirmed Subscribers",
    skip(pool, email_client, body)
)]
pub async fn publish_newsletter(
    pool: web::Data<PgPool>,
    email_client: web::Data<
        EmailClient<startup::GlobalSharedPointerType>,
    >,
    body: web::Json<BodyData<'_>>,
) -> Result<HttpResponse, PublishNewsletterError> {
    publish_newsletter_with_pointer::<
        startup::GlobalSharedPointerType,
    >(pool, email_client, body)
    .await
}

#[tracing::instrument(
    name = "Publishing Newsletter To Confirmed Subscribers",
    skip(pool, email_client, body)
)]
pub async fn publish_newsletter_with_pointer<
    P: SharedPointerHKT,
>(
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient<P>>,
    body: web::Json<BodyData<'_>>,
) -> Result<HttpResponse, PublishNewsletterError>
where
    P::T<str>: Send + Sync,
    P::T<reqwest::Client>: Send + Sync,
{
    let addresses = get_confirmed_subscribers_emails_raw(&pool)
    .await
    .context("Failed to query for confirmed subscribers from database.")
    .map_err(lazy_errors::Error::wrap)?;

    let BodyData {
        title,
        content: Content { html, text },
    } = body.0;
    let subject = P::from_string(title.into_owned());
    let html_content = P::from_string(html.into_owned());
    let text_content = P::from_string(text.into_owned());

    let mut error_stash =
        ErrorStash::<_, _, eyre::Report>::new(
            || "One or more Errors occurred trying to sent newsletter to confirmed subscribers.",
        );

    // Does this chain need refactoring into a seperate method?
    addresses.into_iter()
    .filter_map(|i| {
        SubscriberEmail::try_from(i.email.as_ref().to_string())
        .inspect_err(|e| tracing::warn!("Found subscriber with invalid email while attempting to send a newsletter to them: '{0}'\n
        Error: '{e}'", i.email))
        .ok()
    })
    .map(
        async |i| email_client
        .send_email(
            i.pipe_ref(SubscriberEmail::clone),
            subject.pipe_ref(K1::clone),
            html_content.pipe_ref(K1::clone),
            text_content.pipe_ref(K1::clone),
        ).await
        .map_err(|e| e
            .pipe(eyre::Report::new)
            .wrap_err(format!("Failed to send newsletter to: '{}'", i))
        )
    )
    .pipe(futures::future::join_all)
    .await
    .into_iter()
    .pipe(|i|
        <_ as StashErr::<_, _, _, eyre::Report>>::stash_err(i, &mut error_stash)
    );

    error_stash
        .into_result()
        .map(|_| HttpResponse::Ok().finish())?
        .pipe(Ok)
}

#[derive(Debug, serde::Deserialize)]
struct SubscriberEmailRaw<'a> {
    pub email: Cow<'a, str>,
}

#[tracing::instrument(
    name = "Get email address of confirmed subscribers without any validation.",
    skip(pool)
)]
async fn get_confirmed_subscribers_emails_raw(
    pool: &PgPool,
) -> Result<Vec<SubscriberEmailRaw<'_>>, sqlx::Error> {
    sqlx::query_as!(
        SubscriberEmailRaw::<'_>,
        "--sql
    SELECT email FROM subscriptions 
    WHERE status = 'confirmed'"
    )
    .fetch_all(pool)
    .await
}
