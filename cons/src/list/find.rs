//! Fetch a value from a `ConsList` by type

use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

/// A `ConsList` type that can fetch a value by type
pub trait ConsListFind<T, I> {
    /// The type to find
    type Find;

    /// Fetch an immutable reference to value of type `T`
    fn find(self) -> Self::Find;
}

impl<T, I, CAR, CDR> ConsListFind<T, (I,)> for Cons<Single<CAR>, CDR>
where
    CDR: ConsListFind<T, I>,
{
    type Find = CDR::Find;

    fn find(self) -> Self::Find {
        self.into_cdr().find()
    }
}

impl<CAR, CDR> ConsListFind<CAR, ()> for Cons<Single<CAR>, CDR> {
    type Find = CAR;

    fn find(self) -> Self::Find {
        self.into_car().into_car()
    }
}

impl<CAR> ConsListFind<CAR, ()> for Single<CAR> {
    type Find = CAR;

    fn find(self) -> Self::Find {
        self.into_car()
    }
}

/// Fetch an immutable reference to value of type `T` from `ConsListFind` type `C`
pub fn find<T, I, C: ConsListFind<T, I>>(c: C) -> C::Find {
    c.find()
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_find() {
        let list = list![1, 2.0, '3', "four"];
        println!("List: {:?}", list);

        let int = find::<i32, _, _>(list);
        println!("i32: {:?}", int);
        assert!(int == 1);

        let float = find::<f64, _, _>(list);
        println!("f64: {:?}", float);
        assert!(float.partial_cmp(&2.0) == Some(Ordering::Equal));

        let character = find::<char, _, _>(list);
        println!("char: {:?}", character);
        assert!(character == '3');

        let string = find::<&str, _, _>(list);
        println!("string: {:?}", string);
        assert!(string == "four");
    }
}
