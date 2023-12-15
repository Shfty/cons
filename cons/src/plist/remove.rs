use crate::{cell::{Cons, ConsCell}, single::Single};

enum Left {}
enum Right{}

pub trait ConsPListRemove<K, I> {
    type Remove;

    fn remove(self) -> Self::Remove;
}

impl<K, I, LCAR, LCDR, CDR> ConsPListRemove<K, (I,)> for Cons<Single<LCAR>, Cons<Single<LCDR>, CDR>>
where
    CDR: ConsPListRemove<K, I>,
{
    type Remove = Cons<Single<LCAR>, Cons<Single<LCDR>, CDR::Remove>>;

    fn remove(self) -> Self::Remove {
        let (lcar, cdr) = self.into_destructure();
        let (rcar, cdr) = cdr.into_destructure();
        (lcar, (rcar, cdr.remove()))
    }
}

impl<LCAR, LCDR, RCAR, RCDR> ConsPListRemove<LCAR, Left> for Cons<Single<LCAR>, Cons<Single<LCDR>, Cons<Single<RCAR>, Single<RCDR>>>>
{
    type Remove = Cons<Single<RCAR>, Single<RCDR>>;

    fn remove(self) -> Self::Remove {
        self.into_cdr().into_cdr()
    }
}

impl<LCAR, LCDR, RCAR, RCDR> ConsPListRemove<RCAR, Right> for Cons<Single<LCAR>, Cons<Single<LCDR>, Cons<Single<RCAR>, Single<RCDR>>>>
{
    type Remove = Cons<Single<LCAR>, Single<LCDR>>;

    fn remove(self) -> Self::Remove {
        let (lcar, cdr) = self.into_destructure();
        (lcar, cdr.into_car())
    }
}

impl<CAR, CDR> ConsPListRemove<CAR, ()> for Cons<Single<CAR>, Single<CDR>> {
    type Remove = ();

    fn remove(self) -> Self::Remove {}
}

#[cfg(test)]
mod tests {
    use super::ConsPListRemove;
    use crate::plist;

    #[test]
    fn test_cons_alist_remove() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Float;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Integer;

        let cons_plist = plist! {
            Float => 1.0,
            Integer => 2
        };
        println!("PList: {:?}", cons_plist);

        let cons_plist = ConsPListRemove::<Float, _>::remove(cons_plist);
        println!("PList: {:?}", cons_plist);

        assert!(cons_plist == plist! { Integer => 2 });

        let cons_plist = ConsPListRemove::<Integer, _>::remove(cons_plist);
        println!("PList: {:?}", cons_plist);
    }
}
