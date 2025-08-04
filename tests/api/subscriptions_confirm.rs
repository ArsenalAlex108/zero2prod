use uuid::Uuid;
use zero2prod::utils::Pipe;

use crate::common::{
    self, TestApp, email_server,
    test_dependency_injection::test_database::{
        get_subscriptions_repository::{
            GetSubscriptionsRepository, SubscriptionStatus,
        },
        repository_suspender::RepositorySuspender,
    },
};

#[actix_rt::test]
async fn confirm_without_subscription_token_returns_400() {
    let (app, client) = arrange().await;

    // email_server::get_mock_builder()
    // .respond_with(wiremock::ResponseTemplate::new(200))
    // .expect(0)
    // .mount(&app.email_server)
    // .await;

    let response = client
        .get(format!(
            "{}/subscriptions/confirm",
            &app.address
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        400,
        response.status().as_u16(),
        "The API did not return a 400 Bad Request when the subscription_token is missing"
    );
}

#[actix_rt::test]
async fn subscription_link_returns_200() {
    let (app, _client) = arrange().await;
    let body =
        "name=le%20guin&email=ursula_le_guin%40gmail.com";

    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app
        .post_subscriptions(body)
        .await
        .expect("Failed to execute request.");

    let status = response.status().as_u16();

    assert_eq!(
        200, status,
        "The email API did not return a 200 OK"
    );

    let links = app
        .get_confirmation_links(
            &app.email_server
                .received_requests()
                .await
                .unwrap()[0],
        )
        .unwrap();

    let text_link = links.plain_text.into_owned();

    let confirmation_link = text_link.pipe(|mut i| {
        i.set_port(app.port.pipe(Some)).unwrap();
        i
    });

    assert_eq!(
        confirmation_link.host_str().unwrap(),
        "127.0.0.1",
        "The confirmation link must be correctly mocked."
    );

    let confirmation_response =
        reqwest::get(confirmation_link)
            .await
            .expect("Text link must be callable");

    assert_eq!(
        200,
        confirmation_response.status().as_u16(),
        "The confirmation API did not return a 200 OK when the confirmation link is accessed"
    );
}

#[actix_rt::test]
async fn subscription_link_updates_status_to_confirmed() {
    let (app, _client) = arrange().await;
    let body =
        "name=le%20guin&email=ursula_le_guin%40gmail.com";

    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app
        .post_subscriptions(body)
        .await
        .expect("Failed to execute request.");

    let status = response.status().as_u16();

    assert_eq!(
        200, status,
        "The email API did not return a 200 OK"
    );

    let links = app
        .get_confirmation_links(
            &app.email_server
                .received_requests()
                .await
                .unwrap()[0],
        )
        .unwrap();

    let text_link = links.plain_text.into_owned();

    let confirmation_link = text_link.pipe(|mut i| {
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

    let record = app
        .test_app_state
        .get_subscriptions_repository
        .get_subscriptions("le guin")
        .await
        .expect("Query is successful");

    assert_eq!(record.email, "ursula_le_guin@gmail.com");
    assert_eq!(record.name, "le guin");
    assert_eq!(
        record.status,
        SubscriptionStatus::Confirmed
    );
}

#[actix_rt::test]
async fn confirm_subscription_token_returns_401_with_incorrect_token()
 {
    let (app, client) = arrange().await;

    // New section!
    wiremock::Mock::given(wiremock::matchers::path(
        "/email",
    ))
    .and(wiremock::matchers::method("POST"))
    .respond_with(wiremock::ResponseTemplate::new(200))
    .mount(&app.email_server)
    .await;

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
        .query(&[("subscription_token", token.to_string())])
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        401,
        response.status().as_u16(),
        "The API did not return a 401 Unauthorized when the subscription_token is incorrect"
    );
}

#[actix_rt::test]
async fn confirm_subscription_token_returns_500_with_fatal_database_error()
 {
    let (app, client) = arrange().await;

    // New section!
    wiremock::Mock::given(wiremock::matchers::path(
        "/email",
    ))
    .and(wiremock::matchers::method("POST"))
    .respond_with(wiremock::ResponseTemplate::new(200))
    .mount(&app.email_server)
    .await;

    app.test_app_state
        .repository_suspender
        .suspend()
        .await
        .unwrap();

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
        .query(&[("subscription_token", token.to_string())])
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        500,
        response.status().as_u16(),
        "The API did not return a 500 Internal Server Error after a fatal database error."
    );
}

async fn arrange<'a>() -> (TestApp<'a>, reqwest::Client) {
    (common::spawn_app().await, reqwest::Client::new())
}
