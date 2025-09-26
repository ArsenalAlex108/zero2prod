use std::marker::PhantomData;
use std::{convert::Infallible, ops::Mul};

use std::any::TypeId;
use crate::dependency_injection::app_state::Marker;

pub trait Lift<'a>: naan::HKT1 {
    fn lift<T: 'a>(t: T) -> Self::T<T>;
}

// Is this type HKT marker?
pub trait Lifter<'a> {
    type T<A: 'a>;

    fn lift<T: 'a>(&self, t: T) -> Self::T<T>;
}

pub trait LiftFunctor<'a> {
    type T<A: Lifter<'a>>;
    fn lift_map<F: Lifter<'a>>(self, lift: &F) -> Self::T<F>;
}

impl<'a> LiftFunctor<'a> for () {
    type T<A: Lifter<'a>> = ();

    fn lift_map<F: Lifter<'a>>(self, _: &F) -> Self::T<F> {}
}

impl<'a, A: 'a, B: LiftFunctor<'a>> LiftFunctor<'a> for (A, B) {
    type T<FA: Lifter<'a>> = (FA::T<A>, B::T<FA>);

    fn lift_map<F: Lifter<'a>>(self, lifter: &F) -> Self::T<F> {
        (lifter.lift(self.0),  self.1.lift_map(lifter))
    }
}

pub trait HKT {
    type T<A>;
}

impl<K: naan::HKT1> HKT for K {
    type T<A> = <Self as naan::HKT1>::T<A>;
}

pub trait LifterK<'a, K: HKT> {
    type T<A: 'a>;

    fn lift<T>(&self, t: T) -> Self::T<T>
    where
    T: 'a + TryFrom<K::T<T>> + Into<K::T<T>>;
}

pub trait LiftFunctorK<'a, K: HKT> {
    type T<A: LifterK<'a, K>>;
    fn lift_map<F: LifterK<'a, K>>(self, lift: &F) -> Self::T<F>;
}

impl<'a, K: HKT> LiftFunctorK<'a, K> for () {
    type T<A: LifterK<'a, K>> = ();

    fn lift_map<F: LifterK<'a, K>>(self, _: &F) -> Self::T<F> {}
}

impl<'a, K: HKT, A, B: LiftFunctorK<'a, K>> LiftFunctorK<'a, K> for (A, B)
where
A: 'a + TryFrom<K::T<A>> + Into<K::T<A>> {
    type T<FA: LifterK<'a, K>> = (FA::T<A>, B::T<FA>);

    fn lift_map<F: LifterK<'a, K>>(self, lifter: &F) -> Self::T<F> {
        (lifter.lift(self.0),  self.1.lift_map(lifter))
    }
}

