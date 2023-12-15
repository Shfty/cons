use std::ops::{DerefMut};

use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

use super::{Branch, ConsTree, Leaf};

pub trait ConsTreeDerefMut<'a, I>: ConsTree<I> {
    type DerefMut;

    fn deref_mut(&'a mut self) -> Self::DerefMut;
}

impl<'a, L, R, CAR, CDR> ConsTreeDerefMut<'a, Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: ConsTreeDerefMut<'a, L>,
    CDR: ConsTreeDerefMut<'a, R>,
{
    type DerefMut = Cons<CAR::DerefMut, CDR::DerefMut>;

    fn deref_mut(&'a mut self) -> Self::DerefMut {
        let (car, cdr) = self.destructure_mut();
        (car.deref_mut(), cdr.deref_mut())
    }
}

impl<'a, CAR> ConsTreeDerefMut<'a, Leaf> for Single<CAR>
where
    CAR: DerefMut,
    CAR::Target: 'a,
{
    type DerefMut = Single<&'a mut CAR::Target>;

    fn deref_mut(&'a mut self) -> Self::DerefMut {
        (self.car_mut().deref_mut(),)
    }
}

