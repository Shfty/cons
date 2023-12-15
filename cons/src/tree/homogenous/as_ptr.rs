use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

use super::{Branch, HomogenousConsTree, Leaf};

/// A `ConsTree` type containing only instances of some type `T`
pub trait ConsTreeAsPtr<T, I>: HomogenousConsTree<T, I> {
    fn as_ptr(&self) -> *const T;
}

impl<T, L, R, CAR, CDR> ConsTreeAsPtr<T, Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: ConsTreeAsPtr<T, L>,
    CDR: ConsTreeAsPtr<T, R>,
{
    fn as_ptr(&self) -> *const T {
        self.car().as_ptr()
    }
}

impl<CAR> ConsTreeAsPtr<CAR, Leaf> for Single<CAR> {
    fn as_ptr(&self) -> *const CAR {
        self.car()
    }
}

#[cfg(test)]
mod tests {
    use super::ConsTreeAsPtr;
    use crate::list;

    #[test]
    fn test_cons_tree_as_ptr() {
        let cons_tree = list![list![1, 2, 3], list![4, 5, 6], list![7, 8, 9]];
        let _proof: &dyn ConsTreeAsPtr<_, _> = &cons_tree;
        let ptr = cons_tree.as_ptr();
        println!("Ptr: {:?}", ptr);
    }
}
