use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

pub trait ConsAListSet<K, V, I> {
    type Set;

    fn set(self, v: V) -> Self::Set;
}

impl<K, V, I, CAR, CDR> ConsAListSet<K, V, (I,)> for Cons<CAR, CDR>
where
    CDR: ConsAListSet<K, V, I>,
{
    type Set = Cons<CAR, CDR::Set>;

    fn set(self, v: V) -> Self::Set {
        let (car, cdr) = self.into_destructure();
        (car, cdr.set(v))
    }
}

impl<V, LCAR, LCDR, CDR> ConsAListSet<LCAR, V, ()> for Cons<Single<Cons<LCAR, LCDR>>, CDR> {
    type Set = Cons<Single<Cons<LCAR, V>>, CDR>;

    fn set(self, v: V) -> Self::Set {
        let (car, cdr) = self.into_destructure();
        (((car.into_car().into_car(), v),), cdr)
    }
}

impl<V, CAR, CDR> ConsAListSet<CAR, V, ()> for Single<Cons<CAR, CDR>> {
    type Set = Single<Cons<CAR, V>>;

    fn set(self, v: V) -> Self::Set {
        ((self.into_car().into_car(), v),)
    }
}

#[cfg(test)]
mod tests {
    use super::ConsAListSet;
    use crate::alist;

    #[test]
    fn test_cons_alist_set() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Float;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Integer;

        let cons_alist = alist! {
            Float => 1.0,
            Integer => 2
        };

        println!("AList: {:?}", cons_alist);

        let cons_alist = ConsAListSet::<Float, _, _>::set(cons_alist, 2.0);
        println!("AList: {:?}", cons_alist);

        let cons_alist = ConsAListSet::<Integer, _, _>::set(cons_alist, 3);
        println!("AList: {:?}", cons_alist);

        assert!(cons_alist == alist! { Float => 2.0, Integer => 3 });
    }
}
