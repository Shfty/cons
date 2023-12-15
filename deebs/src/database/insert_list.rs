use cons::{cell::ConsCell, list::as_ref::ConsListAsRef};

use crate::{
    cast_lifetime, Database, DatabaseUpdateViews, InsertIntoTableList, TableList, TableListSculpt,
    ViewList,
};

pub trait DatabaseInsertList<'a, K, V, II, LSI, S, SI> {
    fn insert_list(&'a self, key: K, values: V);
}

impl<'a, K, V, II, LSI, S, SI, TL, VL> DatabaseInsertList<'a, K, V, II, LSI, S, SI>
    for Database<TL, VL>
where
    V: 'a,
    LSI: 'a,
    TL: ConsListAsRef<'a>,
    TL::AsRef: TableListSculpt<V, LSI>,
    <<<TL as ConsListAsRef<'a>>::AsRef as TableListSculpt<V, LSI>>::Sculpt as ConsCell>::CAR:
        TableList<'a>,
    V: InsertIntoTableList<
        'a,
        K,
        II,
        <<TL::AsRef as TableListSculpt<V, LSI>>::Sculpt as ConsCell>::CAR,
    >,
    VL: ViewList<'a, <TL as ConsListAsRef<'a>>::AsRef, S, SI>,
{
    fn insert_list(&'a self, key: K, values: V) {
        let tables = self.tables.as_ref();
        let tables = tables.sculpt().into_car();
        let tables = unsafe { cast_lifetime(&tables) };
        values.insert_into(tables, key);
        self.update_views();
    }
}
