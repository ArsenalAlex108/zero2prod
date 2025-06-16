use std::marker::PhantomData;

use kust::ScopeFunctions;
use naan::HKT1;
use naan::apply::{Applicative, Apply};
use naan::functor::Functor;
use naan::prelude::Semigroup;

pub struct ValidationHKT<E>(PhantomData<E>);

#[derive(derive_more::From, derive_more::Into)]
pub struct Validation<T, E>(Result<T, E>);

impl<E> HKT1 for ValidationHKT<E> {
    type T<A> = Validation<A, E>;
}

impl<T, E> Functor<ValidationHKT<E>, T>
    for Validation<T, E>
{
    fn fmap<AB, B>(
        self,
        f: AB,
    ) -> <ValidationHKT<E> as HKT1>::T<B>
    where
        AB: naan::prelude::F1<T, Ret = B>,
    {
        self.0.map(|i| f.call(i)).into()
    }
}

impl<T, E: Semigroup> Apply<ValidationHKT<E>, T>
    for Validation<T, E>
{
    fn apply_with<A, B, Cloner>(
        self,
        ta: <ValidationHKT<E> as HKT1>::T<A>,
        _cloner: Cloner,
    ) -> <ValidationHKT<E> as HKT1>::T<B>
    where
        T: naan::prelude::F1<A, Ret = B>,
        Cloner: for<'a> naan::prelude::F1<&'a A, Ret = A>,
    {
        // let ta = Cell::new(Some(ta));
        // self.0.map_or_else(
        //     |e1| {
        //         let e1 = Cell::new(Some(e1));
        //         ta.replace(None)
        //         .expect("Only one bracnch should be executed.")
        //         .0.map_or_else(
        //             |e2| {
        //                 Err(e1.replace(None)
        //                 .expect("Only one bracnch should be executed.")
        //                 .append(e2))
        //                     .using(Validation)
        //             },
        //             |_| Err(e1.replace(None).expect("Only one bracnch should be executed.")).using(Validation),
        //         )
        //     },
        //     |f| {
        //         ta.replace(None)
        //         .expect("Only one bracnch should be executed.")
        //         .0.map_or_else(
        //             |e| Err(e).using(Validation),
        //             |a| {
        //                 f.call(a)
        //                     .using(Ok)
        //                     .using(Validation)
        //             },
        //         )
        //     }
        // )
        match self.0 {
            Err(e1) => match ta.0 {
                Err(e2) => {
                    Err(e1.append(e2)).using(Validation)
                }
                Ok(_) => Err(e1).using(Validation),
            },
            Ok(f) => match ta.0 {
                Err(e) => Err(e).using(Validation),
                Ok(a) => {
                    f.call(a).using(Ok).using(Validation)
                }
            },
        }
    }
}

impl<T, E: Semigroup> Applicative<ValidationHKT<E>, T>
    for Validation<T, E>
{
    fn pure(a: T) -> <ValidationHKT<E> as HKT1>::T<T> {
        a.using(Ok).into()
    }
}
