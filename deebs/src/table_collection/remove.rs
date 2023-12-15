use std::collections::BTreeMap;

use crate::RowLock;

use super::TableCollection;

pub trait TableCollectionRemove<K, V>: TableCollection {
    fn remove(&mut self, key: &K);
}

impl<K, V, L> TableCollectionRemove<K, V> for BTreeMap<K, L>
where
    K: Ord + Eq,
    for<'a> L: RowLock<'a, Data = V> + From<V>,
{
    fn remove(&mut self, key: &K) {
        BTreeMap::remove(self, key);
    }
}
