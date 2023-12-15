//! Fetch the last _n_ entries of a `ConsList`

use std::ops::Sub;

use typenum::Unsigned;

use super::{rest::ConsListRest, ConsList};

/// A `ConsList` type that can fetch its last _n_ entries
pub trait ConsListLast<I: Unsigned> {
    /// The last `I` entries
    type Last;

    /// Fetch the last `I` entries
    fn last(self) -> Self::Last;
}

impl<I, T> ConsListLast<I> for T
where
    I: Unsigned,
    T: ConsList,
    <T as ConsList>::Len: Sub<I>,
    T: ConsListRest<<<T as ConsList>::Len as Sub<I>>::Output>,
{
    type Last = T::Rest;

    fn last(self) -> Self::Last {
        self.rest()
    }
}

/// Fetch immutable references to the last `I` entries of `ConsListLast` type `C`
pub fn last<I: Unsigned, C: ConsListLast<I>>(c: C) -> C::Last {
    c.last()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_last() {
        let cons_list = list![1, 2.0, '3', "four"];
        println!("List: {:?}", cons_list);

        let list_last = last::<typenum::U1, _>(cons_list);
        println!("One: {:?}", list_last);
        assert!(list_last == list!["four"]);

        let list_last = last::<typenum::U2, _>(cons_list);
        println!("Two: {:?}", list_last);
        assert!(list_last == list!['3', "four"]);

        let list_last = last::<typenum::U3, _>(cons_list);
        println!("Three: {:?}", list_last);
        assert!(list_last == list![2.0, '3', "four"]);

        let list_last = last::<typenum::U4, _>(cons_list);
        println!("Four: {:?}", list_last);
        assert!(list_last == list![1, 2.0, '3', "four"]);
    }
}
