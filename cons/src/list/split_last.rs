use std::ops::{Add, Sub};

use typenum::{Add1, B1};

use super::{split::ConsListSplit, ConsList};

pub trait ConsListSplitLast<I>: ConsList {
    type SplitLast;

    fn split_last(self) -> Self::SplitLast;
}

impl<I, T> ConsListSplitLast<I> for T
where
    I: Add<B1>,
    <T as ConsList>::Len: Sub<Add1<I>>,
    T: ConsList,
    T: ConsListSplit<<<T as ConsList>::Len as Sub<Add1<I>>>::Output>,
{
    type SplitLast = T::Split;

    fn split_last(self) -> Self::SplitLast {
        self.split()
    }
}

pub fn split_last<I, C: ConsListSplitLast<I>>(c: C) -> C::SplitLast {
    c.split_last()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_split_last() {
        let cons_list = list![1, 2.0, '3', "four"];
        let (car, cdr) = split_last::<typenum::U2, _>(cons_list);
        println!("CAR: {:?}, CDR: {:?}", car, cdr);
        assert!(car == list![1, 2.0], cdr == list!['3', "four"]);
    }
}
