use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

enum Left {}
enum Right {}

pub trait ConsAListRemove<K, I> {
    type Remove;

    fn remove(self) -> Self::Remove;
}

impl<K, I, LCAR, LCDR, CDR> ConsAListRemove<K, (I,)> for Cons<Single<Cons<LCAR, LCDR>>, CDR>
where
    CDR: ConsAListRemove<K, I>,
{
    type Remove = Cons<Single<Cons<LCAR, LCDR>>, CDR::Remove>;

    fn remove(self) -> Self::Remove {
        let (car, cdr) = self.into_destructure();
        (car, cdr.remove())
    }
}

impl<LCAR, LCDR, RCAR, RCDR> ConsAListRemove<LCAR, Left>
    for Cons<Single<Cons<LCAR, LCDR>>, Single<Cons<RCAR, RCDR>>>
{
    type Remove = Single<Cons<RCAR, RCDR>>;

    fn remove(self) -> Self::Remove {
        self.into_cdr()
    }
}

impl<LCAR, LCDR, RCAR, RCDR> ConsAListRemove<RCAR, Right>
    for Cons<Single<Cons<LCAR, LCDR>>, Single<Cons<RCAR, RCDR>>>
{
    type Remove = Single<Cons<LCAR, LCDR>>;

    fn remove(self) -> Self::Remove {
        self.into_car()
    }
}

impl<CAR, CDR> ConsAListRemove<CAR, ()> for Single<Cons<CAR, CDR>> {
    type Remove = ();

    fn remove(self) -> Self::Remove {}
}

#[cfg(test)]
mod tests {
    use super::ConsAListRemove;
    use crate::alist;

    #[test]
    fn test_cons_alist_remove() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Float;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Integer;

        let cons_alist = alist! {
            Float => 1.0,
            Integer => 2
        };
        println!("AList: {:?}", cons_alist);

        let cons_alist = ConsAListRemove::<Float, _>::remove(cons_alist);
        println!("AList: {:?}", cons_alist);

        assert!(cons_alist == alist! { Integer => 2 });

        let cons_alist = ConsAListRemove::<Integer, _>::remove(cons_alist);
        println!("AList: {:?}", cons_alist);
    }
}
