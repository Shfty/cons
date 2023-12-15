use std::sync::RwLock;

use cons::list::{as_ref::ConsListAsRef, find::ConsListFind};

use crate::{Database, ViewKeys};

pub trait DatabaseViewKeys<K, FI, S, VL> {
    fn view_keys<'a, F>(&'a self) -> Vec<K>
    where
        K: 'a + Copy,
        S: 'a,
        VL: ConsListAsRef<'a>,
        VL::AsRef: ConsListFind<F, FI, Find = &'a RwLock<ViewKeys<K, S>>>;
}

impl<K, FI, S, TL, VL> DatabaseViewKeys<K, FI, S, VL> for Database<TL, VL> {
    fn view_keys<'a, F>(&'a self) -> Vec<K>
    where
        K: 'a + Copy,
        S: 'a,
        VL: ConsListAsRef<'a>,
        VL::AsRef: ConsListFind<F, FI, Find = &'a RwLock<ViewKeys<K, S>>>,
    {
        self.views
            .as_ref()
            .find()
            .read()
            .unwrap()
            .keys()
            .copied()
            .collect()
    }
}
