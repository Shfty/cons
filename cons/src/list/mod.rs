//! Machinery for handling cons lists

use std::ops::Add;

use typenum::{Add1, Unsigned, B1};

use crate::{cell::Cons, single::Single};

pub mod append;
pub mod but_last;
pub mod find;
pub mod first;
pub mod generic;
pub mod get;
pub mod collect;
pub mod insert;
pub mod last;
pub mod push_back;
pub mod remove;
pub mod remove_item;
pub mod rest;
pub mod reverse;
pub mod set;
pub mod split;
pub mod split_last;
pub mod pluck;
pub mod sculpt;
pub mod signature;
pub mod as_ref;
pub mod as_mut;
pub mod deref;
pub mod deref_mut;
pub mod option;

#[cfg(feature = "futures")]
pub mod futures;

/// A singly-linked list of nested cons cells
pub trait ConsList {
    /// Type-level list length
    type Len: Unsigned;

    /// List length
    const LEN: usize;

    /// List emptiness
    const IS_EMPTY: bool;

    /// `LEN` getter for type instances
    fn len(&self) -> usize {
        Self::LEN
    }

    /// `IS_EMPTY` getter for type instances
    fn is_empty(&self) -> bool {
        Self::IS_EMPTY
    }
}

/// `len > 1`
impl<CAR, CDR> ConsList for Cons<CAR, CDR>
where
    CDR: ConsList,
    CDR::Len: Add<B1>,
    Add1<CDR::Len>: Unsigned,
{
    type Len = Add1<CDR::Len>;
    const LEN: usize = Self::Len::USIZE;
    const IS_EMPTY: bool = false;
}

/// `len == 1`
impl<CAR> ConsList for Single<CAR> {
    type Len = typenum::U1;
    const LEN: usize = 1;
    const IS_EMPTY: bool = false;
}

/// `len == 0`
impl ConsList for () {
    type Len = typenum::U0;
    const LEN: usize = 0;
    const IS_EMPTY: bool = true;
}

/// Create a `ConsList` value from a comma-separated set of expressions
#[macro_export]
macro_rules ! list {
    () => {
        ()
    };
    (list![$($t:tt)*] $(,)?) => {
        list![$($t)*]
    };
    (list![$($t:tt)*], $($cdr:tt)*) => {
        (
            list![$($t)*],
            list![$($cdr)*]
        )
    };
    () => {};
    ($car:expr $(,)?) => {
        ($car,)
    };
    ($car:expr, $($cdr:tt)*) => {
        (
            ($car,),
            $crate::list![$($cdr)*]
        )
    };
}

/// Create a `ConsList` type from a comma-separated set of types
#[macro_export]
macro_rules ! List {
    (List![$($t:tt)*]) => {
        List![$($t)*]
    };
    (List![$($t:tt)*], $($cdr:tt)*) => {
        (
            List![$($t)*],
            List![$($cdr)*]
        )
    };
    () => {};
    ($car:ty) => {
        ($car,)
    };
    ($car:ty, $($cdr:tt)*) => {
        (
            ($car,),
            $crate::List![$($cdr)*]
        )
    };
}

/// Create a `ConsList` value from a comma-separated set of expressions
#[macro_export]
macro_rules ! unlist {
    ($list:ident => $id:pat) => {
        let $id = $crate::single::ConsSingle::into_car($list);
    };
    ($list:ident => $id:pat, $($tt:tt)*) => {
        let ($id, $list) = $crate::cell::ConsCell::into_destructure($list);
        $crate::unlist!($list => $($tt)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cons_list() {
        type ListType = List![i32, f64, char, &'static str];
        let len = <ListType as ConsList>::Len::USIZE;
        println!("Len: {:?}", len);
        assert!(len == 4);
        let is_empty = ListType::IS_EMPTY;
        assert!(!is_empty);

        let cons_list = list![1, 2.0, '3', "four"];
        let len = cons_list.len();
        assert!(len == 4);
        let is_empty = cons_list.is_empty();
        assert!(!is_empty)
    }
}
