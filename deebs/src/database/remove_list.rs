use cons::{cell::ConsCell, list::as_ref::ConsListAsRef};

use crate::{
    cast_lifetime, Database, DatabaseUpdateViews, RemoveFromTableList, TableList, TableListSculpt,
    ViewList,
};

pub trait DatabaseRemoveList<'a, K, II, LSI, S, SI, TL, VL> {
    fn remove_list<V>(&'a self, key: &K)
    where
        TL: ConsListAsRef<'a>,
        TL::AsRef: TableListSculpt<V, LSI>,
        <<<TL as ConsListAsRef<'a>>::AsRef as TableListSculpt<V, LSI>>::Sculpt as ConsCell>::CAR:
            'a + TableList<'a>,
        V: RemoveFromTableList<
            'a,
            K,
            II,
            <<TL::AsRef as TableListSculpt<V, LSI>>::Sculpt as ConsCell>::CAR,
        >;
}

impl<'a, K, II, LSI, S, SI, TL, VL> DatabaseRemoveList<'a, K, II, LSI, S, SI, TL, VL>
    for Database<TL, VL>
where
    TL: ConsListAsRef<'a>,
    VL: ViewList<'a, <TL as ConsListAsRef<'a>>::AsRef, S, SI>,
{
    fn remove_list<V>(&'a self, key: &K)
    where
        TL: ConsListAsRef<'a>,
        TL::AsRef: TableListSculpt<V, LSI>,
        <<<TL as ConsListAsRef<'a>>::AsRef as TableListSculpt<V, LSI>>::Sculpt as ConsCell>::CAR:
            'a + TableList<'a>,
        V: RemoveFromTableList<
            'a,
            K,
            II,
            <<TL::AsRef as TableListSculpt<V, LSI>>::Sculpt as ConsCell>::CAR,
        >,
    {
        let tables = self.tables.as_ref();
        let tables = tables.sculpt().into_car();
        let tables = unsafe { cast_lifetime(&tables) };
        V::remove_from(tables, key);
        self.update_views();
    }
}
