use crate::{
    authentication::UserId,
    hkt::{
        SharedPointerHKT,
        traversable::traverse_result_future_result,
    },
    idempotency::{
        IdempotencyKey, get_saved_response, save_response,
    },
    startup,
    utils::{Pipe, see_other_response},
};
use actix_web::{HttpResponse, error::InternalError, web};
use const_format::formatcp;
use eyre::Context;
use nameof::name_of;
use sqlx::Executor as _;
use sqlx::PgPool;
use std::ops::DerefMut;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct BodyData<'a> {
    title: Cow<'a, str>,
    content: Content<'a>,
    idempotency_key: Cow<'a, str>,
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
    idempotency_key: Cow<'a, str>,
}

impl<'a> From<FormData<'a>> for BodyData<'a> {
    fn from(value: FormData<'a>) -> Self {
        BodyData {
            title: value.title,
            content: Content {
                html: value.content_html,
                text: value.content_text,
            },
            idempotency_key: value.idempotency_key,
        }
    }
}

#[tracing::instrument(
    name = "Publishing Newsletter To Confirmed Subscribers",
    skip(pool, body)
)]
pub async fn publish_newsletter(
    pool: web::Data<PgPool>,
    body: web::Form<FormData<'_>>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    publish_newsletter_with_pointer::<
        startup::GlobalSharedPointerType,
    >(
        pool,
        body.0.into(),
        user_id.into_inner(),
    )
    .await
}

#[tracing::instrument(
    name = "Publishing Newsletter To Confirmed Subscribers (Generic)",
    skip(pool, body)
)]
async fn publish_newsletter_with_pointer<
    P: SharedPointerHKT,
>(
    pool: web::Data<PgPool>,
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

    let BodyData {
        title,
        content,
        idempotency_key,
    } = body;

    let idempotency_key =
        IdempotencyKey::try_from(idempotency_key)
            .map_err(actix_web::error::ErrorBadRequest)?;

    let mut transaction = pool
        .as_ref()
        .begin()
        .await
        .map_err(redirect_to_self_with_err)?;

    sqlx::query!(
        "--sql
        SET TRANSACTION ISOLATION LEVEL READ COMMITTED;
    "
    )
    .execute(transaction.deref_mut())
    .await
    .map_err(redirect_to_self_with_err)?;

    if let Some(saved_response) = get_saved_response(
        &mut transaction,
        &idempotency_key,
        user_id,
    )
    .await
    .map_err(redirect_to_self_with_err)?
    {
        success().send();
        return Ok(saved_response);
    }

    let addresses = get_confirmed_subscribers_emails_raw(&pool)
    .await
    .context("Failed to query for confirmed subscribers from database.")
    .map_err(redirect_to_self_with_err)?;

    if cfg!(test) {
        let dbg_string = addresses
            .iter()
            .map(|i| i.email.as_ref())
            .enumerate()
            .fold(String::new(), |a, b| {
                a + if b.0 == 0 { "" } else { ", " } + b.1
            });

        tracing::debug!(
            "Print Confirmed Subscribers: {}",
            dbg_string
        );
    }

    let issue_id = insert_newsletter_issue(
        transaction.deref_mut(),
        &title,
        &content.text,
        &content.html,
    )
    .await
    .map_err(redirect_to_self_with_err)?;

    enqueue_delivery_tasks(
        transaction.deref_mut(),
        issue_id,
    )
    .await
    .map_err(redirect_to_self_with_err)?;

    save_response(
        &mut transaction,
        &idempotency_key,
        user_id,
        see_other_response("/admin/newsletters"),
    )
    .await
    .map_err(redirect_to_self_with_err)
    .map(async |r| {
        Ok(())
            .map(async |_| {
                transaction
                    .commit()
                    .await
                    .map_err(redirect_to_self_with_err)
            })
            .pipe(traverse_result_future_result)
            .await
            .inspect(|_| success().send())
            .map(|_| r)
    })
    .pipe(traverse_result_future_result)
    .await
}

pub const SUCCESS_MESSAGE: &str = "Newsletter has been successfully enqueued & \
            will be sent to subscribers shortly.";

pub const ERROR_MESSAGE: &str = "One or more errors occurred trying to post the newsletter.";

fn success() -> actix_web_flash_messages::FlashMessage {
    actix_web_flash_messages::FlashMessage::info(
        SUCCESS_MESSAGE,
    )
}

fn redirect_to_self_with_err<
    T: Debug + Display + 'static,
>(
    cause: T,
) -> actix_web::Error {
    actix_web_flash_messages::FlashMessage::error(
        formatcp!("<p><i>{}</i></p>", ERROR_MESSAGE),
    )
    .send();
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

#[tracing::instrument(skip_all)]
async fn insert_newsletter_issue(
    transaction: &mut sqlx::PgConnection,
    title: &str,
    text_content: &str,
    html_content: &str,
) -> Result<Uuid, sqlx::Error> {
    let newsletter_issue_id = Uuid::new_v4();
    let query = sqlx::query!(
        "--sql
        INSERT INTO newsletter_issues (
            newsletter_issue_id,
            title,
            text_content,
            html_content,
            published_at
        )
        VALUES ($1, $2, $3, $4, now())
        ",
        newsletter_issue_id,
        title,
        text_content,
        html_content
    );
    transaction.execute(query).await?;
    Ok(newsletter_issue_id)
}

#[tracing::instrument(skip_all)]
async fn enqueue_delivery_tasks(
    transaction: &mut sqlx::PgConnection,
    newsletter_issue_id: Uuid,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
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
    );
    transaction.execute(query).await?;
    Ok(())
}
