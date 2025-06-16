use crate::domain::SubscriberEmail;
use crate::hkt::{K1, RefHKT, SharedPointerHKT};
use kust::ScopeFunctions;
use reqwest::Client;

#[derive(Debug, derive_more::Into, derive_more::AsRef)]
pub struct EmailClient<P: RefHKT> {
    http_client: K1<P, Client>,
    base_url: K1<P, str>,
    sender: SubscriberEmail<P>,
}

impl<P: SharedPointerHKT> Clone for EmailClient<P> {
    fn clone(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
            base_url: self.base_url.clone(),
            sender: self.sender.clone(),
        }
    }
}

impl<P: RefHKT> EmailClient<P> {
    pub fn new(
        base_url: K1<P, str>,
        sender: SubscriberEmail<P>,
    ) -> EmailClient<P> {
        EmailClient {
            http_client: Client::new().using(P::new),
            base_url,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail<P>,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
