use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

use super::{pluck::ConsListPluck, push_back::ConsListPushBack};

pub trait ConsListSculptImpl<T, I, A> {
    type SculptImpl: ConsCell;

    fn sculpt_impl(self, acc: A) -> Self::SculptImpl;
}

impl<TCAR, TCDR, ICAR, ICDR, C, A>
    ConsListSculptImpl<Cons<Single<TCAR>, TCDR>, Cons<Single<ICAR>, ICDR>, A> for C
where
    C: ConsListPluck<TCAR, ICAR>,
    C::Pluck: ConsCell,
    A: ConsListPushBack<<C::Pluck as ConsCell>::CAR>,
    <C::Pluck as ConsCell>::CDR: ConsListSculptImpl<TCDR, ICDR, A::PushBack>,
{
    type SculptImpl =
        <<C::Pluck as ConsCell>::CDR as ConsListSculptImpl<TCDR, ICDR, A::PushBack>>::SculptImpl;

    fn sculpt_impl(self, acc: A) -> Self::SculptImpl {
        let (car, cdr) = self.pluck().into_destructure();
        cdr.sculpt_impl(acc.push_back(car))
    }
}

impl<TCAR, ICAR, C, A> ConsListSculptImpl<Single<TCAR>, Single<ICAR>, A> for C
where
    C: ConsListPluck<TCAR, ICAR>,
    C::Pluck: ConsCell,
    A: ConsListPushBack<<C::Pluck as ConsCell>::CAR>,
{
    type SculptImpl = (A::PushBack, <C::Pluck as ConsCell>::CDR);

    fn sculpt_impl(self, acc: A) -> Self::SculptImpl {
        let (car, cdr) = self.pluck().into_destructure();
        (acc.push_back(car), cdr)
    }
}

pub trait ConsListSculpt<T, I> {
    type Sculpt: ConsCell;

    fn sculpt(self) -> Self::Sculpt;
}

impl<T, I, C> ConsListSculpt<T, I> for C
where
    C: ConsListSculptImpl<T, I, ()>,
{
    type Sculpt = C::SculptImpl;

    fn sculpt(self) -> Self::Sculpt {
        self.sculpt_impl(())
    }
}

#[cfg(test)]
mod tests {
    use super::ConsListSculpt;
    use crate::{list, List};

    #[test]
    fn test_cons_list_sculpt() {
        let cons_list = list![1, 2.0, '3', "four"];

        let (sculpted, remainder) = ConsListSculpt::<List![i32, f32], _>::sculpt(cons_list);
        assert!(sculpted == list![1, 2.0] && remainder == list!['3', "four"]);

        let (sculpted, remainder) = ConsListSculpt::<List![char, &str], _>::sculpt(cons_list);
        assert!(sculpted == list!['3', "four"], remainder == list![1, 2.0]);

        let (sculpted, _) = ConsListSculpt::<List![&str, char, f32, i32], _>::sculpt(cons_list);
        assert!(sculpted == list!["four", '3', 2.0, 1]);
    }
}
