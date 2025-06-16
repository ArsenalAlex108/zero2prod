use std::marker::PhantomData;

use const_format::formatcp;
use naan::{apply::Apply, fun::F2Once};
use serde::de;

use crate::{hkt::Validation, utils::Pipe};

pub struct DeserializeError<E: serde::de::Error>(E);

impl<E: serde::de::Error> naan::semigroup::Semigroup for DeserializeError<E> {
    fn append(self, b: Self) -> Self {
        E::custom(self.0.to_string() +"\n" + &b.0.to_string()).pipe(DeserializeError::from)
    }
}

impl<E: serde::de::Error> From<E> for DeserializeError<E> {
    fn from(value: E) -> Self {
        Self(value)
    }
}

impl<E: serde::de::Error> DeserializeError<E> {
    pub fn inner(self) -> E {
        self.0
    } 
}

pub trait Model<A, B> {

    fn new(a: A, b: B) -> Self;

    const FIELD_NAMES: &'static [&'static str];
}

pub struct DeserializeImpl2<T: Model<A, B>, A, B>(T, PhantomData<(A, B)>);

impl<'de, T: Model<A, B> + 'de, A: serde::Deserialize<'de> + Clone + 'de, B: serde::Deserialize<'de> + Clone + 'de> serde::Deserialize<'de>
    for DeserializeImpl2<T, A, B>
{
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<'de, T: Model<A, B>, A: serde::Deserialize<'de> + Clone, B: serde::Deserialize<'de> + Clone>(PhantomData<&'de (T, A, B)>);

        impl<'de, T: Model<A, B>, A: serde::Deserialize<'de> + Clone, B: serde::Deserialize<'de> + Clone>
            serde::de::Visitor<'de>
            for Visitor<'de, T, A, B>
        {
            type Value = DeserializeImpl2<T, A, B>;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str(formatcp!(
                    "struct {}",
                    stringify!(EmailClientSettings)
                ))
            }

            fn visit_seq<V>(
                self,
                seq: V,
            ) -> Result<Self::Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let _ = seq;
                Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Seq,
                    &self,
                ))
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut a: Result<A, V::Error> = Err(
                    de::Error::missing_field(T::FIELD_NAMES[0]),
                );
                let mut b: Result<B, V::Error> =
                    Err(de::Error::missing_field(
                        T::FIELD_NAMES[1],
                    ));
                while let Some(key) = map.next_key()? {
                    if key == T::FIELD_NAMES[0]
                        {
                            a = if a.is_ok() {
                                Err(de::Error::duplicate_field(T::FIELD_NAMES[0]))
                            } else {
                                map.next_value()
                            }
                        }
                    else if key == T::FIELD_NAMES[1] {
                            b = if b.is_ok() {
                                Err(de::Error::duplicate_field(T::FIELD_NAMES[1]))
                            } else {
                                map.next_value()
                            }
                        }
                    else {return Err(de::Error::unknown_variant(key, T::FIELD_NAMES))
                    }
                }

                Validation::from(Ok(T::new.curry()))
                .apply(a.map_err(DeserializeError::from).into())
                .apply(b.map_err(DeserializeError::from).into())
                .pipe(Result::from)
                .map_err(DeserializeError::inner)
                .map(|i| DeserializeImpl2(i, PhantomData))
            }
        }

        deserializer.deserialize_seq(
            Visitor::<'de, T, A, B>(PhantomData),
        )
    }
}