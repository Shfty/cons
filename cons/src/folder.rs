pub trait Folder<A, N> {
    type Folded;

    fn fold(&mut self, acc: A, next: N) -> Self::Folded;
}
