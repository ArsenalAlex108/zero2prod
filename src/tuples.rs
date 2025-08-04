use std::convert::Infallible;

use crate::dependency_injection::app_state::Marker;

pub trait Lift<'a>: naan::HKT1 {
    fn lift<T: 'a>(t: T) -> Self::T<T>;
}

// Is this type HKT marker?
pub trait Lifter<'a> {
    type T<A: 'a>;

    fn lift<T: 'a>(&self, t: T) -> Self::T<T>;
}

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
