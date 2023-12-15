use cons::{
    array::ConsArray,
    cell::Cons,
    single::Single,
    tree::{Branch, Leaf},
};
use typenum::Unsigned;

use super::CharImage;

/// A `ConsTree` of `CharImage` types
pub trait CharImageConsTree<I> {}

impl<'a, L, R, CAR, CDR> CharImageConsTree<Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: CharImageConsTree<L>,
    CDR: CharImageConsTree<R>,
{
}

impl<'a, CAR> CharImageConsTree<Leaf> for Single<CAR> where CAR: CharImage {}

/// A `ConsArray` of `CharImage` types
pub trait CharImageConsArray<TI, L: Unsigned, AI>:
    CharImageConsTree<TI> + ConsArray<L, AI>
{
}

impl<TI, L: Unsigned, AI, T> CharImageConsArray<TI, L, AI> for T where
    T: CharImageConsTree<TI> + ConsArray<L, AI>
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char_image::text::TextLine;
    use cons::list;

    #[test]
    fn test_char_image_cons_tree() {
        let cons_tree = list![
            TextLine::new("One"),
            TextLine::new("Two"),
            TextLine::new("Three")
        ];
        let _proof: &dyn CharImageConsTree<_> = &cons_tree;
    }

    #[test]
    fn test_char_image_cons_array() {
        let cons_tree = list![
            list![
                TextLine::new("One"),
                TextLine::new("Two"),
                TextLine::new("Three")
            ],
            list![
                TextLine::new("Four"),
                TextLine::new("Five"),
                TextLine::new("Six")
            ],
            list![
                TextLine::new("Seven"),
                TextLine::new("Eight"),
                TextLine::new("Nine")
            ],
        ];
        let _proof: &dyn CharImageConsArray<_, _, _, InnerLen = _> = &cons_tree;
    }
}
