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
