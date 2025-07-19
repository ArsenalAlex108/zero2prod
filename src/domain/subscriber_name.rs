use std::ops::Deref;

use crate::{
    domain::macros::define_enum_derived,
    hkt::{K1, RefHKT, SharedPointerHKT},
};
use kust::ScopeFunctions;
use unicode_segmentation::UnicodeSegmentation;

/// Deserialization will panic if invariants are not satisfied.
#[derive(
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    derive_more::Into,
    derive_more::AsRef,
    derive_more::Display,
)]
pub struct SubscriberName<P: RefHKT>(K1<P, str>);

const SUBSCRIBER_NAME_MAX_LENGTH: usize = 256;
const SUBSCRIBE_NAME_FORBIDDEN_CHARACTERS: &[char] =
    &['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

impl<P: RefHKT> SubscriberName<P> {
    pub fn parse(
        s: P::T<str>,
    ) -> Result<SubscriberName<P>, SubscriberNameParseError>
    {
        let s = K1::<P, _>::newtype(s);
        // `.trim()` returns a view over the input `s` without trailing
        // whitespace-like characters.
        // `.is_empty` checks if the view contains any character.
        let s_ref = s.deref();
        let is_empty_or_whitespace =
            s_ref.trim().is_empty();
        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and ``).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_long = s_ref.graphemes(true).count()
            > SUBSCRIBER_NAME_MAX_LENGTH;
        // Iterate over all characters in the input `s` to check if any of them matches
        // one of the characters in the forbidden array.
        let contains_forbidden_characters =
            s_ref.chars().any(|c| {
                SUBSCRIBE_NAME_FORBIDDEN_CHARACTERS
                    .contains(&c)
            });

        use SubscriberNameParseError as E;

        if is_empty_or_whitespace {
            E::IsEmptyOrWhitespace.using(Err)
        } else if is_too_long {
            E::IsTooLong.using(Err)
        } else if contains_forbidden_characters {
            E::ContainsForbiddenCharacters.using(Err)
        } else {
            s.using(SubscriberName::<P>).using(Ok)
        }
    }
}

impl<P: RefHKT> Deref for SubscriberName<P> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<P: SharedPointerHKT> Clone for SubscriberName<P> {
    fn clone(&self) -> Self {
        Self(P::clone(self.0.inner_ref()))
    }
}

impl<P: RefHKT> TryFrom<&str> for SubscriberName<P> {
    type Error = SubscriberNameParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SubscriberName::try_from(
            value
                .using(Box::<str>::from)
                .using(P::from_box),
        )
    }
}

impl<P: RefHKT> TryFrom<String> for SubscriberName<P> {
    type Error = SubscriberNameParseError;
    fn try_from(
        value: String,
    ) -> Result<Self, Self::Error> {
        SubscriberName::try_from(
            value
                .using(Box::<str>::from)
                .using(P::from_box),
        )
    }
}

impl<P: RefHKT> TryFrom<K1<P, str>> for SubscriberName<P> {
    type Error = SubscriberNameParseError;

    fn try_from(
        value: K1<P, str>,
    ) -> Result<Self, Self::Error> {
        SubscriberName::parse(value.inner())
    }
}

/// Do not let deserialized data bypass invariants.
impl<'de, P: RefHKT> serde::Deserialize<'de>
    for SubscriberName<P>
{
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        K1::<P, str>::deserialize(deserializer).map(|i|SubscriberName::try_from(i.deref()).expect("Deserialized data are expected to satisfy invariants. Panic occurred because D::Error is opaque."))
    }
}

define_enum_derived! {
    pub enum SubscriberNameParseError {
        #[error("Name is empty or contains whitespace.")]
        IsEmptyOrWhitespace,
        #[error("Name is longer than {}", SUBSCRIBER_NAME_MAX_LENGTH)]
        IsTooLong,
        #[error("Name must not contain forbidden characters: [{}]", SUBSCRIBE_NAME_FORBIDDEN_CHARACTERS.iter().map(char::to_string).collect::<Vec<_>>().join(", "))]
        ContainsForbiddenCharacters,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::subscriber_name::{
            SUBSCRIBE_NAME_FORBIDDEN_CHARACTERS,
            SUBSCRIBER_NAME_MAX_LENGTH, SubscriberName,
        },
        hkt::RcHKT,
    };
    use claims::{assert_err, assert_ok};

    // Does this violates DRY?
    #[test]
    fn a_n_grapheme_long_name_is_valid() {
        let name = "a".repeat(SUBSCRIBER_NAME_MAX_LENGTH);
        assert_ok!(SubscriberName::<RcHKT>::parse(
            name.into()
        ));
    }

    #[test]
    fn a_name_longer_than_n_graphemes_is_rejected() {
        let name =
            "a".repeat(SUBSCRIBER_NAME_MAX_LENGTH + 1);
        assert_err!(SubscriberName::<RcHKT>::parse(
            name.into()
        ));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::<RcHKT>::parse(
            name.into()
        ));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::<RcHKT>::parse(
            name.into()
        ));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected()
    {
        SUBSCRIBE_NAME_FORBIDDEN_CHARACTERS.iter().for_each(
            |name| {
                let name = name.to_string();
                assert_err!(
                    SubscriberName::<RcHKT>::parse(
                        name.into()
                    )
                );
            },
        )
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::<RcHKT>::parse(
            name.into()
        ));
    }
}