fn test() {
    
    struct DoubleLifter;

    // Original Goal is using enum to implement trait for variants
    #[derive(Debug, Clone, derive_more::From, derive_more::TryInto)]
    enum Number {
        #[from(i32, i8)]
        I32(i32),
        #[from]
        F32(f32)
    }

    trait NumberVariant {}
    impl NumberVariant for i32 {}
    impl NumberVariant for f32 {}

    trait CloneHKT: HKT {
        type CloneT<T>: Clone + From<Self::T<T>> + Into<Self::T<T>>;
    }

    #[derive(Debug)]
    struct NumberT<A>(Number, PhantomData<A>);

    impl<A> Clone for NumberT<A> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1)
        }
    }

    impl<A> From<Number> for NumberT<A> {
        fn from(value: Number) -> Self {
            NumberT(value, PhantomData)
        }
    }

    impl<A> From<NumberT<A>> for Number {
        fn from(value: NumberT<A>) -> Self {
            value.0
        }
    }

    impl<A> From<i8> for NumberT<A> {
        fn from(value: i8) -> Self {
            Self::from(Number::from(value))
        }
    }

    impl HKT for Number {
        type T<A> = Number;
    }

    impl MultHKT for Number {

        type F<A> = NumberT<A>
            where A: TryFrom<Self::T<A>> + 'static + Into<Self::T<A>>
            ;
    }

    impl CloneHKT for Number {

        type CloneT<T> = NumberT<T>;
    }

    trait MultHKT: HKT + Sized {
        type F<A>: From<Self::T<A>> + Mul<Self::F<A>, Output = Self::F<A>> + Into<Self::T<A>> + From<i8>
            where A: TryFrom<Self::T<A>> + 'static + Into<Self::T<A>>,
            ;
    }

    impl<A> Mul<NumberT<A>> for NumberT<A>
    // Constraint on Phantom parameter, but not completely sound
    // Any type can implement TryFrom<Number> & Into<Number> (e.g. a wrapper of one of the variants), which can be dangerous!
    where A: TryFrom<Number> + Into<Number> + 'static {
        type Output = NumberT<A>;
    
        fn mul(self, rhs: NumberT<A>) -> Self::Output {

            if TypeId::of::<A>() == TypeId::of::<i32>() {
                return Number::I32(i32::try_from(self.0).unwrap()
                * i32::try_from(rhs.0).unwrap())
                .into();
            }
            else if TypeId::of::<A>() == TypeId::of::<f32>() {
                return Number::F32(f32::try_from(self.0).unwrap()
                * f32::try_from(rhs.0).unwrap())
                .into();
            }

            // Use try methods or compare TypeIds
            // i32::try_from(
            //     k.clone()
            // )
            // .map(double_func)
            // .map(K::from)
            // .ok()
            // .or_else(||
            //     f32::try_from(
            //         k.clone()
            //     )
            //     .map(double_func)
            //     .map(K::from)
            //     .ok()
            // )
            // .map(T::try_from)
            // .unwrap()
            // .ok()
            // .unwrap()
            panic!("Unhandled variant")
        }
    }

    // Try replacing K with implementation
    impl<K: HKT> LifterK<'static, K> for DoubleLifter
    where
    K: MultHKT + CloneHKT {
        type T<A: 'static> = A;
        
        fn lift<T>(&self, t: T) -> Self::T<T>
        where
        T: 'static + TryFrom<K::T<T>> + Into<K::T<T>> {

            #[inline]
            fn into<T: Into<R>, R>(t: T) -> R {
                t.into()
            }

            let k = K::CloneT::<T>::from(t.into());

            let c: K::T<T> = double_func(
                K::F::<T>::from(k.clone().into()),
                K::F::<T>::from(k.into()),
            ).into();
            
            c
            .try_into()
            .ok()
            .unwrap()
        }

    }

    // IT WORKS !!!
    let result = LiftFunctorK::<'static, Number>::lift_map((1_i32, (1.0_f32, ())), &DoubleLifter);

    // i8 does not impl TryFrom<Number>
    // let result = LiftFunctorK::<'static, Number>::lift_map((1_i8, (1.0_f32, ())), &DoubleLifter);

    // impl<A: Mul<Output = A> + From<i8>> FnOnce(A) -> A
    fn double_func<A: Mul<Output = A> + From<i8>>(a: A, b: A) -> A {
        a * b * 2.into()
    }

    let func: &dyn FnOnce(i32, i32) -> i32 = &double_func;

    double_func(1, 1);
    
    //  Where constraint is resolved
    //  For homogeneous - Functors for short, the constraint is resolved by using the non-genreric version for map(f)
    //      Definition of map & f doesn't contain constraints imposed by the implementor
    //  For tuples
}


// Not worth turning K to K::T<A> because:
// Harder to add constraints to K::T<A> e.g. Clone
// Some constraints are overflowing again (Maybe fixable by converting From to Into)

// Original Code:
// pub trait LifterK<'a, K> {
//     type T<A: 'a>;

//     fn lift<T>(&self, t: T) -> Self::T<T>
//     where K: From<T>,
//     T: 'a + TryFrom<K>;
//     // 
// }

// pub trait LiftFunctorK<'a, K> {
//     type T<A: LifterK<'a, K>>;
//     fn lift_map<F: LifterK<'a, K>>(self, lift: &F) -> Self::T<F>;
// }

// impl<'a, K> LiftFunctorK<'a, K> for () {
//     type T<A: LifterK<'a, K>> = ();

//     fn lift_map<F: LifterK<'a, K>>(self, _: &F) -> Self::T<F> {}
// }

// impl<'a, K, A, B: LiftFunctorK<'a, K>> LiftFunctorK<'a, K> for (A, B)
// where K: From<A>,
// A: 'a + TryFrom<K> {
//     type T<FA: LifterK<'a, K>> = (FA::T<A>, B::T<FA>);

//     fn lift_map<F: LifterK<'a, K>>(self, lifter: &F) -> Self::T<F> {
//         (lifter.lift(self.0),  self.1.lift_map(lifter))
//     }
// }

// fn test() {
    
//     struct DoubleLifter;

//     // Original Goal is using enum to implement trait for variants
//     #[derive(Debug, Clone, derive_more::From, derive_more::TryInto)]
//     enum Number {
//         #[from(i32, i8)]
//         I32(i32),
//         #[from]
//         F32(f32)
//     }

