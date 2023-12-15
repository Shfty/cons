//! Remove a value from a `ConsList` by type

use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

/// A `ConsList` type that can remove a value by type
pub trait ConsListRemoveItem<T, I> {
    /// The type to remove
    type RemoveItem;

    /// Remove an item by type
    fn remove_item(self) -> Self::RemoveItem;
}

impl<T, I, CAR, LCDR, RCDR> ConsListRemoveItem<T, (I,)> for Cons<Single<CAR>, Cons<LCDR, RCDR>>
where
    (LCDR, RCDR): ConsListRemoveItem<T, I>,
{
    type RemoveItem = (
        (CAR,),
        <(LCDR, RCDR) as ConsListRemoveItem<T, I>>::RemoveItem,
    );

    fn remove_item(self) -> Self::RemoveItem {
        let (car, cdr) = self.into_destructure();
        (car, cdr.remove_item())
    }
}

impl<CAR, CDR> ConsListRemoveItem<CAR, ()> for Cons<Single<CAR>, Single<CDR>> {
    type RemoveItem = (CDR,);

    fn remove_item(self) -> Self::RemoveItem {
        self.into_cdr()
    }
}

impl<CAR, CDR> ConsListRemoveItem<CDR, ((),)> for Cons<Single<CAR>, Single<CDR>> {
    type RemoveItem = (CAR,);

    fn remove_item(self) -> Self::RemoveItem {
        self.into_car()
    }
}

impl<CAR> ConsListRemoveItem<CAR, ()> for Single<CAR> {
    type RemoveItem = ();

    fn remove_item(self) -> Self::RemoveItem {}
}

/// Remove a value of type `T` from `ConsListRemoveItem` type `C`
pub fn remove_item<T, I, C: ConsListRemoveItem<T, I>>(c: C) -> C::RemoveItem {
    c.remove_item()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list::ConsList;
    use crate::list;

    #[test]
    #[allow(clippy::unit_arg)]
    fn test_cons_list_remove_item() {
        let list = list![1, 2.0, '3', "four"];
        println!("List: {:?}", list);

        let list = remove_item::<&str, _, _>(list);
        println!("string: {:?}", list);
        assert!(list == list![1, 2.0, '3']);

        let list = remove_item::<f64, _, _>(list);
        println!("f64: {:?}", list);
        assert!(list == list![1, '3']);

        let list = remove_item::<i32, _, _>(list);
        println!("i32: {:?}", list);
        assert!(list == list!['3']);

        let list = remove_item::<char, _, _>(list);
        println!("char: {:?}", list);

        assert!(list.is_empty());
    }
}
