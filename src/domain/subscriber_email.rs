use kust::ScopeFunctions;
use nameof::name_of;
use std::ops::Deref;
use validator::ValidateEmail;

use crate::{
    domain::macros::define_enum_derived,
    hkt::{RefHKT, SharedPointerHKT, K1},
};

/// Deserialization will panic if invariants are not satisfied.
#[derive(
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Into,
    derive_more::AsRef,
    serde::Serialize,
)]
pub struct SubscriberEmail<P: RefHKT>(
    K1<P, str>,
);

// Just an example, derived impl is possible thanks to K1<P,str> : Debug
impl<P: RefHKT> std::fmt::Debug
    for SubscriberEmail<P>
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_tuple(
            name_of!(type SubscriberEmail<P>),
        )
        .field(&self.0)
        .finish()
    }
}

impl<P: RefHKT> Deref
    for SubscriberEmail<P>
{
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<P: SharedPointerHKT> Clone for SubscriberEmail<P> {
    fn clone(&self) -> Self {
        Self(P::clone(&self.0.inner_ref()))
    }
}

impl<P: RefHKT> SubscriberEmail<P> {
    /// See also: try_from(K1<P,A>)
    pub fn parse(
        str: P::T<str>,
    ) -> Result<
        SubscriberEmail<P>,
        SubscriberEmailParseError,
    > {
        use SubscriberEmailParseError as E;
        let str = K1::<P, _>::newtype(str);
        if str.deref().validate_email() {
            str.using(SubscriberEmail::<P>)
                .using(Ok)
        } else {
            format!(
                "'{}' is not valid email.",
                str.deref()
            )
            .using(E::Other)
            .using(Err)
        }
    }
}

impl<P: RefHKT> TryFrom<&str>
    for SubscriberEmail<P>
{
    type Error = SubscriberEmailParseError;
    fn try_from(
        value: &str,
    ) -> Result<Self, Self::Error> {
        SubscriberEmail::try_from(
            value
                .using(Box::<str>::from)
                .using(P::from_box),
        )
    }
}

impl<P: RefHKT> TryFrom<String>
    for SubscriberEmail<P>
{
    type Error = SubscriberEmailParseError;
    fn try_from(
        value: String,
    ) -> Result<Self, Self::Error> {
        SubscriberEmail::try_from(
            value
                .using(Box::<str>::from)
                .using(P::from_box),
        )
    }
}

impl<P: RefHKT> TryFrom<K1<P, str>>
    for SubscriberEmail<P>
{
    type Error = SubscriberEmailParseError;

    fn try_from(
        value: K1<P, str>,
    ) -> Result<Self, Self::Error> {
        SubscriberEmail::parse(value.inner())
    }
}

/// Do not let deserialized data bypass invariants.
impl<'de, P: RefHKT>
    serde::Deserialize<'de>
    for SubscriberEmail<P>
{
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        K1::<P, str>::deserialize(deserializer).map(|i|SubscriberEmail::try_from(i.deref()).expect("Deserialized data are expected to satisfy invariants. Panic occurred because D::Error is opaque."))
    }
}

define_enum_derived! {
    pub enum SubscriberEmailParseError {
        #[error("{0}")]
        Other(String)
    }
}

#[cfg(test)]
mod tests {
    use crate::hkt::RcHKT;

    use super::SubscriberEmail;
    use claim::assert_err;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use kust::ScopeFunctions;
    #[test]
    fn empty_string_is_rejected() {
        let email = "";
        assert_err!(
            SubscriberEmail::<RcHKT>::parse(
                email.into()
            )
        );
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com";
        assert_err!(
            SubscriberEmail::<RcHKT>::parse(
                email.into()
            )
        );
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com";
        assert_err!(
            SubscriberEmail::<RcHKT>::parse(
                email.into()
            )
        );
    }

    #[derive(Debug, Clone)]
    struct ValidEmail(pub String);

    impl quickcheck::Arbitrary for ValidEmail {
        fn arbitrary<T: quickcheck::Gen>(
            g: &mut T,
        ) -> Self {
            SafeEmail()
                .fake_with_rng::<String, _>(g)
                .using(Self)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(
        email: ValidEmail,
    ) -> bool {
        SubscriberEmail::<RcHKT>::parse(
            email.0.into(),
        )
        .is_ok()
    }
}
