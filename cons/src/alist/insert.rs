use crate::{
    cell::{Cons, ConsCell},
    single::{Single, ConsSingle},
};

pub trait ConsAListInsert<K, V> {
    type Insert;

    fn insert(self, v: V) -> Self::Insert;
}

impl<K, V, CAR, CDR> ConsAListInsert<K, V> for Cons<CAR, CDR>
where
    CDR: ConsAListInsert<K, V>,
{
    type Insert = Cons<CAR, CDR::Insert>;

    fn insert(self, v: V) -> Self::Insert {
        let (car, cdr) = self.into_destructure();
        (car, cdr.insert(v))
    }
}

#[allow(clippy::type_complexity)]
impl<K, V, CAR, CDR> ConsAListInsert<K, V> for Single<Cons<CAR, CDR>>
where
    K: Default,
{
    type Insert = Cons<Single<Cons<CAR, CDR>>, Single<Cons<K, V>>>;

    fn insert(self, v: V) -> Self::Insert {
        ((self.into_car(),), ((K::default(), v),))
    }
}

impl<K, V> ConsAListInsert<K, V> for ()
where
    K: Default,
{
    type Insert = Single<Cons<K, V>>;

    fn insert(self, v: V) -> Self::Insert {
        ((K::default(), v),)
    }
}

#[cfg(test)]
#[allow(clippy::unit_arg)]
mod tests {
    use super::ConsAListInsert;
    use crate::alist;

    #[test]
    fn test_cons_alist_insert() {
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Float;

        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Integer;

        let cons_alist = alist! {};
        println!("AList: {:?}", cons_alist);

        let cons_alist = cons_alist.insert(2.0);
        println!("AList: {:?}", cons_alist);

        assert!(cons_alist == alist! { Float => 2.0 });

        let cons_alist = cons_alist.insert(3);
        println!("AList: {:?}", cons_alist);

        assert!(cons_alist == alist! { Float => 2.0, Integer => 3 });
    }
}
