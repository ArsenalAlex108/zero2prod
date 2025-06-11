use crate::{
    domain::{
        SubscriberEmailParseError,
        SubscriberName, SubscriberNameParseError,
        macros::define_enum_derived,
        subscriber_email::SubscriberEmail,
    },
    hkt::{K1, SharedPointerHKT},
    routes::SubscribeFormData,
};
use std::ops::Deref;

#[derive(
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Into,
    derive_more::AsRef,
    serde::Serialize,
    serde::Deserialize,
)]
//#[display("{{\n email: {} \n name: {} \n}}", *email, *name)]
pub struct NewSubscriber<P: SharedPointerHKT> {
    pub email: SubscriberEmail<P>,
    pub name: SubscriberName<P>,
}

define_enum_derived! {
    pub enum NewSubscriberParseError {
        #[error("Email: {0}")]
        SubscriberEmailParseError(#[from] SubscriberEmailParseError),
        #[error("Name: {0}")]
        SubscriberNameParseError(#[from] SubscriberNameParseError)
    }
}

impl<P: SharedPointerHKT>
    TryFrom<SubscribeFormData>
    for NewSubscriber<P>
{
    type Error = NewSubscriberParseError;

    fn try_from(
        value: SubscribeFormData,
    ) -> Result<Self, Self::Error> {
        SubscriberName::<P>::try_from(
            value.name.as_str(),
        )
        .map_err(NewSubscriberParseError::from)
        .and_then(|name| {
            SubscriberEmail::<P>::try_from(
                value.email.as_str(),
            )
            .map(|email| NewSubscriber {
                email,
                name,
            })
            .map_err(
                NewSubscriberParseError::from,
            )
        })
    }
}
