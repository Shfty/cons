mod write_debug;
mod write_display;

use std::{fmt::{Debug, Display}, marker::PhantomData};

pub use write_debug::*;
pub use write_display::*;

use cons::tree::{ConsTree, homogenous::iter::ConsTreeIter};

/// A wrapper struct that implements `Debug` and `Display` on behalf of its underlying `ConsAList` type
pub struct ConsAListFormatter<'a, C, I> {
    cons_alist: &'a C,
    _phantom: PhantomData<I>
}

impl<'a, C, I> Debug for ConsAListFormatter<'a, C, I>
where
    C: ConsAListWriteDebug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_map = f.debug_map();
        self.cons_alist.write_debug(&mut debug_map);
        debug_map.finish()
    }
}

impl<'a, C, I> Display for ConsAListFormatter<'a, C, I>
where
    C: ConsAListWriteDisplay,
    for<'b> C::WriteDisplay: ConsTreeIter<'b, std::fmt::Result, I>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for r in self.cons_alist.write_display(f).iter() {
            match r {
                Ok(_) => (),
                Err(e) => return Err(*e),
            };
        }
        write!(f, "}}")?;
        Ok(())
    }
}

/// A `ConsAList` that can be wrapped in a `ConsAListFormatter` for `Debug` and `Display` purposes
pub trait ConsListFormat<'a, I>: Sized {
    fn format(&'a self) -> ConsAListFormatter<'a, Self, I>;
}

impl<'a, I, T> ConsListFormat<'a, I> for T
where
    T: 'a + ConsTree<I> + ConsAListWriteDebug + ConsAListWriteDisplay,
{
    fn format(&'a self) -> ConsAListFormatter<'a, Self, I> {
        ConsAListFormatter {
            cons_alist: &self,
            _phantom: Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cons::alist;

    #[test]
    fn test_cons_alist_format() {
        let cons_alist = alist![1 => 2.5, '3' => "four"];
        println!("\nCons AList Display:\n{}", cons_alist.format());
        println!("\nCons AList Debug:\n{:?}\n", cons_alist.format());
        println!("\nCons AList Debug Multiline:\n{:#?}\n", cons_alist.format());
    }
}
