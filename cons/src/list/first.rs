//! Fetch entries before index _n_

use std::ops::Sub;

use typenum::{Sub1, UInt, Unsigned, B1};

use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

/// A `ConsList` that can fetch its first _n_ entries
pub trait ConsListFirst<I> {
    /// Immutable references to the first `I` entries
    type First;

    /// Fetch the first `I` entries
    fn first(self) -> Self::First;
}

/// `len > 1, idx > 0`
impl<U, BA, BB, CAR, CDR> ConsListFirst<UInt<UInt<U, BA>, BB>> for Cons<Single<CAR>, CDR>
where
    U: Unsigned,
    UInt<UInt<U, BA>, BB>: Sub<B1>,
    CDR: ConsListFirst<Sub1<UInt<UInt<U, BA>, BB>>>,
{
    type First = ((CAR,), CDR::First);

    fn first(self) -> Self::First {
        let (car, cdr) = self.into_destructure();
        (car, cdr.first())
    }
}

/// `len > 1, idx > 0`
impl<CAR, CDR> ConsListFirst<typenum::U1> for Cons<Single<CAR>, CDR> {
    type First = (CAR,);

    fn first(self) -> Self::First {
        self.into_car()
    }
}

/// `len == 1, idx == 1`
impl<CAR> ConsListFirst<typenum::U1> for Single<CAR> {
    type First = (CAR,);

    fn first(self) -> Self::First {
        self
    }
}

/// Fetch immutable references to the first `I` entries of `ConsListFirst` type `C`
pub fn first<I: Unsigned, C: ConsListFirst<I>>(c: C) -> C::First {
    c.first()
}

#[cfg(test)]
mod tests {
    use crate::list;

    use super::*;

    #[test]
    fn test_cons_list_first() {
        let cons_list = list![1, 2.0, '3', "four"];

        let list_first = first::<typenum::U1, _>(cons_list);
        println!("One: {:?}", list_first);
        assert!(list_first == list![1]);

        let list_first = first::<typenum::U2, _>(cons_list);
        println!("Two: {:?}", list_first);
        assert!(list_first == list![1, 2.0]);

        let list_first = first::<typenum::U3, _>(cons_list);
        println!("Three: {:?}", list_first);
        assert!(list_first == list![1, 2.0, '3']);

        let list_first = first::<typenum::U4, _>(cons_list);
        println!("Four: {:?}", list_first);
        assert!(list_first == list![1, 2.0, '3', "four"]);
    }
}
