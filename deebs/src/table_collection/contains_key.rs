use std::collections::BTreeMap;

use crate::RowLock;

use super::TableCollection;

pub trait TableCollectionContainsKey<K, V>: TableCollection {
    fn contains_key(&self, key: &K) -> bool;
}

impl<K, V, L> TableCollectionContainsKey<K, V> for BTreeMap<K, L>
where
    K: Ord + Eq,
    for<'a> L: RowLock<'a, Data = V> + From<V>,
{
    fn contains_key(&self, key: &K) -> bool {
        BTreeMap::contains_key(self, key)
    }
}
