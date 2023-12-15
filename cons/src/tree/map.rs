use crate::{
    cell::{Cons, ConsCell},
    mapper::Mapper,
    single::{Single, ConsSingle},
};

use super::{Branch, ConsTree, Leaf};

pub trait ConsTreeMap<I, M>: ConsTree<I> {
    type Map;

    fn map(self, mapper: &mut M) -> Self::Map;
}

impl<L, R, M, CAR, CDR> ConsTreeMap<Branch<L, R>, M> for Cons<CAR, CDR>
where
    CAR: ConsTreeMap<L, M>,
    CDR: ConsTreeMap<R, M>,
{
    type Map = Cons<CAR::Map, CDR::Map>;

    fn map(self, mapper: &mut M) -> Self::Map {
        let (car, cdr) = self.into_destructure();
        (car.map(mapper), cdr.map(mapper))
    }
}

impl<M, CAR> ConsTreeMap<Leaf, M> for Single<CAR>
where
    M: Mapper<CAR>,
{
    type Map = Single<M::Mapped>;

    fn map(self, mapper: &mut M) -> Self::Map {
        (mapper.run(self.into_car()),)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    struct OptionUnwrap;

    impl<T> Mapper<Option<T>> for OptionUnwrap {
        type Mapped = T;

        fn run(&mut self, t: Option<T>) -> Self::Mapped {
            t.unwrap()
        }
    }

    #[test]
    fn test_cons_tree_map() {
        let cons_tree = list![Some(1), Some(2.0), Some('3'), Some("four")];
        let mapped = cons_tree.map(&mut OptionUnwrap);
        assert!(mapped == list![1, 2.0, '3', "four"]);
    }
}
