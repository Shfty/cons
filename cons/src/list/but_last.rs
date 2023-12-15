//! Fetch all but the last _n_ elements of a `ConsList`

use std::ops::Sub;

use typenum::Unsigned;

use super::{first::ConsListFirst, ConsList};

/// A `ConsList` type that can fetch all but its last _n_ elements
pub trait ConsListButLast<I: Unsigned> {
    /// `ConsList` of immutable references to the last `I` elements
    type ButLast;

    /// Fetch all but the last `I` values by immutable reference
    fn but_last(self) -> Self::ButLast;
}

impl<I, T> ConsListButLast<I> for T
where
    I: Unsigned,
    T: ConsList,
    <T as ConsList>::Len: Sub<I>,
    <<T as ConsList>::Len as Sub<I>>::Output: Unsigned,
    T: ConsListFirst<<<T as ConsList>::Len as Sub<I>>::Output>,
{
    type ButLast = T::First;

    fn but_last(self) -> Self::ButLast {
        self.first()
    }
}

/// Fetch all but the last `I` values of `ConsListButLast` type `C` by immutable reference
pub fn but_last<I: Unsigned, C: ConsListButLast<I>>(c: C) -> C::ButLast {
    c.but_last()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_but_last() {
        let cons_list = list![1, 2.0, '3', "four"];
        println!("List: {:?}", cons_list);

        let list_but_last = but_last::<typenum::U0, _>(cons_list);
        println!("Zero: {:?}", list_but_last);
        assert!(list_but_last == cons_list);

        let list_but_last = but_last::<typenum::U1, _>(cons_list);
        println!("One: {:?}", list_but_last);
        assert!(list_but_last == list![1, 2.0, '3']);

        let list_but_last = but_last::<typenum::U2, _>(cons_list);
        println!("Two: {:?}", list_but_last);
        assert!(list_but_last == list![1, 2.0]);

        let list_but_last = but_last::<typenum::U3, _>(cons_list);
        println!("Three: {:?}", list_but_last);
        assert!(list_but_last == list![1]);
    }
}
