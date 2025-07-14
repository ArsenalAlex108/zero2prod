
use actix_web::body::MessageBody;
use actix_web::error::InternalError;
use actix_web::middleware::Next;

use crate::hkt::traversable::traverse_result_future_result;
use crate::session_state::TypedSession;
use crate::utils::{Pipe, see_other_response};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{FromRequest, HttpMessage};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
)]
pub struct UserId(Uuid);

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<
    ServiceResponse<impl MessageBody>,
    actix_web::Error,
> {
    let (http_request, payload) = req.parts_mut();
    let session =
        TypedSession::from_request(http_request, payload)
            .await?;

    session
        .get_required_user_id()
        .map(async |user_id| {
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        })
        .map_err(|e| {
            InternalError::from_response(
                e,
                see_other_response("/login"),
            )
            .pipe(actix_web::Error::from)
        })
        .pipe(traverse_result_future_result)
        .await
}
