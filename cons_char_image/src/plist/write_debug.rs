use std::fmt::{Debug, DebugMap};

use cons::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

/// A `ConsList` type whose contents can be written to a `DebugList` builder
pub trait ConsPListWriteDebug {
    fn write_debug(&self, d: &mut DebugMap);
}

impl<LCAR, LCDR, CDR> ConsPListWriteDebug for Cons<Single<LCAR>, Cons<Single<LCDR>, CDR>>
where
    LCAR: Debug,
    LCDR: Debug,
    CDR: ConsPListWriteDebug,
{
    fn write_debug(&self, d: &mut DebugMap) {
        let (car, cdr) = self.destructure();
        let (lcar, lcdr) = cdr.destructure();
        d.entry(car.car(), lcar.car());
        lcdr.write_debug(d);
    }
}

impl<CAR, CDR> ConsPListWriteDebug for Cons<Single<CAR>, Single<CDR>>
where
    CAR: Debug,
    CDR: Debug,
{
    fn write_debug(&self, d: &mut DebugMap) {
        let (car, cdr) = self.destructure();
        d.entry(car.car(), cdr.car());
    }
}
