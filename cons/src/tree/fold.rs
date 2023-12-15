use crate::{
    cell::{Cons, ConsCell},
    folder::Folder,
    single::{Single, ConsSingle},
};

use super::{Branch, ConsTree, Leaf};

pub trait ConsTreeFold<I, F, A>: ConsTree<I> {
    type Fold;
    type Acc;

    fn fold(self, folder: &mut F, acc: Self::Acc) -> Self::Fold;
}

impl<L, R, F, A, CAR, CDR> ConsTreeFold<Branch<L, R>, F, A> for Cons<CAR, CDR>
where
    CAR: ConsTreeFold<L, F, A, Acc = A>,
    CDR: ConsTreeFold<
        R,
        F,
        <CAR as ConsTreeFold<L, F, A>>::Fold,
        Acc = <CAR as ConsTreeFold<L, F, A>>::Fold,
    >,
{
    type Fold = CDR::Fold;
    type Acc = A;

    fn fold(self, folder: &mut F, acc: Self::Acc) -> Self::Fold {
        let (car, cdr) = self.into_destructure();
        let car_folded = car.fold(folder, acc);
        cdr.fold(folder, car_folded)
    }
}

impl<F, A, CAR> ConsTreeFold<Leaf, F, A> for Single<CAR>
where
    F: Folder<A, CAR>,
{
    type Fold = F::Folded;
    type Acc = A;

    fn fold(self, folder: &mut F, acc: Self::Acc) -> Self::Fold {
        folder.fold(acc, self.into_car())
    }
}

#[cfg(test)]
mod tests {
    use super::ConsTreeFold;
    use crate::{folder::Folder, list};

    struct StringifyFold {
        pub separator: &'static str,
    }

    impl<N> Folder<String, N> for StringifyFold
    where
        N: ToString,
    {
        type Folded = String;

        fn fold(&mut self, acc: String, next: N) -> Self::Folded {
            acc + &next.to_string() + self.separator
        }
    }

    #[test]
    fn test_cons_tree_fold() {
        let cons_tree = list![1, 2.0, '3', "four"];

        let folded = ConsTreeFold::<_, _, _>::fold(
            cons_tree,
            &mut StringifyFold { separator: " " },
            String::new(),
        );

        println!("Folded: {:?}", folded);

        assert!(folded == "1 2 3 four ");
    }
}
