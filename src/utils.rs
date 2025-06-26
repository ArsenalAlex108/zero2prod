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
