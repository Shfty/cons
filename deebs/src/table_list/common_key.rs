use std::collections::BTreeSet;

use cons::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

use crate::{cast_lifetime, TableListKeys, TableListSculpt, TableLock, TableReadGuardContainsKey};

/// Test whether a given key exists
pub trait TableListCommonKey<'a, K> {
    fn common_key(&'a self, key: &K) -> bool;
}

impl<'a, K, T, CDR> TableListCommonKey<'a, K> for Cons<Single<T>, CDR>
where
    T: TableLock<'a>,
    <T as TableLock<'a>>::ReadGuard: TableReadGuardContainsKey<'a, K, T::Data>,
    CDR: TableListCommonKey<'a, K>,
{
    fn common_key(&'a self, key: &K) -> bool {
        let (car, cdr) = self.destructure();
        let car = car.car();
        let guard = car.read();
        let guard = unsafe { cast_lifetime(&guard) };
        guard.contains_key(key) && cdr.common_key(key)
    }
}

impl<'a, K, T> TableListCommonKey<'a, K> for Single<T>
where
    T: TableLock<'a>,
    <T as TableLock<'a>>::ReadGuard: TableReadGuardContainsKey<'a, K, T::Data>,
{
    fn common_key(&'a self, key: &K) -> bool {
        let car = self.car();
        let guard = car.read();
        let guard = unsafe { cast_lifetime(&guard) };
        guard.contains_key(key)
    }
}

/// Sculpting variant of TableListCommonKey
pub trait SculptedTableListCommonKey<'a, S, SI, K>: TableListSculpt<S, SI> {
    fn sculpted_common_key(self, key: &K) -> bool;
}

impl<'a, S, SI, K, T> SculptedTableListCommonKey<'a, S, SI, K> for T
where
    T: TableListSculpt<S, SI>,
    <<T as TableListSculpt<S, SI>>::Sculpt as ConsCell>::CAR: 'a + TableListCommonKey<'a, K>,
{
    fn sculpted_common_key(self, key: &K) -> bool {
        let tables = self.sculpt().into_car();
        let tables = unsafe { cast_lifetime(&tables) };
        tables.common_key(key)
    }
}

/// Plural variant of SculptedTableListCommonKey
pub trait TableListCommonKeys<'a, K>: TableListKeys<'a, K>
where
    K: 'a,
{
    type KeyIter: Iterator<Item = &'a K>;

    fn common_keys(&'a self) -> Self::KeyIter;
}

impl<'a, K, T> TableListCommonKeys<'a, K> for T
where
    K: 'a + Copy + Ord + Eq,
    T: TableListKeys<'a, K>,
    T: TableListCommonKey<'a, K>,
{
    type KeyIter = std::collections::btree_set::IntoIter<&'a K>;

    fn common_keys(&'a self) -> Self::KeyIter {
        self.keys()
            .filter(move |key| self.common_key(key))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
}
