use std::{rc::Rc, sync::Arc};

use naan::HKT1;

pub enum ArcHKT {}
pub enum RcHKT {}
pub trait SharedPointerHKT: HKT1 {
    fn new<T>(v: T) -> Self::T<T>;
    fn from_box<T>(v: Box<T>) -> Self::T<T>;
    fn deref<T>(value: &Self::T<T>) -> &T;
    fn try_unwrap<T>(value: Self::T<T>) -> Result<T, Self::T<T>>;
    fn get_mut<T>(value: &mut Self::T<T>) -> Option<&mut T>;
    fn make_mut<T: Clone>(value: &mut Self::T<T>) -> &mut T;
    fn strong_count<T>(value: &Self::T<T>) -> usize;
    fn clone<T>(value: &Self::T<T>) -> Self::T<T>;
}

impl HKT1 for ArcHKT {
    type T<A> = Arc<A>;
}

impl SharedPointerHKT for ArcHKT {
    fn new<T>(v: T) -> Self::T<T> {
        Arc::new(v)
    }
    fn from_box<T>(v: Box<T>) -> Self::T<T> {
        v.into()
    }

    fn deref<T>(value: &Self::T<T>) -> &T {
        value
    }

    fn try_unwrap<T>(value: Self::T<T>) -> Result<T, Self::T<T>> {
        Arc::try_unwrap(value)
    }

    fn get_mut<T>(value: &mut Self::T<T>) -> Option<&mut T> {
        Arc::get_mut(value)
    }

    fn make_mut<T: Clone>(value: &mut Self::T<T>) -> &mut T {
        Arc::make_mut(value)
    }

    fn strong_count<T>(value: &Self::T<T>) -> usize {
        Arc::strong_count(value)
    }

    fn clone<T>(value: &Self::T<T>) -> Self::T<T> {
        Arc::clone(value)
    }
}

impl HKT1 for RcHKT {
    type T<A> = Rc<A>;
}

impl SharedPointerHKT for RcHKT {
    fn new<T>(v: T) -> Self::T<T> {
        Rc::new(v)
    }

    fn from_box<T>(v: Box<T>) -> Self::T<T> {
        v.into()
    }

    fn deref<T>(value: &Self::T<T>) -> &T {
        &value
    }

    fn try_unwrap<T>(value: Self::T<T>) -> Result<T, Self::T<T>> {
        Rc::try_unwrap(value)
    }

    fn get_mut<T>(value: &mut Self::T<T>) -> Option<&mut T> {
        Rc::get_mut(value)
    }

    fn make_mut<T: Clone>(value: &mut Self::T<T>) -> &mut T {
        Rc::make_mut(value)
    }

    fn strong_count<T>(value: &Self::T<T>) -> usize {
        Rc::strong_count(value)
    }

    fn clone<T>(value: &Self::T<T>) -> Self::T<T> {
        Rc::clone(value)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

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
            Self(PointerK::clone(&self.0))
        }
    }

    fn any_shared_ptr_test<PointerK>()
    where PointerK : super::SharedPointerHKT {
        let a = MyStruct::<PointerK>("Hello World!".using(Box::<str>::from).using(PointerK::new));

        let b_arc = PointerK::clone(&a.0);
    }

    #[test]
    fn arc_test() {
        any_shared_ptr_test::<ArcHKT>();
        any_shared_ptr_test::<RcHKT>();
    }
}
