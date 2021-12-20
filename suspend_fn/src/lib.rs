#![doc = include_str!("../../README.md")]

pub use suspend_fn_proc_macro::{suspend, suspend_fn, suspend_move};

#[doc(hidden)]
#[allow(clippy::wrong_self_convention)]
pub mod __private {
    use std::future::Future;
    pub trait AsFuture {
        type Output;
        fn as_future(self) -> std::future::Ready<Self::Output>;
    }

    impl<T> AsFuture for Expr<T> {
        type Output = T;
        fn as_future(self) -> std::future::Ready<Self::Output> {
            std::future::ready(self.0)
        }
    }
    pub struct Expr<T>(pub T);

    impl<T: Future> Expr<T> {
        pub fn as_future(self) -> impl Future<Output = T::Output> {
            self.0
        }
    }
}
