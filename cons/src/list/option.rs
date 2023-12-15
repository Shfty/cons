use crate::{cell::{Cons, ConsCell}, single::{ConsSingle, Single}};

/// `ConsList` of `Option` types
pub trait OptionConsList: Default {
    type Values;

    fn all_some(&self) -> bool;
    fn unwrap(self) -> Self::Values;
    fn take(&mut self) -> Self;
}

impl<T, CDR> OptionConsList for Cons<Single<Option<T>>, CDR>
where
    CDR: OptionConsList,
{
    type Values = Cons<Single<T>, CDR::Values>;

    fn all_some(&self) -> bool {
        self.car().car().is_some() && self.cdr().all_some()
    }

    fn unwrap(self) -> Self::Values {
        let (car, cdr) = self.into_destructure();
        (car.unwrap(), cdr.unwrap())
    }

    fn take(&mut self) -> Self {
        let (car, cdr) = self.destructure_mut();
        (car.take(), cdr.take())
    }
}

impl<T> OptionConsList for Single<Option<T>> {
    type Values = Single<T>;

    fn all_some(&self) -> bool {
        self.car().is_some()
    }

    fn unwrap(self) -> Self::Values {
        (self.into_car().unwrap(),)
    }

    fn take(&mut self) -> Self {
        (self.car_mut().take(),)
    }
}
