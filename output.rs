pub mod configuration {
    use config::Config;
    use hmac::Hmac;
    use nameof::name_of;
    use secrecy::SecretString;
    use serde_aux::field_attributes::deserialize_number_from_string;
    use sqlx::ConnectOptions;
    use sqlx::postgres::{PgConnectOptions, PgSslMode};
    use std::convert::TryFrom;
    use tracing_log::log;
    use crate::domain::{SubscriberEmail, SubscriberEmailParseError};
    use crate::hkt::{HKT1Unsized, K1, RefHKT, SharedPointerHKT};
    use crate::utils::Pipe;
    pub mod generated {
        use crate::configuration::*;
        use crate::hkt::RefHKT;
        /// #derive generated code with incorrect constraint P : serde::de::Deserialize<>
        /// Below is generated code with above constraint removed.
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, P: RefHKT> _serde::Deserialize<'de> for Settings<P> {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl _serde::de::Visitor<'_> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "database" => _serde::__private::Ok(__Field::__field0),
                                "application" => _serde::__private::Ok(__Field::__field1),
                                "email_client" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"database" => _serde::__private::Ok(__Field::__field0),
                                b"application" => _serde::__private::Ok(__Field::__field1),
                                b"email_client" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de, P: RefHKT> {
                        marker: _serde::__private::PhantomData<Settings<P>>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de, P: RefHKT> _serde::de::Visitor<'de> for __Visitor<'de, P> {
                        type Value = Settings<P>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Settings",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                K1<P, DatabaseSettings<P>>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Settings with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                K1<P, ApplicationSettings<P>>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Settings with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                K1<P, EmailClientSettings<P>>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Settings with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Settings {
                                database: __field0,
                                application: __field1,
                                email_client: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                K1<P, DatabaseSettings<P>>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                K1<P, ApplicationSettings<P>>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                K1<P, EmailClientSettings<P>>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "database",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                K1<P, DatabaseSettings<P>>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "application",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                K1<P, ApplicationSettings<P>>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "email_client",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                K1<P, EmailClientSettings<P>>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("database")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("application")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("email_client")?
                                }
                            };
                            _serde::__private::Ok(Settings {
                                database: __field0,
                                application: __field1,
                                email_client: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &[&str] = &["database", "application", "email_client"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Settings",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Settings<P>>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, P: RefHKT> _serde::Deserialize<'de> for ApplicationSettings<P> {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl _serde::de::Visitor<'_> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "port" => _serde::__private::Ok(__Field::__field0),
                                "host" => _serde::__private::Ok(__Field::__field1),
                                "base_url" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"port" => _serde::__private::Ok(__Field::__field0),
                                b"host" => _serde::__private::Ok(__Field::__field1),
                                b"base_url" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de, P: RefHKT> {
                        marker: _serde::__private::PhantomData<ApplicationSettings<P>>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de, P: RefHKT> _serde::de::Visitor<'de> for __Visitor<'de, P> {
                        type Value = ApplicationSettings<P>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ApplicationSettings",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u16,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ApplicationSettings with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                K1<P, str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ApplicationSettings with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                K1<P, str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ApplicationSettings with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ApplicationSettings {
                                port: __field0,
                                host: __field1,
                                base_url: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u16> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("port"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u16>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("host"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "base_url",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("port")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("host")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("base_url")?
                                }
                            };
                            _serde::__private::Ok(ApplicationSettings {
                                port: __field0,
                                host: __field1,
                                base_url: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &[&str] = &["port", "host", "base_url"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ApplicationSettings",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                ApplicationSettings<P>,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, P: RefHKT> _serde::Deserialize<'de> for DatabaseSettings<P> {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl _serde::de::Visitor<'_> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "username" => _serde::__private::Ok(__Field::__field0),
                                "password" => _serde::__private::Ok(__Field::__field1),
                                "port" => _serde::__private::Ok(__Field::__field2),
                                "host" => _serde::__private::Ok(__Field::__field3),
                                "database_name" => _serde::__private::Ok(__Field::__field4),
                                "require_ssl" => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"username" => _serde::__private::Ok(__Field::__field0),
                                b"password" => _serde::__private::Ok(__Field::__field1),
                                b"port" => _serde::__private::Ok(__Field::__field2),
                                b"host" => _serde::__private::Ok(__Field::__field3),
                                b"database_name" => _serde::__private::Ok(__Field::__field4),
                                b"require_ssl" => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de, P: RefHKT> {
                        marker: _serde::__private::PhantomData<DatabaseSettings<P>>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de, P: RefHKT> _serde::de::Visitor<'de> for __Visitor<'de, P> {
                        type Value = DatabaseSettings<P>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DatabaseSettings",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                K1<P, str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DatabaseSettings with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                K1<P, str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DatabaseSettings with 6 elements",
                                        ),
                                    );
                                }
                            };
                            #[allow(clippy::blocks_in_conditions)]
                            let __field2 = match {
                                #[doc(hidden)]
                                struct __DeserializeWith<'de, P: RefHKT> {
                                    value: u16,
                                    phantom: _serde::__private::PhantomData<
                                        DatabaseSettings<P>,
                                    >,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de, P: RefHKT> _serde::Deserialize<'de>
                                for __DeserializeWith<'de, P> {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::__private::Ok(__DeserializeWith {
                                            value: deserialize_number_from_string(__deserializer)?,
                                            phantom: _serde::__private::PhantomData,
                                            lifetime: _serde::__private::PhantomData,
                                        })
                                    }
                                }
                                _serde::__private::Option::map(
                                    _serde::de::SeqAccess::next_element::<
                                        __DeserializeWith<'de, P>,
                                    >(&mut __seq)?,
                                    |__wrap| __wrap.value,
                                )
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DatabaseSettings with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                K1<P, str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct DatabaseSettings with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                K1<P, str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct DatabaseSettings with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct DatabaseSettings with 6 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DatabaseSettings {
                                username: __field0,
                                password: __field1,
                                port: __field2,
                                host: __field3,
                                database_name: __field4,
                                require_ssl: __field5,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<u16> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<bool> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "username",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "password",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("port"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some({
                                            #[doc(hidden)]
                                            struct __DeserializeWith<'de, P: RefHKT> {
                                                value: u16,
                                                phantom: _serde::__private::PhantomData<
                                                    DatabaseSettings<P>,
                                                >,
                                                lifetime: _serde::__private::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de, P: RefHKT> _serde::Deserialize<'de>
                                            for __DeserializeWith<'de, P> {
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::__private::Ok(__DeserializeWith {
                                                        value: deserialize_number_from_string(__deserializer)?,
                                                        phantom: _serde::__private::PhantomData,
                                                        lifetime: _serde::__private::PhantomData,
                                                    })
                                                }
                                            }
                                            match _serde::de::MapAccess::next_value::<
                                                __DeserializeWith<'de, P>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__wrapper) => __wrapper.value,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        });
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("host"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "database_name",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "require_ssl",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("username")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("password")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::missing_field("port"),
                                    );
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("host")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("database_name")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("require_ssl")?
                                }
                            };
                            _serde::__private::Ok(DatabaseSettings {
                                username: __field0,
                                password: __field1,
                                port: __field2,
                                host: __field3,
                                database_name: __field4,
                                require_ssl: __field5,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &[&str] = &[
                        "username",
                        "password",
                        "port",
                        "host",
                        "database_name",
                        "require_ssl",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DatabaseSettings",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DatabaseSettings<P>,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, P: RefHKT> _serde::Deserialize<'de> for EmailClientSettings<P> {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl _serde::de::Visitor<'_> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "base_url" => _serde::__private::Ok(__Field::__field0),
                                "sender_email" => _serde::__private::Ok(__Field::__field1),
                                "authorization_token" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                "timeout_milliseconds" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"base_url" => _serde::__private::Ok(__Field::__field0),
                                b"sender_email" => _serde::__private::Ok(__Field::__field1),
                                b"authorization_token" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                b"timeout_milliseconds" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de, P: RefHKT> {
                        marker: _serde::__private::PhantomData<EmailClientSettings<P>>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de, P: RefHKT> _serde::de::Visitor<'de> for __Visitor<'de, P> {
                        type Value = EmailClientSettings<P>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct EmailClientSettings",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                K1<P, str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct EmailClientSettings with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                K1<P, str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct EmailClientSettings with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                K1<P, str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct EmailClientSettings with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                u64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct EmailClientSettings with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(EmailClientSettings {
                                base_url: __field0,
                                sender_email: __field1,
                                authorization_token: __field2,
                                timeout_milliseconds: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<u64> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "base_url",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "sender_email",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "authorization_token",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "timeout_milliseconds",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("base_url")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("sender_email")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("authorization_token")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field(
                                        "timeout_milliseconds",
                                    )?
                                }
                            };
                            _serde::__private::Ok(EmailClientSettings {
                                base_url: __field0,
                                sender_email: __field1,
                                authorization_token: __field2,
                                timeout_milliseconds: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &[&str] = &[
                        "base_url",
                        "sender_email",
                        "authorization_token",
                        "timeout_milliseconds",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "EmailClientSettings",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                EmailClientSettings<P>,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    const APP_ENVIRONMENT: &str = {
        let _ = || {
            let _ = &APP_ENVIRONMENT;
        };
        "APP_ENVIRONMENT"
    };
    pub struct Settings<P: HKT1Unsized> {
        pub database: K1<P, DatabaseSettings<P>>,
        pub application: K1<P, ApplicationSettings<P>>,
        pub email_client: K1<P, EmailClientSettings<P>>,
    }
    #[allow(missing_docs)]
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl<P: HKT1Unsized> Settings<P> {
        #[inline]
        pub const fn new(
            database: K1<P, DatabaseSettings<P>>,
            application: K1<P, ApplicationSettings<P>>,
            email_client: K1<P, EmailClientSettings<P>>,
        ) -> Settings<P> {
            Settings {
                database: database,
                application: application,
                email_client: email_client,
            }
        }
    }
    impl<P: SharedPointerHKT> Clone for Settings<P> {
        fn clone(&self) -> Self {
            Self {
                database: self.database.clone(),
                application: self.application.clone(),
                email_client: self.email_client.clone(),
            }
        }
    }
    pub struct DatabaseSettings<P: HKT1Unsized> {
        pub username: K1<P, str>,
        pub password: K1<P, str>,
        pub port: u16,
        pub host: K1<P, str>,
        pub database_name: K1<P, str>,
        pub require_ssl: bool,
    }
    #[allow(missing_docs)]
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl<P: HKT1Unsized> DatabaseSettings<P> {
        #[inline]
        pub const fn new(
            username: K1<P, str>,
            password: K1<P, str>,
            port: u16,
            host: K1<P, str>,
            database_name: K1<P, str>,
            require_ssl: bool,
        ) -> DatabaseSettings<P> {
            DatabaseSettings {
                username: username,
                password: password,
                port: port,
                host: host,
                database_name: database_name,
                require_ssl: require_ssl,
            }
        }
    }
    impl<P: SharedPointerHKT> Clone for DatabaseSettings<P> {
        fn clone(&self) -> Self {
            Self {
                username: self.username.clone(),
                password: self.password.clone(),
                port: self.port,
                host: self.host.clone(),
                database_name: self.database_name.clone(),
                require_ssl: self.require_ssl,
            }
        }
    }
    pub struct ApplicationSettings<P: HKT1Unsized> {
        pub port: u16,
        pub host: K1<P, str>,
        pub base_url: K1<P, str>,
        pub hmac_secret: K1<P, HmacSecret<P>>,
    }
    #[allow(missing_docs)]
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl<P: HKT1Unsized> ApplicationSettings<P> {
        #[inline]
        pub const fn new(
            port: u16,
            host: K1<P, str>,
            base_url: K1<P, str>,
            hmac_secret: K1<P, HmacSecret<P>>,
        ) -> ApplicationSettings<P> {
            ApplicationSettings {
                port: port,
                host: host,
                base_url: base_url,
                hmac_secret: hmac_secret,
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, P: HKT1Unsized> _serde::Deserialize<'de> for ApplicationSettings<P>
        where
            P: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "port" => _serde::__private::Ok(__Field::__field0),
                            "host" => _serde::__private::Ok(__Field::__field1),
                            "base_url" => _serde::__private::Ok(__Field::__field2),
                            "hmac_secret" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"port" => _serde::__private::Ok(__Field::__field0),
                            b"host" => _serde::__private::Ok(__Field::__field1),
                            b"base_url" => _serde::__private::Ok(__Field::__field2),
                            b"hmac_secret" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de, P: HKT1Unsized>
                where
                    P: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<ApplicationSettings<P>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de, P: HKT1Unsized> _serde::de::Visitor<'de> for __Visitor<'de, P>
                where
                    P: _serde::Deserialize<'de>,
                {
                    type Value = ApplicationSettings<P>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ApplicationSettings",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            u16,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ApplicationSettings with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            K1<P, str>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct ApplicationSettings with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            K1<P, str>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct ApplicationSettings with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            K1<P, HmacSecret<P>>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct ApplicationSettings with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ApplicationSettings {
                            port: __field0,
                            host: __field1,
                            base_url: __field2,
                            hmac_secret: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u16> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<K1<P, str>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            K1<P, HmacSecret<P>>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("port"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u16>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("host"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "base_url",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<K1<P, str>>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "hmac_secret",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            K1<P, HmacSecret<P>>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("port")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("host")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("base_url")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("hmac_secret")?
                            }
                        };
                        _serde::__private::Ok(ApplicationSettings {
                            port: __field0,
                            host: __field1,
                            base_url: __field2,
                            hmac_secret: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "port",
                    "host",
                    "base_url",
                    "hmac_secret",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ApplicationSettings",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ApplicationSettings<P>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<P: SharedPointerHKT> Clone for ApplicationSettings<P> {
        fn clone(&self) -> Self {
            Self {
                port: self.port,
                host: self.host.clone(),
                base_url: self.base_url.clone(),
                hmac_secret: self.hmac_secret.clone(),
            }
        }
    }
    pub struct EmailClientSettings<P: HKT1Unsized> {
        pub base_url: K1<P, str>,
        pub sender_email: K1<P, str>,
        pub authorization_token: K1<P, str>,
        pub timeout_milliseconds: u64,
    }
    #[allow(missing_docs)]
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl<P: HKT1Unsized> EmailClientSettings<P> {
        #[inline]
        pub const fn new(
            base_url: K1<P, str>,
            sender_email: K1<P, str>,
            authorization_token: K1<P, str>,
            timeout_milliseconds: u64,
        ) -> EmailClientSettings<P> {
            EmailClientSettings {
                base_url: base_url,
                sender_email: sender_email,
                authorization_token: authorization_token,
                timeout_milliseconds: timeout_milliseconds,
            }
        }
    }
    impl<P: HKT1Unsized> EmailClientSettings<P> {
        pub fn timeout(&self) -> std::time::Duration {
            std::time::Duration::from_millis(self.timeout_milliseconds)
        }
    }
    impl<P: SharedPointerHKT> EmailClientSettings<P> {
        pub fn sender(&self) -> Result<SubscriberEmail<P>, SubscriberEmailParseError> {
            SubscriberEmail::try_from(self.sender_email.clone())
        }
    }
    impl<P: SharedPointerHKT> Clone for EmailClientSettings<P> {
        fn clone(&self) -> Self {
            Self {
                base_url: self.base_url.clone(),
                sender_email: self.sender_email.clone(),
                authorization_token: self.authorization_token.clone(),
                timeout_milliseconds: self.timeout_milliseconds,
            }
        }
    }
    pub struct HmacSecret<P: HKT1Unsized>(pub K1<P, SecretString>);
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl<P: HKT1Unsized> derive_more::with_trait::Deref for HmacSecret<P> {
        type Target = K1<P, SecretString>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl<P: HKT1Unsized> derive_more::core::convert::AsRef<K1<P, SecretString>>
    for HmacSecret<P> {
        #[inline]
        fn as_ref(&self) -> &K1<P, SecretString> {
            &self.0
        }
    }
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl<P: HKT1Unsized> derive_more::core::convert::From<(K1<P, SecretString>)>
    for HmacSecret<P> {
        #[inline]
        fn from(value: (K1<P, SecretString>)) -> Self {
            HmacSecret(value)
        }
    }
    #[allow(clippy::unused_unit)]
    #[automatically_derived]
    impl<P: HKT1Unsized> derive_more::core::convert::From<HmacSecret<P>>
    for (K1<P, SecretString>) {
        #[inline]
        fn from(value: HmacSecret<P>) -> Self {
            (<K1<P, SecretString> as derive_more::core::convert::From<_>>::from(value.0))
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, P: HKT1Unsized> _serde::Deserialize<'de> for HmacSecret<P>
        where
            P: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de, P: HKT1Unsized>
                where
                    P: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<HmacSecret<P>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de, P: HKT1Unsized> _serde::de::Visitor<'de> for __Visitor<'de, P>
                where
                    P: _serde::Deserialize<'de>,
                {
                    type Value = HmacSecret<P>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct HmacSecret",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: K1<P, SecretString> = <K1<
                            P,
                            SecretString,
                        > as _serde::Deserialize>::deserialize(__e)?;
                        _serde::__private::Ok(HmacSecret(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            K1<P, SecretString>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct HmacSecret with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(HmacSecret(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "HmacSecret",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<HmacSecret<P>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub enum Environment {
        Local,
        Production,
    }
    impl Environment {
        pub fn as_str(&self) -> &'static str {
            match self {
                Environment::Local => "local",
                Environment::Production => "production",
            }
        }
    }
    impl TryFrom<String> for Environment {
        type Error = String;
        fn try_from(s: String) -> Result<Self, Self::Error> {
            match s.to_lowercase().as_str() {
                "local" => Ok(Self::Local),
                "production" => Ok(Self::Production),
                other => {
                    Err(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "{0} is not a supported environment. Use either `local` or `production`.",
                                    other,
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    pub fn get_configuration<P: RefHKT>() -> Result<Settings<P>, config::ConfigError> {
        use std::env::current_dir;
        let configuration_directory = current_dir()
            .unwrap_or_else(|_| {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "\'{0}()\' failed",
                            {
                                let _ = || {
                                    let _ = &current_dir;
                                };
                                "current_dir"
                            },
                        ),
                    );
                }
            })
            .join("configuration");
        config::Config::builder()
            .add_source(
                config::File::from(configuration_directory.join("base")).required(true),
            )
            .add_source(
                config::File::from(
                        configuration_directory
                            .join(
                                std::env::var(APP_ENVIRONMENT)
                                    .unwrap_or_else(|_| { "local".to_string() })
                                    .pipe(Environment::try_from)
                                    .expect("Parse env var failed.")
                                    .as_str(),
                            ),
                    )
                    .required(true),
            )
            .add_source(config::Environment::with_prefix("app").separator("__"))
            .build()
            .and_then(Config::try_deserialize::<Settings<P>>)
    }
    impl<P: RefHKT> DatabaseSettings<P> {
        pub fn without_db(&self) -> PgConnectOptions {
            PgConnectOptions::new()
                .host(&self.host)
                .username(&self.username)
                .password(&self.password)
                .port(self.port)
                .ssl_mode(
                    if self.require_ssl { PgSslMode::Require } else { PgSslMode::Prefer },
                )
        }
        pub fn with_db(&self) -> PgConnectOptions {
            self.without_db()
                .database(&self.database_name)
                .log_statements(log::LevelFilter::Trace)
        }
    }
}
