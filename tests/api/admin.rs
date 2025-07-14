use crate::common::{self, assert_is_redirect_to};
use crate::common::{
    create_test_newsletter_writer,
    get_test_newsletter_writer,
};
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;
use zero2prod::utils::Pipe;

#[actix_web::test]
async fn redirect_to_login_if_accessed_and_session_validation_failed()
 {
    let app = common::spawn_app().await;

    let response = app.get_admin_dashboard()
    .await
    .expect("Access to admin dashboard should return a response even if denied access.");

    assert_is_redirect_to(&response, "/login");
}

#[actix_web::test]
pub async fn changing_password_logout_login_workflow_succeeds()
 {
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    let test_user = get_test_newsletter_writer();

    let _response = app.post_login(
        &serde_json::json!({
            "username": test_user.username.as_ref(),
            "password": test_user.raw_password.as_ref().expose_secret(),
        })
    )
    .await
    .expect("Request succeed.");

    let new_password =
        Uuid::new_v4().to_string().pipe(SecretString::from);

    let _response = app.post_reset_password(
        &serde_json::json!({
            "old_password": test_user.raw_password.as_ref().expose_secret(),
            "new_password": &new_password.expose_secret(),
            "confirm_new_password": &new_password.expose_secret(),
        })
    )
    .await
    .expect("This should always return a response.");

    let response = app
        .post_logout()
        .await
        .expect("Logout should succeed.");

    assert_is_redirect_to(&response, "/login");

    let response = app
        .post_login(&serde_json::json!({
            "username": test_user.username.as_ref(),
            "password": new_password.expose_secret(),
        }))
        .await
        .expect("Request succeed.");

    assert_is_redirect_to(&response, "/admin/dashboard");

    let html = app
        .get_admin_dashboard_html()
        .await
        .expect("Login should return page body.");

    assert!(html.contains(
        format!("Welcome {}", &test_user.username).as_str()
    ));
}
