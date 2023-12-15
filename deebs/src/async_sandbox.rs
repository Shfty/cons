use async_std::{stream::FromIter, sync::RwLock};
use async_trait::async_trait;
use std::{collections::btree_map::Keys, ops::Deref};
use std::{collections::BTreeMap, fmt::Debug};

#[async_trait]
trait TableLock<'a, K, V> {
    type Read;

    async fn read(&'a self) -> Self::Read;
}

#[async_trait]
impl<'a, K, V, T> TableLock<'a, K, V> for RwLock<T>
where
    T: 'a + TableCollection<'a, K, V> + Send + Sync,
    K: 'a + Ord + Eq + Send + Sync,
    V: 'a + Debug + Unpin + Send + Sync,
{
    type Read = async_std::sync::RwLockReadGuard<'a, T>;

    async fn read(&'a self) -> Self::Read {
        RwLock::read(self).await
    }
}

trait TableCollection<'a, K, V> {
    type Keys;
    type Get;

    fn keys(&'a self) -> Self::Keys;
    fn get(&'a self, key: &K) -> Option<Self::Get>;
}

impl<'a, K, V> TableCollection<'a, K, V> for BTreeMap<K, RwLock<V>>
where
    K: 'a + Ord + Eq + Send + Sync,
    V: 'a + Send + Sync,
{
    type Keys = Keys<'a, K, RwLock<V>>;
    type Get = &'a RwLock<V>;

    fn keys(&'a self) -> Self::Keys {
        BTreeMap::keys(self)
    }

    fn get(&'a self, key: &K) -> Option<Self::Get> {
        BTreeMap::get(self, key)
    }
}
#[async_trait]
trait RowLock<'a> {
    type Read;

    async fn read(&'a self) -> Self::Read;
}

#[async_trait]
impl<'a, T> RowLock<'a> for RwLock<T>
where
    T: 'a + Send + Sync,
{
    type Read = async_std::sync::RwLockReadGuard<'a, T>;

    async fn read(&'a self) -> Self::Read {
        RwLock::read(self).await
    }
}

#[cfg(test)]
mod tests {
    use futures::{stream::FuturesUnordered, StreamExt};

    use super::*;

    async fn async_sandbox() {
        let row_lock_a = RwLock::new(1);
        let row_lock_b = RwLock::new(2);
        let row_lock_c = RwLock::new(3);

        let table_collection: BTreeMap<usize, RwLock<i32>> =
            vec![(0, row_lock_a), (1, row_lock_b), (2, row_lock_c)]
                .into_iter()
                .collect();

        let table_lock = RwLock::new(table_collection);

        let table_guard = TableLock::read(&table_lock).await;
        for key in TableCollection::keys(table_guard.deref()) {
            let row_lock = TableCollection::get(table_guard.deref(), key).unwrap();
            let row_guard = RowLock::read(row_lock).await;
            let value = row_guard.deref();
            println!("Key: {:?}, Value: {:?}", key, value);
        }
    }

    #[test]
    fn test_async_sandbox() {
        async_std::task::block_on(async_sandbox())
    }
}
