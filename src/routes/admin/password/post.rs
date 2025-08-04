use std::{borrow::Cow, ops::Not};

use actix_web::{
    HttpResponse, http::header::LOCATION, web,
};
use argon2::{Argon2, password_hash::SaltString};
use rand::thread_rng;
use secrecy::{ExposeSecret, SecretString};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::{
    authentication::{self, UserId},
    database::transactional::authentication::AuthenticationRepository,
    dependency_injection::app_state::Inject,
    utils::{Pipe, see_other_response},
};
use argon2::PasswordHasher;

#[derive(serde::Deserialize)]
#[allow(clippy::struct_field_names)]
pub struct FormData<'a> {
    pub old_password: Cow<'a, SecretString>,
    pub new_password: Cow<'a, SecretString>,
    pub confirm_new_password: Cow<'a, SecretString>,
}

pub async fn post_reset_password<
    A: AuthenticationRepository,
>(
    user_id: web::ReqData<UserId>,
    form_data: web::Form<FormData<'_>>,
    authentication_repository: Inject<A>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id: Uuid = user_id.into_inner().into();

    let salted_password = authentication_repository
        .get_hashed_credentials_from_user_id(user_id)
        .await
        .map_err(
            actix_web::error::ErrorInternalServerError,
        )?
        .map(|credentials| credentials.salted_password)
        .ok_or_else(|| {
            actix_web::error::ErrorNotFound(
                "User not found",
            )
        })?;

    let new_password = form_data.0.new_password.as_ref();
    let new_password_len = new_password
        .expose_secret()
        .graphemes(true)
        .count();

    if new_password_len <= 12 || new_password_len >= 129 {
        actix_web_flash_messages::FlashMessage::error(format!("Password length must be between 12 and 129 characters in graphemes, but was {}.", new_password_len)).send();

        return see_other_response("/admin/reset_password")
            .pipe(Ok);
    }

    if authentication::validate_password(
        &form_data.0.old_password,
        &salted_password,
    )
    .map_err(actix_web::error::ErrorInternalServerError)?
    .not()
    {
        actix_web_flash_messages::FlashMessage::error(
            "Incorrect password.",
        )
        .send();

        return see_other_response("/admin/reset_password")
            .pipe(Ok);
    }

    if form_data.0.new_password.expose_secret()
        != form_data.0.confirm_new_password.expose_secret()
    {
        actix_web_flash_messages::FlashMessage::error("New password does not match Confirm New password.").send();

        return see_other_response("/admin/reset_password")
            .pipe(Ok);
    }

    let salt = SaltString::generate(thread_rng());

    let hash = Argon2::default()
        .hash_password(
            form_data
                .new_password
                .expose_secret()
                .as_bytes(),
            &salt,
        )
        .map_err(
            actix_web::error::ErrorInternalServerError,
        )?;

    let hash = hash
        .serialize()
        .to_string()
        .pipe(SecretString::from);

    authentication_repository
        .update_password(user_id, &hash)
        .await
        .map_err(
            actix_web::error::ErrorInternalServerError,
        )?;

    actix_web_flash_messages::FlashMessage::info(
        "Resetted Password successfully!",
    )
    .send();

    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/reset_password"))
        .finish()
        .pipe(Ok)
}