//     trait NumberVariant {}
//     impl NumberVariant for i32 {}
//     impl NumberVariant for f32 {}


//     #[derive(Debug)]
//     struct NumberT<A>(Number, PhantomData<A>);

//     impl<A> From<Number> for NumberT<A> {
//         fn from(value: Number) -> Self {
//             NumberT(value, PhantomData)
//         }
//     }

//     impl<A> From<NumberT<A>> for Number {
//         fn from(value: NumberT<A>) -> Self {
//             value.0
//         }
//     }

//     impl<A> From<i8> for NumberT<A> {
//         fn from(value: i8) -> Self {
//             Self::from(Number::from(value))
//         }
//     }

//     impl MultHKT for Number {
//         type F<A> = NumberT<A>
//             where A: TryFrom<Number> + 'static,
//             Number: From<A> 
//             //+ From<Self::F<A>>
//             ;
//     }

//     trait MultHKT: Sized {
//         type F<A>: From<Self> + Mul<Self::F<A>, Output = Self::F<A>> + Into<Self> + From<i8>
//             where A: TryFrom<Self> + 'static,
//             Self: From<A>
//             ;
//     }

//     impl<A> Mul<NumberT<A>> for NumberT<A>
//     // Constraint on Phantom parameter, but not completely sound
//     // Any type can implement TryFrom<Number> & Into<Number> (e.g. a wrapper of one of the variants), which can be dangerous!
//     where A: TryFrom<Number> + Into<Number> + 'static,
//     Number: From<A> {
//         type Output = NumberT<A>;
    
//         fn mul(self, rhs: NumberT<A>) -> Self::Output {

//             if TypeId::of::<A>() == TypeId::of::<i32>() {
//                 return Number::I32(i32::try_from(self.0).unwrap()
//                 * i32::try_from(rhs.0).unwrap())
//                 .into();
//             }
//             else if TypeId::of::<A>() == TypeId::of::<f32>() {
//                 return Number::F32(f32::try_from(self.0).unwrap()
//                 * f32::try_from(rhs.0).unwrap())
//                 .into();
//             }

//             // Use try methods or compare TypeIds
//             // i32::try_from(
//             //     k.clone()
//             // )
//             // .map(double_func)
//             // .map(K::from)
//             // .ok()
//             // .or_else(||
//             //     f32::try_from(
//             //         k.clone()
//             //     )
//             //     .map(double_func)
//             //     .map(K::from)
//             //     .ok()
//             // )
//             // .map(T::try_from)
//             // .unwrap()
//             // .ok()
//             // .unwrap()
//             panic!("Unhandled variant")
//         }
//     }

//     // Try replacing K with implementation
//     impl<K> LifterK<'static, K> for DoubleLifter
//     where
//     K: MultHKT + Clone {
//         type T<A: 'static> = A;
        
//         fn lift<T>(&self, t: T) -> Self::T<T>
//         where K: From<T>,
//         T: 'static + TryFrom<K> {
//             let k = K::from(t);

//             let c: K = double_func(
//                 K::F::<T>::from(k.clone()),
//                 K::F::<T>::from(k),
//             ).into();
            
//             c
//             .try_into()
//             .ok()
//             .unwrap()
//         }

//     }

//     // IT WORKS !!!
//     let result = LiftFunctorK::<'static, Number>::lift_map((1_i32, (1.0_f32, ())), &DoubleLifter);

//     // i8 does not impl TryFrom<Number>
//     // let result = LiftFunctorK::<'static, Number>::lift_map((1_i8, (1.0_f32, ())), &DoubleLifter);

//     // impl<A: Mul<Output = A> + From<i8>> FnOnce(A) -> A
//     fn double_func<A: Mul<Output = A> + From<i8>>(a: A, b: A) -> A {
//         a * b * 2.into()
//     }

//     let func: &dyn FnOnce(i32, i32) -> i32 = &double_func;

//     double_func(1, 1);
    
//     //  Where constraint is resolved
//     //  For homogeneous - Functors for short, the constraint is resolved by using the non-genreric version for map(f)
//     //      Definition of map & f doesn't contain constraints imposed by the implementor
//     //  For tuples
// }
pub trait LifterMut<'a> {
    type T<A: 'a>;

    fn lift<T: 'a>(&mut self, t: T) -> Self::T<T>;
}

#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
)]
pub struct ThinDataHKT(Infallible);

impl Marker for ThinDataHKT {}

impl naan::HKT1 for ThinDataHKT {
    type T<T> = actix_web::web::ThinData<T>;
}

