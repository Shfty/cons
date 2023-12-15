use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

pub trait ConsPListGet<K, I> {
    type Get;

    fn get(self) -> Self::Get;
}

impl<K, I, LCAR, LCDR, CDR> ConsPListGet<K, (I,)> for Cons<Single<LCAR>, Cons<Single<LCDR>, CDR>>
where
    CDR: ConsPListGet<K, I>,
{
    type Get = CDR::Get;

    fn get(self) -> Self::Get {
        self.into_cdr().into_cdr().get()
    }
}

impl<LCAR, LCDR, CDR> ConsPListGet<LCAR, ()> for Cons<Single<LCAR>, Cons<Single<LCDR>, CDR>> {
    type Get = Cons<LCAR, LCDR>;

    fn get(self) -> Self::Get {
        let (car, cdr) = self.into_destructure();
        (car.into_car(), cdr.into_car().into_car())
    }
}

impl<CAR, CDR> ConsPListGet<CAR, ()> for Cons<Single<CAR>, Single<CDR>> {
    type Get = Cons<CAR, CDR>;

    fn get(self) -> Self::Get {
        let (car, cdr) = self.into_destructure();
        (car.into_car(), cdr.into_car())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plist;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct One;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Two;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Three;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Four;

    #[test]
    fn test_cons_plist_get() {
        let cons_plist = plist! {
            One => 1,
            Two => 2.0,
            Three => '3',
            Four => "four"
        };

        let one = ConsPListGet::<One, _>::get(cons_plist);
        assert!(one == (One, 1));

        let two = ConsPListGet::<Two, _>::get(cons_plist);
        assert!(two == (Two, 2.0));

        let three = ConsPListGet::<Three, _>::get(cons_plist);
        assert!(three == (Three, '3'));

        let four = ConsPListGet::<Four, _>::get(cons_plist);
        assert!(four == (Four, "four"));
    }
}
