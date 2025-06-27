use zero2prod::utils::Pipe;

use crate::common::{self, TestApp, email_server};

#[actix_web::test]
async fn newsletter_is_sent_to_confirmed_subscribers() {
    // Spawn app V
    let app = common::spawn_app().await;

    // Create confirmed subscribers using public APIs and Mock V
    create_confirmed_subscribers(&app).await;

    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Create newsletter V
    // A sketch of the newsletter payload structure.
    // We might change it later on.
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
        "text": "Newsletter body as plain text",
        "html": "<p>Newsletter body as HTML</p>",
        }
    });

    // Send using public API
    let response = app
        .post_newsletter(&newsletter_request_body)
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 200);
}

#[actix_web::test]
async fn newsletter_is_not_sent_to_unconfirmed_subscribers()
{
    // Spawn app V
    let app = common::spawn_app().await;

    // Create unconfirmed subscribers using public APIs and Mock V
    create_unconfirmed_subscribers(&app).await;

    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    // Create newsletter V
    // A sketch of the newsletter payload structure.
    // We might change it later on.
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
        "text": "Newsletter body as plain text",
        "html": "<p>Newsletter body as HTML</p>",
        }
    });

    // Send to Mock email client
    let response = app
        .post_newsletter(&newsletter_request_body)
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 200);
    // Confirm Mock email client received no requests
}

#[actix_web::test]
async fn newsletter_returns_400_for_invalid_body() {
    // Spawn app V
    let app = common::spawn_app().await;

    // Create newsletter V
    // A sketch of the newsletter payload structure.
    // We might change it later on.
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
        "text": "Newsletter body as plain text",
        // Missing:
        // "html": "<p>Newsletter body as HTML</p>",
        }
    });

    // Send using public API
    let response = app
        .post_newsletter(&newsletter_request_body)
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 400, "todo");
}

#[actix_web::test]
async fn newsletter_returns_500_for_fatal_database_error() {
    // Spawn app V
    let app = common::spawn_app().await;

    sqlx::query!(
        "--sql
    DROP SCHEMA public CASCADE"
    )
    .execute(&app.db_pool)
    .await
    .unwrap();

    // Create newsletter V
    // A sketch of the newsletter payload structure.
    // We might change it later on.
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
        "text": "Newsletter body as plain text",
        "html": "<p>Newsletter body as HTML</p>",
        }
    });

    // Send using public API
    let response = app
        .post_newsletter(&newsletter_request_body)
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 500, "todo");
}

async fn create_confirmed_subscribers(app: &TestApp<'_>) {
    create_unconfirmed_subscribers(app).await;

    let confirmation_links =
        app.email_server.received_requests().await.unwrap()
            [0]
        .pipe_ref(|i| app.get_confirmation_links(i))
        .unwrap();

    let confirmation_link = confirmation_links
        .plain_text
        .into_owned()
        .pipe(|mut i| {
            i.set_port(app.port.pipe(Some)).unwrap();
            i
        });

    assert_eq!(
        confirmation_link.host_str().unwrap(),
        "127.0.0.1",
        "The confirmation link must be correctly mocked."
    );

    let _confirmation_response =
        reqwest::get(confirmation_link)
            .await
            .expect("Text link must be callable");
}

async fn create_unconfirmed_subscribers(app: &TestApp<'_>) {
    let mock_guard = email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    let body =
        "name=le%20guin&email=ursula_le_guin%40gmail.com";

    app.post_subscriptions(body)
        .await
        .and_then(reqwest::Response::error_for_status)
        .unwrap();

    drop(mock_guard);
}
