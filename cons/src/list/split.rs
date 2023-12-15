use std::ops::Sub;

use typenum::{Bit, Sub1, UInt, UTerm, Unsigned, B1};

use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

use super::push_back::ConsListPushBack;

/// A `ConsList` that can split itself by repeatedly appending its head to another `ConsList`
pub trait ConsListSplitImpl<I, F> {
    type SplitImpl;

    fn split_impl(self, first: F) -> Self::SplitImpl;
}

impl<U, B, F, CAR, CDR> ConsListSplitImpl<UInt<U, B>, F> for Cons<Single<CAR>, CDR>
where
    U: Unsigned,
    B: Bit,
    F: ConsListPushBack<CAR>,
    UInt<U, B>: Sub<B1>,
    CDR: ConsListSplitImpl<Sub1<UInt<U, B>>, <F as ConsListPushBack<CAR>>::PushBack>,
{
    type SplitImpl = CDR::SplitImpl;

    fn split_impl(self, first: F) -> Self::SplitImpl {
        let (car, cdr) = self.into_destructure();
        cdr.split_impl(first.push_back(car.into_car()))
    }
}

impl<F, CAR, CDR> ConsListSplitImpl<UTerm, F> for Cons<Single<CAR>, CDR>
where
    F: ConsListPushBack<CAR>,
{
    type SplitImpl = Cons<<F as ConsListPushBack<CAR>>::PushBack, CDR>;

    fn split_impl(self, first: F) -> Self::SplitImpl {
        let (car, cdr) = self.into_destructure();
        (first.push_back(car.into_car()), cdr)
    }
}

/// A `ConsList` type that can split itself into two new `ConsList` types at a given type-level index
pub trait ConsListSplit<I> {
    type Split;

    fn split(self) -> Self::Split;
}

impl<T, I> ConsListSplit<I> for T
where
    T: ConsListSplitImpl<I, ()>,
{
    type Split = T::SplitImpl;

    fn split(self) -> Self::Split {
        self.split_impl(())
    }
}

pub fn split<I, C: ConsListSplit<I>>(c: C) -> C::Split {
    c.split()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_split() {
        let cons_list = list![1, 2.0, '3', "four"];
        let (car, cdr) = split::<typenum::U1, _>(cons_list);
        assert!(car == list![1, 2.0], cdr == list!['3', "four"]);
    }
}
