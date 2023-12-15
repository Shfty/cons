use std::pin::Pin;

use futures::Future;

use crate::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

use super::FutureConsList;

/// `ConsList` of `Future`s that can be pinned on the stack
pub trait FutureConsListPinMut<'a>: FutureConsList {
    type PinMut;

    fn pin_mut(&'a mut self) -> Self::PinMut;
}

impl<'a, F, CDR> FutureConsListPinMut<'a> for Cons<Single<F>, CDR>
where
    F: 'a + Future,
    CDR: FutureConsListPinMut<'a>,
{
    type PinMut = Cons<Single<Pin<&'a mut F>>, CDR::PinMut>;

    fn pin_mut(&'a mut self) -> Self::PinMut {
        let (car, cdr) = self.destructure_mut();
        let car = car.car_mut();
        ((unsafe { Pin::new_unchecked(car) },), cdr.pin_mut())
    }
}

impl<'a, F> FutureConsListPinMut<'a> for Single<F>
where
    F: 'a + Future,
{
    type PinMut = Single<Pin<&'a mut F>>;

    fn pin_mut(&'a mut self) -> Self::PinMut {
        let car = self.car_mut();
        (unsafe { Pin::new_unchecked(car) },)
    }
}
