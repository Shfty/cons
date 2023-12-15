use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

use super::ConsList;

/// A `ConsList` type that can replace its terminating element with some value (ex. another `ConsList` type)
pub trait ConsListAppend<C> {
    type Append;

    fn append(self, c: C) -> Self::Append;
}

impl<C, CAR, CDR> ConsListAppend<C> for Cons<Single<CAR>, CDR>
where
    C: ConsList,
    CDR: ConsListAppend<C>,
{
    type Append = ((CAR,), CDR::Append);

    fn append(self, c: C) -> Self::Append {
        let (car, cdr) = self.into_destructure();
        (car, cdr.append(c))
    }
}

impl<C, CAR> ConsListAppend<C> for Single<CAR> {
    type Append = ((CAR,), C);

    fn append(self, c: C) -> Self::Append {
        (self, c)
    }
}

impl<C> ConsListAppend<C> for () {
    type Append = C;

    fn append(self, c: C) -> Self::Append {
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_cons_list_append() {
        println!();
        let cons_list = list![1, 2.0, '3', "four"];
        println!("Cons List: {:?}", cons_list);
        let cons_list = cons_list.append(list![String::from("Five"), 6, 7.0, '8']);
        println!("Appended: {:?}", cons_list);
        assert!(cons_list == list![1, 2.0, '3', "four", String::from("Five"), 6, 7.0, '8']);
    }
}