impl<'a> Lift<'a> for ThinDataHKT {
    fn lift<T: 'a>(t: T) -> Self::T<T> {
        actix_web::web::ThinData(t)
    }
}

pub trait TupleMap9<T, U, V, W, X, Y, Z, A, B> {
    #[allow(clippy::type_complexity)]
    fn lift_map<'a, F>(
        self,
    ) -> (
        F::T<T>,
        F::T<U>,
        F::T<V>,
        F::T<W>,
        F::T<X>,
        F::T<Y>,
        F::T<Z>,
        F::T<A>,
        F::T<B>,
    )
    where
        F: Lift<'a>,
        U: 'a,
        T: 'a,
        V: 'a,
        W: 'a,
        X: 'a,
        Y: 'a,
        Z: 'a,
        A: 'a,
        B: 'a;

    #[allow(clippy::type_complexity)]
    fn map<'a, G>(
        self,
        f: &G,
    ) -> (
        G::T<T>,
        G::T<U>,
        G::T<V>,
        G::T<W>,
        G::T<X>,
        G::T<Y>,
        G::T<Z>,
        G::T<A>,
        G::T<B>,
    )
    where
        G: Lifter<'a>,
        U: 'a,
        T: 'a,
        V: 'a,
        W: 'a,
        X: 'a,
        Y: 'a,
        Z: 'a,
        A: 'a,
        B: 'a;

    #[allow(clippy::type_complexity)]
    fn map_mut<'a, G>(
        self,
        f: &mut G,
    ) -> (
        G::T<T>,
        G::T<U>,
        G::T<V>,
        G::T<W>,
        G::T<X>,
        G::T<Y>,
        G::T<Z>,
        G::T<A>,
        G::T<B>,
    )
    where
        G: LifterMut<'a>,
        U: 'a,
        T: 'a,
        V: 'a,
        W: 'a,
        X: 'a,
        Y: 'a,
        Z: 'a,
        A: 'a,
        B: 'a;
}

impl<T, U, V, W, X, Y, Z, A, B>
    TupleMap9<T, U, V, W, X, Y, Z, A, B>
    for (T, U, V, W, X, Y, Z, A, B)
{
    fn lift_map<'a, F>(
        self,
    ) -> (
        F::T<T>,
        F::T<U>,
        F::T<V>,
        F::T<W>,
        F::T<X>,
        F::T<Y>,
        F::T<Z>,
        F::T<A>,
        F::T<B>,
    )
    where
        F: Lift<'a>,
        U: 'a,
        T: 'a,
        V: 'a,
        W: 'a,
        X: 'a,
        Y: 'a,
        Z: 'a,
        A: 'a,
        B: 'a,
    {
        (
            F::lift(self.0),
            F::lift(self.1),
            F::lift(self.2),
            F::lift(self.3),
            F::lift(self.4),
            F::lift(self.5),
            F::lift(self.6),
            F::lift(self.7),
            F::lift(self.8),
        )
    }

    fn map<'a, G>(
        self,
        f: &G,
    ) -> (
        G::T<T>,
        G::T<U>,
        G::T<V>,
        G::T<W>,
        G::T<X>,
        G::T<Y>,
        G::T<Z>,
        G::T<A>,
        G::T<B>,
    )
    where
        G: Lifter<'a>,
        U: 'a,
        T: 'a,
        V: 'a,
        W: 'a,
        X: 'a,
        Y: 'a,
        Z: 'a,
        A: 'a,
        B: 'a,
    {
        (
            f.lift(self.0),
            f.lift(self.1),
            f.lift(self.2),
            f.lift(self.3),
            f.lift(self.4),
            f.lift(self.5),
            f.lift(self.6),
            f.lift(self.7),
            f.lift(self.8),
        )
    }

    fn map_mut<'a, G>(
        self,
        f: &mut G,
    ) -> (
        G::T<T>,
        G::T<U>,
        G::T<V>,
        G::T<W>,
        G::T<X>,
        G::T<Y>,
        G::T<Z>,
        G::T<A>,
        G::T<B>,
    )
    where
        G: LifterMut<'a>,
        U: 'a,
        T: 'a,
        V: 'a,
        W: 'a,
        X: 'a,
        Y: 'a,
        Z: 'a,
        A: 'a,
        B: 'a,
    {
        (
            f.lift(self.0),
            f.lift(self.1),
            f.lift(self.2),
            f.lift(self.3),
            f.lift(self.4),
            f.lift(self.5),
            f.lift(self.6),
            f.lift(self.7),
            f.lift(self.8),
        )
    }
}
