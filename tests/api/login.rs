use std::ops::Not;

use secrecy::ExposeSecret;
use uuid::Uuid;

use crate::common::{
    self, assert_is_redirect_to,
    create_test_newsletter_writer,
    get_test_newsletter_writer,
};

#[actix_web::test]
pub async fn login_failure_redirects_back_with_err_message()
{
    let app = common::spawn_app().await;

    let response = app
        .post_login(&serde_json::json!({
            "username": Uuid::new_v4().to_string(),
            "password": Uuid::new_v4().to_string(),
        }))
        .await
        .expect("Request succeed.");

    // Auto redirect disabled in reqwest Client.
    assert_eq!(response.status(), 303);
    assert_is_redirect_to(&response, "/login");

    // Follow redirect by using the same reqwest Client.
    let html_page = app.get_login_html().await.expect(
        "Getting body of login page should not fail.",
    );

    dbg!(&html_page);

    assert!(html_page.contains(r#"Authentication failed"#));

    // Reload
    let html_page = app.get_login_html().await.expect(
        "Getting body of login page should not fail.",
    );
    assert!(
        html_page
            .contains(r#"Authentication failed"#)
            .not()
    );
}

#[actix_web::test]
pub async fn login_success_redirects_to_admin_page() {
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    let test_user = get_test_newsletter_writer();

    let response = app.post_login(
        &serde_json::json!({
            "username": test_user.username.as_ref(),
            "password": test_user.raw_password.as_ref().expose_secret(),
        })
    )
    .await
    .expect("Request succeed.");

    assert_is_redirect_to(&response, "/admin/dashboard");

    let body = app.get_admin_dashboard_html().await
    .expect("GET admin dashboard should succeed after successful login.");
    assert!(body.contains(
        format!("Welcome {}", &test_user.username).as_str()
    ));
}
