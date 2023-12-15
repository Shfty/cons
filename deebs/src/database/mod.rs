mod all_keys;
mod insert;
mod insert_list;
mod map_keys;
mod map_keys_mut;
mod map_view;
mod map_view_mut;
mod remove;
mod remove_list;
mod update_views;
mod view_keys;

pub use all_keys::*;
pub use insert::*;
pub use insert_list::*;
pub use map_keys::*;
pub use map_keys_mut::*;
pub use map_view::*;
pub use map_view_mut::*;
pub use remove::*;
pub use remove_list::*;
pub use update_views::*;
pub use view_keys::*;

use cons::list::push_back::ConsListPushBack;
use std::{fmt::Debug, sync::RwLock};

use crate::ViewKeys;

pub type View<T> = RwLock<ViewKeys<usize, T>>;

/// Collection of data tables and views
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Database<TL, VL> {
    tables: TL,
    views: VL,
}

impl<TL, VL> Database<TL, VL> {
    pub fn add_table<T>(self) -> Database<TL::PushBack, VL>
    where
        T: Default,
        TL: ConsListPushBack<T>,
    {
        let tables = self.tables.push_back(T::default());
        Database {
            tables,
            views: self.views,
        }
    }

    pub fn add_view<T>(self) -> Database<TL, VL::PushBack>
    where
        T: Default,
        VL: ConsListPushBack<T>,
    {
        let views = self.views.push_back(T::default());
        Database {
            tables: self.tables,
            views,
        }
    }
}
