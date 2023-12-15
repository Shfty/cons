mod common_key;
mod insert;
mod insert_into_table_list;
mod keys;
mod pluck;
mod remove;
mod remove_from_table_list;
mod sculpt;

pub use common_key::*;
pub use insert::*;
pub use insert_into_table_list::*;
pub use keys::*;
pub use pluck::*;
pub use remove::*;
pub use remove_from_table_list::*;
pub use sculpt::*;

use super::*;

/// Cons list of tables
pub trait TableList<'a> {
    type TableCollection;
    type RowLock;
    type Data;
    type ReadGuards;
    type WriteGuards;

    fn read(&'a self) -> Self::ReadGuards;
    fn write(&'a self) -> Self::WriteGuards;
}

impl<'a, CAR, CDR> TableList<'a> for Cons<Single<CAR>, CDR>
where
    CAR: TableLock<'a>,
    CDR: TableList<'a>,
{
    type TableCollection = Cons<Single<CAR::TableCollection>, CDR::TableCollection>;
    type RowLock = Cons<Single<CAR::RowLock>, CDR::RowLock>;
    type Data = Cons<Single<CAR::Data>, CDR::Data>;
    type ReadGuards = Cons<Single<CAR::ReadGuard>, CDR::ReadGuards>;
    type WriteGuards = Cons<Single<CAR::WriteGuard>, CDR::WriteGuards>;

    fn read(&'a self) -> Self::ReadGuards {
        let (car, cdr) = self.destructure();
        (car.read(), cdr.read())
    }

    fn write(&'a self) -> Self::WriteGuards {
        let (car, cdr) = self.destructure();
        (car.write(), cdr.write())
    }
}

impl<'a, CAR> TableList<'a> for Single<CAR>
where
    CAR: TableLock<'a>,
{
    type TableCollection = Single<CAR::TableCollection>;
    type RowLock = Single<CAR::RowLock>;
    type Data = Single<CAR::Data>;
    type ReadGuards = Single<CAR::ReadGuard>;
    type WriteGuards = Single<CAR::WriteGuard>;

    fn read(&'a self) -> Self::ReadGuards {
        (self.car().read(),)
    }

    fn write(&'a self) -> Self::WriteGuards {
        (self.car().write(),)
    }
}

impl<'a> TableList<'a> for () {
    type TableCollection = ();
    type RowLock = ();
    type Data = ();
    type ReadGuards = ();
    type WriteGuards = ();

    fn read(&'a self) -> Self::ReadGuards {}

    fn write(&'a self) -> Self::WriteGuards {}
}
