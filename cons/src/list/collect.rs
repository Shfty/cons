use crate::{
    cell::Cons,
    tree::{Branch, Leaf},
    single::Single,
};

pub trait CollectConsList<C, I>: Iterator {
    fn collect_list(self) -> C;
}

impl<T, CDR, R, It> CollectConsList<Cons<Single<T>, CDR>, Branch<Leaf, R>> for It
where
    It: Iterator<Item = T> + CollectConsList<CDR, R>,
{
    fn collect_list(mut self) -> Cons<Single<T>, CDR> {
        let car = (self.next().unwrap(),);
        (car, CollectConsList::<CDR, R>::collect_list(self))
    }
}

impl<T, It> CollectConsList<Single<T>, Leaf> for It
where
    It: Iterator<Item = T>,
{
    fn collect_list(mut self) -> Single<T> {
        (self.next().unwrap(),)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{list, List};

    #[test]
    fn test_cons_list_collect() {
        let iter = vec![5, 4, 3, 2, 1].into_iter();
        let list: List![i32, i32, i32, i32] = iter.collect_list();
        println!("List: {:?}", list);
        assert!(list == list![5, 4, 3, 2]);
    }
}
