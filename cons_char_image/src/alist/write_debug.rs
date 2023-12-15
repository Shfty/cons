use std::fmt::{Debug, DebugMap};

use cons::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

/// A `ConsList` type whose contents can be written to a `DebugList` builder
pub trait ConsAListWriteDebug {
    fn write_debug(&self, d: &mut DebugMap);
}

impl<LCAR, LCDR, CDR> ConsAListWriteDebug for Cons<Single<Cons<LCAR, LCDR>>, CDR>
where
    LCAR: Debug,
    LCDR: Debug,
    CDR: ConsAListWriteDebug,
{
    fn write_debug(&self, d: &mut DebugMap) {
        let (car, cdr) = self.destructure();
        let (lcar, lcdr) = car.car().destructure();
        d.entry(lcar, lcdr);
        cdr.write_debug(d);
    }
}

impl<CAR, CDR> ConsAListWriteDebug for Single<Cons<CAR, CDR>>
where
    CAR: Debug,
    CDR: Debug,
{
    fn write_debug(&self, d: &mut DebugMap) {
        let (car, cdr) = self.car().destructure();
        d.entry(car, cdr);
    }
}
