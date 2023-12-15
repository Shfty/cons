//! Reverse the order of a `ConsList`

use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

/// A `ConsList` that can be reversed by repeatedly popping its head into another `ConsList`
pub trait ConsListReverseImpl<R> {
    type Reverse;

    fn reverse_impl(self, r: R) -> Self::Reverse;
}

impl<R, CAR, CDR> ConsListReverseImpl<R> for Cons<Single<CAR>, CDR>
where
    CDR: ConsListReverseImpl<Cons<Single<CAR>, R>>,
{
    type Reverse = CDR::Reverse;

    fn reverse_impl(self, r: R) -> Self::Reverse {
        let (car, cdr) = self.into_destructure();
        cdr.reverse_impl((car, r))
    }
}

impl<R, CAR> ConsListReverseImpl<R> for Single<CAR> {
    type Reverse = Cons<Single<CAR>, R>;

    fn reverse_impl(self, r: R) -> Self::Reverse {
        (self, r)
    }
}

/// A `ConsList` that can have its elements reversed
pub trait ConsListReverse {
    type Reverse;

    fn reverse(self) -> Self::Reverse;
}

impl<CAR, CDR> ConsListReverse for ((CAR,), CDR)
where
    CDR: ConsListReverseImpl<(CAR,)>,
{
    type Reverse = CDR::Reverse;

    fn reverse(self) -> Self::Reverse {
        let (car, cdr) = self;
        cdr.reverse_impl(car)
    }
}

pub fn reverse<C: ConsListReverse>(c: C) -> C::Reverse {
    c.reverse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_reverse() {
        let cons_list = list![1, 2.0, '3', "four"];
        let cons_list = cons_list.reverse();
        assert!(cons_list == list!["four", '3', 2.0, 1]);
    }
}
