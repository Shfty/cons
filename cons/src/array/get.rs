use typenum::Unsigned;

use crate::{
    cell::{Cons, ConsCell},
    index::{Here, There},
    list::{get::ConsListGet, ConsList},
    single::Single,
};

use super::ConsArray;

/// A `ConsArray` type that can fetch a row from type-level index `I`
pub trait ConsArrayGet<'a, I, AL: Unsigned, AI>: ConsArray<AL, AI> {
    type Get;

    fn get(&'a self) -> Self::Get;
}

impl<'a, I, AL, AI, CAR, CDR> ConsArrayGet<'a, I, AL, There<AI>> for Cons<CAR, CDR>
where
    AL: Unsigned,
    Self: ConsList,
    CAR: ConsList<Len = AL> + ConsListGet<I>,
    CAR::Get: 'a,
    CDR: ConsArrayGet<'a, I, AL, AI, InnerLen = AL>,
{
    type Get = Cons<Single<&'a CAR::Get>, CDR::Get>;

    fn get(&'a self) -> Self::Get {
        let (car, cdr) = self.destructure();
        ((car.get(),), cdr.get())
    }
}

impl<'a, I, AL, CAR, CDR> ConsArrayGet<'a, I, AL, Here> for Cons<CAR, CDR>
where
    Self: ConsList,
    AL: Unsigned,
    CAR: ConsList<Len = AL> + ConsListGet<I>,
    CAR::Get: 'a,
    CDR: ConsList<Len = AL> + ConsListGet<I>,
    CDR::Get: 'a,
{
    type Get = Cons<Single<&'a CAR::Get>, Single<&'a CDR::Get>>;

    fn get(&'a self) -> Self::Get {
        let (car, cdr) = self.destructure();
        ((car.get(),), (cdr.get(),))
    }
}

#[cfg(test)]
mod tests {
    use super::ConsArrayGet;
    use crate::list;

    #[test]
    fn test_cons_array_get() {
        let cons_array = list![
            list![1, 2, 3, 4],
            list![4.0, 5.0, 6.0, 7.0],
            list!['7', '8', '9', '0']
        ];

        println!("Cons Array: {:?}", cons_array);

        let _proof: &dyn ConsArrayGet<typenum::U1, _, _, Get = _, InnerLen = _> = &cons_array;
        let result = ConsArrayGet::<typenum::U1, _, _>::get(&cons_array);
        println!("Result: {:?}", result);

        let transposed = list![
            ConsArrayGet::<typenum::U0, _, _>::get(&cons_array),
            ConsArrayGet::<typenum::U1, _, _>::get(&cons_array),
            ConsArrayGet::<typenum::U2, _, _>::get(&cons_array),
            ConsArrayGet::<typenum::U3, _, _>::get(&cons_array),
        ];

        println!("Transposed: {:?}", transposed);
    }
}
