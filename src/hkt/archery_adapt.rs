use kust::ScopeFunctions;
use naan::HKT1;
use nameof::name_of;
use std::{fmt::Debug, ops::Deref, rc::Rc, sync::Arc};

use crate::hkt;

// pub mod bespoke {
//     pub trait From<T>: Sized {
//         fn from(value: T) -> Self;
//     }

//     pub trait Into<T>: Sized {
//         fn into(self) -> T;
//     }
// }

#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum ArcHKT {}
#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum RcHKT {}
#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BoxHKT {}

pub trait Infallible:
    Sized + std::fmt::Debug + 'static + Eq + Ord
{
}

impl<T: Sized + std::fmt::Debug + 'static + Eq + Ord>
    Infallible for T
{
}

pub trait HKT1Unsized: Infallible {
    type T<A: ?Sized>;
}

pub trait RefHKT: HKT1Unsized {
    fn new<T>(v: T) -> K1<Self, T>;
    fn from_box<T: ?Sized>(v: Box<T>) -> K1<Self, T>;
    fn deref<T: ?Sized>(value: &Self::T<T>) -> &T;

    #[must_use]
    fn from_string(v: String) -> K1<Self, str> {
        Self::from_box(Box::<str>::from(v))
    }
    #[must_use]
    fn from_str(v: &str) -> K1<Self, str> {
        Self::from_box(Box::<str>::from(v))
    }
    #[must_use]
    fn from_static_str(v: &'static str) -> K1<Self, str> {
        Self::from_box(Box::<str>::from(v))
    }
    #[must_use]
    fn from_vec<T>(v: Vec<T>) -> K1<Self, [T]> {
        Self::from_box(Box::<[T]>::from(v))
    }
}

pub trait SharedPointerHKT: RefHKT {
    fn try_unwrap<T>(
        value: Self::T<T>,
    ) -> Result<T, Self::T<T>>;
    fn get_mut<T: ?Sized>(
        value: &mut Self::T<T>,
    ) -> Option<&mut T>;
    fn make_mut<T: Clone>(value: &mut Self::T<T>)
    -> &mut T;
    fn strong_count<T: ?Sized>(value: &Self::T<T>)
    -> usize;
    fn clone<T: ?Sized>(value: &Self::T<T>) -> K1<Self, T>;
}

pub trait SendHKT: HKT1Unsized {}
pub trait SyncHKT: HKT1Unsized {}

impl SendHKT for ArcHKT {}

impl SyncHKT for ArcHKT {}

unsafe impl<P: SendHKT, A: ?Sized> Send for K1<P, A> {}

unsafe impl<P: SyncHKT, A: ?Sized> Sync for K1<P, A> {}

// // pub struct SharedPointer<T>(T);

// // impl<P: HKT1Unsized, A: ?Sized> Clone for SharedPointer<P::T<A>> {

// // }

// pub trait SharedPointerExt<
//     P: SharedPointerHKT,
//     A: ?Sized,
// > where
//     Self: Into<P::T<A>>,
// {
//     fn as_ref(&self) -> &A;
//     fn newtype(self) -> K1<P, A>;
//     fn into_k<K: Into<P::T<A>>>(self)
//     -> K1<P, A>;
//     fn from_k<K: From<P::T<A>>>(self) -> K
//     where
//         Self: Sized;

//     fn p<P2: HKT1Unsized>(self) -> K1<P2, A>
//     where
//         P2::T<A>: From<P::T<A>>;

//     fn pa<P2: HKT1Unsized, A2: ?Sized>(
//         self,
//     ) -> K1<P2, A2>
//     where
//         P2::T<A2>: From<P::T<A>>;
// }

// impl<P: SharedPointerHKT, A: ?Sized>
//     SharedPointerExt<P, A> for P::T<A>
// {
//     fn as_ref(&self) -> &A {
//         P::deref(&self)
//     }

//     fn newtype(self) -> K1<P, A> {
//         K1(self)
//     }

//     fn into_k<K: Into<P::T<A>>>(
//         self,
//     ) -> K1<P, A> {
//         K1(self)
//     }

//     fn from_k<K: From<P::T<A>>>(self) -> K
//     where
//         Self: Sized,
//     {
//         K::from(self.into())
//     }

//     fn p<P2: HKT1Unsized>(self) -> K1<P2, A>
//     where
//         P2::T<A>: From<P::T<A>>,
//     {
//         K1(P2::T::<A>::from(self.into()))
//     }

//     fn pa<P2: HKT1Unsized, A2: ?Sized>(
//         self,
//     ) -> K1<P2, A2>
//     where
//         P2::T<A2>: From<P::T<A>>,
//     {
//         K1(P2::T::<A2>::from(self.into()))
//     }
// }

/// New type wrapper for `P::T`<A> of HKT P
pub struct K1<P: HKT1Unsized, A: ?Sized>(P::T<A>);

pub fn newtype<P: HKT1Unsized, A: ?Sized>(
    value: P::T<A>,
) -> K1<P, A> {
    K1(value)
}

#[allow(unreachable_code)]
#[automatically_derived]
impl<P: RefHKT, A: ?Sized> derive_more::core::fmt::Display
    for K1<P, A>
where
    A: derive_more::core::fmt::Display,
{
    fn fmt(
        &self,
        __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
    ) -> derive_more::core::fmt::Result {
        derive_more::core::fmt::Display::fmt(
            &**self,
            __derive_more_f,
        )
    }
}

