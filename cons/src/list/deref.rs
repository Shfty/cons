use std::ops::Deref;

use crate::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

pub trait ConsListDeref<'a> {
    type Deref;

    fn deref(&'a self) -> Self::Deref;
}

impl<'a, CAR, CDR> ConsListDeref<'a> for Cons<Single<CAR>, CDR>
where
    CAR: 'a + Deref,
    CDR: ConsListDeref<'a>,
{
    type Deref = Cons<Single<&'a CAR::Target>, CDR::Deref>;

    fn deref(&'a self) -> Self::Deref {
        let (car, cdr) = self.destructure();
        (car.deref(), cdr.deref())
    }
}

impl<'a, CAR> ConsListDeref<'a> for Single<CAR>
where
    CAR: 'a + Deref,
{
    type Deref = Single<&'a CAR::Target>;

    fn deref(&'a self) -> Self::Deref {
        (self.car().deref(),)
    }
}
