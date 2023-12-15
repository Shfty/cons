use cons::{
    cell::{Cons, ConsCell},
    single::Single,
};

use cons::list::push_back::ConsListPushBack;

use super::TableListPluck;

pub trait TableListSculptImpl<T, I, A> {
    type SculptImpl: ConsCell;

    fn sculpt_impl(self, acc: A) -> Self::SculptImpl;
}

impl<TCAR, TCDR, ICAR, ICDR, C, A>
    TableListSculptImpl<Cons<Single<TCAR>, TCDR>, Cons<Single<ICAR>, ICDR>, A> for C
where
    C: TableListPluck<TCAR, ICAR>,
    C::Pluck: ConsCell,
    A: ConsListPushBack<<C::Pluck as ConsCell>::CAR>,
    <C::Pluck as ConsCell>::CDR: TableListSculptImpl<TCDR, ICDR, A::PushBack>,
{
    type SculptImpl =
        <<C::Pluck as ConsCell>::CDR as TableListSculptImpl<TCDR, ICDR, A::PushBack>>::SculptImpl;

    fn sculpt_impl(self, acc: A) -> Self::SculptImpl {
        let (car, cdr) = self.pluck().into_destructure();
        cdr.sculpt_impl(acc.push_back(car))
    }
}

impl<TCAR, ICAR, C, A> TableListSculptImpl<Single<TCAR>, Single<ICAR>, A> for C
where
    C: TableListPluck<TCAR, ICAR>,
    C::Pluck: ConsCell,
    A: ConsListPushBack<<C::Pluck as ConsCell>::CAR>,
{
    type SculptImpl = (A::PushBack, <C::Pluck as ConsCell>::CDR);

    fn sculpt_impl(self, acc: A) -> Self::SculptImpl {
        let (car, cdr) = self.pluck().into_destructure();
        (acc.push_back(car), cdr)
    }
}

pub trait TableListSculpt<T, I> {
    type Sculpt: ConsCell;

    fn sculpt(self) -> Self::Sculpt;
}

impl<T, I, C> TableListSculpt<T, I> for C
where
    C: TableListSculptImpl<T, I, ()>,
{
    type Sculpt = C::SculptImpl;

    fn sculpt(self) -> Self::Sculpt {
        self.sculpt_impl(())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, sync::RwLock};

    use super::TableListSculpt;
    use cons::{list, list::as_ref::ConsListAsRef, List};

    #[test]
    fn test_table_list_sculpt() {
        let table_list = list![
            RwLock::new(BTreeMap::<usize, RwLock<i32>>::new()),
            RwLock::new(BTreeMap::<usize, RwLock<f32>>::new()),
            RwLock::new(BTreeMap::<usize, RwLock<char>>::new()),
            RwLock::new(BTreeMap::<usize, RwLock<String>>::new()),
        ];

        let refs = table_list.as_ref();

        let (sculpted, remainder) = TableListSculpt::<List![i32, f32], _>::sculpt(refs);
        println!("Sculpted: {:?}, remainder: {:?}", sculpted, remainder);

        let (sculpted, remainder) = TableListSculpt::<List![char, String], _>::sculpt(refs);
        println!("Sculpted: {:?}, remainder: {:?}", sculpted, remainder);

        let (sculpted, _) = TableListSculpt::<List![String, char, f32, i32], _>::sculpt(refs);
        println!("Sculpted: {:?}, remainder: {:?}", sculpted, remainder);
    }
}
