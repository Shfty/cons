use cons::list::as_ref::ConsListAsRef;

use super::DatabaseUpdateViews;
use crate::{Database, TableListRemove, ViewList};

pub trait DatabaseRemove<'a, K, V, I, S, SI> {
    fn remove(&'a self, key: &K);
}

impl<'a, K, V, I, S, SI, TL, VL> DatabaseRemove<'a, K, V, I, S, SI> for Database<TL, VL>
where
    TL: ConsListAsRef<'a> + TableListRemove<'a, K, V, I>,
    VL: ViewList<'a, <TL as ConsListAsRef<'a>>::AsRef, S, SI>,
{
    fn remove(&'a self, key: &K) {
        self.tables.remove(key);
        self.update_views();
    }
}
