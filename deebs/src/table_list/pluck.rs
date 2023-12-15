use cons::{
    cell::{Cons, ConsCell},
    list::append::ConsListAppend,
    single::{ConsSingle, Single},
};

use crate::TableLock;

pub trait TableListPluck<T, I> {
    type Pluck;

    fn pluck(self) -> Self::Pluck;
}

/// length > 2, target in tail
#[allow(clippy::clippy::type_complexity)]
impl<T, I, CAR, CDR> TableListPluck<T, ((I,),)> for Cons<CAR, CDR>
where
    CDR: TableListPluck<T, (I,)>,
    CDR::Pluck: ConsCell,
    CAR: ConsListAppend<<CDR::Pluck as ConsCell>::CDR>,
{
    type Pluck = Cons<
        <CDR::Pluck as ConsCell>::CAR,
        <CAR as ConsListAppend<<CDR::Pluck as ConsCell>::CDR>>::Append,
    >;

    fn pluck(self) -> Self::Pluck {
        let (car, cdr) = self.into_destructure();
        let (head, tail) = cdr.pluck().into_destructure();
        (head, car.append(tail))
    }
}

/// length > 2, target between head and tail
impl<'a, T, CAR, RCAR, RCDR> TableListPluck<T, ((),)> for Cons<CAR, Cons<Single<RCAR>, RCDR>>
where
    CAR: ConsListAppend<RCDR>,
    RCAR: TableLock<'a, Data = T>,
{
    type Pluck = (RCAR, <CAR as ConsListAppend<RCDR>>::Append);

    fn pluck(self) -> Self::Pluck {
        let (car, cdr) = self.into_destructure();
        let (rcar, rcdr) = cdr.into_destructure();
        (rcar.into_car(), car.append(rcdr))
    }
}

/// length > 1, target in head
impl<'a, T, CAR, CDR> TableListPluck<T, ()> for Cons<Single<CAR>, CDR>
where
    CAR: TableLock<'a, Data = T>,
{
    type Pluck = (CAR, CDR);

    fn pluck(self) -> Self::Pluck {
        let (lcar, cdr) = self.into_destructure();
        (lcar.into_car(), cdr)
    }
}

/// length > 1, target in tail
impl<'a, T, CAR, CDR> TableListPluck<T, ((),)> for Cons<CAR, Single<CDR>>
where
    CDR: TableLock<'a, Data = T>,
{
    type Pluck = (CDR, CAR);

    fn pluck(self) -> Self::Pluck {
        let (car, cdr) = self.into_destructure();
        (cdr.into_car(), car)
    }
}

/// length == 1
impl<'a, T, CAR> TableListPluck<T, ()> for Single<CAR>
where
    CAR: TableLock<'a, Data = T>,
{
    type Pluck = (CAR, ());

    fn pluck(self) -> Self::Pluck {
        (self.into_car(), ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cons::list;
    use std::{collections::BTreeMap, sync::RwLock};

    #[test]
    fn test_table_list_pluck() {
        let table_list = list![
            RwLock::new(BTreeMap::<usize, RwLock<i32>>::new()),
            RwLock::new(BTreeMap::<usize, RwLock<f32>>::new()),
            RwLock::new(BTreeMap::<usize, RwLock<char>>::new()),
            RwLock::new(BTreeMap::<usize, RwLock<String>>::new()),
        ];

        let _plucked = TableListPluck::<i32, _>::pluck(table_list);
    }
}
