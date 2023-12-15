use std::ops::RangeBounds;

use crate::tree::length::ConsTreeLength;
use typenum::Unsigned;

use super::as_mut_ptr::ConsTreeAsMutPtr;

/// A `HomogenousConsTree` type that can be accessed by immutable slice
pub trait ConsTreeAsMutSlice<T, I>: ConsTreeAsMutPtr<T, I> {
    fn as_mut_slice<R: RangeBounds<usize>>(&mut self, range: R) -> &mut [T];
}

impl<T, I, CDR> ConsTreeAsMutSlice<T, I> for CDR
where
    CDR: ConsTreeAsMutPtr<T, I> + ConsTreeLength<I>,
{
    fn as_mut_slice<R: RangeBounds<usize>>(&mut self, range: R) -> &mut [T] {
        let ofs = match range.start_bound() {
            std::ops::Bound::Included(start) => *start,
            std::ops::Bound::Excluded(start) => *start + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let len = match range.end_bound() {
            std::ops::Bound::Included(end) => *end + 1 - ofs,
            std::ops::Bound::Excluded(end) => *end - ofs,
            std::ops::Bound::Unbounded => CDR::Len::USIZE - ofs,
        };

        assert!(ofs + len <= CDR::Len::USIZE, "Range is out of bounds");

        let ptr = self.as_mut_ptr();
        unsafe {
            let ptr = ptr.add(ofs);
            std::slice::from_raw_parts_mut(ptr, len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_tree_as_mut_slice() {
        println!();

        let mut cons_tree = list![list![1, 2, 3], list![4, 5, 6], list![7, 8, 9]];

        let slice = cons_tree.as_mut_slice(0..9);
        println!("Slice 0..9: {:?}", slice);

        assert!(
            cons_tree
                .as_mut_slice(..)
                .iter()
                .copied()
                .collect::<Vec<i32>>()
                == vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        )
    }
}
