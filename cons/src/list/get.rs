//! Get a value from a `ConsList` by type-level index

use std::ops::Sub;

use typenum::{Sub1, UInt, UTerm, Unsigned, B1};

use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

/// A `ConsList` type that can get a value by type-level index `I`
pub trait ConsListGet<I> {
    /// The type at index `I`
    type Get;

    /// Get value at index `I`
    fn get(&self) -> &Self::Get;
}

/// `index > 0`
impl<U, B, CAR, CDR> ConsListGet<UInt<U, B>> for Cons<Single<CAR>, CDR>
where
    UInt<U, B>: Sub<B1>,
    Sub1<UInt<U, B>>: Unsigned,
    CDR: ConsListGet<Sub1<UInt<U, B>>>,
{
    type Get = CDR::Get;

    fn get(&self) -> &Self::Get {
        self.cdr().get()
    }
}

/// `index == 0`
impl<CAR, CDR> ConsListGet<UTerm> for Cons<Single<CAR>, CDR> {
    type Get = CAR;

    fn get(&self) -> &Self::Get {
        self.car().car()
    }
}

/// `index == 0`
impl<CAR> ConsListGet<UTerm> for Single<CAR> {
    type Get = CAR;

    fn get(&self) -> &Self::Get {
        self.car()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_get() {
        let list = list![1, 2.0, '3', "four"];
        println!("List: {:?}", list);

        let index_0 = ConsListGet::<typenum::U0>::get(&list);
        assert!(*index_0 == 1);

        let index_1 = ConsListGet::<typenum::U1>::get(&list);
        assert!(index_1.partial_cmp(&2.0) == Some(Ordering::Equal));

        let index_2 = ConsListGet::<typenum::U2>::get(&list);
        assert!(*index_2 == '3');

        let index_3 = ConsListGet::<typenum::U3>::get(&list);
        assert!(*index_3 == "four");

        println!(
            "Indices:\n0: {:?},\n1: {:?},\n2: {:?},\n3: {:?}",
            index_0, index_1, index_2, index_3
        );
    }
}
