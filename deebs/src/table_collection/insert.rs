use std::collections::BTreeMap;

use crate::RowLock;

use super::TableCollection;

pub trait TableCollectionInsert<K, V>: TableCollection {
    fn insert(&mut self, key: K, value: V);
}

impl<K, V, L> TableCollectionInsert<K, V> for BTreeMap<K, L>
where
    K: Ord + Eq,
    for<'a> L: RowLock<'a, Data = V> + From<V>,
{
    fn insert(&mut self, key: K, value: V) {
        BTreeMap::insert(self, key, From::from(value));
    }
}
