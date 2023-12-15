use cons::{
    cell::ConsCell,
    list::{as_ref::ConsListAsRef, deref::ConsListDeref, deref_mut::ConsListDerefMut},
};

use crate::{
    cast_lifetime, cast_lifetime_mut, Database, RowLockList, TableList, TableListSculpt,
    TableReadGuardList,
};

pub trait DatabaseMapKeysMut<K, I, ISI, MSI, F, TL> {
    fn map_keys_mut<'a, IS, MS>(&'a self, keys: I, f: F)
    where
        K: 'a,
        IS: 'a,
        MS: 'a,
        ISI: 'a,
        MSI: 'a,
        TL: TableList<'a> + ConsListAsRef<'a>,
        <TL as TableList<'a>>::ReadGuards: TableReadGuardList<'a, K>,
        TL::AsRef: Copy + TableListSculpt<IS, ISI> + TableListSculpt<MS, MSI>,
    // Immutable
        <<TL::AsRef as TableListSculpt<IS, ISI>>::Sculpt as ConsCell>::CAR: TableList<'a>,
        <<<TL::AsRef as TableListSculpt<IS, ISI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards: TableReadGuardList<'a, K>,
        <<<<<TL::AsRef as TableListSculpt<IS, ISI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
            'a,
        >>::ReadGuards: ConsListDeref<'a>,
    // Mutable
        <<TL::AsRef as TableListSculpt<MS, MSI>>::Sculpt as ConsCell>::CAR: TableList<'a>,
        <<<TL::AsRef as TableListSculpt<MS, MSI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards: TableReadGuardList<'a, K>,
        <<<<<TL::AsRef as TableListSculpt<MS, MSI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
            'a,
        >>::WriteGuards: ConsListDerefMut<'a>,
    // Other
        I: Iterator<Item = &'a K>,
        F: FnMut(
            &K,
            <<<<<<TL::AsRef as TableListSculpt<IS, ISI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
                'a,
            >>::ReadGuards as ConsListDeref<'a>>::Deref,
            <<<<<<TL::AsRef as TableListSculpt<MS, MSI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
                'a,
            >>::WriteGuards as ConsListDerefMut<'a>>::DerefMut,
        );
}

impl<K, I, ISI, MSI, F, TL, VL> DatabaseMapKeysMut<K, I, ISI, MSI, F, TL> for Database<TL, VL> {
    fn map_keys_mut<'a, IS, MS>(&'a self, keys: I, mut f: F)
    where
        K: 'a,
        IS: 'a,
        MS: 'a,
        ISI: 'a,
        MSI: 'a,
        TL: TableList<'a> + ConsListAsRef<'a>,
        <TL as TableList<'a>>::ReadGuards: TableReadGuardList<'a, K>,
        TL::AsRef: Copy + TableListSculpt<IS, ISI> + TableListSculpt<MS, MSI>,
    // Immutable
        <<TL::AsRef as TableListSculpt<IS, ISI>>::Sculpt as ConsCell>::CAR: TableList<'a>,
        <<<TL::AsRef as TableListSculpt<IS, ISI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards: TableReadGuardList<'a, K>,
        <<<<<TL::AsRef as TableListSculpt<IS, ISI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
            'a,
        >>::ReadGuards: ConsListDeref<'a>,
    // Mutable
        <<TL::AsRef as TableListSculpt<MS, MSI>>::Sculpt as ConsCell>::CAR: TableList<'a>,
        <<<TL::AsRef as TableListSculpt<MS, MSI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards: TableReadGuardList<'a, K>,
        <<<<<TL::AsRef as TableListSculpt<MS, MSI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
            'a,
        >>::WriteGuards: ConsListDerefMut<'a>,
    // Other
        I: Iterator<Item = &'a K>,
        F: FnMut(
            &K,
            <<<<<<TL::AsRef as TableListSculpt<IS, ISI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
                'a,
            >>::ReadGuards as ConsListDeref<'a>>::Deref,
            <<<<<<TL::AsRef as TableListSculpt<MS, MSI>>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<'a, K>>::Get as RowLockList<
                'a,
            >>::WriteGuards as ConsListDerefMut<'a>>::DerefMut,
    ){
        let tables = self.tables.as_ref();

        let ref_tables = TableListSculpt::<IS, ISI>::sculpt(tables).into_car();
        let ref_tables = unsafe { cast_lifetime(&ref_tables) };
        let ref_table_read_guards = ref_tables.read();
        let ref_table_read_guards = unsafe { cast_lifetime(&ref_table_read_guards) };

        let mut_tables = TableListSculpt::<MS, MSI>::sculpt(tables).into_car();
        let mut_tables = unsafe { cast_lifetime(&mut_tables) };
        let mut_table_read_guards = mut_tables.read();
        let mut_table_read_guards = unsafe { cast_lifetime(&mut_table_read_guards) };

        for key in keys {
            let ref_row_locks = ref_table_read_guards.get(key);
            let ref_row_locks = unsafe { cast_lifetime(&ref_row_locks) };

            let ref_row_read_guards = ref_row_locks.read();
            let ref_row_read_guards = unsafe { cast_lifetime(&ref_row_read_guards) };

            let mut_row_locks = mut_table_read_guards.get(key);
            let mut_row_locks = unsafe { cast_lifetime(&mut_row_locks) };

            let mut mut_row_write_guards = mut_row_locks.write();
            let mut_row_write_guards = unsafe { cast_lifetime_mut(&mut mut_row_write_guards) };

            let ref_row_refs = ref_row_read_guards.deref();
            let mut_row_refs = mut_row_write_guards.deref_mut();

            f(key, ref_row_refs, mut_row_refs)
        }
    }
}
