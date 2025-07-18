pub trait Pipe {
    #[inline]
    fn pipe<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        f(self)
    }

    #[inline]
    fn pipe_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Self) -> R,
    {
        f(self)
    }

    fn ref_cast(&self) -> &Self {
        self
    }

    #[inline]
    fn pipe_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        f(self)
    }
}

impl<T: ?Sized> Pipe for T {}

pub fn not_called<A, B>(_: A) -> B {
    panic!("This function is not supposed to be called.")
}

pub fn unpack_result_to_some_tuple<T, E>(
    i: Result<T, E>,
) -> (Option<T>, Option<E>) {
    match i {
        Ok(t) => (Some(t), None),
        Err(e) => (None, Some(e)),
    }
}

pub fn unpack_result_to_result_tuple<T, E>(
    i: Result<T, E>,
) -> (Result<T, ()>, Result<(), E>) {
    match i {
        Ok(t) => (Ok(t), Ok(())),
        Err(e) => (Err(()), Err(e)),
    }
}

pub fn see_other_response(
    location: &str,
) -> actix_web::HttpResponse {
    actix_web::HttpResponse::SeeOther()
        .insert_header((
            actix_web::http::header::LOCATION,
            location,
        ))
        .finish()
}

pub async fn await_sequential<I>(
    iter: I,
) -> Vec<<I::Item as Future>::Output>
where
    I: IntoIterator,
    I::Item: Future,
{
    let iter = iter.into_iter();

    let mut results = Vec::with_capacity(
        iter.size_hint().pipe(|bound| {
            if let Some(max) = bound.1 {
                max
            } else {
                bound.0
            }
        }),
    );

    for i in iter {
        results.push(i.await)
    }

    results
}

// #[derive(derive_more::AsRef)]
// pub struct LateInit<T>(Option<T>);

// impl<T> Default for LateInit<T> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }

// impl<T> LateInit<T> {

//     pub fn new(value: Option<T>) -> Self {
//         LateInit(value)
//     }

//     pub fn into_inner(self) -> Option<T> {
//         self.0
//     }

//     pub fn unwrap(self) -> T {
//         self.0.expect("Value was not initialized.")
//     }
// }

// impl<T> AsRef<T> for LateInit<T> {
//     fn as_ref(&self) -> &T {
//         self.0.as_ref()
//         .expect("Value was not initialized.")
//     }
// }

// impl<T> AsMut<T> for LateInit<T> {
//     fn as_mut(&mut self) -> &mut T {
//         self.0.as_mut()
//         .expect("Value was not initialized.")
//     }
// }

// impl<T> Deref for LateInit<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         self.0.as_ref()
//         .expect("Value was not initialized.")
//     }
// }

// impl<T> DerefMut for LateInit<T> {
//     fn deref_mut(&mut self) -> &mut T {
//         self.0.as_mut()
//         .expect("Value was not initialized.")
//     }
// }

// impl<T> From<T> for LateInit<T> {
//     fn from(value: T) -> Self {
//         Self(Some(value))
//     }
// }

pub struct SyncMutCell<T>(SyncCell<Option<T>>);

pub struct SyncMutCellBorrow<'a, T> {
    value: OnceLock<T>,
    origin: &'a SyncMutCell<T>,
}

impl<T> SyncMutCell<T> {
    pub fn borrow(&self) -> SyncMutCellBorrow<'_, T> {
        self.try_borrow()
        .expect("Value must not be mut borrowed more than once at a time.")
    }

    pub fn try_borrow(
        &self,
    ) -> Option<SyncMutCellBorrow<'_, T>> {
        self.0.replace(None).map(|value| {
            SyncMutCellBorrow {
                value: value.pipe(OnceLock::from),
                origin: self,
            }
        })
    }
}

impl<T> From<T> for SyncMutCell<T> {
    fn from(value: T) -> Self {
        SyncMutCell(SyncCell::new(Some(value)))
    }
}

impl<T> DerefMut for SyncMutCellBorrow<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.get_mut()
        .expect("Borrowed value should have been initialized as guaranteed by MutCell::borrow.")
    }
}

impl<T> Deref for SyncMutCellBorrow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.get()
        .expect("Borrowed value should have been initialized as guaranteed by MutCell::borrow.")
    }
}

impl<T> Drop for SyncMutCellBorrow<'_, T> {
    fn drop(&mut self) {
        self.origin.0.replace(self.value.take());
    }
}

pub struct MutCell<T>(Cell<Option<T>>);

pub struct MutCellBorrow<'a, T> {
    value: OnceCell<T>,
    origin: &'a MutCell<T>,
}

impl<T> MutCell<T> {
    pub fn borrow(&self) -> MutCellBorrow<'_, T> {
        self.try_borrow()
        .expect("Value must not be mut borrowed more than once at a time.")
    }

    pub fn try_borrow(
        &self,
    ) -> Option<MutCellBorrow<'_, T>> {
        self.0.replace(None).map(|value| MutCellBorrow {
            value: value.pipe(OnceCell::from),
            origin: self,
        })
    }
}

impl<T> From<T> for MutCell<T> {
    fn from(value: T) -> Self {
        MutCell(Cell::new(Some(value)))
    }
}

impl<T> DerefMut for MutCellBorrow<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.get_mut()
        .expect("Borrowed value should have been initialized as guaranteed by MutCell::borrow.")
    }
}

impl<T> Deref for MutCellBorrow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.get()
        .expect("Borrowed value should have been initialized as guaranteed by MutCell::borrow.")
    }
}

impl<T> Drop for MutCellBorrow<'_, T> {
    fn drop(&mut self) {
        self.origin.0.replace(self.value.take());
    }
}

use std::sync::OnceLock;
use std::{
    cell::{Cell, OnceCell},
    ops::{Deref, DerefMut},
};
use sync_cell::SyncCell;

pub use crate::trait_cast;

#[macro_export]
macro_rules! trait_cast {
    ($T: path) => {{
        |i| {
            #[inline]
            fn cast(x: impl $T) -> impl $T {
                x
            }
            cast(i)
        }
    }};
}
