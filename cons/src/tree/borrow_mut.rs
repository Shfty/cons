use std::borrow::BorrowMut;

use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

use super::{Branch, Leaf};

pub trait ConsTreeBorrowMut<'a, I, B> {
    type BorrowMut;

    fn borrow_mut(&'a mut self) -> Self::BorrowMut;
}

impl<'a, L, R, LB, RB, CAR, CDR> ConsTreeBorrowMut<'a, Branch<L, R>, Branch<LB, RB>>
    for Cons<CAR, CDR>
where
    CAR: ConsTreeBorrowMut<'a, L, LB>,
    CDR: ConsTreeBorrowMut<'a, R, RB>,
{
    type BorrowMut = Cons<CAR::BorrowMut, CDR::BorrowMut>;

    fn borrow_mut(&'a mut self) -> Self::BorrowMut {
        let (car, cdr) = self.destructure_mut();
        (car.borrow_mut(), cdr.borrow_mut())
    }
}

impl<'a, B, CAR> ConsTreeBorrowMut<'a, Leaf, B> for Single<CAR>
where
    CAR: BorrowMut<B>,
    B: 'a,
{
    type BorrowMut = Single<&'a mut B>;

    fn borrow_mut(&'a mut self) -> Self::BorrowMut {
        (self.car_mut().borrow_mut(),)
    }
}

#[cfg(test)]
mod tests {
    use super::ConsTreeBorrowMut;
    use crate::list;

    #[test]
    fn test_cons_tree_borrow() {
        let mut cons_tree = list![1, 2.0, '3', "four"];
        let borrowed = cons_tree.borrow_mut();
        assert!(borrowed == list![&mut 1, &mut 2.0, &mut '3', &mut "four"]);
    }
}
