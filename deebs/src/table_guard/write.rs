use std::{ops::DerefMut, sync::RwLockWriteGuard};

use crate::{RowLock, TableCollection, TableCollectionInsert, TableCollectionRemove};

/// Write guard over a table
pub trait TableWriteGuard<'a> {
    type Data;
}

impl<'a, T> TableWriteGuard<'a> for RwLockWriteGuard<'a, T>
where
    T: TableCollection,
{
    type Data = <T::RowLock as RowLock<'a>>::Data;
}

pub trait TableWriteGuardInsert<'a, K, V>: TableWriteGuard<'a> {
    fn insert(&'a mut self, key: K, value: V);
}

impl<'a, K, V, T> TableWriteGuardInsert<'a, K, V> for RwLockWriteGuard<'a, T>
where
    T: TableCollectionInsert<K, V>,
{
    fn insert(&'a mut self, key: K, value: V) {
        self.deref_mut().insert(key, value)
    }
}

pub trait TableWriteGuardRemove<'a, K, V>: TableWriteGuard<'a> {
    fn remove(&'a mut self, key: &K);
}

impl<'a, K, V, T> TableWriteGuardRemove<'a, K, V> for RwLockWriteGuard<'a, T>
where
    T: TableCollectionRemove<K, V>,
{
    fn remove(&'a mut self, key: &K) {
        self.deref_mut().remove(key)
    }
}
