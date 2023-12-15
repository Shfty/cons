pub mod get;
pub mod split;

use typenum::Unsigned;

use crate::{
    cell::Cons,
    list::ConsList,
    index::{Here, There},
};

/// A `ConsList` type containing other `ConsLists` of length `L`
pub trait ConsArray<L: Unsigned, I> {
    type InnerLen: Unsigned;
}

impl<L, I, CAR, CDR> ConsArray<L, There<I>> for Cons<CAR, CDR>
where
    L: Unsigned,
    Self: ConsList,
    CAR: ConsList<Len = L>,
    CDR: ConsArray<L, I, InnerLen = L>,
{
    type InnerLen = L;
}

impl<L, CAR, CDR> ConsArray<L, Here> for Cons<CAR, CDR>
where
    Self: ConsList,
    L: Unsigned,
    CAR: ConsList<Len = L>,
    CDR: ConsList<Len = L>,
{
    type InnerLen = L;
}

#[cfg(test)]
mod tests {
    use super::ConsArray;
    use crate::list;

    #[test]
    fn test_cons_array() {
        println!();

        let cons_array = list![
            list![1, 2, 3, 4],
            list![4.0, 5.0, 6.0, 7.0],
            list!['7', '8', '9', '0']
        ];

        let _proof: &dyn ConsArray<_, _, InnerLen = _> = &cons_array;
    }
}
