use std::borrow::Borrow;

use crate::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

pub trait ConsAListBorrow<'a, B> {
    type Borrow;

    fn borrow(&'a self) -> Self::Borrow;
}

impl<'a, BLCAR, BLCDR, BCDR, LCAR, LCDR, CDR>
    ConsAListBorrow<'a, Cons<Single<Cons<BLCAR, BLCDR>>, BCDR>>
    for Cons<Single<Cons<LCAR, LCDR>>, CDR>
where
    BLCAR: 'a,
    BLCDR: 'a,
    LCAR: 'a + Borrow<BLCAR>,
    LCDR: 'a + Borrow<BLCDR>,
    CDR: 'a + ConsAListBorrow<'a, BCDR>,
{
    type Borrow = Cons<Single<Cons<&'a BLCAR, &'a BLCDR>>, CDR::Borrow>;

    fn borrow(&'a self) -> Self::Borrow {
        let (car, cdr) = self.destructure();
        let (lcar, lcdr) = car.car().destructure();
        (((lcar.borrow(), lcdr.borrow()),), cdr.borrow())
    }
}

impl<'a, BCAR, BCDR, CAR, CDR> ConsAListBorrow<'a, Single<Cons<BCAR, BCDR>>> for Single<Cons<CAR, CDR>>
where
    BCAR: 'a,
    BCDR: 'a,
    CAR: 'a + Borrow<BCAR>,
    CDR: 'a + Borrow<BCDR>,
{
    type Borrow = ((&'a BCAR, &'a BCDR),);

    fn borrow(&'a self) -> Self::Borrow {
        let (car, cdr) = self.car().destructure();
        ((car.borrow(), cdr.borrow()),)
    }
}
