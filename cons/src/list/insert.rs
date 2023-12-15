//! Set a value in a `ConsList` by type-level index

use std::ops::Sub;

use typenum::{Sub1, UInt, UTerm, Unsigned, B1};

use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

/// A `ConsList` type that can insert a value by type-level index `I`
pub trait ConsListInsert<I, T> {
    /// The type at index `I`
    type Insert;

    /// Set value at index `I` by value
    fn insert(self, value: T) -> Self::Insert;
}

impl<U, B, T, CAR, CDR> ConsListInsert<UInt<U, B>, T> for Cons<Single<CAR>, CDR>
where
    UInt<U, B>: Sub<B1>,
    Sub1<UInt<U, B>>: Unsigned,
    CDR: ConsListInsert<Sub1<UInt<U, B>>, T>,
{
    type Insert = ((CAR,), CDR::Insert);

    fn insert(self, value: T) -> Self::Insert {
        let (car, cdr) = self.into_destructure();
        (car, cdr.insert(value))
    }
}

impl<T, CAR, CDR> ConsListInsert<UTerm, T> for ((CAR,), CDR) {
    type Insert = ((T,), Self);

    fn insert(self, value: T) -> Self::Insert {
        ((value,), self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_insert() {
        let list = list![1, 2.0, '3', "four"];
        println!("List: {:?}", list);

        let list = ConsListInsert::<typenum::U0, _>::insert(list, 2);
        assert!(list == list![2, 1, 2.0, '3', "four"]);

        let list = ConsListInsert::<typenum::U1, _>::insert(list, 3.0);
        assert!(list == list![2, 3.0, 1, 2.0, '3', "four"]);

        let list = ConsListInsert::<typenum::U2, _>::insert(list, '4');
        assert!(list == list![2, 3.0, '4', 1, 2.0, '3', "four"]);

        let list = ConsListInsert::<typenum::U3, _>::insert(list, "five");
        assert!(list == list![2, 3.0, '4', "five", 1, 2.0, '3', "four"]);

        println!("List: {:?}", list);

        let list = ConsListInsert::<typenum::U2, _>::insert(list, '8');
        println!("List: {:?}", list);
        assert!(list == list![2, 3.0, '8', '4', "five", 1, 2.0, '3', "four"]);
    }
}
