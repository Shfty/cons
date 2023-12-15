use std::{
    ops::Deref,
    sync::{RwLockReadGuard, RwLockWriteGuard},
};

/// Row wrapper that can hand out read / write guards
use super::*;
pub trait RowLock<'a> {
    type Data;
    type ReadGuard: Deref<Target = Self::Data>;
    type WriteGuard: Deref<Target = Self::Data>;

    fn read(&'a self) -> Self::ReadGuard;
    fn write(&'a self) -> Self::WriteGuard;
}

impl<'a, T> RowLock<'a> for RwLock<T>
where
    T: 'a,
{
    type Data = T;
    type ReadGuard = RwLockReadGuard<'a, T>;
    type WriteGuard = RwLockWriteGuard<'a, T>;

    fn read(&'a self) -> Self::ReadGuard {
        self.read().unwrap()
    }

    fn write(&'a self) -> Self::WriteGuard {
        self.write().unwrap()
    }
}

impl<'a, T> RowLock<'a> for &'a RwLock<T>
where
    T: 'a,
{
    type Data = T;
    type ReadGuard = RwLockReadGuard<'a, T>;
    type WriteGuard = RwLockWriteGuard<'a, T>;

    fn read(&'a self) -> Self::ReadGuard {
        self.deref().read().unwrap()
    }

    fn write(&'a self) -> Self::WriteGuard {
        self.deref().write().unwrap()
    }
}

pub trait RowLockList<'a> {
    type ReadGuards;
    type WriteGuards;

    fn read(&'a self) -> Self::ReadGuards;
    fn write(&'a self) -> Self::WriteGuards;
}

impl<'a, CAR, CDR> RowLockList<'a> for Cons<Single<CAR>, CDR>
where
    CAR: RowLock<'a>,
    CDR: RowLockList<'a>,
{
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

impl<'a, CAR> RowLockList<'a> for Single<CAR>
where
    CAR: RowLock<'a>,
{
    type ReadGuards = Single<CAR::ReadGuard>;
    type WriteGuards = Single<CAR::WriteGuard>;

    fn read(&'a self) -> Self::ReadGuards {
        (self.car().read(),)
    }

    fn write(&'a self) -> Self::WriteGuards {
        (self.car().write(),)
    }
}
