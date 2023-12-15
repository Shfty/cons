//! Set a value in a `ConsList` by type-level index

use std::ops::Sub;

use typenum::{Sub1, UInt, UTerm, Unsigned, B1};

use crate::cell::{Cons, ConsCell};

/// A `ConsList` type that can set a value by type-level index `I`
pub trait ConsListRemove<I, T> {
    /// The type at index `I`
    type Remove;

    /// Set value at index `I` by value
    fn remove(self) -> Self::Remove;
}

impl<U, B, T, CAR, CDR> ConsListRemove<UInt<U, B>, T> for Cons<CAR, CDR>
where
    UInt<U, B>: Sub<B1>,
    Sub1<UInt<U, B>>: Unsigned,
    CDR: ConsListRemove<Sub1<UInt<U, B>>, T>,
{
    type Remove = Cons<CAR, CDR::Remove>;

    fn remove(self) -> Self::Remove {
        let (car, cdr) = self.into_destructure();
        (car, cdr.remove())
    }
}

impl<CAR, CDR> ConsListRemove<UTerm, CAR> for Cons<CAR, CDR> {
    type Remove = CDR;

    fn remove(self) -> Self::Remove {
        self.1
    }
}

/// Set value at type-level index `I` in `ConsListIndex` type `C` by immutable reference
pub fn remove<I: Unsigned, T, C: ConsListRemove<I, T>>(c: C) -> C::Remove {
    c.remove()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_remove() {
        let list = list![1, 2.0, '3', "four"];
        println!("List: {:?}", list);

        let list = remove::<typenum::U0, _, _>(list);
        assert!(list == list![2.0, '3', "four"]);

        let list = remove::<typenum::U1, _, _>(list);
        assert!(list == list![2.0, "four"]);

        println!("List: {:?}", list);
    }
}
