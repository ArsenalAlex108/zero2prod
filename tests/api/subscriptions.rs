use reqwest::Client;
use uuid::Uuid;

use crate::common::{self, TestApp};

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = common::spawn_app().await;

    let body =
        "name=le%20guin&email=ursula_le_guin%40gmail.com";
    // Act
    let response = app_address
        .post_subscriptions(body)
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!(
        "--sql
    SELECT email, name FROM subscriptions"
    )
    .fetch_one(&app_address.db_pool)
    .await
    .expect("Failed to fetch saved subscriptions.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
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
async fn confirm_subscription_token_returns_200_with_correct_token()
 {
    let (app, client) = arrange().await;
    let token = Uuid::new_v4();
    let response = client
        .get(format!(
            "{}/subscriptions/confirm",
            &app.address
        ))
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded",
        )
        .body(format!("subscription_token={}", token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return a 200 OK when the subscription_token is correct"
    );
}

#[actix_rt::test]
async fn confirm_subscription_token_returns_401_with_correct_token()
 {
    let (app, client) = arrange().await;
    let token = Uuid::new_v4();
    let response = client
        .get(format!(
            "{}/subscriptions/confirm",
            &app.address
        ))
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded",
        )
        .body(format!("subscription_token={}", token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        401,
        response.status().as_u16(),
        "The API did not return a 401 Unauthorized when the subscription_token is incorrect"
    );
}

async fn arrange<'a>() -> (TestApp<'a>, Client) {
    (common::spawn_app().await, reqwest::Client::new())
}
