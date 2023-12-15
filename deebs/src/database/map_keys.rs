use cons::{
    cell::ConsCell,
    list::{as_ref::ConsListAsRef, deref::ConsListDeref},
};

use crate::{cast_lifetime, Database, RowLockList, TableList, TableListSculpt, TableReadGuardList};

pub trait DatabaseMapKeys<K, I, F, SI, TL> {
    fn map_keys<'a, S>(&'a self, keys: I, f: F)
    where
        K: 'a,
        I: Iterator<Item = &'a K>,
        TL: TableList<'a> + ConsListAsRef<'a>,
        <TL as TableList<'a>>::ReadGuards: TableReadGuardList<'a, K>,
        TL::AsRef: TableListSculpt<S, SI>,
        <<TL::AsRef as TableListSculpt<S, SI>>::Sculpt as ConsCell>::CAR: 'a + TableList<'a>,
        <<<TL::AsRef as TableListSculpt<S, SI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards: TableReadGuardList<'a, K>,
        <<<<<TL::AsRef as TableListSculpt<S, SI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
            'a,
        >>::ReadGuards: ConsListDeref<'a>,
        F: FnMut(
            &K,
            <<<<<<<TL as ConsListAsRef<'a>>::AsRef as TableListSculpt<S, SI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
                'a,
            >>::ReadGuards as ConsListDeref<'a>>::Deref,
        ) ;
}

impl<K, I, F, SI, TL, VL> DatabaseMapKeys<K, I, F, SI, TL> for Database<TL, VL> {
    fn map_keys<'a, S>(&'a self, keys: I, mut f: F)
    where
        K: 'a,
        I: Iterator<Item = &'a K>,
        TL: TableList<'a> + ConsListAsRef<'a>,
        <TL as TableList<'a>>::ReadGuards: TableReadGuardList<'a, K>,
        TL::AsRef: TableListSculpt<S, SI>,
        <<TL::AsRef as TableListSculpt<S, SI>>::Sculpt as ConsCell>::CAR: 'a + TableList<'a>,
        <<<TL::AsRef as TableListSculpt<S, SI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards: TableReadGuardList<'a, K>,
        <<<<<TL::AsRef as TableListSculpt<S, SI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
            'a,
        >>::ReadGuards: ConsListDeref<'a>,
        F: FnMut(
            &K,
            <<<<<<<TL as ConsListAsRef<'a>>::AsRef as TableListSculpt<S, SI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
                'a,
            >>::ReadGuards as ConsListDeref<'a>>::Deref,
    ){
        let tables = self.tables.as_ref();
        let tables = tables.sculpt().into_car();
        let tables = unsafe { cast_lifetime(&tables) };
        let table_read_guards = tables.read();
        let table_read_guards = unsafe { cast_lifetime(&table_read_guards) };

        for key in keys {
            let row_locks = table_read_guards.get(key);
            let row_locks = unsafe { cast_lifetime(&row_locks) };

            let row_read_guards = row_locks.read();
            let row_read_guards = unsafe { cast_lifetime(&row_read_guards) };

            let row_refs = row_read_guards.deref();

            f(key, row_refs)
        }
    }
}
