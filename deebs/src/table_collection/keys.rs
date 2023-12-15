use std::collections::BTreeMap;

use crate::RowLock;

use super::TableCollection;

pub trait TableCollectionKeys<'a, K>: TableCollection {
    type Keys;

    fn keys(&'a self) -> Self::Keys;
}

impl<'a, K, V, L> TableCollectionKeys<'a, K> for BTreeMap<K, L>
where
    K: 'a + Ord + Eq,
    for<'b> L: 'a + RowLock<'b, Data = V> + From<V>,
{
    type Keys = std::collections::btree_map::Keys<'a, K, L>;
    fn keys(&'a self) -> Self::Keys {
        BTreeMap::keys(self)
    }
}
