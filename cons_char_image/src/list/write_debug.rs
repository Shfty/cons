use std::fmt::{Debug, DebugList};

use cons::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

/// A `ConsList` type whose contents can be written to a `DebugList` builder
pub trait ConsListWriteDebug {
    fn write_debug(&self, d: &mut DebugList);
}

impl<CAR, CDR> ConsListWriteDebug for Cons<Single<CAR>, CDR>
where
    CAR: Debug,
    CDR: ConsListWriteDebug,
{
    fn write_debug(&self, d: &mut DebugList) {
        let (car, cdr) = self.destructure();
        d.entry(car.car());
        cdr.write_debug(d);
    }
}

impl<CAR> ConsListWriteDebug for Single<CAR>
where
    CAR: Debug,
{
    fn write_debug(&self, d: &mut DebugList) {
        d.entry(self.car());
    }
}
