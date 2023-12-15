use super::DatabaseUpdateViews;
use crate::{Database, TableListInsert};

pub trait DatabaseInsert<'a, K, V, I, S, SI>: DatabaseUpdateViews<'a, S, SI> {
    fn insert(&'a self, key: K, value: V);
}

impl<'a, K, V, I, S, SI, TL, VL> DatabaseInsert<'a, K, V, I, S, SI> for Database<TL, VL>
where
    Self: DatabaseUpdateViews<'a, S, SI>,
    TL: TableListInsert<'a, K, V, I>,
{
    fn insert(&'a self, key: K, value: V) {
        self.tables.insert(key, value);
        self.update_views();
    }
}
