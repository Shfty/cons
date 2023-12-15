use crate::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};

use super::append::ConsListAppend;

pub trait ConsListPluck<T, I> {
    type Pluck;

    fn pluck(self) -> Self::Pluck;
}

/// length > 2, target in tail
#[allow(clippy::clippy::type_complexity)]
impl<T, I, CAR, CDR> ConsListPluck<T, ((I,),)> for Cons<CAR, CDR>
where
    CDR: ConsListPluck<T, (I,)>,
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
impl<CAR, RCAR, RCDR> ConsListPluck<RCAR, ((),)> for Cons<CAR, Cons<Single<RCAR>, RCDR>>
where
    CAR: ConsListAppend<RCDR>,
{
    type Pluck = (RCAR, <CAR as ConsListAppend<RCDR>>::Append);

    fn pluck(self) -> Self::Pluck {
        let (car, cdr) = self.into_destructure();
        let (rcar, rcdr) = cdr.into_destructure();
        (rcar.into_car(), car.append(rcdr))
    }
}

/// length > 1, target in head
impl<CAR, CDR> ConsListPluck<CAR, ()> for Cons<Single<CAR>, CDR> {
    type Pluck = (CAR, CDR);

    fn pluck(self) -> Self::Pluck {
        let (lcar, cdr) = self.into_destructure();
        (lcar.into_car(), cdr)
    }
}

/// length > 1, target in tail
impl<CAR, CDR> ConsListPluck<CDR, ((),)> for Cons<CAR, Single<CDR>> {
    type Pluck = (CDR, CAR);

    fn pluck(self) -> Self::Pluck {
        let (car, cdr) = self.into_destructure();
        (cdr.into_car(), car)
    }
}

/// length == 1
impl<CAR> ConsListPluck<CAR, ()> for Single<CAR> {
    type Pluck = (CAR, ());

    fn pluck(self) -> Self::Pluck {
        (self.into_car(), ())
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::ConsListPluck;
    use crate::list;

    #[test]
    fn test_cons_list_pluck() {
        let cons_list = list![1, 2.0, '3', "four"];

        let (int, rem) = ConsListPluck::<i32, _>::pluck(cons_list);
        assert!(int == 1, rem == list![2.0, '3', "four"]);

        let (float, rem) = ConsListPluck::<f64, _>::pluck(cons_list);
        assert!(float.partial_cmp(&2.0).unwrap() == Ordering::Equal, rem == list![1, '3', "four"]);

        let (char, rem) = ConsListPluck::<char, _>::pluck(cons_list);
        assert!(char == '3', rem == list![1, 2.0, "four"]);

        let (string, rem) = ConsListPluck::<&str, _>::pluck(cons_list);
        assert!(string == "four", rem == list![1, 2.0, '3']);

        let (int, cons_list) = ConsListPluck::<i32, _>::pluck(cons_list);
        assert!(int == 1, cons_list == list![2.0, '3', "four"]);

        let (float, cons_list) = ConsListPluck::<f64, _>::pluck(cons_list);
        assert!(float.partial_cmp(&2.0).unwrap() == Ordering::Equal, cons_list == list!['3', "four"]);

        let (char, cons_list) = ConsListPluck::<char, _>::pluck(cons_list);
        assert!(char == '3', cons_list == list!["four"]);

        let (string, _) = ConsListPluck::<&str, _>::pluck(cons_list);
        assert!(string == "four");
    }
}
