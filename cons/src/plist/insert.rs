use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

pub trait ConsPListInsert<K, V> {
    type Insert;

    fn insert(self, v: V) -> Self::Insert;
}

impl<K, V, LCAR, LCDR, CDR> ConsPListInsert<K, V> for Cons<Single<LCAR>, Cons<Single<LCDR>, CDR>>
where
    CDR: ConsPListInsert<K, V>,
{
    type Insert = Cons<Single<LCAR>, Cons<Single<LCDR>, CDR::Insert>>;

    fn insert(self, v: V) -> Self::Insert {
        let (lcar, cdr) = self.into_destructure();
        let (lcdr, cdr) = cdr.into_destructure();
        (lcar, (lcdr, cdr.insert(v)))
    }
}

#[allow(clippy::type_complexity)]
impl<K, V, CAR, CDR> ConsPListInsert<K, V> for Cons<Single<CAR>, Single<CDR>>
where
    K: Default,
{
    type Insert = Cons<Single<CAR>, Cons<Single<CDR>, Cons<Single<K>, Single<V>>>>;

    fn insert(self, v: V) -> Self::Insert {
        let (car, cdr) = self.into_destructure();
        (car, (cdr, ((K::default(),), (v,))))
    }
}

impl<K, V> ConsPListInsert<K, V> for ()
where
    K: Default,
{
    type Insert = Cons<Single<K>, Single<V>>;

    fn insert(self, v: V) -> Self::Insert {
        ((K::default(),), (v,))
    }
}

#[cfg(test)]
mod tests {
    use super::ConsPListInsert;
    use crate::plist;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct One;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Two;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Three;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Four;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Five;

    #[test]
    fn test_cons_plist_insert() {
        let cons_plist = plist! {
            typenum::U0::default() => 1,
            typenum::U1::default() => 2.0,
            typenum::U2::default() => '3',
            typenum::U3::default() => "four"
        };

        let cons_plist =
            ConsPListInsert::<typenum::U4, _>::insert(cons_plist, String::from("Five"));

        assert!(
            cons_plist
                == plist![
                    typenum::U0::default() => 1,
                    typenum::U1::default() => 2.0,
                    typenum::U2::default() => '3',
                    typenum::U3::default() => "four",
                    typenum::U4::default() => String::from("Five")
                ]
        );
    }
}
