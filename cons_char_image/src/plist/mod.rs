mod write_debug;
mod write_display;

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

pub use write_debug::*;
pub use write_display::*;

use cons::tree::{homogenous::iter::ConsTreeIter, ConsTree};

/// A wrapper struct that implements `Debug` and `Display` on behalf of its underlying `ConsPList` type
pub struct ConsPListFormatter<'a, C, I> {
    cons_plist: &'a C,
    _phantom: PhantomData<I>,
}

impl<'a, C, I> Debug for ConsPListFormatter<'a, C, I>
where
    C: ConsPListWriteDebug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_map = f.debug_map();
        self.cons_plist.write_debug(&mut debug_map);
        debug_map.finish()
    }
}

impl<'a, C, I> Display for ConsPListFormatter<'a, C, I>
where
    C: ConsPListWriteDisplay,
    for<'b> C::WriteDisplay: ConsTreeIter<'b, std::fmt::Result, I>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for r in self.cons_plist.write_display(f).iter() {
            match r {
                Ok(_) => (),
                Err(e) => return Err(*e),
            };
        }
        write!(f, "}}")?;
        Ok(())
    }
}

/// A `ConsAList` that can be wrapped in a `ConsPListFormatter` for `Debug` and `Display` purposes
pub trait ConsPListFormat<'a, I>: Sized {
    fn format(&'a self) -> ConsPListFormatter<'a, Self, I>;
}

impl<'a, I, T> ConsPListFormat<'a, I> for T
where
    T: 'a + ConsTree<I> + ConsPListWriteDebug + ConsPListWriteDisplay,
{
    fn format(&'a self) -> ConsPListFormatter<'a, Self, I> {
        ConsPListFormatter {
            cons_plist: &self,
            _phantom: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cons::plist;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct One;

    impl Display for One {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("One")
        }
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Two;

    impl Display for Two {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Two")
        }
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Three;

    impl Display for Three {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Three")
        }
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Four;

    impl Display for Four {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Four")
        }
    }

    #[test]
    fn test_cons_plist_format() {
        let cons_plist = plist! {
            One => 1,
            Two => 2.0,
            Three => '3',
            Four => "four"
        };
        
        println!("\nCons PList Display:\n{}", cons_plist.format());
        println!("\nCons PList Debug:\n{:?}\n", cons_plist.format());
        println!(
            "\nCons PList Debug Multiline:\n{:#?}\n",
            cons_plist.format()
        );
    }
}
