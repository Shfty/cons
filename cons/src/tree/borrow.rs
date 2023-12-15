use std::borrow::Borrow;

use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

use super::{Branch, Leaf};

pub trait ConsTreeBorrow<'a, B, I> {
    type Borrow: Copy;

    fn borrow(&'a self) -> Self::Borrow;
}

impl<'a, L, R, LB, RB, CAR, CDR> ConsTreeBorrow<'a, Branch<LB, RB>, Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: ConsTreeBorrow<'a, L, LB>,
    CDR: ConsTreeBorrow<'a, R, RB>,
{
    type Borrow = Cons<CAR::Borrow, CDR::Borrow>;

    fn borrow(&'a self) -> Self::Borrow {
        let (car, cdr) = self.destructure();
        (car.borrow(), cdr.borrow())
    }
}

impl<'a, B, CAR> ConsTreeBorrow<'a, B, Leaf> for Single<CAR>
where
    CAR: Borrow<B>,
    B: 'a,
{
    type Borrow = Single<&'a B>;

    fn borrow(&'a self) -> Self::Borrow {
        (self.car().borrow(),)
    }
}

#[cfg(test)]
mod tests {
    use super::ConsTreeBorrow;
    use crate::list;

    #[test]
    fn test_cons_tree_borrow() {
        let cons_tree = list![1, 2.0, '3', "four"];
        let borrowed = cons_tree.borrow();
        assert!(borrowed == list![&1, &2.0, &'3', &"four"]);
    }
}
