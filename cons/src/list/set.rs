//! Set a value in a `ConsList` by type-level index

use std::ops::Sub;

use typenum::{Sub1, UInt, UTerm, Unsigned, B1};

use crate::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

/// A `ConsList` type that can set a value by type-level index `I`
pub trait ConsListSet<I, T> {
    /// Set value at index `I` by value
    fn set(&mut self, value: T);
}

impl<U, B, T, CAR, CDR> ConsListSet<UInt<U, B>, T> for Cons<Single<CAR>, CDR>
where
    UInt<U, B>: Sub<B1>,
    Sub1<UInt<U, B>>: Unsigned,
    CDR: ConsListSet<Sub1<UInt<U, B>>, T>,
{
    fn set(&mut self, value: T) {
        self.cdr_mut().set(value)
    }
}

impl<CAR, CDR> ConsListSet<UTerm, CAR> for Cons<Single<CAR>, CDR> {
    fn set(&mut self, value: CAR) {
        *self.car_mut().car_mut() = value;
    }
}

impl<T> ConsListSet<UTerm, T> for Single<T> {
    fn set(&mut self, value: T) {
        *self.car_mut() = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_set() {
        let mut list = list![1, 2.0, '3', "four"];
        println!("List: {:?}", list);

        ConsListSet::<typenum::U0, _>::set(&mut list, 2);
        assert!(list == list![2, 2.0, '3', "four"]);
        ConsListSet::<typenum::U1, _>::set(&mut list, 3.0);
        assert!(list == list![2, 3.0, '3', "four"]);
        ConsListSet::<typenum::U2, _>::set(&mut list, '4');
        assert!(list == list![2, 3.0, '4', "four"]);
        ConsListSet::<typenum::U3, _>::set(&mut list, "five");
        assert!(list == list![2, 3.0, '4', "five"]);

        println!("List: {:?}", list);

        ConsListSet::<typenum::U2, _>::set(&mut list, '8');
        println!("List: {:?}", list);
        assert!(list == list![2, 3.0, '8', "five"]);
    }
}
