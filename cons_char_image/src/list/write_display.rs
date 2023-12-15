use cons::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};
use std::fmt::Display;

/// A `ConsList` type whose contents can be written to a `std::fmt::Formatter` via the `Display` trait
pub trait ConsListWriteDisplay {
    type WriteDisplay;

    fn write_display(&self, f: &mut std::fmt::Formatter<'_>) -> Self::WriteDisplay;
}

impl<CAR, CDR> ConsListWriteDisplay for Cons<Single<CAR>, CDR>
where
    CAR: Display,
    CDR: ConsListWriteDisplay,
{
    type WriteDisplay = ((std::fmt::Result,), CDR::WriteDisplay);

    fn write_display(&self, f: &mut std::fmt::Formatter<'_>) -> Self::WriteDisplay {
        let (car, cdr) = self.destructure();
        ((write!(f, "{}, ", car.car()),), cdr.write_display(f))
    }
}

impl<CAR> ConsListWriteDisplay for Single<CAR>
where
    CAR: Display,
{
    type WriteDisplay = (std::fmt::Result,);

    fn write_display(&self, f: &mut std::fmt::Formatter<'_>) -> Self::WriteDisplay {
        (write!(f, "{}", self.car()),)
    }
}
