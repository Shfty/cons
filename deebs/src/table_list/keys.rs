use cons::{cell::{Cons, ConsCell}, single::{ConsSingle, Single}};

use crate::{TableLock, TableReadGuardKeys};

/// Fetch all keys from the provided table list
pub trait TableListKeys<'a, K: 'a> {
    type Keys: Iterator<Item = &'a K>;

    fn keys(&'a self) -> Self::Keys;
}

impl<'a, K, T, CDR> TableListKeys<'a, K> for Cons<Single<T>, CDR>
where
    K: 'a,
    T: TableLock<'a>,
    <T as TableLock<'a>>::ReadGuard: TableReadGuardKeys<'a, K>,
    <<T as TableLock<'a>>::ReadGuard as TableReadGuardKeys<'a, K>>::Keys: Iterator<Item = &'a K>,
    CDR: TableListKeys<'a, K>,
{
    type Keys = std::iter::Chain<
        <<T as TableLock<'a>>::ReadGuard as TableReadGuardKeys<'a, K>>::Keys,
        CDR::Keys,
    >;

    fn keys(&'a self) -> Self::Keys {
        let (car, cdr) = self.destructure();
        let car = car.car();
        let guard = car.read();
        let ptr: *const <T as TableLock>::ReadGuard = &guard;
        let r;
        unsafe {
            r = ptr.as_ref().unwrap();
        }
        r.keys().chain(cdr.keys())
    }
}

impl<'a, K, T> TableListKeys<'a, K> for Single<T>
where
    K: 'a,
    T: TableLock<'a>,
    <T as TableLock<'a>>::ReadGuard: TableReadGuardKeys<'a, K>,
    <<T as TableLock<'a>>::ReadGuard as TableReadGuardKeys<'a, K>>::Keys: Iterator<Item = &'a K>,
{
    type Keys = <<T as TableLock<'a>>::ReadGuard as TableReadGuardKeys<'a, K>>::Keys;

    fn keys(&'a self) -> Self::Keys {
        let guard = self.car().read();
        let ptr: *const <T as TableLock>::ReadGuard = &guard;
        let r;
        unsafe {
            r = ptr.as_ref().unwrap();
        }
        r.keys()
    }
}
