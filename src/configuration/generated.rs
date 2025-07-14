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
    #[allow(
        unused_extern_crates,
        clippy::useless_attribute
    )]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, P: RefHKT> _serde::Deserialize<'de>
        for Settings<P>
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
                ) -> _serde::__private::fmt::Result
                {
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
                        0u64 => _serde::__private::Ok(
                            __Field::__field0,
                        ),
                        1u64 => _serde::__private::Ok(
                            __Field::__field1,
                        ),
                        2u64 => _serde::__private::Ok(
                            __Field::__field2,
                        ),
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                        "database" => {
                            _serde::__private::Ok(
                                __Field::__field0,
                            )
                        }
                        "application" => {
                            _serde::__private::Ok(
                                __Field::__field1,
                            )
                        }
                        "email_client" => {
                            _serde::__private::Ok(
                                __Field::__field2,
                            )
                        }
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                        b"database" => {
                            _serde::__private::Ok(
                                __Field::__field0,
                            )
                        }
                        b"application" => {
                            _serde::__private::Ok(
                                __Field::__field1,
                            )
                        }
                        b"email_client" => {
                            _serde::__private::Ok(
                                __Field::__field2,
                            )
                        }
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                marker: _serde::__private::PhantomData<
                    Settings<P>,
                >,
                lifetime:
                    _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, P: RefHKT> _serde::de::Visitor<'de>
                for __Visitor<'de, P>
            {
                type Value = Settings<P>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result
                {
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
                    while let _serde::__private::Some(
                        __key,
                    ) =
                        _serde::de::MapAccess::next_key::<
                            __Field,
                        >(
                            &mut __map
                        )?
                    {
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
            const FIELDS: &[&str] = &[
                "database",
                "application",
                "email_client",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Settings",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        Settings<P>,
                    >,
                    lifetime:
                        _serde::__private::PhantomData,
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
    #[allow(
        unused_extern_crates,
        clippy::useless_attribute
    )]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, P: RefHKT> _serde::Deserialize<'de>
        for ApplicationSettings<P>
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
                ) -> _serde::__private::fmt::Result
                {
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
                        0u64 => _serde::__private::Ok(
                            __Field::__field0,
                        ),
                        1u64 => _serde::__private::Ok(
                            __Field::__field1,
                        ),
                        2u64 => _serde::__private::Ok(
                            __Field::__field2,
                        ),
                        3u64 => _serde::__private::Ok(
                            __Field::__field3,
                        ),
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                        "port" => _serde::__private::Ok(
                            __Field::__field0,
                        ),
                        "host" => _serde::__private::Ok(
                            __Field::__field1,
                        ),
                        "base_url" => {
                            _serde::__private::Ok(
                                __Field::__field2,
                            )
                        }
                        "hmac_secret" => {
                            _serde::__private::Ok(
                                __Field::__field3,
                            )
                        }
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                        b"port" => _serde::__private::Ok(
                            __Field::__field0,
                        ),
                        b"host" => _serde::__private::Ok(
                            __Field::__field1,
                        ),
                        b"base_url" => {
                            _serde::__private::Ok(
                                __Field::__field2,
                            )
                        }
                        b"hmac_secret" => {
                            _serde::__private::Ok(
                                __Field::__field3,
                            )
                        }
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                marker: _serde::__private::PhantomData<
                    ApplicationSettings<P>,
                >,
                lifetime:
                    _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, P: RefHKT> _serde::de::Visitor<'de>
                for __Visitor<'de, P>
            {
                type Value = ApplicationSettings<P>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result
                {
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
                    _serde::__private::Ok(
                        ApplicationSettings {
                            port: __field0,
                            host: __field1,
                            base_url: __field2,
                            hmac_secret: __field3,
                        },
                    )
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
                    while let _serde::__private::Some(
                        __key,
                    ) =
                        _serde::de::MapAccess::next_key::<
                            __Field,
                        >(
                            &mut __map
                        )?
                    {
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
                    _serde::__private::Ok(
                        ApplicationSettings {
                            port: __field0,
                            host: __field1,
                            base_url: __field2,
                            hmac_secret: __field3,
                        },
                    )
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
                    marker: _serde::__private::PhantomData::<
                        ApplicationSettings<P>,
                    >,
                    lifetime:
                        _serde::__private::PhantomData,
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
    #[allow(
        unused_extern_crates,
        clippy::useless_attribute
    )]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, P: RefHKT> _serde::Deserialize<'de>
        for DatabaseSettings<P>
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
                ) -> _serde::__private::fmt::Result
                {
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
                        0u64 => _serde::__private::Ok(
                            __Field::__field0,
                        ),
                        1u64 => _serde::__private::Ok(
                            __Field::__field1,
                        ),
                        2u64 => _serde::__private::Ok(
                            __Field::__field2,
                        ),
                        3u64 => _serde::__private::Ok(
                            __Field::__field3,
                        ),
                        4u64 => _serde::__private::Ok(
                            __Field::__field4,
                        ),
                        5u64 => _serde::__private::Ok(
                            __Field::__field5,
                        ),
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                        "username" => {
                            _serde::__private::Ok(
                                __Field::__field0,
                            )
                        }
                        "password" => {
                            _serde::__private::Ok(
                                __Field::__field1,
                            )
                        }
                        "port" => _serde::__private::Ok(
                            __Field::__field2,
                        ),
                        "host" => _serde::__private::Ok(
                            __Field::__field3,
                        ),
                        "database_name" => {
                            _serde::__private::Ok(
                                __Field::__field4,
                            )
                        }
                        "require_ssl" => {
                            _serde::__private::Ok(
                                __Field::__field5,
                            )
                        }
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                        b"username" => {
                            _serde::__private::Ok(
                                __Field::__field0,
                            )
                        }
                        b"password" => {
                            _serde::__private::Ok(
                                __Field::__field1,
                            )
                        }
                        b"port" => _serde::__private::Ok(
                            __Field::__field2,
                        ),
                        b"host" => _serde::__private::Ok(
                            __Field::__field3,
                        ),
                        b"database_name" => {
                            _serde::__private::Ok(
                                __Field::__field4,
                            )
                        }
                        b"require_ssl" => {
                            _serde::__private::Ok(
                                __Field::__field5,
                            )
                        }
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                marker: _serde::__private::PhantomData<
                    DatabaseSettings<P>,
                >,
                lifetime:
                    _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, P: RefHKT> _serde::de::Visitor<'de>
                for __Visitor<'de, P>
            {
                type Value = DatabaseSettings<P>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result
                {
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
                        struct __DeserializeWith<'de, P: RefHKT>
                            {
                                value: u16,
                                phantom: _serde::__private::PhantomData<
                                    DatabaseSettings<P>,
                                >,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                        #[automatically_derived]
                        impl<'de, P: RefHKT>
                            _serde::Deserialize<'de>
                            for __DeserializeWith<'de, P>
                        {
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
                        _serde::__private::Some(
                            __value,
                        ) => __value,
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
                    _serde::__private::Ok(
                        DatabaseSettings {
                            username: __field0,
                            password: __field1,
                            port: __field2,
                            host: __field3,
                            database_name: __field4,
                            require_ssl: __field5,
                        },
                    )
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
                    while let _serde::__private::Some(
                        __key,
                    ) =
                        _serde::de::MapAccess::next_key::<
                            __Field,
                        >(
                            &mut __map
                        )?
                    {
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
                                __field2 =
                                    _serde::__private::Some(
                                        {
                                            #[doc(hidden)]
                                            struct __DeserializeWith<'de, P: RefHKT>
                                        {
                                            value: u16,
                                            phantom: _serde::__private::PhantomData<
                                                DatabaseSettings<P>,
                                            >,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                            #[automatically_derived]
                                            impl<'de, P: RefHKT> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de, P>
                                        {
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
                                        },
                                    );
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
                        _serde::__private::Some(
                            __field2,
                        ) => __field2,
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
                    _serde::__private::Ok(
                        DatabaseSettings {
                            username: __field0,
                            password: __field1,
                            port: __field2,
                            host: __field3,
                            database_name: __field4,
                            require_ssl: __field5,
                        },
                    )
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
                    lifetime:
                        _serde::__private::PhantomData,
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
    #[allow(
        unused_extern_crates,
        clippy::useless_attribute
    )]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, P: RefHKT> _serde::Deserialize<'de>
        for EmailClientSettings<P>
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
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result
                {
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
                        0u64 => _serde::__private::Ok(
                            __Field::__field0,
                        ),
                        1u64 => _serde::__private::Ok(
                            __Field::__field1,
                        ),
                        2u64 => _serde::__private::Ok(
                            __Field::__field2,
                        ),
                        3u64 => _serde::__private::Ok(
                            __Field::__field3,
                        ),
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                        "base_url" => {
                            _serde::__private::Ok(
                                __Field::__field0,
                            )
                        }
                        "sender_email" => {
                            _serde::__private::Ok(
                                __Field::__field1,
                            )
                        }
                        "authorization_token" => {
                            _serde::__private::Ok(
                                __Field::__field2,
                            )
                        }
                        "timeout_milliseconds" => {
                            _serde::__private::Ok(
                                __Field::__field3,
                            )
                        }
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                        b"base_url" => {
                            _serde::__private::Ok(
                                __Field::__field0,
                            )
                        }
                        b"sender_email" => {
                            _serde::__private::Ok(
                                __Field::__field1,
                            )
                        }
                        b"authorization_token" => {
                            _serde::__private::Ok(
                                __Field::__field2,
                            )
                        }
                        b"timeout_milliseconds" => {
                            _serde::__private::Ok(
                                __Field::__field3,
                            )
                        }
                        _ => _serde::__private::Ok(
                            __Field::__ignore,
                        ),
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
                marker: _serde::__private::PhantomData<
                    EmailClientSettings<P>,
                >,
                lifetime:
                    _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, P: RefHKT> _serde::de::Visitor<'de>
                for __Visitor<'de, P>
            {
                type Value = EmailClientSettings<P>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result
                {
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
                    _serde::__private::Ok(
                        EmailClientSettings {
                            base_url: __field0,
                            sender_email: __field1,
                            authorization_token: __field2,
                            timeout_milliseconds: __field3,
                        },
                    )
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
                    while let _serde::__private::Some(
                        __key,
                    ) =
                        _serde::de::MapAccess::next_key::<
                            __Field,
                        >(
                            &mut __map
                        )?
                    {
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
                    _serde::__private::Ok(
                        EmailClientSettings {
                            base_url: __field0,
                            sender_email: __field1,
                            authorization_token: __field2,
                            timeout_milliseconds: __field3,
                        },
                    )
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
                    lifetime:
                        _serde::__private::PhantomData,
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
    #[allow(
        unused_extern_crates,
        clippy::useless_attribute
    )]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, P: RefHKT> _serde::Deserialize<'de>
        for HmacSecret<P>
    {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, P: RefHKT> {
                marker: _serde::__private::PhantomData<
                    HmacSecret<P>,
                >,
                lifetime:
                    _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, P: RefHKT> _serde::de::Visitor<'de>
                for __Visitor<'de, P>
            {
                type Value = HmacSecret<P>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result
                {
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
                    _serde::__private::Ok(HmacSecret(
                        __field0,
                    ))
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
                    _serde::__private::Ok(HmacSecret(
                        __field0,
                    ))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "HmacSecret",
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        HmacSecret<P>,
                    >,
                    lifetime:
                        _serde::__private::PhantomData,
                },
            )
        }
    }
};

// Old impl with validation error collection (but Clone is required)
// impl<'de, P: RefHKT> serde::Deserialize<'de>
//     for EmailClientSettings<P>
// {
//     fn deserialize<D>(
//         deserializer: D,
//     ) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         struct EmailClientSettingsVisitor<P>(
//             PhantomData<P>,
//         );

//         impl<'de, P: RefHKT>
//             serde::de::Visitor<'de>
//             for EmailClientSettingsVisitor<P>
//         {
//             type Value = EmailClientSettings<P>;

//             fn expecting(
//                 &self,
//                 formatter: &mut std::fmt::Formatter,
//             ) -> std::fmt::Result {
//                 formatter.write_str(formatcp!(
//                     "struct {}",
//                     stringify!(EmailClientSettings)
//                 ))
//             }

//             fn visit_seq<A>(
//                 self,
//                 seq: A,
//             ) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::SeqAccess<'de>,
//             {
//                 let _ = seq;
//                 Err(serde::de::Error::invalid_type(
//                     serde::de::Unexpected::Seq,
//                     &self,
//                 ))
//             }

//             fn visit_map<A>(
//                 self,
//                 mut map: A,
//             ) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::MapAccess<'de>,
//             {
//                 let mut base_url: Result<_, A::Error> = Err(
//                     de::Error::missing_field("base_url"),
//                 );
//                 let mut sender_email: Result<_, A::Error> =
//                     Err(de::Error::missing_field(
//                         "sender_email",
//                     ));
//                 while let Some(key) = map.next_key()? {
//                     match key {
//                         "base_url" => {
//                             base_url = if base_url.is_ok() {
//                                 Err(de::Error::duplicate_field(name_of!(base_url)))
//                             } else {
//                                 map.next_value()
//                             }
//                         }
//                         "sender_email" => {
//                             sender_email = if sender_email.is_ok() {
//                                 Err(de::Error::duplicate_field(name_of!(sender_email)))
//                             } else {
//                                 map.next_value()
//                             }
//                         }
//                         _ => return Err(de::Error::unknown_variant(key, &["base_url", "sender_email"]))
//                     }
//                 }

//                 Validation::from(Ok(EmailClientSettings::new.curry()))
//                 .apply_with(base_url.map_err(DeserializeError::from).into(), utils::not_called)
//                 .apply_with(sender_email.map_err(DeserializeError::from).into(), utils::not_called)
//                 .pipe(Result::from)
//                 .map_err(DeserializeError::inner)
//             }
//         }

//         deserializer.deserialize_seq(
//             EmailClientSettingsVisitor::<P>(PhantomData),
//         )
//     }
// }
