use crate::{
    authentication::UserId,
    database::transactional::{
        authentication::AuthenticationRepository,
        issue_delivery_queue::IssueDeliveryQueueRepository,
        newsletters::NewslettersRepository,
        persistence::{
            HeaderPairRecord, PersistenceRepository,
            SavedResponseBody,
        },
        unit_of_work::{BeginUnitOfWork, UnitOfWork},
    },
    dependency_injection::app_state::Inject,
    hkt::SharedPointerHKT,
    idempotency::IdempotencyKey,
    startup,
    utils::{Pipe, see_other_response},
};
use actix_web::{
    HttpResponse, error::InternalError, http::StatusCode,
    web,
};
use const_format::formatcp;
use nameof::name_of;
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
    skip(
        authentication_repository,
        begin_unit_of_work,
        issue_delivery_queue_repository,
        newsletters_repository,
        persistence_repository,
        body
    )
)]
pub async fn publish_newsletter<
    A: AuthenticationRepository,
    B: BeginUnitOfWork,
    I: IssueDeliveryQueueRepository<
        UnitOfWork = B::UnitOfWork,
    >,
    N: NewslettersRepository<UnitOfWork = B::UnitOfWork>,
    Pr: PersistenceRepository<UnitOfWork = B::UnitOfWork>,
>(
    authentication_repository: Inject<A>,
    begin_unit_of_work: Inject<B>,
    issue_delivery_queue_repository: Inject<I>,
    newsletters_repository: Inject<N>,
    persistence_repository: Inject<Pr>,
    body: web::Form<FormData<'_>>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    publish_newsletter_with_pointer::<
        startup::GlobalSharedPointerType,
        _,
        _,
        _,
        _,
        _,
    >(
        authentication_repository,
        begin_unit_of_work,
        issue_delivery_queue_repository,
        newsletters_repository,
        persistence_repository,
        body.0.into(),
        user_id.into_inner(),
    )
    .await
}

fn restore_saved_response(
    saved_response: SavedResponseBody,
) -> Result<HttpResponse, actix_web::Error> {
    success().send();

    let status_code = StatusCode::from_u16(
        saved_response.response_status_code,
    )
    .map_err(|e| {
        InternalError::from_response(
            e,
            HttpResponse::build(
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .finish(),
        )
    })?;
    let mut response = HttpResponse::build(status_code);
    for HeaderPairRecord { name, value } in
        saved_response.response_headers
    {
        response.append_header((name, value));
    }

    Ok(response.body(saved_response.response_body))
}

async fn persist_response<
    P: PersistenceRepository<UnitOfWork = U>,
    U: UnitOfWork,
>(
    persistence_repository: &P,
    unit_of_work: &mut U,
    user_id: UserId,
    idempotency_key: &IdempotencyKey<'_>,
) -> Result<
    HttpResponse<actix_web::body::BoxBody>,
    actix_web::Error,
> {
    let http_response =
        see_other_response("/admin/newsletters");

    let status_code = http_response.status().as_u16();

    let (headers_response, body) =
        http_response.into_parts();

    let headers = headers_response
        .headers()
        .iter()
        .map(|pair| {
            let (name, value) = pair;
            let name = name.as_str().to_owned();
            let value = value.as_bytes().to_owned();
            HeaderPairRecord { name, value }
        })
        .collect::<Vec<_>>();

    let body = actix_web::body::to_bytes(body)
        .await
        .map_err(|e| {
            InternalError::from_response(
                e,
                HttpResponse::build(
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .finish(),
            )
        })?;

    persistence_repository
        .save_response_body(
            unit_of_work,
            user_id.into(),
            idempotency_key,
            status_code,
            headers,
            body.iter().as_slice(),
        )
        .await
        .map_err(redirect_to_self_with_err)?;

    Ok(headers_response
        .set_body(body)
        .map_into_boxed_body())
}

#[tracing::instrument(
    name = "Publishing Newsletter To Confirmed Subscribers (Generic)",
    skip(
        authentication_repository,
        begin_unit_of_work,
        issue_delivery_queue_repository,
        newsletters_repository,
        persistence_repository,
        body
    )
)]
async fn publish_newsletter_with_pointer<
    P: SharedPointerHKT,
    A: AuthenticationRepository,
    B: BeginUnitOfWork,
    I: IssueDeliveryQueueRepository<
        UnitOfWork = B::UnitOfWork,
    >,
    N: NewslettersRepository<UnitOfWork = B::UnitOfWork>,
    Pr: PersistenceRepository<UnitOfWork = B::UnitOfWork>,
>(
    authentication_repository: Inject<A>,
    begin_unit_of_work: Inject<B>,
    issue_delivery_queue_repository: Inject<I>,
    newsletters_repository: Inject<N>,
    persistence_repository: Inject<Pr>,
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

    let username = authentication_repository
        .get_hashed_credentials_from_user_id(user_id)
        .await
        .map_err(
            actix_web::error::ErrorInternalServerError,
        )?
        .ok_or_else(|| {
            actix_web::error::ErrorNotFound(
                "User not found",
            )
        })
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

    let mut unit_of_work = begin_unit_of_work
        .begin()
        .await
        .map_err(redirect_to_self_with_err)?;

    if let Some(saved_response) = persistence_repository
        .get_saved_response_body(
            &mut unit_of_work,
            &idempotency_key,
            user_id,
        )
        .await
        .map_err(|e| {
            redirect_to_self_with_err(e.into_owned())
        })?
    {
        return restore_saved_response(saved_response);
    }

    let issue_id = newsletters_repository
        .insert_newsletter_issue(
            &mut unit_of_work,
            &title,
            &content.text,
            &content.html,
        )
        .await
        .map_err(redirect_to_self_with_err)?;

    issue_delivery_queue_repository
        .enqueue_delivery_tasks(&mut unit_of_work, issue_id)
        .await
        .map_err(redirect_to_self_with_err)?;

    let headers_response = persist_response(
        &*persistence_repository,
        &mut unit_of_work,
        user_id.into(),
        &idempotency_key,
    )
    .await
    .map_err(redirect_to_self_with_err)?;

    unit_of_work
        .commit()
        .await
        .map_err(redirect_to_self_with_err)?;

    success().send();

    headers_response.pipe(Ok)
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
