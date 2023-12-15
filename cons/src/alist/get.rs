use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

pub trait ConsAListGet<K, I> {
    type Get;

    fn get(self) -> Self::Get;
}

impl<K, I, CAR, CDR> ConsAListGet<K, (I,)> for Cons<CAR, CDR>
where
    CDR: ConsAListGet<K, I>,
{
    type Get = CDR::Get;

    fn get(self) -> Self::Get {
        self.into_cdr().get()
    }
}

impl<LCAR, LCDR, CDR> ConsAListGet<LCAR, ()> for Cons<Single<Cons<LCAR, LCDR>>, CDR> {
    type Get = Cons<LCAR, LCDR>;

    fn get(self) -> Self::Get {
        self.into_car().into_car()
    }
}

impl<CAR, CDR> ConsAListGet<CAR, ()> for Single<Cons<CAR, CDR>> {
    type Get = Cons<CAR, CDR>;

    fn get(self) -> Self::Get {
        self.into_car()
    }
}

#[cfg(test)]
mod tests {
    use super::ConsAListGet;
    use crate::alist;

    #[test]
    fn test_cons_alist_get() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Float;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Integer;

        let cons_alist = alist! {
            Float => 1.0,
            Integer => 2
        };

        let float = ConsAListGet::<Float, _>::get(cons_alist);
        assert!(float == (Float, 1.0));

        let int = ConsAListGet::<Integer, _>::get(cons_alist);
        assert!(int == (Integer, 2));
    }
}
