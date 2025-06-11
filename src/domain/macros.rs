macro_rules! define_string_newtype_derived {
    ($in: item) => {
        #[derive(
            Debug,
            Hash,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            serde::Deserialize,
            derive_more::Deref,
            derive_more::AsRef,
            derive_more::Into,
            derive_more::Display,
        )]
        #[as_ref(str, String)]
        #[into(String)]
        $in
    };
}

macro_rules! define_enum_derived {
    ($in: item) => {
        #[derive(
            Debug,
            thiserror::Error,
            Hash,
            PartialEq,
            Eq,
        )]
        $in
    };
}

pub(crate) use define_enum_derived;
pub(crate) use define_string_newtype_derived;
