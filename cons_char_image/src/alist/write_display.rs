use cons::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};
use std::fmt::Display;

/// A `ConsList` type whose contents can be written to a `std::fmt::Formatter` via the `Display` trait
pub trait ConsAListWriteDisplay {
    type WriteDisplay;

    fn write_display(&self, f: &mut std::fmt::Formatter<'_>) -> Self::WriteDisplay;
}

impl<LCAR, LCDR, CDR> ConsAListWriteDisplay for Cons<Single<Cons<LCAR, LCDR>>, CDR>
where
    LCAR: Display,
    LCDR: Display,
    CDR: ConsAListWriteDisplay,
{
    type WriteDisplay = Cons<Single<std::fmt::Result>, CDR::WriteDisplay>;

    fn write_display(&self, f: &mut std::fmt::Formatter<'_>) -> Self::WriteDisplay {
        let (car, cdr) = self.destructure();
        let (lcar, lcdr) = car.car().destructure();
        ((write!(f, "{}: {}, ", lcar, lcdr),), cdr.write_display(f))
    }
}

impl<CAR, CDR> ConsAListWriteDisplay for Single<Cons<CAR, CDR>>
where
    CAR: Display,
    CDR: Display,
{
    type WriteDisplay = Single<std::fmt::Result>;

    fn write_display(&self, f: &mut std::fmt::Formatter<'_>) -> Self::WriteDisplay {
        let (car, cdr) = self.car();
        (write!(f, "{}: {}", car, cdr),)
    }
}
