use std::ops::Add;

use typenum::Unsigned;

use crate::{cell::Cons, single::Single};

use super::{Branch, Leaf};

pub trait ConsTreeLength<I> {
    type Len: Unsigned;

    fn len(&self) -> usize {
        Self::Len::USIZE
    }

    fn is_empty(&self) -> bool {
        Self::Len::USIZE == 0
    }
}

impl<L, R, CAR, CDR> ConsTreeLength<Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: ConsTreeLength<L>,
    CDR: ConsTreeLength<R>,
    CAR::Len: Add<CDR::Len>,
    <CAR::Len as Add<CDR::Len>>::Output: Unsigned,
{
    type Len = <CAR::Len as Add<CDR::Len>>::Output;
}

impl<CAR> ConsTreeLength<Leaf> for Single<CAR> {
    type Len = typenum::U1;
}

#[cfg(test)]
mod tests {
    use super::ConsTreeLength;
    use crate::list;

    #[test]
    fn test_cons_tree_length() {
        let cons_tree = list![
            list![1, 2, 3],
            list![4, 5, 6],
            list![7, 8, 9]
        ];

        let _proof: &dyn ConsTreeLength<_, Len = _> = &cons_tree;

        let length = cons_tree.len();
        let is_empty = cons_tree.is_empty();

        assert!(length == 9);
        assert!(!is_empty);
    }
}
