use std::ops::{DerefMut};

use crate::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

pub trait ConsListDerefMut<'a> {
    type DerefMut;

    fn deref_mut(&'a mut self) -> Self::DerefMut;
}

impl<'a, CAR, CDR> ConsListDerefMut<'a> for Cons<Single<CAR>, CDR>
where
    CAR: 'a + DerefMut,
    CDR: ConsListDerefMut<'a>,
{
    type DerefMut = Cons<Single<&'a mut CAR::Target>, CDR::DerefMut>;

    fn deref_mut(&'a mut self) -> Self::DerefMut {
        let (car, cdr) = self.destructure_mut();
        (car.deref_mut(), cdr.deref_mut())
    }
}

impl<'a, CAR> ConsListDerefMut<'a> for Single<CAR>
where
    CAR: 'a + DerefMut,
{
    type DerefMut = Single<&'a mut CAR::Target>;

    fn deref_mut(&'a mut self) -> Self::DerefMut {
        (self.car_mut().deref_mut(),)
    }
}
