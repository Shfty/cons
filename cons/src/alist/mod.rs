pub mod borrow;
pub mod generic;
pub mod get;
pub mod insert;
pub mod remove;
pub mod set;

use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

/// An association list of nested cons cells
pub trait ConsAList {}

impl<LCAR, LCDR, CDR> ConsAList for Cons<Single<Cons<LCAR, LCDR>>, CDR> where CDR: ConsAList {}

impl<CAR> ConsAList for Single<CAR> where CAR: ConsCell {}

/// Create a `ConsAList` type from a set of key-value pairs
#[macro_export]
macro_rules! alist {
    () => {
        $crate::list![]
    };
    (
        $(
            $key:expr => $value:expr
        ),*$(,)?
    ) => {
        $crate::list![$(($key, $value)),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alist;

    #[test]
    fn test_cons_alist() {
        let cons_alist = alist! {
            0 => 1.0,
            '2' => "three",
        };
        let _proof: &dyn ConsAList = &cons_alist;
    }
}
