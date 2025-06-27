use crate::common::{self, email_server, spawn_app};

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = common::spawn_app().await;

    let body =
        "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // New section!
    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    let response = app
        .post_subscriptions(body)
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_persists_new_user() {
    // Arrange
    let app = common::spawn_app().await;

    let body =
        "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // New section!
    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    let _ = app
        .post_subscriptions(body)
        .await
        .expect("Failed to execute request.");
    // Assert

    let saved = sqlx::query!(
        "--sql  
    SELECT email, name, status FROM subscriptions"
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved subscriptions.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "pending_confirmation");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = common::spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        (
            "email=ursula_le_guin%40gmail.com",
            "missing the name",
        ),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app_address
            .post_subscriptions(invalid_body)
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_empty()
 {
    // Arrange
    let app = common::spawn_app().await;
    let test_cases = vec![
        (
            "name=&email=ursula_le_guin%40gmail.com",
            "empty name",
        ),
        ("name=Ursula&email=", "empty email"),
        (
            "name=Ursula&email=definitely-not-an-email",
            "invalid email",
        ),
    ];
    for (body, description) in test_cases {
        // Act
        let response = app
            .post_subscriptions(body)
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}

#[actix_rt::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data()
 {
    // Arrange
    let app = spawn_app().await;
    let body =
        "name=le%20guin&email=ursula_le_guin%40gmail.com";

    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    let result =
        app.post_subscriptions(body.to_string()).await;
    // Assert
    claim::assert_ok!(result);
    // Mock asserts on drop
}
#[actix_rt::test]
async fn subscribe_sends_a_confirmation_email_with_a_link()
{
    // Arrange
    let app = spawn_app().await;
    let body =
        "name=le%20guin&email=ursula_le_guin%40gmail.com";

    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    // Act
    let result =
        app.post_subscriptions(body.to_string()).await;
    // Assert
    claim::assert_ok!(result);
    // Get the first intercepted request
    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()[0];
    // Parse the body as JSON, starting from raw bytes
    let body =
        app.get_confirmation_links(email_request).unwrap();
    // The two links should be identical
    assert_eq!(
        body.html.as_ref(),
        body.plain_text.as_ref()
    );
}

#[actix_web::test]
async fn subscription_fails_after_fatal_database_error() {
    let app = spawn_app().await;

    sqlx::query!(
        "--sql
    DROP SCHEMA public CASCADE"
    )
    .execute(&app.db_pool)
    .await
    .unwrap();

    let response = app.post_subscriptions(
        "name=le%20guin&email=ursula_le_guin%40gmail.com"
    ).await
    .unwrap();

    assert_eq!(response.status(), 500);
}
