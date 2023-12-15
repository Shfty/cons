use crate::{cell::Cons, single::Single};

pub mod generic;
pub mod get;
pub mod set;
pub mod insert;
pub mod remove;

/// A property list of nested cons cells
pub trait ConsPList {}

impl<LCAR, LCDR, CDR> ConsPList for Cons<Single<LCAR>, Cons<Single<LCDR>, CDR>> where CDR: ConsPList {}

impl<CAR, CDR> ConsPList for Cons<Single<CAR>, Single<CDR>> {}

/// Create a `ConsPList` type from a set of key-value pairs
#[macro_export]
macro_rules! plist {
    (
        $(
            $key:expr => $value:expr $(,)?
        )*
    ) => {
        $crate::list![
            $(
                $key,
                $value
            ),*
        ]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cons_plist() {
        let cons_plist = plist! {
            0 => 1.0,
            '2' => "three",
        };

        println!("Cons plist: {:?}", cons_plist);
        let _proof: &dyn ConsPList = &cons_plist;
    }
}
