use crate::{
    database::transactional::{
        subscriptions::SubscriptionsRepository,
        unit_of_work::{BeginUnitOfWork, UnitOfWork as _},
    },
    dependency_injection::app_state::Inject,
    domain::{
        NewSubscriber, NewSubscriberParseError,
        SubscriberEmail,
    },
    email_client::EmailClient,
    hkt::{
        RefHKT, SendHKT, SharedPointerHKT, SyncHKT,
        traversable::traverse_result_future_result,
    },
    startup::{self, ApplicationBaseUrl},
    utils::Pipe,
};
use actix_web::{
    HttpResponse, Responder,
    http::StatusCode,
    web::{self},
};
use const_format::formatcp;
use eyre::WrapErr;
use std::ops::Deref;

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    pub email: String,
    pub name: String,
}

const SUBSCRIBE_INSTRUMENT_NAME: &str =
    formatcp!("Enter Route '{}':", "subscribe");

#[tracing::instrument(
    name = SUBSCRIBE_INSTRUMENT_NAME,
    skip(form, email_client, base_url, begin_unit_of_work,
        subscriptions_repository),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe<
    B: BeginUnitOfWork,
    S: SubscriptionsRepository<UnitOfWork = B::UnitOfWork>,
>(
    form: web::Form<SubscribeFormData>,
    email_client: web::Data<
        EmailClient<startup::GlobalSharedPointerType>,
    >,
    base_url: web::ThinData<
        ApplicationBaseUrl<
            startup::GlobalSharedPointerType,
        >,
    >,
    begin_unit_of_work: Inject<B>,
    subscriptions_repository: Inject<S>,
) -> impl Responder {
    subscribe_with_shared_pointer(
        form,
        email_client,
        &base_url,
        &*begin_unit_of_work,
        &*subscriptions_repository,
    )
    .pipe(Box::pin)
    .await
}

#[tracing::instrument(
    name = SUBSCRIBE_INSTRUMENT_NAME,
    skip(form, email_client, base_url, begin_unit_of_work,
        subscriptions_repository),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe_with_shared_pointer<
    P: SharedPointerHKT + SendHKT + SyncHKT,
    DataP: RefHKT,
    B: BeginUnitOfWork,
    S: SubscriptionsRepository<UnitOfWork = B::UnitOfWork>,
>(
    form: web::Form<SubscribeFormData>,
    email_client: web::Data<EmailClient<P>>,
    base_url: &ApplicationBaseUrl<DataP>,
    begin_unit_of_work: &B,
    subscriptions_repository: &S,
) -> Result<HttpResponse, SubscribeError> {
    begin_unit_of_work.begin()
        .await
        .context("Failed to acquire Postgres connection from pool.")
        .map_err(SubscribeError::from)
        .map(async |mut unit_of_work| {
            NewSubscriber::try_from(form.0)
                .map_err(SubscribeError::from)
                .map(async |subscriber| {
                    subscriptions_repository.insert_subscriber::<P>(
                        &mut unit_of_work,
                        &subscriber,
                    )
                    .await
                    .context("Failed to insert new subscriber to database.")
                    .map_err(SubscribeError::from)
                    .map(async |subscriber_id| {
                        subscriptions_repository.store_token::<P>(
                            &mut unit_of_work,
                            &subscriber_id,
                        )
                        .await
                        .context("Failed to store confirmation token of new subscriber to database.")
                        .map_err(SubscribeError::from)
                    })
                    .pipe(traverse_result_future_result)
                    .await
                    .map(async |token| {
                        send_confirmation_email(
                            &email_client,
                            &subscriber,
                            base_url
                                .deref()
                                .deref()
                                .0
                                .ref_cast(),
                            token.to_string().as_str(),
                        )
                        .await
                        .context("Failed to send confirmation email to new subscriber's email.")
                        .map_err(SubscribeError::from)
                    })
                    .pipe(traverse_result_future_result)
                    .await
                })
                .pipe(traverse_result_future_result)
                .await
                .map(async |()| {
                    unit_of_work
                        .commit()
                        .await
                        .context("Failed to commit new subscription transaction.")
                        .map_err(SubscribeError::from)
                })
                .pipe(traverse_result_future_result)
                .await
        })
        .pipe(traverse_result_future_result)
        .await
        .map(|()| HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Send confirmation email to new subscriber",
    skip(
        email_client,
        new_subscriber,
        confirmation_link,
        subscription_token
    )
)]
pub async fn send_confirmation_email<
    P: SharedPointerHKT,
>(
    email_client: &EmailClient<P>,
    new_subscriber: &NewSubscriber<P>,
    confirmation_link: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        confirmation_link, subscription_token
    );

    email_client
        .send_email(
            new_subscriber.email.pipe_ref(SubscriberEmail::clone),
            "Welcome!"
                .pipe(P::from_static_str),
            format!(
                "Welcome to our newsletter!<br />\
                Click <a href=\"{}\">here</a> to confirm your subscription.",
                confirmation_link
            )
                .pipe(P::from_string),
            format!(
                "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
                confirmation_link
            )
                .pipe(P::from_string),
        ).await
}

#[derive(Debug, thiserror::Error)]
pub enum SubscribeError {
    #[error("Parsing new subscriber failed: {0}")]
    Validation(#[from] NewSubscriberParseError),
    #[error("{0}")]
    Unexpected(#[from] eyre::Report), // #[error("Database Error occured while attempting to store subscription token.")]
                                      // StoreTokenError(#[from] StoreTokenError),
                                      // #[error("Database Error occured while attempting to insert subscriber.")]
                                      // InsertSubscriberError(#[from] InsertSubscriberError),

                                      // #[error("Database Error occurred.")]
                                      // Sqlx(#[source] sqlx::Error),
                                      // #[error("Http Request failed.")]
                                      // Reqwest(#[from] reqwest::Error),
}

impl actix_web::ResponseError for SubscribeError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SubscribeError::Validation(_) => {
                StatusCode::BAD_REQUEST
            }
            SubscribeError::Unexpected(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
