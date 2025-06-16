pub trait Pipe {
    fn pipe<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

impl<T: ?Sized> Pipe for T {}

pub fn not_called<A, B>(_: A) -> B {
    panic!("This function is not supposed to be called.")
}
