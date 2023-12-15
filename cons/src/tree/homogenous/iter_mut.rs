use std::slice::IterMut;

use super::as_mut_slice::ConsTreeAsMutSlice;

/// `ConsTreeAsSlice` type that can be iterated over
pub trait ConsTreeIterMut<'a, T, I>: ConsTreeAsMutSlice<T, I> {
    fn iter_mut(&'a mut self) -> IterMut<T>;
}

impl<'a, C, T, I> ConsTreeIterMut<'a, T, I> for C
where
    C: ConsTreeAsMutSlice<T, I>,
    T: 'a,
{
    fn iter_mut(&'a mut self) -> IterMut<T> {
        self.as_mut_slice(..).iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_tree_iter_mut() {
        let mut cons_tree = list![list![1, 2, 3], list![4, 5, 6], list![7, 8, 9]];
        for item in cons_tree.iter_mut() {
            println!("Item: {:?}", item);
        }

        assert!(cons_tree.iter_mut().collect::<Vec<_>>() == vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
    }
}
