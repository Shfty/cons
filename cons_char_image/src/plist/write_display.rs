use cons::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};
use std::fmt::Display;

/// A `ConsList` type whose contents can be written to a `std::fmt::Formatter` via the `Display` trait
pub trait ConsPListWriteDisplay {
    type WriteDisplay;

    fn write_display(&self, f: &mut std::fmt::Formatter<'_>) -> Self::WriteDisplay;
}

impl<LCAR, LCDR, CDR> ConsPListWriteDisplay for Cons<Single<LCAR>, Cons<Single<LCDR>, CDR>>
where
    LCAR: Display,
    LCDR: Display,
    CDR: ConsPListWriteDisplay,
{
    type WriteDisplay =
        Cons<Single<std::fmt::Result>, Cons<Single<std::fmt::Result>, CDR::WriteDisplay>>;

    fn write_display(&self, f: &mut std::fmt::Formatter<'_>) -> Self::WriteDisplay {
        let (car, cdr) = self.destructure();
        let (lcar, lcdr) = cdr.destructure();
        (
            (write!(f, "{}: {}, ", car.car(), lcar.car()),),
            ((Ok(()),), lcdr.write_display(f)),
        )
    }
}

impl<CAR, CDR> ConsPListWriteDisplay for Cons<Single<CAR>, Single<CDR>>
where
    CAR: Display,
    CDR: Display,
{
    type WriteDisplay = Cons<Single<std::fmt::Result>, Single<std::fmt::Result>>;

    fn write_display(&self, f: &mut std::fmt::Formatter<'_>) -> Self::WriteDisplay {
        let (car, cdr) = self.destructure();
        ((write!(f, "{}: {}", car.car(), cdr.car()),), (Ok(()),))
    }
}
