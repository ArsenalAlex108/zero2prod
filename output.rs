mod archery_adapt {
    use kust::ScopeFunctions;
    use naan::HKT1;
    use nameof::name_of;
    use std::{fmt::Debug, ops::Deref, rc::Rc, sync::Arc};
    use crate::hkt;
    pub enum ArcHKT {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ArcHKT {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArcHKT {
        #[inline]
        fn clone(&self) -> ArcHKT {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ArcHKT {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArcHKT {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArcHKT {
        #[inline]
        fn eq(&self, other: &ArcHKT) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArcHKT {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArcHKT {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArcHKT,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArcHKT {
        #[inline]
        fn cmp(&self, other: &ArcHKT) -> ::core::cmp::Ordering {
            match *self {}
        }
    }
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl derive_more::core::fmt::Display for ArcHKT {
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            match *self {}
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
        impl _serde::Serialize for ArcHKT {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {}
            }
        }
    };
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
        impl<'de> _serde::Deserialize<'de> for ArcHKT {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {}
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
                            "variant identifier",
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
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 0",
                                    ),
                                )
                            }
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
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
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
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
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
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ArcHKT>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ArcHKT;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum ArcHKT",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        _serde::__private::Result::map(
                            _serde::de::EnumAccess::variant::<__Field>(__data),
                            |(__impossible, _)| match __impossible {},
                        )
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ArcHKT",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ArcHKT>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub enum RcHKT {}
    #[automatically_derived]
    impl ::core::fmt::Debug for RcHKT {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RcHKT {
        #[inline]
        fn clone(&self) -> RcHKT {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for RcHKT {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RcHKT {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RcHKT {
        #[inline]
        fn eq(&self, other: &RcHKT) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for RcHKT {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for RcHKT {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RcHKT,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for RcHKT {
        #[inline]
        fn cmp(&self, other: &RcHKT) -> ::core::cmp::Ordering {
            match *self {}
        }
    }
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl derive_more::core::fmt::Display for RcHKT {
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            match *self {}
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
        impl _serde::Serialize for RcHKT {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {}
            }
        }
    };
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
        impl<'de> _serde::Deserialize<'de> for RcHKT {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {}
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
                            "variant identifier",
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
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 0",
                                    ),
                                )
                            }
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
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
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
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
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
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<RcHKT>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RcHKT;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum RcHKT",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        _serde::__private::Result::map(
                            _serde::de::EnumAccess::variant::<__Field>(__data),
                            |(__impossible, _)| match __impossible {},
                        )
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "RcHKT",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<RcHKT>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub enum BoxHKT {}
    #[automatically_derived]
    impl ::core::fmt::Debug for BoxHKT {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BoxHKT {
        #[inline]
        fn clone(&self) -> BoxHKT {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for BoxHKT {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BoxHKT {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BoxHKT {
        #[inline]
        fn eq(&self, other: &BoxHKT) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for BoxHKT {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for BoxHKT {
        #[inline]
        fn partial_cmp(
            &self,
            other: &BoxHKT,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for BoxHKT {
        #[inline]
        fn cmp(&self, other: &BoxHKT) -> ::core::cmp::Ordering {
            match *self {}
        }
    }
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl derive_more::core::fmt::Display for BoxHKT {
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            match *self {}
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
        impl _serde::Serialize for BoxHKT {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {}
            }
        }
    };
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
        impl<'de> _serde::Deserialize<'de> for BoxHKT {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {}
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
                            "variant identifier",
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
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 0",
                                    ),
                                )
                            }
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
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
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
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
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
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<BoxHKT>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = BoxHKT;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum BoxHKT",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        _serde::__private::Result::map(
                            _serde::de::EnumAccess::variant::<__Field>(__data),
                            |(__impossible, _)| match __impossible {},
                        )
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "BoxHKT",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<BoxHKT>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub trait Infallible: Sized + std::fmt::Debug + 'static + Eq + Ord {}
    pub trait HKT1Unsized: Infallible {
        type T<A: ?Sized>;
    }
    pub trait RefHKT: HKT1Unsized {
        fn new<T>(v: T) -> K1<Self, T>;
        fn from_box<T: ?Sized>(v: Box<T>) -> K1<Self, T>;
        fn deref<T: ?Sized>(value: &Self::T<T>) -> &T;
    }
    pub trait SharedPointerHKT: RefHKT {
        fn try_unwrap<T>(value: Self::T<T>) -> Result<T, Self::T<T>>;
        fn get_mut<T: ?Sized>(value: &mut Self::T<T>) -> Option<&mut T>;
        fn make_mut<T: ?Sized + Clone>(value: &mut Self::T<T>) -> &mut T;
        fn strong_count<T: ?Sized>(value: &Self::T<T>) -> usize;
        fn clone<T: ?Sized>(value: &Self::T<T>) -> K1<Self, T>;
    }
    /// New type wrapper for P::T<A> of HKT P
    pub struct K1<P: HKT1Unsized, A: ?Sized>(P::T<A>);
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl<P: HKT1Unsized, A: ?Sized> derive_more::core::fmt::Display for K1<P, A>
    where
        P::T<A>: derive_more::core::fmt::Display,
    {
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            let _0 = &self.0;
            derive_more::core::fmt::Display::fmt(_0, __derive_more_f)
        }
    }
    pub fn newtype<P: HKT1Unsized, A: ?Sized>(value: P::T<A>) -> K1<P, A> {
        K1(value)
    }
    impl<P: HKT1Unsized, A: ?Sized> K1<P, A> {
        pub fn inner(self) -> P::T<A> {
            self.0
        }
        pub fn inner_ref(&self) -> &P::T<A> {
            &self.0
        }
        pub fn inner_mut(&mut self) -> &mut P::T<A> {
            &mut self.0
        }
        pub fn newtype(value: P::T<A>) -> K1<P, A> {
            K1(value)
        }
    }
    impl<P: RefHKT, A: ?Sized> Deref for K1<P, A> {
        type Target = A;
        fn deref(&self) -> &Self::Target {
            P::deref(&self.0)
        }
    }
    impl<P: RefHKT, A: ?Sized> AsRef<A> for K1<P, A> {
        fn as_ref(&self) -> &A {
            P::deref(&self.0)
        }
    }
    impl<P: SharedPointerHKT, A: ?Sized> Clone for K1<P, A> {
        fn clone(&self) -> Self {
            P::clone(&self.0)
        }
    }
    /// Consider adding PartialEqHKT to allow implementors to customize and/or optimize impl
    impl<P: RefHKT, A: ?Sized + PartialEq> PartialEq for K1<P, A> {
        fn eq(&self, other: &Self) -> bool {
            self.deref().eq(other.deref())
        }
    }
    impl<P: RefHKT, A: ?Sized + Eq> Eq for K1<P, A> {}
    impl<P: RefHKT, A: ?Sized + PartialOrd> PartialOrd for K1<P, A> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.deref().partial_cmp(&other.deref())
        }
    }
    impl<P: RefHKT, A: ?Sized + Ord> Ord for K1<P, A> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.deref().cmp(&other.deref())
        }
    }
    impl<P: RefHKT, A: ?Sized + std::hash::Hash> std::hash::Hash for K1<P, A> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.deref().hash(state);
        }
    }
    impl<P: RefHKT, A: ?Sized + std::fmt::Debug> std::fmt::Debug for K1<P, A> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple({
                    {
                        let _ = || {
                            let _: K1<P, A>;
                        };
                        "K1<P,A>"
                    }
                })
                .field(&self.deref())
                .finish()
        }
    }
    impl<P: RefHKT, A: ?Sized + serde::ser::Serialize> serde::ser::Serialize
    for K1<P, A> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.deref().serialize(serializer)
        }
    }
    impl<
        'de,
        P: RefHKT,
        A: ?Sized + serde::de::Deserialize<'de>,
    > serde::de::Deserialize<'de> for K1<P, A> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            A::deserialize::<D>(deserializer).map(P::new)
        }
    }
    impl<
        'de,
        P: RefHKT,
        A: ?Sized + serde::de::Deserialize<'de>,
    > serde::de::Deserialize<'de> for K1<P, [A]> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Box::<[A]>::deserialize::<D>(deserializer).map(P::from_box)
        }
    }
    impl<'de, P: RefHKT> serde::de::Deserialize<'de> for K1<P, str> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Box::<str>::deserialize::<D>(deserializer).map(P::from_box)
        }
    }
    impl Infallible for ArcHKT {}
    impl HKT1 for ArcHKT {
        type T<A> = Arc<A>;
    }
    impl hkt::Debug for ArcHKT {
        fn fmt_k<A: ?Sized + std::fmt::Debug>(
            value: &Self::T<A>,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            Arc::<A>::fmt(value, f)
        }
    }
    impl HKT1Unsized for ArcHKT {
        type T<A: ?Sized> = Arc<A>;
    }
    impl RefHKT for ArcHKT {
        fn new<T>(v: T) -> K1<Self, T> {
            Arc::new(v).using(K1::newtype)
        }
        fn from_box<T: ?Sized>(v: Box<T>) -> K1<Self, T> {
            v.using(Arc::<T>::from).using(K1::newtype)
        }
        fn deref<T: ?Sized>(value: &Self::T<T>) -> &T {
            value
        }
    }
    impl SharedPointerHKT for ArcHKT {
        fn try_unwrap<T>(value: Self::T<T>) -> Result<T, Self::T<T>> {
            Arc::try_unwrap(value)
        }
        fn get_mut<T: ?Sized>(value: &mut Self::T<T>) -> Option<&mut T> {
            Arc::get_mut(value)
        }
        fn make_mut<T: ?Sized + Clone>(value: &mut Self::T<T>) -> &mut T {
            Arc::make_mut(value)
        }
        fn strong_count<T: ?Sized>(value: &Self::T<T>) -> usize {
            Arc::strong_count(value)
        }
        fn clone<T: ?Sized>(value: &Self::T<T>) -> K1<Self, T> {
            Arc::clone(value).using(K1::newtype)
        }
    }
    impl HKT1 for RcHKT {
        type T<A> = Rc<A>;
    }
    impl Infallible for RcHKT {}
    impl hkt::Debug for RcHKT {
        fn fmt_k<A: ?Sized + std::fmt::Debug>(
            value: &Self::T<A>,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            Rc::<A>::fmt(value, f)
        }
    }
    impl HKT1Unsized for RcHKT {
        type T<A: ?Sized> = Rc<A>;
    }
    impl RefHKT for RcHKT {
        fn new<T>(v: T) -> K1<Self, T> {
            Rc::new(v).using(K1::newtype)
        }
        fn from_box<T: ?Sized>(v: Box<T>) -> K1<Self, T> {
            v.using(Rc::<T>::from).using(K1::newtype)
        }
        fn deref<T: ?Sized>(value: &Self::T<T>) -> &T {
            &value
        }
    }
    impl SharedPointerHKT for RcHKT {
        fn try_unwrap<T>(value: Self::T<T>) -> Result<T, Self::T<T>> {
            Rc::try_unwrap(value)
        }
        fn get_mut<T: ?Sized>(value: &mut Self::T<T>) -> Option<&mut T> {
            Rc::get_mut(value)
        }
        fn make_mut<T: ?Sized + Clone>(value: &mut Self::T<T>) -> &mut T {
            Rc::make_mut(value)
        }
        fn strong_count<T: ?Sized>(value: &Self::T<T>) -> usize {
            Rc::strong_count(value)
        }
        fn clone<T: ?Sized>(value: &Self::T<T>) -> K1<Self, T> {
            Rc::clone(value).using(K1::newtype)
        }
    }
    impl Infallible for BoxHKT {}
    impl HKT1Unsized for BoxHKT {
        type T<A: ?Sized> = Box<A>;
    }
    impl RefHKT for BoxHKT {
        fn new<T>(v: T) -> K1<Self, T> {
            K1::newtype(Box::new(v))
        }
        fn from_box<T: ?Sized>(v: Box<T>) -> K1<Self, T> {
            K1::newtype(v)
        }
        fn deref<T: ?Sized>(value: &Self::T<T>) -> &T {
            value.deref()
        }
    }
}
