use super::*;

/// List of valid common keys for a given set of rows
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewKeys<K, S> {
    keys: Vec<K>,
    _phantom: PhantomData<S>,
}

impl<K, S> ViewKeys<K, S> {
    pub fn update<'a, TL>(&mut self, table_list: TL)
    where
        K: 'a + Copy + Ord + Eq,
        TL: 'a + TableListCommonKeys<'a, K>,
    {
        let table_list = unsafe { cast_lifetime(&table_list) };
        self.keys = table_list.common_keys().into_iter().copied().collect()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.iter()
    }
}

/// Cons list of views
pub trait ViewList<'a, TL, S, SI> {
    fn update(&self, table_list: TL);
}

impl<'a, TL, SCAR, SCDR, SICAR, SICDR, K, CDR>
    ViewList<'a, TL, Cons<SCAR, SCDR>, Cons<SICAR, SICDR>>
    for Cons<Single<RwLock<ViewKeys<K, SCAR>>>, CDR>
where
    K: 'a + Copy + Ord + Eq,
    TL: Copy,
    TL: TableListSculpt<SCAR, SICAR>,
    <TL::Sculpt as ConsCell>::CAR: 'a + TableListKeys<'a, K>,
    <TL::Sculpt as ConsCell>::CAR: TableListCommonKey<'a, K>,
    CDR: ViewList<'a, TL, SCDR, SICDR>,
{
    fn update(&self, table_list: TL) {
        let (car, cdr) = self.destructure();
        let car = car.car();
        let car_sculpt = table_list.sculpt().into_car();
        car.write().unwrap().update(car_sculpt);
        cdr.update(table_list);
    }
}

impl<'a, K, TL, S, SI> ViewList<'a, TL, S, SI> for Single<RwLock<ViewKeys<K, S>>>
where
    K: 'a + Copy + Ord + Eq,
    TL: Copy,
    TL: 'a + TableListSculpt<S, SI>,
    <TL::Sculpt as ConsCell>::CAR: 'a + TableListKeys<'a, K>,
    <TL::Sculpt as ConsCell>::CAR: TableListCommonKey<'a, K>,
{
    fn update(&self, table_list: TL) {
        self.car()
            .write()
            .unwrap()
            .update(table_list.sculpt().into_car());
    }
}

impl<'a, TL, S, SI> ViewList<'a, TL, S, SI> for () {
    fn update(&self, _: TL) {}
}
