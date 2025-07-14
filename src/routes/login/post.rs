use std::borrow::Cow;

use actix_web::HttpResponse;
use actix_web::http::header::LOCATION;
use actix_web::web;
use nameof::name_of;
use secrecy::SecretString;
use sqlx::PgPool;

use crate::authentication;
use crate::authentication::BasicAuthCredentials;
use crate::authentication::NewsletterWritersAuthenticationError;
use crate::session_state::TypedSession;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LoginFormData<'a> {
    username: Cow<'a, str>,
    password: Cow<'a, SecretString>,
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed.")]
    Authentication(
        #[source] lazy_errors::Error<eyre::Report>,
    ),
    #[error(transparent)]
    Unexpected(#[from] lazy_errors::Error<eyre::Report>),
}

impl From<NewsletterWritersAuthenticationError>
    for LoginError
{
    fn from(
        value: NewsletterWritersAuthenticationError,
    ) -> Self {
        match value {
            NewsletterWritersAuthenticationError::Authentication(_) => LoginError::Authentication(value.into()),
            NewsletterWritersAuthenticationError::Unexpected(_) => LoginError::Unexpected(value.into()),
        }
    }
}

impl actix_web::ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        let encoded_error =
            urlencoding::Encoded::new(self.to_string());

        HttpResponse::build(self.status_code())
            .insert_header((
                LOCATION,
                format!("/login?error={}", encoded_error),
            ))
            .finish()
    }

    // fn status_code(&self) -> actix_web::http::StatusCode {
    //     use actix_web::http::StatusCode;

    //     match self {
    //         LoginError::Authentication(_) => StatusCode::UNAUTHORIZED,
    //         LoginError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
    //     }
    // }

    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;
        StatusCode::SEE_OTHER
    }
}

#[tracing::instrument(
    name = "Handling POST login request.",
    skip(pool, form, session)
)]
pub async fn login(
    pool: web::Data<PgPool>,
    form: web::Form<LoginFormData<'_>>,
    session: TypedSession,
) -> Result<
    HttpResponse,
    actix_web::error::InternalError<LoginError>,
> {
    let credentials = BasicAuthCredentials {
        username: form.0.username,
        raw_password: form.0.password,
    };

    tracing::Span::current().record(
        name_of!(username in BasicAuthCredentials),
        tracing::field::display(&credentials.username),
    );

    authentication::authenticate_newsletter_writer(
        pool.as_ref(),
        credentials,
    )
    .await
    .map_err(LoginError::from)
    .and_then(|user_id| {
        tracing::Span::current().record(
            name_of!(user_id),
            tracing::field::display(&user_id),
        );
        session.renew();
        session
            .insert_user_id(user_id)
            .map_err(eyre::Report::new)
            .map_err(lazy_errors::Error::wrap)
            .map_err(LoginError::Unexpected)
            .map(|_| {
                HttpResponse::SeeOther()
                    .insert_header((
                        LOCATION,
                        "/admin/dashboard",
                    ))
                    .finish()
            })
    })
    .map_err(|e| {
        actix_web_flash_messages::FlashMessage::error(
            e.to_string(),
        )
        .send();

        actix_web::error::InternalError::from_response(
            e,
            HttpResponse::SeeOther()
                .insert_header((LOCATION, "/login"))
                .finish(),
        )
    })
}
