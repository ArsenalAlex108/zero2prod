use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;
use zero2prod::{
    authentication,
    database::transactional::authentication::AuthenticationRepository,
    utils::Pipe,
};

use crate::common::{
    self, assert_is_redirect_to,
    create_test_newsletter_writer,
    get_test_newsletter_writer,
};

#[actix_web::test]
async fn get_reset_password_redirect_to_login_without_valid_session()
 {
    let app = common::spawn_app().await;

    let response = app
        .get_reset_password_form()
        .await
        .expect("This should always return a response.");

    assert_is_redirect_to(&response, "/login");
}

#[actix_web::test]
async fn post_reset_password_redirect_to_login_without_valid_session()
 {
    let app = common::spawn_app().await;

    let response = app.post_reset_password(
        &serde_json::json!({
            "old_password": Uuid::new_v4().to_string(),
            "new_password": Uuid::new_v4().to_string(),
            "confirm_new_password": Uuid::new_v4().to_string()
        })
    )
    .await
    .expect("This should always return a response.");

    assert_is_redirect_to(&response, "/login");
}

#[actix_web::test]
async fn reset_password_redirect_to_itself_with_error_after_invalid_le_new_password()
 {
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    let test_user = get_test_newsletter_writer();

    let _login_response =
        app.post_login_with_default().await.expect(
            "Logging in with test user must be successful.",
        );

    let response = app.post_reset_password(
        &serde_json::json!({
            "old_password": test_user.raw_password.as_ref().expose_secret(),
            "new_password": "",
            "confirm_new_password": "",
        })
    )
    .await
    .expect("This should always return a response.");

    assert_is_redirect_to(
        &response,
        "/admin/reset_password",
    );

    let text = app
        .get_reset_password_form()
        .await
        .expect("This should always return a response.")
        .text()
        .await
        .expect("Response should have a body.");

    assert!(text.contains("Password length must be between 12 and 129 characters in graphemes, but was 0."));
}

#[actix_web::test]
async fn reset_password_redirect_to_itself_with_error_after_incorrect_old_password()
 {
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    let _login_response =
        app.post_login_with_default().await.expect(
            "Logging in with test user must be successful.",
        );

    let response = app.post_reset_password(
        &serde_json::json!({
            "old_password": Uuid::new_v4().to_string(),
            "new_password": Uuid::new_v4().to_string(),
            "confirm_new_password": Uuid::new_v4().to_string()
        })
    )
    .await
    .expect("This should always return a response.");

    assert_is_redirect_to(
        &response,
        "/admin/reset_password",
    );

    let text = app
        .get_reset_password_form()
        .await
        .expect("This should always return a response.")
        .text()
        .await
        .expect("Response should have a body.");

    assert!(text.contains("Incorrect password"));
}

#[actix_web::test]
async fn reset_password_redirect_to_itself_with_error_after_new_password_does_not_match_confirm_new_password()
 {
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    let test_user = get_test_newsletter_writer();

    let _login_response =
        app.post_login_with_default().await.expect(
            "Logging in with test user must be successful.",
        );

    let response = app.post_reset_password(
        &serde_json::json!({
            "old_password": test_user.raw_password.as_ref().expose_secret(),
            "new_password": Uuid::new_v4().to_string(),
            "confirm_new_password": Uuid::new_v4().to_string(),
        })
    )
    .await
    .expect("This should always return a response.");

    assert_is_redirect_to(
        &response,
        "/admin/reset_password",
    );

    let text = app
        .get_reset_password_form()
        .await
        .expect("This should always return a response.")
        .text()
        .await
        .expect("Response should have a body.");

    assert!(text.contains(
        "New password does not match Confirm New password."
    ));
}

#[actix_web::test]
async fn reset_password_redirect_to_dashboard_with_success_after_correct_old_password_and_valid_new_password()
 {
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    let test_user = get_test_newsletter_writer();

    let _login_response =
        app.post_login_with_default().await.expect(
            "Logging in with test user must be successful.",
        );

    let new_password = Uuid::new_v4().to_string();

    let response = app.post_reset_password(
        &serde_json::json!({
            "old_password": test_user.raw_password.as_ref().expose_secret(),
            "new_password": &new_password,
            "confirm_new_password": &new_password,
        })
    )
    .await
    .expect("This should always return a response.");

    assert_is_redirect_to(
        &response,
        "/admin/reset_password",
    );

    let text = app
        .get_reset_password_form()
        .await
        .expect(
            "A response body should always be returned.",
        )
        .text()
        .await
        .unwrap();

    dbg!(&text);

    assert!(
        text.contains("Resetted Password successfully")
    );
}

#[actix_web::test]
async fn reset_password_persists_new_password_to_database_after_success()
 {
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    let test_user = get_test_newsletter_writer();

    let _login_response =
        app.post_login_with_default().await.expect(
            "Logging in with test user must be successful.",
        );

    let new_password = Uuid::new_v4().to_string();

    let _response = app.post_reset_password(
        &serde_json::json!({
            "old_password": test_user.raw_password.as_ref().expose_secret(),
            "new_password": &new_password,
            "confirm_new_password": &new_password,
        })
    )
    .await
    .expect("This should always return a response.");

    let new_password_hash = app.app_state
    .authentication_repository
    .get_hashed_credentials_from_username(
        &test_user.username,
    )
    .await
    .expect("Fetching test user from database should be successful.")
    .expect("Test user should exist in database.")
    .salted_password;

    let new_password =
        new_password.pipe(SecretString::from);

    assert!(
        authentication::validate_password(
            &new_password,
            &new_password_hash
        )
        .expect(
            "Password validation should be successful."
        ),
        "Password hash should have been updated to new password."
    );
}
