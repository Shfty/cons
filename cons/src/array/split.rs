use typenum::Unsigned;

use crate::{
    cell::{Cons, ConsCell},
    index::{Here, There},
    list::{split::ConsListSplit, ConsList},
};

use super::ConsArray;

/// A `ConsArray` type that can split its rows into two at type-level index `I`
pub trait ConsArraySplit<I, AL: Unsigned, AI>: ConsArray<AL, AI> {
    type Split;

    fn split(self) -> Self::Split;
}

#[allow(clippy::clippy::type_complexity)]
impl<I, AL, AI, CAR, CDR> ConsArraySplit<I, AL, There<AI>> for Cons<CAR, CDR>
where
    AL: Unsigned,
    Self: ConsList,
    CAR: ConsList<Len = AL> + ConsListSplit<I>,
    CDR: ConsArraySplit<I, AL, AI, InnerLen = AL>,
    <CAR as ConsListSplit<I>>::Split: ConsCell,
    <CDR as ConsArraySplit<I, AL, AI>>::Split: ConsCell,
{
    type Split = (
        (
            <<CAR as ConsListSplit<I>>::Split as ConsCell>::CAR,
            <<CDR as ConsArraySplit<I, AL, AI>>::Split as ConsCell>::CAR,
        ),
        (
            <<CAR as ConsListSplit<I>>::Split as ConsCell>::CDR,
            <<CDR as ConsArraySplit<I, AL, AI>>::Split as ConsCell>::CDR,
        ),
    );

    fn split(self) -> Self::Split {
        let (car, cdr) = self.into_destructure();
        let (car_head, car_tail) = car.split().into_destructure();
        let (cdr_head, cdr_tail) = cdr.split().into_destructure();
        ((car_head, cdr_head), (car_tail, cdr_tail))
    }
}

#[allow(clippy::clippy::type_complexity)]
impl<I, AL, CAR, CDR> ConsArraySplit<I, AL, Here> for Cons<CAR, CDR>
where
    Self: ConsList,
    AL: Unsigned,
    CAR: ConsList<Len = AL> + ConsListSplit<I>,
    CDR: ConsList<Len = AL> + ConsListSplit<I>,
    CAR::Split: ConsCell,
    CDR::Split: ConsCell,
{
    type Split = (
        (
            <<CAR as ConsListSplit<I>>::Split as ConsCell>::CAR,
            <<CDR as ConsListSplit<I>>::Split as ConsCell>::CAR,
        ),
        (
            <<CAR as ConsListSplit<I>>::Split as ConsCell>::CDR,
            <<CDR as ConsListSplit<I>>::Split as ConsCell>::CDR,
        ),
    );

    fn split(self) -> Self::Split {
        let (car, cdr) = self.into_destructure();
        let (car_head, car_tail) = car.split().into_destructure();
        let (cdr_head, cdr_tail) = cdr.split().into_destructure();
        ((car_head, cdr_head), (car_tail, cdr_tail))
    }
}

#[cfg(test)]
mod tests {
    use super::ConsArraySplit;
    use crate::list;

    #[test]
    fn test_cons_array_split() {
        let cons_array = list![
            list![1, 2, 3, 4],
            list![4.0, 5.0, 6.0, 7.0],
            list!['7', '8', '9', '0']
        ];

        println!("Cons Array: {:?}", cons_array);

        let (head, cons_array) = ConsArraySplit::<typenum::U0, _, _>::split(cons_array);

        println!("Head: {:?}\nTail:{:?}", head, cons_array);
    }
}
