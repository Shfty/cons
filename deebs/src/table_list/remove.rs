use cons::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

use crate::{TableLock, TableWriteGuardRemove};

/// Remove value from table with matching data type
pub trait TableListRemove<'a, K, V, I> {
    fn remove(&'a self, key: &K);
}

impl<'a, K, V, I, T, CDR> TableListRemove<'a, K, V, (I,)> for Cons<Single<T>, CDR>
where
    T: TableLock<'a>,
    CDR: TableListRemove<'a, K, V, I>,
{
    fn remove(&'a self, key: &K) {
        self.cdr().remove(key)
    }
}

impl<'a, K, V, T, CDR> TableListRemove<'a, K, V, ()> for Cons<Single<T>, CDR>
where
    T: TableLock<'a, Data = V>,
    <T as TableLock<'a>>::WriteGuard: TableWriteGuardRemove<'a, K, V>,
{
    fn remove(&'a self, key: &K) {
        let mut guard = self.car().car().write();
        let ptr: *mut <T as TableLock>::WriteGuard = &mut guard;
        let r;
        unsafe {
            r = ptr.as_mut().unwrap();
        }
        r.remove(key);
    }
}

impl<'a, K, V, T> TableListRemove<'a, K, V, ()> for Single<T>
where
    T: TableLock<'a, Data = V>,
    <T as TableLock<'a>>::WriteGuard: TableWriteGuardRemove<'a, K, V>,
{
    fn remove(&'a self, key: &K) {
        let mut guard = self.car().write();
        let ptr: *mut <T as TableLock>::WriteGuard = &mut guard;
        let r;
        unsafe {
            r = ptr.as_mut().unwrap();
        }
        r.remove(key)
    }
}
