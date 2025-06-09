use crate::domain::{subscriber_email::SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}