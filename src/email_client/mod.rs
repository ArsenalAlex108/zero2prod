use crate::domain::SubscriberEmail;
use crate::hkt::{K1, RefHKT, SharedPointerHKT};
use kust::ScopeFunctions;
use reqwest::Client;

#[allow(clippy::pedantic)]
pub mod generated;

#[derive(Debug, derive_more::Into)]
pub struct EmailClient<P: RefHKT> {
    http_client: K1<P, Client>,
    base_url: K1<P, str>,
    sender: SubscriberEmail<P>,
    authorization_token: K1<P, str>,
}

const X_POSTMARK_SERVER_TOKEN_HEADER: &str =
    "X-Postmark-Server-Token";

impl<P: SharedPointerHKT> Clone for EmailClient<P> {
    fn clone(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
            base_url: self.base_url.clone(),
            sender: self.sender.clone(),
            authorization_token: self
                .authorization_token
                .clone(),
        }
    }
}

impl<P: RefHKT> EmailClient<P> {
    pub fn new(
        base_url: K1<P, str>,
        sender: SubscriberEmail<P>,
        authorization_token: K1<P, str>,
        timeout: std::time::Duration,
    ) -> EmailClient<P> {
        EmailClient {
            http_client: Client::builder()
                .timeout(timeout)
                .build()
                .unwrap()
                .using(P::new),
            base_url,
            sender,
            authorization_token,
        }
    }
}
impl<P: SharedPointerHKT> EmailClient<P> {
    pub async fn send_email(
        &self,
        recipient: SubscriberEmail<P>,
        subject: K1<P, str>,
        html_content: K1<P, str>,
        text_content: K1<P, str>,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);

        if cfg!(test) {
            tracing::debug!(
                "Print Recipient: {}",
                recipient.as_ref().to_string()
            );
        }

        let request_body = SendEmailRequest {
            from: self.base_url.clone(),
            to: recipient.into(),
            subject,
            html_body: html_content,
            text_body: text_content,
        };

        self.http_client
            .post(&url)
            .header(
                X_POSTMARK_SERVER_TOKEN_HEADER,
                &*self.authorization_token,
            )
            .json(&request_body)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map(|_| ())
    }
}

// Prefer references over RC pointers.
//#[derive(serde::Serialize)]
//#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<P: RefHKT> {
    from: K1<P, str>,
    to: K1<P, str>,
    subject: K1<P, str>,
    html_body: K1<P, str>,
    text_body: K1<P, str>,
}

#[cfg(test)]
mod tests {

    use crate::domain::SubscriberEmail;
    use crate::email_client::{
        EmailClient, X_POSTMARK_SERVER_TOKEN_HEADER,
    };
    use crate::hkt::{K1, RcHKT, RefHKT, SharedPointerHKT};
    use crate::utils::Pipe;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use uuid::Uuid;
    use wiremock::matchers::{
        any, header, header_exists, method, path,
    };
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct SendEmailBodyMatcher;
    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(
            &self,
            request: &wiremock::Request,
        ) -> bool {
            // Try to parse the body as a JSON value
            let result: Result<serde_json::Value, _> =
                serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                // Check that all the mandatory fields are populated
                // without inspecting the field values
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some()
            } else {
                // If parsing failed, do not match the request
                false
            }
        }
    }

    /// Generate a random email subject
    fn subject() -> String {
        Sentence(1..2).fake()
    }
    /// Generate a random email content
    fn content() -> String {
        Paragraph(1..10).fake()
    }
    /// Generate a random subscriber email
    fn email<P: RefHKT>() -> SubscriberEmail<P> {
        SubscriberEmail::try_from(
            SafeEmail()
                .fake::<String>()
                .pipe(P::from_string),
        )
        .unwrap()
    }

    fn email_client<P: RefHKT>(
        base_url: K1<P, str>,
    ) -> EmailClient<P> {
        EmailClient::new(
            base_url,
            email(),
            //Random alpha-numerical-with-hyphens string
            Uuid::new_v4().to_string().pipe(P::from_string),
            std::time::Duration::from_millis(200),
        )
    }

    #[tokio::test]
    async fn send_email_sends_expected_request() {
        send_email_sends_expected_request_generic::<RcHKT>(
        )
        .await;
    }

    async fn send_email_sends_expected_request_generic<
        P: SharedPointerHKT,
    >() {
        // Arrange
        let mock_server = MockServer::start().await;

        let email_client = email_client(
            mock_server.uri().pipe(P::from_string),
        );

        Mock::given(header_exists(
            X_POSTMARK_SERVER_TOKEN_HEADER,
        ))
        .and(header("Content-Type", "application/json"))
        .and(path("/email"))
        .and(method("POST"))
        .and(SendEmailBodyMatcher)
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

        // Act
        let send_result = email_client
            .send_email(
                email(),
                subject().pipe(P::from_string),
                content().pipe(P::from_string),
                content().pipe(P::from_string),
            )
            .await;

        // Assert
        claims::assert_ok!(send_result);
    }

    #[tokio::test]
    async fn send_email_fails_if_server_returns_500() {
        send_email_fails_if_server_returns_code_n_generic::<
            RcHKT,
        >(500)
        .await;
    }

    async fn send_email_fails_if_server_returns_code_n_generic<
        P: SharedPointerHKT,
    >(
        n: u16,
    ) {
        // Arrange
        let mock_server = MockServer::start().await;

        let email_client = email_client(
            mock_server.uri().pipe(P::from_string),
        );

        Mock::given(any())
            .respond_with(ResponseTemplate::new(n))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let send_result = email_client
            .send_email(
                email(),
                subject().pipe(P::from_string),
                content().pipe(P::from_string),
                content().pipe(P::from_string),
            )
            .await;

        // Assert
        claims::assert_err!(send_result);
    }

    #[tokio::test]
    async fn send_email_fails_if_server_takes_3_minutes() {
        send_email_fails_if_server_takes_too_long_generic::<
            RcHKT,
        >(180)
        .await;
    }

    async fn send_email_fails_if_server_takes_too_long_generic<
        P: SharedPointerHKT,
    >(
        secs: u64,
    ) {
        // Arrange
        let mock_server = MockServer::start().await;

        let email_client = email_client(
            mock_server.uri().pipe(P::from_string),
        );

        Mock::given(any())
            .respond_with(
                ResponseTemplate::new(500).set_delay(
                    std::time::Duration::from_secs(secs),
                ),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let send_result = email_client
            .send_email(
                email(),
                subject().pipe(P::from_string),
                content().pipe(P::from_string),
                content().pipe(P::from_string),
            )
            .await;

        // Assert
        claims::assert_err!(send_result);
    }
}
