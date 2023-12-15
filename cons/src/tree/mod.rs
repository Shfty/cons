use crate::{cell::Cons, single::Single};

pub mod borrow;
pub mod borrow_mut;
pub mod deref;
pub mod deref_mut;
pub mod fold;
pub mod homogenous;
pub mod length;
pub mod map;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Branch<L, R>(L, R);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Leaf;

pub trait ConsTree<I> {}

impl<L, R, CAR, CDR> ConsTree<Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: ConsTree<L>,
    CDR: ConsTree<R>,
{
}

impl<CAR> ConsTree<Leaf> for Single<CAR> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_tree() {
        let cons_list = list![1, 2.0, '3', "four"];
        let _proof: &dyn ConsTree<_> = &cons_list;

        let cons_tree = list![
            list![
                1,
                list!["foo", "bar", list![7, 8]],
                2,
                list!["foo", list![7, list![7, 8]], "baz"],
                4,
                list!["five", "six", list![7, 8]]
            ],
            "five",
            list!['3', '4', '5'],
            "four"
        ];
        let _proof: &dyn ConsTree<_> = &cons_tree;
    }
}
