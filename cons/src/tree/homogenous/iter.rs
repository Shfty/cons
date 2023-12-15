use std::slice::Iter;

use super::as_slice::ConsTreeAsSlice;

/// `ConsTreeAsSlice` type that can be iterated over
pub trait ConsTreeIter<'a, T, I>: ConsTreeAsSlice<T, I> {
    fn iter(&'a self) -> Iter<T>;
}

impl<'a, C, T, I> ConsTreeIter<'a, T, I> for C
where
    C: ConsTreeAsSlice<T, I>,
    T: 'a,
{
    fn iter(&'a self) -> Iter<T> {
        self.as_slice(..).iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_tree_iter() {
        let cons_tree = list![list![1, 2, 3], list![4, 5, 6], list![7, 8, 9]];
        for item in cons_tree.iter() {
            println!("Item: {:?}", item);
        }

        assert!(cons_tree.iter().collect::<Vec<_>>() == vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
    }
}
