use std::{ops::Deref, sync::RwLockReadGuard};

use cons::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

use crate::{RowLock, RowLockList, TableCollection, TableCollectionContainsKey, TableCollectionGet, TableCollectionKeys};

/// Read guard over a table
pub trait TableReadGuard<'a>: Deref {
    type TableCollection: TableCollection;
    type Data;
}

impl<'a, T> TableReadGuard<'a> for RwLockReadGuard<'a, T>
where
    T: TableCollection,
{
    type TableCollection = T;
    type Data = <T::RowLock as RowLock<'a>>::Data;
}

pub trait TableReadGuardKeys<'a, K>: TableReadGuard<'a> {
    type Keys;

    fn keys(&'a self) -> Self::Keys;
}

impl<'a, K, T> TableReadGuardKeys<'a, K> for RwLockReadGuard<'a, T>
where
    T: TableCollectionKeys<'a, K>,
{
    type Keys = T::Keys;

    fn keys(&'a self) -> Self::Keys {
        self.deref().keys()
    }
}

pub trait TableReadGuardContainsKey<'a, K, V>: TableReadGuard<'a> {
    fn contains_key(&'a self, key: &K) -> bool;
}

impl<'a, K, V, T> TableReadGuardContainsKey<'a, K, V> for RwLockReadGuard<'a, T>
where
    T: TableCollectionContainsKey<K, V>,
{
    fn contains_key(&'a self, key: &K) -> bool {
        self.deref().contains_key(key)
    }
}

pub trait TableReadGuardGet<'a, K>: TableReadGuard<'a> {
    fn get(
        &'a self,
        key: &K,
    ) -> &<<Self as TableReadGuard>::TableCollection as TableCollection>::RowLock;
}

impl<'a, K, T> TableReadGuardGet<'a, K> for RwLockReadGuard<'a, T>
where
    T: TableCollectionGet<K>,
{
    fn get(&'a self, key: &K) -> &<T as TableCollection>::RowLock {
        self.deref().get(key)
    }
}

pub trait TableReadGuardList<'a, K> {
    type Get: RowLockList<'a>;

    fn get(&'a self, key: &K) -> Self::Get;
}

impl<'a, K, CAR, CDR> TableReadGuardList<'a, K> for Cons<Single<CAR>, CDR>
where
    CAR: TableReadGuard<'a>,
    CAR::Target: TableCollectionGet<K>,
    <<CAR as Deref>::Target as TableCollection>::RowLock: 'a,
    &'a <<CAR as Deref>::Target as TableCollection>::RowLock: RowLock<'a>,
    CDR: TableReadGuardList<'a, K>,
{
    type Get = Cons<Single<&'a <CAR::Target as TableCollection>::RowLock>, CDR::Get>;

    fn get(&'a self, key: &K) -> Self::Get {
        let (car, cdr) = self.destructure();
        (car.get(key), cdr.get(key))
    }
}

impl<'a, K, CAR> TableReadGuardList<'a, K> for Single<CAR>
where
    CAR: TableReadGuard<'a>,
    CAR::Target: TableCollectionGet<K>,
    <<CAR as Deref>::Target as TableCollection>::RowLock: 'a,
    &'a <<CAR as Deref>::Target as TableCollection>::RowLock: RowLock<'a>,
{
    type Get = Single<&'a <CAR::Target as TableCollection>::RowLock>;

    fn get(&'a self, key: &K) -> Self::Get {
        (self.car().deref().get(key),)
    }
}
