use fake::Fake as _;
use nameof::name_of;
use uuid::Uuid;
use zero2prod::{routes::newsletter, utils::Pipe};

use crate::common::{
    self, TestApp, assert_is_redirect_to,
    create_test_newsletter_writer, email_server,
};

#[actix_web::test]
async fn newsletter_is_sent_to_confirmed_subscribers_and_redirects_to_self_with_success()
 {
    // Spawn app V
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    // Create confirmed subscribers using public APIs and Mock V
    create_confirmed_subscribers(&app).await;

    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_login_with_default().await.unwrap();

    // Create newsletter V
    // A sketch of the newsletter payload structure.
    // We might change it later on.
    let newsletter_request_body =
        a_valid_newsletter_request_body();

    // Send using public API
    let response = app
        .post_newsletter(&newsletter_request_body)
        .await
        .expect("Failed to send request.");

    assert_is_redirect_to(&response, "/admin/newsletters");

    let text = app
        .get_newsletter_form()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert!(text.contains(newsletter::SUCCESS_MESSAGE));

    app.dispatch_all_pending_emails().await;
}

#[actix_web::test]
async fn newsletter_is_sent_to_confirmed_subscribers_only_once_per_idempotency_key()
 {
    // Spawn app V
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    // Create confirmed subscribers using public APIs and Mock V
    create_confirmed_subscribers(&app).await;

    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_login_with_default().await.unwrap();

    // Create newsletter V
    // A sketch of the newsletter payload structure.
    // We might change it later on.
    let newsletter_request_body =
        a_valid_newsletter_request_body();

    let call_api = async || {
        // Send using public API
        let response = app
            .post_newsletter(&newsletter_request_body)
            .await
            .expect("Failed to send request.");

        assert_is_redirect_to(
            &response,
            "/admin/newsletters",
        );

        let text = app
            .get_newsletter_form()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert!(text.contains(newsletter::SUCCESS_MESSAGE));
    };

    call_api().await;
    call_api().await;

    app.dispatch_all_pending_emails().await;
}

#[actix_web::test]
async fn newsletter_is_sent_to_confirmed_subscribers_only_once_between_concurrent_requests()
 {
    // Spawn app V
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    // Create confirmed subscribers using public APIs and Mock V
    create_confirmed_subscribers(&app).await;

    email_server::get_mock_builder()
        .respond_with(
            wiremock::ResponseTemplate::new(200).set_delay(
                std::time::Duration::from_secs(1),
            ),
        )
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_login_with_default().await.unwrap();

    // Create newsletter V
    // A sketch of the newsletter payload structure.
    // We might change it later on.
    let newsletter_request_body =
        a_valid_newsletter_request_body();

    // Send using public API
    let call_api = async || {
        app.post_newsletter(&newsletter_request_body)
            .await
            .unwrap()
    };

    let (response1, response2) =
        tokio::join!(call_api(), call_api());

    assert_eq!(response1.status(), response2.status());
    assert_eq!(
        response1.text().await.unwrap(),
        response2.text().await.unwrap()
    );

    app.dispatch_all_pending_emails().await;
    // Mock verify on Drop
}

#[actix_web::test]
async fn transient_errors_do_not_cause_duplicate_deliveries_on_retries()
 {
    // Arrange
    let app = common::spawn_app().await;
    let newsletter_request_body =
        a_valid_newsletter_request_body();
    // Two subscribers instead of one!
    create_confirmed_subscribers(&app).await;
    create_confirmed_subscribers(&app).await;
    create_test_newsletter_writer(&app).await;
    app.post_login_with_default()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // Part 1 - Submit newsletter form
    // Email delivery fails for the second subscriber
    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .named("First send request")
        .up_to_n_times(1)
        .expect(1)
        .mount(&app.email_server)
        .await;
    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(500))
        .named("Second send request")
        .up_to_n_times(1)
        .expect(1)
        .mount(&app.email_server)
        .await;
    let response = app
        .post_newsletter(&newsletter_request_body)
        .await
        .unwrap();
    // Currently maps 500 => 303 with opaque error message.
    assert_eq!(response.status().as_u16(), 303);
    // Part 2 - Retry submitting the form
    // Email delivery will succeed for both subscribers now
    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(1)
        .named("Delivery retry")
        .mount(&app.email_server)
        .await;
    let response = app
        .post_newsletter(&newsletter_request_body)
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 303);

    app.dispatch_all_pending_emails().await;
    // Mock verifies on Drop that we did not send out duplicates
}

#[actix_web::test]
async fn newsletter_is_not_sent_to_unconfirmed_subscribers()
{
    // Spawn app V
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    // Create unconfirmed subscribers using public APIs and Mock V
    create_unconfirmed_subscribers(&app).await;

    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    app.post_login_with_default().await.unwrap();

    // Create newsletter V
    // A sketch of the newsletter payload structure.
    // We might change it later on.
    let newsletter_request_body =
        a_valid_newsletter_request_body();

    // Send to Mock email client
    let response = app
        .post_newsletter(&newsletter_request_body)
        .await
        .expect("Failed to send request.");

    assert_is_redirect_to(&response, "/admin/newsletters");

    let text = app
        .get_newsletter_form()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert!(text.contains(newsletter::SUCCESS_MESSAGE));

    app.dispatch_all_pending_emails().await;
    // Confirm Mock email client received no requests
}

#[actix_web::test]
async fn post_unauthorized_access_redirects_to_login() {
    let app = common::spawn_app().await;

    create_test_newsletter_writer(&app).await;

    email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body =
        a_valid_newsletter_request_body();

    // Send to Mock email client
    let response = app
        .post_newsletter(&newsletter_request_body)
        .await
        .expect("Failed to send request.");

    assert_is_redirect_to(&response, "/login");
}

async fn create_confirmed_subscribers(app: &TestApp<'_>) {
    let confirmation_link =
        create_unconfirmed_subscribers(app).await;

    assert_eq!(
        confirmation_link.host_str().unwrap(),
        "127.0.0.1",
        "The confirmation link must be correctly mocked."
    );

    let _confirmation_response =
        reqwest::get(confirmation_link)
            .await
            .expect("Text link must be callable")
            .error_for_status()
            .unwrap();
}

async fn create_unconfirmed_subscribers(
    app: &TestApp<'_>,
) -> reqwest::Url {
    let mock_guard = email_server::get_mock_builder()
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    let name =
        fake::faker::name::en::Name().fake::<String>();
    let email = fake::faker::internet::en::SafeEmail()
        .fake::<String>();

    let body = serde_json::json!({
        name_of!(name): name,
        name_of!(email): email,
    })
    .pipe(serde_urlencoded::to_string)
    .unwrap();

    app.post_subscriptions(body)
        .await
        .and_then(reqwest::Response::error_for_status)
        .unwrap();

    let confirmation_links = app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap()
        .pipe_ref(|i| app.get_confirmation_links(i))
        .unwrap();

    let confirmation_link = confirmation_links
        .plain_text
        .into_owned()
        .pipe(|mut i| {
            i.set_port(app.port.pipe(Some)).unwrap();
            i
        });

    drop(mock_guard);

    confirmation_link
}

fn a_valid_newsletter_request_body() -> serde_json::Value {
    serde_json::json!({
        "title": "Newsletter title",
        "content_text": "Newsletter body as plain text",
        "content_html": "<p>Newsletter body as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    })
}
