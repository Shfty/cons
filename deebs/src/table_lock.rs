use std::{ops::Deref, sync::{RwLock, RwLockReadGuard, RwLockWriteGuard}};

use crate::RowLock;

use super::{TableCollection, TableReadGuard, TableWriteGuard};

/// Table wrapper that can hand out read / write guards
pub trait TableLock<'a> {
    type TableCollection;
    type RowLock;
    type Data;
    type ReadGuard: TableReadGuard<'a, Data = Self::Data>;
    type WriteGuard: TableWriteGuard<'a, Data = Self::Data>;

    fn read(&'a self) -> Self::ReadGuard;
    fn write(&'a self) -> Self::WriteGuard;
}

impl<'a, T> TableLock<'a> for RwLock<T>
where
    T: 'a + TableCollection,
    T::RowLock: RowLock<'a>
{
    type TableCollection = T;
    type RowLock = T::RowLock;
    type Data = <T::RowLock as RowLock<'a>>::Data;
    type ReadGuard = RwLockReadGuard<'a, T>;
    type WriteGuard = RwLockWriteGuard<'a, T>;

    fn read(&'a self) -> Self::ReadGuard {
        self.read().unwrap()
    }

    fn write(&'a self) -> Self::WriteGuard {
        self.write().unwrap()
    }
}


impl<'a, T> TableLock<'a> for &'a RwLock<T>
where
    T: 'a + TableCollection,
{
    type TableCollection = T;
    type RowLock = T::RowLock;
    type Data = <T::RowLock as RowLock<'a>>::Data;
    type ReadGuard = RwLockReadGuard<'a, T>;
    type WriteGuard = RwLockWriteGuard<'a, T>;

    fn read(&'a self) -> Self::ReadGuard {
        self.deref().read().unwrap()
    }

    fn write(&'a self) -> Self::WriteGuard {
        self.deref().write().unwrap()
    }
}
