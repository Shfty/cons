use crate::{cell::Cons, single::Single};

pub trait ConsListRefSignature<'a> {
    type Ref;
}

impl<'a, CAR, CDR> ConsListRefSignature<'a> for Cons<Single<CAR>, CDR>
where
    CAR: 'a,
    CDR: ConsListRefSignature<'a>,
{
    type Ref = Cons<Single<&'a CAR>, CDR::Ref>;
}

impl<'a, CAR> ConsListRefSignature<'a> for Single<CAR>
where
    CAR: 'a,
{
    type Ref = Single<&'a CAR>;
}
