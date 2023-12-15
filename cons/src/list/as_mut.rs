use crate::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

pub trait ConsListAsMut<'a> {
    type AsMut;

    fn as_mut(&'a mut self) -> Self::AsMut;
}

impl<'a, CAR, CDR> ConsListAsMut<'a> for Cons<Single<CAR>, CDR>
where
    CAR: 'a,
    CDR: ConsListAsMut<'a>,
{
    type AsMut = Cons<Single<&'a mut CAR>, CDR::AsMut>;

    fn as_mut(&'a mut self) -> Self::AsMut {
        let (car, cdr) = self.destructure_mut();
        (car.as_mut(), cdr.as_mut())
    }
}

impl<'a, CAR> ConsListAsMut<'a> for Single<CAR>
where
    CAR: 'a,
{
    type AsMut = Single<&'a mut CAR>;

    fn as_mut(&'a mut self) -> Self::AsMut {
        (self.car_mut(),)
    }
}
