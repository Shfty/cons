use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

use super::{Branch, HomogenousConsTree, Leaf};

/// A `ConsTree` type containing only instances of some type `T`
pub trait ConsTreeAsMutPtr<T, I>: HomogenousConsTree<T, I> {
    fn as_mut_ptr(&mut self) -> *mut T;
}

impl<T, L, R, CAR, CDR> ConsTreeAsMutPtr<T, Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: ConsTreeAsMutPtr<T, L>,
    CDR: ConsTreeAsMutPtr<T, R>,
{
    fn as_mut_ptr(&mut self) -> *mut T {
        self.car_mut().as_mut_ptr()
    }
}

impl<CAR> ConsTreeAsMutPtr<CAR, Leaf> for Single<CAR> {
    fn as_mut_ptr(&mut self) -> *mut CAR {
        self.car_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::ConsTreeAsMutPtr;
    use crate::list;

    #[test]
    fn test_cons_tree_as_mut_ptr() {
        let mut cons_tree = list![list![1, 2, 3], list![4, 5, 6], list![7, 8, 9]];
        let _proof: &dyn ConsTreeAsMutPtr<_, _> = &cons_tree;
        let ptr = cons_tree.as_mut_ptr();
        println!("Ptr: {:?}", ptr);
    }
}
