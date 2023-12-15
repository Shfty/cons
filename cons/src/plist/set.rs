use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

pub trait ConsPListSet<K, V, I> {
    type Set;

    fn set(self, v: V) -> Self::Set;
}

impl<K, V, I, LCAR, LCDR, CDR> ConsPListSet<K, V, (I,)>
    for Cons<Single<LCAR>, Cons<Single<LCDR>, CDR>>
where
    CDR: ConsPListSet<K, V, I>,
{
    type Set = Cons<Single<LCAR>, Cons<Single<LCDR>, CDR::Set>>;

    fn set(self, v: V) -> Self::Set {
        let (car, cdr) = self.into_destructure();
        let (rcar, rcdr) = cdr.into_destructure();
        (car, (rcar, rcdr.set(v)))
    }
}

impl<LCAR, LCDR, CDR> ConsPListSet<LCAR, LCDR, ()> for Cons<Single<LCAR>, Cons<Single<LCDR>, CDR>> {
    type Set = Self;

    fn set(self, v: LCDR) -> Self::Set {
        let (car, cdr) = self.into_destructure();
        (car, ((v,), cdr.into_cdr()))
    }
}

impl<CAR, CDR> ConsPListSet<CAR, CDR, ()> for Cons<Single<CAR>, Single<CDR>> {
    type Set = Self;

    fn set(self, v: CDR) -> Self::Set {
        (self.into_car(), (v,))
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
    fn test_cons_plist_set() {
        let cons_plist = plist! {
            One => 1,
            Two => 2.0,
            Three => '3',
            Four => "four"
        };

        let cons_plist = ConsPListSet::<One, _, _>::set(cons_plist, 2);
        let cons_plist = ConsPListSet::<Two, _, _>::set(cons_plist, 3.0);
        let cons_plist = ConsPListSet::<Three, _, _>::set(cons_plist, '4');
        let cons_plist = ConsPListSet::<Four, _, _>::set(cons_plist, "five");

        assert!(
            cons_plist == plist! {
                One => 2,
                Two => 3.0,
                Three => '4',
                Four => "five"
            }
        );
    }
}