// fn rc_clone_safety_test() {
//     use kust::ScopeFunctions;
//     let boxed = RcHKT::new(1).using(Box::new);
//     let rc_cloned = boxed
//         .deref()
//         .clone()
//         .inner()
//         .using(Box::new);
//     let mapped = boxed
//         .using(|i| *i)
//         .inner()
//         .using(Box::new);

//     // Can't avoid cloning T in Result<T,E>
// }

// impl<P: HKT1Unsized, A: ?Sized> bespoke::From<K1<P,A>> for P::T<A> {
//     fn from(value: K1<P,A>) -> Self {
//         value.0
//     }
// }

// impl<P: HKT1Unsized, A: ?Sized>
//     bespoke::From<P::T<A>> for K1<P, A>
// {
//     fn from(value: P::T<A>) -> Self {
//         K1(value)
//     }
// }

// impl<P: HKT1Unsized, A: ?Sized>
//     bespoke::Into<K1<P, A>> for P::T<A>
// {
//     fn into(self) -> K1<P, A> {
//         K1(self)
//     }
// }

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
// pub trait SharedPointerK1<P: SharedPointerHKT, A>:
//     Deref<Target = A> + AsRef<A> + Clone
// {
// }

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

/// Consider adding `PartialEqHKT` to allow implementors to customize and/or optimize impl
impl<P: RefHKT, A: ?Sized + PartialEq> PartialEq
    for K1<P, A>
{
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(&**other)
    }
}

impl<P: RefHKT, A: ?Sized + Eq> Eq for K1<P, A> {}

impl<P: RefHKT, A: ?Sized + PartialOrd> PartialOrd
    for K1<P, A>
{
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        self.deref().partial_cmp(&**other)
    }
}

impl<P: RefHKT, A: ?Sized + Ord> Ord for K1<P, A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.deref().cmp(&**other)
    }
}

impl<P: RefHKT, A: ?Sized + std::hash::Hash> std::hash::Hash
    for K1<P, A>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl<P: RefHKT, A: ?Sized + std::fmt::Debug> std::fmt::Debug
    for K1<P, A>
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_tuple(name_of!(type K1<P,A>))
            .field(&&**self)
            .finish()
    }
}

impl<P: RefHKT, A: ?Sized + serde::ser::Serialize>
    serde::ser::Serialize for K1<P, A>
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.deref().serialize(serializer)
    }
}

impl<'de, P: RefHKT, A: serde::de::Deserialize<'de>>
    serde::de::Deserialize<'de> for K1<P, A>
{
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        A::deserialize::<D>(deserializer).map(P::new)
    }
}

impl<'de, P: RefHKT, A: serde::de::Deserialize<'de>>
    serde::de::Deserialize<'de> for K1<P, [A]>
{
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Box::<[A]>::deserialize::<D>(deserializer)
            .map(P::from_box)
    }
}

impl<'de, P: RefHKT> serde::de::Deserialize<'de>
    for K1<P, str>
{
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Box::<str>::deserialize::<D>(deserializer)
            .map(P::from_box)
    }
}

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
    fn try_unwrap<T>(
        value: Self::T<T>,
    ) -> Result<T, Self::T<T>> {
        Arc::try_unwrap(value)
    }

    fn get_mut<T: ?Sized>(
        value: &mut Self::T<T>,
    ) -> Option<&mut T> {
        Arc::get_mut(value)
    }

    fn make_mut<T: Clone>(
        value: &mut Self::T<T>,
    ) -> &mut T {
        Arc::make_mut(value)
    }

    fn strong_count<T: ?Sized>(
        value: &Self::T<T>,
    ) -> usize {
        Arc::strong_count(value)
    }

    fn clone<T: ?Sized>(value: &Self::T<T>) -> K1<Self, T> {
        Arc::clone(value).using(K1::newtype)
    }
}

impl HKT1 for RcHKT {
    type T<A> = Rc<A>;
}

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
        value
    }
}

impl SharedPointerHKT for RcHKT {
    fn try_unwrap<T>(
        value: Self::T<T>,
    ) -> Result<T, Self::T<T>> {
        Rc::try_unwrap(value)
    }

    fn get_mut<T: ?Sized>(
        value: &mut Self::T<T>,
    ) -> Option<&mut T> {
        Rc::get_mut(value)
    }

    fn make_mut<T: Clone>(
        value: &mut Self::T<T>,
    ) -> &mut T {
        Rc::make_mut(value)
    }

    fn strong_count<T: ?Sized>(
        value: &Self::T<T>,
    ) -> usize {
        Rc::strong_count(value)
    }

    fn clone<T: ?Sized>(value: &Self::T<T>) -> K1<Self, T> {
        Rc::clone(value).using(K1::newtype)
    }
}

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
        &**value
    }
}

#[cfg(test)]
mod tests {

    use crate::hkt::RcHKT;

    use super::ArcHKT;
    use kust::ScopeFunctions;
    struct MyStruct<PointerK>(PointerK::T<Box<str>>)
    where
        PointerK: super::SharedPointerHKT;

    impl<PointerK> Clone for MyStruct<PointerK>
    where
        PointerK: super::SharedPointerHKT,
    {
        fn clone(&self) -> Self {
            Self(PointerK::clone(&self.0).inner())
        }
    }

    fn any_shared_ptr_test<PointerK>()
    where
        PointerK: super::SharedPointerHKT,
    {
        let a = MyStruct::<PointerK>(
            "Hello World!"
                .using(Box::<str>::from)
                .using(PointerK::new)
                .inner(),
        );

        let _b_arc = PointerK::clone(&a.0);
    }

    #[test]
    fn arc_test() {
        any_shared_ptr_test::<ArcHKT>();
        any_shared_ptr_test::<RcHKT>();
    }
}
