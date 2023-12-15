use cons::list::as_ref::ConsListAsRef;

use crate::{Database, ViewList};

pub trait DatabaseUpdateViews<'a, S, SI> {
    fn update_views(&'a self);
}

impl<'a, S, SI, TL, VL> DatabaseUpdateViews<'a, S, SI> for Database<TL, VL>
where
    TL: 'a + ConsListAsRef<'a>,
    TL::AsRef: 'a,
    VL: ViewList<'a, TL::AsRef, S, SI>,
{
    fn update_views(&'a self) {
        let tables = self.tables.as_ref();
        self.views.update(tables)
    }
}
