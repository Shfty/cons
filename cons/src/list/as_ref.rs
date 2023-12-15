use crate::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

pub trait ConsListAsRef<'a> {
    type AsRef;

    fn as_ref(&'a self) -> Self::AsRef;
}

impl<'a, CAR, CDR> ConsListAsRef<'a> for Cons<Single<CAR>, CDR>
where
    CAR: 'a,
    CDR: ConsListAsRef<'a>,
{
    type AsRef = Cons<Single<&'a CAR>, CDR::AsRef>;

    fn as_ref(&'a self) -> Self::AsRef {
        let (car, cdr) = self.destructure();
        (car.as_ref(), cdr.as_ref())
    }
}

impl<'a, CAR> ConsListAsRef<'a> for Single<CAR>
where
    CAR: 'a,
{
    type AsRef = Single<&'a CAR>;

    fn as_ref(&'a self) -> Self::AsRef {
        (self.car(),)
    }
}
