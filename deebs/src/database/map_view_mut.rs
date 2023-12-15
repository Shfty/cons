use std::sync::RwLock;

use cons::{
    cell::ConsCell,
    list::{
        append::ConsListAppend, as_ref::ConsListAsRef, deref::ConsListDeref,
        deref_mut::ConsListDerefMut, find::ConsListFind,
    },
};

use crate::{
    cast_lifetime, DatabaseMapKeysMut, DatabaseViewKeys, RowLockList, TableList, TableListSculpt,
    TableReadGuardList, ViewKeys,
};

use super::View;

pub trait DatabaseMapViewMut<
    'a,
    Key,
    ViewSculpt,
    ViewFindIndex,
    Views,
    Tables,
    ImmutableSculptIndex,
    MutableSculptIndex,
    Func,
>:
    DatabaseViewKeys<Key, ViewFindIndex, ViewSculpt, Views>
    + DatabaseMapKeysMut<
        Key,
        std::slice::Iter<'a, Key>,
        ImmutableSculptIndex,
        MutableSculptIndex,
        Func,
        Tables,
    > where
    Key: 'a + Copy,
{
    fn map_view_mut<ImmutableSignature, MutableSignature>(&'a self, f: Func)
    where
        // DatabaseViewKeys
        ImmutableSignature: 'a + ConsListAppend<MutableSignature>,
        MutableSignature: 'a,
        ViewSculpt: 'a,
        Views: ConsListAsRef<'a>,
        <Views as ConsListAsRef<'a>>::AsRef: ConsListFind<
            &'a RwLock<
                ViewKeys<usize, <ImmutableSignature as ConsListAppend<MutableSignature>>::Append>,
            >,
            ViewFindIndex,
            Find = &'a std::sync::RwLock<ViewKeys<Key, ViewSculpt>>,
        >,

        Tables: 'a,
        <Tables as TableList<'a>>::ReadGuards: TableReadGuardList<'a, Key>,
        Tables: TableList<'a> + ConsListAsRef<'a>,
        <Tables as ConsListAsRef<'a>>::AsRef: Copy,

        // Immutable
        ImmutableSculptIndex: 'a,
        <Tables as ConsListAsRef<'a>>::AsRef:
            TableListSculpt<ImmutableSignature, ImmutableSculptIndex>,
        <<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<
            ImmutableSignature,
            ImmutableSculptIndex,
        >>::Sculpt as ConsCell>::CAR: TableList<'a>,
        <<<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<
            ImmutableSignature,
            ImmutableSculptIndex,
        >>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards: TableReadGuardList<'a, Key>,
        <<<<<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<
            ImmutableSignature,
            ImmutableSculptIndex,
        >>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<
            'a,
            Key,
        >>::Get as RowLockList<'a>>::ReadGuards: ConsListDeref<'a>,

        // Mutable
        MutableSculptIndex: 'a,
        <Tables as ConsListAsRef<'a>>::AsRef: TableListSculpt<MutableSignature, MutableSculptIndex>,
        <<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<
            MutableSignature,
            MutableSculptIndex,
        >>::Sculpt as ConsCell>::CAR: TableList<'a>,
        <<<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<
            MutableSignature,
            MutableSculptIndex,
        >>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards: TableReadGuardList<'a, Key>,
        <<<<<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<
            MutableSignature,
            MutableSculptIndex,
        >>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<
            'a,
            Key,
        >>::Get as RowLockList<'a>>::WriteGuards: ConsListDerefMut<'a>,

        // Function
        Func: FnMut(
            &Key,
            <<<<<<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<
                ImmutableSignature,
                ImmutableSculptIndex,
            >>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<
                'a,
                Key,
            >>::Get as RowLockList<'a>>::ReadGuards as ConsListDeref<'a>>::Deref,
            <<<<<<<Tables as ConsListAsRef<'a>>::AsRef as TableListSculpt<
                MutableSignature,
                MutableSculptIndex,
            >>::Sculpt as ConsCell>::CAR as TableList<'a>>::ReadGuards as TableReadGuardList<
                'a,
                Key,
            >>::Get as RowLockList<'a>>::WriteGuards as ConsListDerefMut<'a>>::DerefMut,
        ),
    {
        let keys = self.view_keys::<&View<ImmutableSignature::Append>>();
        let keys = unsafe { cast_lifetime(&keys) };
        self.map_keys_mut::<ImmutableSignature, MutableSignature>(keys.iter(), f);
    }
}

impl<
        'a,
        Key,
        ViewSculpt,
        ViewFindIndex,
        Views,
        Tables,
        ImmutableSculptIndex,
        MutableSculptIndex,
        Func,
        T,
    >
    DatabaseMapViewMut<
        'a,
        Key,
        ViewSculpt,
        ViewFindIndex,
        Views,
        Tables,
        ImmutableSculptIndex,
        MutableSculptIndex,
        Func,
    > for T
where
    Key: 'a + Copy,
    T: DatabaseViewKeys<Key, ViewFindIndex, ViewSculpt, Views>
        + DatabaseMapKeysMut<
            Key,
            std::slice::Iter<'a, Key>,
            ImmutableSculptIndex,
            MutableSculptIndex,
            Func,
            Tables,
        >,
{
}
