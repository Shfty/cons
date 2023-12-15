use cons::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

use crate::{TableLock, TableWriteGuardInsert};

/// Insert value into table with matching data type
pub trait TableListInsert<'a, K, V, I> {
    fn insert(&'a self, key: K, value: V);
}

/// size > 1, target in tail
impl<'a, K, V, I, T, CDR> TableListInsert<'a, K, V, (I,)> for Cons<Single<T>, CDR>
where
    T: TableLock<'a>,
    CDR: TableListInsert<'a, K, V, I>,
{
    fn insert(&'a self, key: K, value: V) {
        self.cdr().insert(key, value)
    }
}

/// size > 1, target in head
impl<'a, K, V, T, CDR> TableListInsert<'a, K, V, ()> for Cons<Single<T>, CDR>
where
    T: TableLock<'a, Data = V>,
    <T as TableLock<'a>>::WriteGuard: TableWriteGuardInsert<'a, K, V>,
{
    fn insert(&'a self, key: K, value: V) {
        let mut guard = self.car().car().write();
        let ptr: *mut <T as TableLock>::WriteGuard = &mut guard;
        let r;
        unsafe {
            r = ptr.as_mut().unwrap();
        }
        r.insert(key, value);
    }
}

/// size == 1
impl<'a, K, V, T> TableListInsert<'a, K, V, ()> for Single<T>
where
    T: TableLock<'a, Data = V>,
    <T as TableLock<'a>>::WriteGuard: TableWriteGuardInsert<'a, K, V>,
{
    fn insert(&'a self, key: K, value: V) {
        let mut guard = self.car().write();
        let ptr: *mut <T as TableLock>::WriteGuard = &mut guard;
        let r;
        unsafe {
            r = ptr.as_mut().unwrap();
        }
        r.insert(key, value);
    }
}
