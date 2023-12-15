use super::*;

mod insert;
mod remove;
mod keys;
mod contains_key;
mod get;

pub use insert::*;
pub use remove::*;
pub use keys::*;
pub use contains_key::*;
pub use get::*;

/// Key-value collection used to store rows
pub trait TableCollection {
    type RowLock: for<'a> RowLock<'a>;
}

impl<K, V> TableCollection for BTreeMap<K, V>
where
    for<'a> V: RowLock<'a>,
{
    type RowLock = V;
}
