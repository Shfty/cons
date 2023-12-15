use crate::{Database, TableListKeys};

pub trait DatabaseAllKeys<'a, K, TL>
where
    K: 'a,
    TL: TableListKeys<'a, K>,
{
    fn all_keys(&'a self) -> TL::Keys;
}

impl<'a, K, TL, VL> DatabaseAllKeys<'a, K, TL> for Database<TL, VL>
where
    K: 'a,
    TL: TableListKeys<'a, K>,
{
    fn all_keys(&'a self) -> TL::Keys {
        self.tables.keys()
    }
}
