use crate::{
    cell::{Cons, ConsCell},
    single::Single,
};

pub trait ConsListPushBack<T> {
    type PushBack;

    fn push_back(self, value: T) -> Self::PushBack;
}

impl<T, CAR, CDR> ConsListPushBack<T> for Cons<Single<CAR>, CDR>
where
    CDR: ConsListPushBack<T>,
{
    type PushBack = ((CAR,), CDR::PushBack);

    fn push_back(self, value: T) -> Self::PushBack {
        let (car, cdr) = self.into_destructure();
        (car, cdr.push_back(value))
    }
}

impl<T, CAR> ConsListPushBack<T> for Single<CAR> {
    type PushBack = ((CAR,), (T,));

    fn push_back(self, value: T) -> Self::PushBack {
        (self, (value,))
    }
}

impl<T> ConsListPushBack<T> for () {
    type PushBack = (T,);

    fn push_back(self, value: T) -> Self::PushBack {
        (value,)
    }
}

#[cfg(test)]
mod tests {
    use super::ConsListPushBack;
    use crate::list;

    #[test]
    fn test_cons_list_push_back() {
        let cons_list = list![1, 2.0, '3', "four"];
        let cons_list = cons_list.push_back(String::from("Five"));
        assert!(cons_list == list![1, 2.0, '3', "four", String::from("Five")]);
    }
}
