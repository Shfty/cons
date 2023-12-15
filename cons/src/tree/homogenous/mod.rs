pub mod as_mut_ptr;
pub mod as_mut_slice;
pub mod as_ptr;
pub mod as_slice;
pub mod iter;
pub mod iter_mut;

use crate::{cell::Cons, single::Single};

use super::{Branch, ConsTree, Leaf};

/// A `ConsTree` type containing only instances of some type `T`
pub trait HomogenousConsTree<T, I>: ConsTree<I> {}

impl<T, L, R, CAR, CDR> HomogenousConsTree<T, Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: HomogenousConsTree<T, L>,
    CDR: HomogenousConsTree<T, R>,
{
}

impl<CAR, I> HomogenousConsTree<CAR, I> for Single<CAR> where Self: ConsTree<I> {}

#[cfg(test)]
mod tests {
    use super::HomogenousConsTree;
    use crate::list;

    #[test]
    fn test_homogenous_cons_tree() {
        let cons_tree = list![list![1, 2, 3], list![4, 5, 6], list![7, 8, 9]];

        let _proof: &dyn HomogenousConsTree<_, _> = &cons_tree;
    }
}
