use futures::{future::Fuse, Future, FutureExt};

use crate::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

use super::FutureConsList;

/// `ConsList` of `Future`s
pub trait FutureConsListFuse: FutureConsList {
    type Fuse;

    fn fuse(self) -> Self::Fuse;
}

impl<F, CDR> FutureConsListFuse for Cons<Single<F>, CDR>
where
    F: Future,
    CDR: FutureConsListFuse,
{
    type Fuse = Cons<Single<Fuse<F>>, CDR::Fuse>;

    fn fuse(self) -> Self::Fuse {
        let (car, cdr) = self.into_destructure();
        let car = car.into_car();
        ((car.fuse(),), cdr.fuse())
    }
}

impl<F> FutureConsListFuse for Single<F>
where
    F: Future,
{
    type Fuse = Single<Fuse<F>>;

    fn fuse(self) -> Self::Fuse {
        let car = self.into_car();
        (car.fuse(),)
    }
}
