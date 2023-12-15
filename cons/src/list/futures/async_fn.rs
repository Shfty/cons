use futures::Future;

use crate::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

/// An `async fn` with no parameters
pub trait AsyncFn<F: Future> {
    fn run(&self) -> F;
}

impl<F, T> AsyncFn<F> for T
where
    F: Future,
    T: Fn() -> F,
{
    fn run(&self) -> F {
        self()
    }
}

/// `ConsList` of `async fn` types with no parameters
pub trait AsyncFnConsList<F> {
    type Run;

    fn run(&self) -> Self::Run;
}

impl<FCAR, FCDR, CAR, CDR> AsyncFnConsList<Cons<FCAR, FCDR>> for Cons<Single<CAR>, CDR>
where
    FCAR: Future,
    CAR: AsyncFn<FCAR>,
    CDR: AsyncFnConsList<FCDR>,
{
    type Run = Cons<Single<FCAR>, CDR::Run>;

    fn run(&self) -> Self::Run {
        let (car, cdr) = self.destructure();
        ((car.car().run(),), cdr.run())
    }
}

impl<F, T> AsyncFnConsList<F> for Single<T>
where
    F: Future,
    T: AsyncFn<F>,
{
    type Run = Single<F>;

    fn run(&self) -> Self::Run {
        (self.car().run(),)
    }
}
