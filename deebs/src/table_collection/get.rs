use std::collections::BTreeMap;

use crate::RowLock;

use super::TableCollection;

pub trait TableCollectionGet<K>: TableCollection {
    fn get(&self, key: &K) -> &Self::RowLock;
}

impl<K, V> TableCollectionGet<K> for BTreeMap<K, V>
where
    K: Ord + Eq,
    for<'a> V: RowLock<'a>,
{
    fn get(&self, key: &K) -> &Self::RowLock {
        BTreeMap::get(self, key).unwrap()
    }
}
