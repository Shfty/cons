use cons::{cell::Cons, single::Single};

use crate::{TableList, TableListRemove};

/// A `ConsList` that can remove values from a `TableList` based on its types
pub trait RemoveFromTableList<'a, K, I, TL>
where
    TL: TableList<'a>,
{
    fn remove_from(table_list: &'a TL, key: &K);
}

impl<'a, K, LI, RI, TL, CAR, CDR> RemoveFromTableList<'a, K, Cons<LI, RI>, TL>
    for Cons<Single<CAR>, CDR>
where
    K: Copy,
    TL: TableList<'a> + TableListRemove<'a, K, CAR, LI>,
    CDR: RemoveFromTableList<'a, K, RI, TL>,
{
    fn remove_from(table_list: &'a TL, key: &K) {
        table_list.remove(key);
        CDR::remove_from(table_list, key);
    }
}

impl<'a, K, I, TL, CAR> RemoveFromTableList<'a, K, Single<I>, TL> for Single<CAR>
where
    TL: TableList<'a> + TableListRemove<'a, K, CAR, I>,
{
    fn remove_from(table_list: &'a TL, key: &K) {
        table_list.remove(key)
    }
}
