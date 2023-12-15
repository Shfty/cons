use std::ops::Deref;

use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

use super::{Branch, ConsTree, Leaf};

pub trait ConsTreeDeref<'a, I>: ConsTree<I> {
    type Deref;

    fn deref(&'a self) -> Self::Deref;
}

impl<'a, L, R, CAR, CDR> ConsTreeDeref<'a, Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: ConsTreeDeref<'a, L>,
    CDR: ConsTreeDeref<'a, R>,
{
    type Deref = Cons<CAR::Deref, CDR::Deref>;

    fn deref(&'a self) -> Self::Deref {
        let (car, cdr) = self.destructure();
        (car.deref(), cdr.deref())
    }
}

impl<'a, CAR> ConsTreeDeref<'a, Leaf> for Single<CAR>
where
    CAR: Deref,
    CAR::Target: 'a,
{
    type Deref = Single<&'a CAR::Target>;

    fn deref(&'a self) -> Self::Deref {
        (self.car().deref(),)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::list;

    #[test]
    fn test_cons_tree_map() {
        let cons_tree = list![Arc::new(1), Arc::new(2.0), Arc::new('3'), Arc::new("four")];
        let mapped = cons_tree.deref();
        assert!(mapped == list![&1, &2.0, &'3', &"four"]);
    }
}
