use std::borrow::Cow;

use crate::utils::Pipe;

#[derive(
    Debug,
    Clone,
    derive_more::Into,
    derive_more::AsRef,
    derive_more::Deref,
)]
#[as_ref(forward)]
#[deref(forward)]
#[into(Cow<'a, str>, String)]
pub struct IdempotencyKey<'a>(Cow<'a, str>);

impl<'a> TryFrom<Cow<'a, str>> for IdempotencyKey<'a> {
    type Error = eyre::Report;
    fn try_from(
        value: Cow<'a, str>,
    ) -> Result<Self, Self::Error> {
        if value.is_empty() {
            eyre::bail!("Idempotency key cannot be empty.");
        }
        const MAX_LENGTH: usize = 50;
        if value.len() > MAX_LENGTH {
            eyre::bail!(
                "Idempotency key cannot be longer than {MAX_LENGTH} chars."
            );
        }

        value.pipe(Self).pipe(Ok)
    }
}

impl IdempotencyKey<'_> {
    pub fn into_owned<'b>(self) -> IdempotencyKey<'b> {
        IdempotencyKey(
            self.0.into_owned().pipe(Cow::<str>::from),
        )
    }
}

impl TryFrom<String> for IdempotencyKey<'_> {
    type Error = eyre::Report;

    fn try_from(
        value: String,
    ) -> Result<Self, Self::Error> {
        IdempotencyKey::try_from(
            value.pipe(Cow::<str>::from),
        )
    }
}

impl<'a> TryFrom<&'a str> for IdempotencyKey<'a> {
    type Error = eyre::Report;

    fn try_from(
        value: &'a str,
    ) -> Result<Self, Self::Error> {
        IdempotencyKey::try_from(
            value.pipe(Cow::<str>::from),
        )
    }
}
