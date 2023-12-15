use cons::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

use crate::{TableList, TableListInsert};

/// A `ConsList` of values that can be inserted into the tables of a `TableList`
pub trait InsertIntoTableList<'a, K, I, TL>
where
    TL: TableList<'a>,
{
    fn insert_into(self, tables: &'a TL, key: K);
}

impl<'a, K, LI, RI, TL, CAR, CDR> InsertIntoTableList<'a, K, Cons<LI, RI>, TL>
    for Cons<Single<CAR>, CDR>
where
    K: Copy,
    TL: TableList<'a> + TableListInsert<'a, K, CAR, LI>,
    CDR: InsertIntoTableList<'a, K, RI, TL>,
{
    fn insert_into(self, table_list: &'a TL, key: K) {
        let (car, cdr) = self.into_destructure();
        car.insert_into(table_list, key);
        cdr.insert_into(table_list, key);
    }
}

impl<'a, K, I, TL, CAR> InsertIntoTableList<'a, K, Single<I>, TL> for Single<CAR>
where
    TL: TableList<'a> + TableListInsert<'a, K, CAR, I>,
{
    fn insert_into(self, table_list: &'a TL, key: K) {
        table_list.insert(key, self.into_car())
    }
}
