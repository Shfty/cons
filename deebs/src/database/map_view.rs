use std::sync::RwLock;

use cons::{
    cell::ConsCell,
    list::{as_ref::ConsListAsRef, deref::ConsListDeref, find::ConsListFind},
};

use crate::{
    cast_lifetime, DatabaseMapKeys, DatabaseViewKeys, RowLockList, TableList, TableListSculpt,
    TableReadGuardList, ViewKeys,
};

use super::View;

pub trait DatabaseMapView<'a, Key, ViewSculpt, ViewFindIndex, Views, Tables, SculptIndex, Func>:
    DatabaseViewKeys<Key, ViewFindIndex, ViewSculpt, Views>
    + DatabaseMapKeys<Key, std::slice::Iter<'a, Key>, Func, SculptIndex, Tables>
where
    Key: 'a + Copy,
{
    fn map_view<Signature>(&'a self, f: Func)
    where
    // DatabaseViewKeys
        Signature: 'a,
        ViewSculpt: 'a,
        Views: ConsListAsRef<'a>,
        <Views as ConsListAsRef<'a>>::AsRef: ConsListFind<&'a RwLock<ViewKeys<usize, Signature>>, ViewFindIndex, Find = &'a std::sync::RwLock<ViewKeys<Key, ViewSculpt>>>,

    // DatabaseMapKeys
        Tables: 'a,
        SculptIndex: 'a,
        Tables: TableList<'a> + ConsListAsRef<'a>,
        <Tables as ConsListAsRef<'a>>::AsRef: TableListSculpt<Signature, SculptIndex>,
        <<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<Signature, SculptIndex>>::Sculpt as ConsCell>::CAR: TableList<'a>,
        <<<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<Signature, SculptIndex>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards: TableReadGuardList<'a, Key>,
        <<<<<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<Signature, SculptIndex>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, Key>>::Get as RowLockList<'a>>::ReadGuards: ConsListDeref<'a>,
        <Tables as TableList<'a>>::ReadGuards: TableReadGuardList<'a, Key>,
        Func: FnMut(&Key, <<<<<<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<Signature, SculptIndex>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, Key>>::Get as RowLockList<'a>>::ReadGuards as ConsListDeref<'a>>::Deref),
    {
        let keys = self.view_keys::<&View<Signature>>();
        let keys = unsafe { cast_lifetime(&keys) };
        self.map_keys::<Signature>(keys.iter(), f);
    }
}

impl<'a, Key, ViewSculpt, ViewFindIndex, Views, Tables, SculptIndex, Func, T>
    DatabaseMapView<'a, Key, ViewSculpt, ViewFindIndex, Views, Tables, SculptIndex, Func> for T
where
    Key: 'a + Copy,
    T: DatabaseViewKeys<Key, ViewFindIndex, ViewSculpt, Views>
        + DatabaseMapKeys<Key, std::slice::Iter<'a, Key>, Func, SculptIndex, Tables>,
{
}
