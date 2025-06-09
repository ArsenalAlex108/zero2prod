use std::{rc::Rc, sync::Arc};

use kust::ScopeFunctions;
use validator::ValidateEmail;

use crate::domain::macros::{define_enum_derived, define_string_newtype_derived};

define_string_newtype_derived! {
    pub struct SubscriberEmail(String);
}

impl SubscriberEmail {
    pub fn parse(str: String) -> Result<SubscriberEmail, SubscriberEmailParseError> {
        use SubscriberEmailParseError as E;
        if str.validate_email() {
            str.using(SubscriberEmail).using(Ok)
        } else {
            format!("'{}' is not valid email.", str)
            .using(E::Other)
            .using(Err)
        }
    }
}

impl TryFrom<String> for SubscriberEmail {
    type Error = SubscriberEmailParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SubscriberEmail::parse(value)
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
    use super::SubscriberEmail;
    use claim::assert_err;
    use kust::ScopeFunctions;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[derive(Debug, Clone)]
    struct ValidEmail(pub String);

    impl quickcheck::Arbitrary for ValidEmail {
        fn arbitrary<T: quickcheck::Gen>(g: &mut T) -> Self {
            SafeEmail()
            .fake_with_rng::<String, _>(g)
            .using(Self)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(email: ValidEmail) -> bool {
        SubscriberEmail::parse(email.0).is_ok()
    }
}
