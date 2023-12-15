mod write_debug;
mod write_display;

pub use write_debug::*;
pub use write_display::*;

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use cons::tree::{homogenous::iter::ConsTreeIter, ConsTree};

/// A wrapper struct that implements `Debug` and `Display` on behalf of its underlying `ConsList` type
pub struct ConsListFormatter<'a, C, I> {
    cons_list: &'a C,
    _phantom: PhantomData<I>,
}

impl<'a, C, I> Display for ConsListFormatter<'a, C, I>
where
    C: ConsListWriteDisplay,
    for<'b> C::WriteDisplay: ConsTreeIter<'b, std::fmt::Result, I>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for r in self.cons_list.write_display(f).iter() {
            match r {
                Ok(_) => (),
                Err(e) => return Err(*e),
            };
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<'a, C, I> Debug for ConsListFormatter<'a, C, I>
where
    C: ConsListWriteDebug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_list = f.debug_list();
        self.cons_list.write_debug(&mut debug_list);
        debug_list.finish()
    }
}

/// A `ConsList` that can be wrapped in a `ConsListFormatter` for `Debug` and `Display` purposes
pub trait ConsListFormat<'a, I>:
    Sized + ConsTree<I>
{
    fn format(&'a self) -> ConsListFormatter<'a, Self, I>;
}

impl<'a, T, I> ConsListFormat<'a, I> for T
where
    T: 'a + ConsTree<I>,
{
    fn format(&'a self) -> ConsListFormatter<'a, Self, I> {
        ConsListFormatter {
            cons_list: &self,
            _phantom: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cons::list;

    #[test]
    fn test_cons_list_format() {
        let cons_list = list![1, 2.5, '3', "four"];
        println!("\nCons List Display:\n{}", cons_list.format());
        println!("\nCons List Debug:\n{:?}\n", cons_list.format());
        println!("\nCons List Debug Multiline:\n{:#?}\n", cons_list.format());
    }
}
