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
