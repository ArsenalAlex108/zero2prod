use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

use crate::{
    authentication::{
        self, BasicAuthCredentials,
        NewsletterWritersAuthenticationError, UserId,
    },
    domain::SubscriberEmail,
    email_client::EmailClient,
    hkt::{K1, SharedPointerHKT},
    startup,
    utils::{Pipe, see_other_response},
};
use actix_web::{HttpResponse, error::InternalError, web};
use eyre::Context;
use lazy_errors::{ErrorStash, StashErr};
use nameof::name_of;
use sqlx::PgPool;
use uuid::Uuid;

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

#[derive(Debug, serde::Deserialize)]
pub struct FormData<'a> {
    title: Cow<'a, str>,
    content_html: Cow<'a, str>,
    content_text: Cow<'a, str>,
}

impl<'a> From<FormData<'a>> for BodyData<'a> {
    fn from(value: FormData<'a>) -> Self {
        BodyData {
            title: value.title,
            content: Content {
                html: value.content_html,
                text: value.content_text,
            },
        }
    }
}

#[deprecated(
    note = "Authentication Errors are never returned."
)]
#[derive(Debug, thiserror::Error)]
pub enum PublishNewsletterError {
    #[error("Authentication failed.")]
    Authentication(
        #[source] lazy_errors::Error<eyre::Report>,
    ),
    #[error(transparent)]
    Unexpected(#[from] lazy_errors::Error<eyre::Report>),
}

impl From<NewsletterWritersAuthenticationError>
    for PublishNewsletterError
{
    fn from(
        value: NewsletterWritersAuthenticationError,
    ) -> Self {
        match value {
            NewsletterWritersAuthenticationError::Authentication(_) => PublishNewsletterError::Authentication(value.into()),
            NewsletterWritersAuthenticationError::Unexpected(_) => PublishNewsletterError::Unexpected(value.into()),
        }
    }
}

impl actix_web::ResponseError for PublishNewsletterError {
    fn error_response(
        &self,
    ) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            PublishNewsletterError::Authentication(_) =>
                HttpResponse::Unauthorized()
                .insert_header((
                    actix_web::http::header::WWW_AUTHENTICATE,
                "Basic realm=\"publish\""
                    .pipe(actix_web::http::header::HeaderValue::from_str)
                    .expect("The above literal is valid Header value.")
                ))
                .finish()
            ,
            PublishNewsletterError::Unexpected(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[tracing::instrument(
    name = "Publishing Newsletter To Confirmed Subscribers",
    skip(pool, email_client, body)
)]
pub async fn publish_newsletter(
    pool: web::Data<PgPool>,
    email_client: web::Data<
        EmailClient<startup::GlobalSharedPointerType>,
    >,
    body: web::Form<FormData<'_>>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    publish_newsletter_with_pointer::<
        startup::GlobalSharedPointerType,
    >(pool, email_client, body.0.into(), user_id.into_inner())
    .await
    .map_err(|e| {
        actix_web_flash_messages::FlashMessage::error("<p><i>One or more errors occurred trying to post the newsletter.</i></p>").send();
        e
    })
}

#[tracing::instrument(
    name = "Sending newsletters to confirmed subscribers.",
    skip(subscriber_emails, email_client)
)]
async fn send_newsletter<P: SharedPointerHKT>(
    body: BodyData<'_>,
    subscriber_emails: impl IntoIterator<
        Item = SubscriberEmailRaw<'_>,
    >,
    email_client: &EmailClient<P>,
) -> Result<(), lazy_errors::Error<eyre::Report>> {
    let BodyData {
        title,
        content: Content { html, text },
    } = body;
    let subject = P::from_string(title.into_owned());
    let html_content = P::from_string(html.into_owned());
    let text_content = P::from_string(text.into_owned());

    let mut error_stash =
        ErrorStash::<_, _, eyre::Report>::new(
            || "One or more Errors occurred trying to sent newsletter to confirmed subscribers.",
        );

    subscriber_emails.into_iter()
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

    error_stash.into_result()
}

#[tracing::instrument(
    name = "Publishing Newsletter To Confirmed Subscribers",
    skip(pool, email_client, body)
)]
async fn publish_newsletter_with_pointer<
    P: SharedPointerHKT,
>(
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient<P>>,
    body: BodyData<'_>,
    user_id: UserId,
) -> Result<HttpResponse, actix_web::Error>
where
    P::T<str>: Send + Sync,
    P::T<reqwest::Client>: Send + Sync,
{
    let user_id: Uuid = user_id.into();

    tracing::Span::current().record(
        name_of!(user_id),
        tracing::field::display(&user_id),
    );

    let username = sqlx::query!("--sql
        SELECT username
        FROM newsletter_writers
        WHERE user_id = $1
        ",
        &user_id
    ).fetch_one(pool.as_ref())
    .await
    .context("Failed to find user with matching user_id in the database.")
    .map_err(redirect_to_self_with_err)?
    .username;

    tracing::Span::current().record(
        name_of!(username),
        tracing::field::display(&username),
    );

    let addresses = get_confirmed_subscribers_emails_raw(&pool)
    .await
    .context("Failed to query for confirmed subscribers from database.")
    .map_err(redirect_to_self_with_err)?;

    send_newsletter(body, addresses, email_client.as_ref())
        .await
        .map(|_| {
            actix_web_flash_messages::FlashMessage::info(
                "Newsletter successfully posted",
            )
            .send();
            see_other_response("/admin/newsletters")
        })
        .map_err(redirect_to_self_with_err)?
        .pipe(Ok)
}

fn redirect_to_self_with_err<
    T: Debug + Display + 'static,
>(
    cause: T,
) -> actix_web::Error {
    actix_web_flash_messages::FlashMessage::error("<p><i>One or more errors occurred trying to post the newsletter.</i></p>").send();
    see_other_response("/admin/newsletters").pipe(|r| {
        InternalError::from_response(cause, r).into()
    })
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
