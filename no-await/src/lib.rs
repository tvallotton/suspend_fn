#![allow(clippy::wrong_self_convention)]
pub mod __private {
    use std::future::Future;
    trait AsFuture {
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

#[cfg(test)]
#[tokio::test]
async fn test() {
    println!("{}", crate::__private::Expr(async { 1 }).as_future().await);

    println!("{}", Expr(10).as_future().await);
}

