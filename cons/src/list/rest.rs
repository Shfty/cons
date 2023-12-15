//! Fetch entries after index _n_

use std::ops::Sub;

use typenum::{Sub1, UInt, UTerm, Unsigned, B1};

use crate::{cell::Cons, single::Single};

/// A `ConsList` that can fetch entries after a given index
pub trait ConsListRest<I> {
    /// Entries after index `I`
    type Rest;

    /// Fetch the entries after index `I`
    fn rest(self) -> Self::Rest;
}

/// `index > 0`
impl<U, B, CAR, CDR> ConsListRest<UInt<U, B>> for Cons<CAR, CDR>
where
    UInt<U, B>: Sub<B1>,
    Sub1<UInt<U, B>>: Unsigned,
    CDR: ConsListRest<Sub1<UInt<U, B>>>,
{
    type Rest = CDR::Rest;

    fn rest(self) -> Self::Rest {
        self.1.rest()
    }
}

/// `index == 0`
impl<CAR, CDR> ConsListRest<UTerm> for Cons<CAR, CDR> {
    type Rest = Self;

    fn rest(self) -> Self::Rest {
        self
    }
}

/// `index == 0`
impl<CAR> ConsListRest<UTerm> for Single<CAR> {
    type Rest = Self;

    fn rest(self) -> Self::Rest {
        self
    }
}

/// Fetch immutable references to the entries after index `I` in `ConsListRest` type `C`
pub fn rest<I: Unsigned, C: ConsListRest<I>>(c: C) -> C::Rest {
    c.rest()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_rest() {
        let cons_list = list![1, 2.0, '3', "four"];
        println!("List: {:?}", cons_list);

        let list_rest = rest::<typenum::U0, _>(cons_list);
        println!("Zero: {:?}", list_rest);
        assert!(list_rest == list![1, 2.0, '3', "four"]);

        let list_rest = rest::<typenum::U1, _>(cons_list);
        println!("One: {:?}", list_rest);
        assert!(list_rest == list![2.0, '3', "four"]);

        let list_rest = rest::<typenum::U2, _>(cons_list);
        println!("Two: {:?}", list_rest);
        assert!(list_rest == list!['3', "four"]);

        let list_rest = rest::<typenum::U3, _>(cons_list);
        println!("Three: {:?}", list_rest);
        assert!(list_rest == list!["four"]);
    }
}
