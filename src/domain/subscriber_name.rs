use unicode_segmentation::UnicodeSegmentation;
use kust::ScopeFunctions;
use crate::domain::macros::{define_string_newtype_derived, define_enum_derived};

define_string_newtype_derived! {
    pub struct SubscriberName(String);
}

const SUBSCRIBER_NAME_MAX_LENGTH: usize = 256;
const SUBSCRIBE_NAME_FORBIDDEN_CHARACTERS: &[char] =  &['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, SubscriberNameParseError> {
        // `.trim()` returns a view over the input `s` without trailing
        // whitespace-like characters.
        // `.is_empty` checks if the view contains any character.
        let is_empty_or_whitespace = s.trim().is_empty();
        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and ``).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_long = s.graphemes(true).count() > SUBSCRIBER_NAME_MAX_LENGTH as usize;
        // Iterate over all characters in the input `s` to check if any of them matches
        // one of the characters in the forbidden array.
        let contains_forbidden_characters = s.chars().any(|c|(&*SUBSCRIBE_NAME_FORBIDDEN_CHARACTERS).contains(&c));

        use SubscriberNameParseError as E;

        if is_empty_or_whitespace {
            E::IsEmptyOrWhitespace.using(Err)
        } else if is_too_long {
            E::IsTooLong.using(Err)
        } else if contains_forbidden_characters {
            E::ContainsForbiddenCharacters.using(Err)
        } else {
            s.using(SubscriberName).using(Ok)
        }
    }
}

define_enum_derived! {
    pub enum SubscriberNameParseError {
        #[error("Name is empty or contains whitespace.")]
        IsEmptyOrWhitespace,
        #[error("Name is longer than {}", SUBSCRIBER_NAME_MAX_LENGTH)]
        IsTooLong,
        #[error("Name must not contain forbidden characters: [{}]", SUBSCRIBE_NAME_FORBIDDEN_CHARACTERS.into_iter().map(char::to_string).collect::<Vec<_>>().join(", "))]
        ContainsForbiddenCharacters,
    }
}


#[cfg(test)]
mod tests {
    use crate::domain::subscriber_name::{SubscriberName, SUBSCRIBER_NAME_MAX_LENGTH, SUBSCRIBE_NAME_FORBIDDEN_CHARACTERS};
    use claim::{assert_err, assert_ok};

    // Does this violates DRY? 
    #[test]
    fn a_n_grapheme_long_name_is_valid() {
        let name = "a".repeat(SUBSCRIBER_NAME_MAX_LENGTH as usize);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_n_graphemes_is_rejected() {
        let name = "a".repeat(SUBSCRIBER_NAME_MAX_LENGTH as usize + 1);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        SUBSCRIBE_NAME_FORBIDDEN_CHARACTERS.iter().for_each(|name| {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        })
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}