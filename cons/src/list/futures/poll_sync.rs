use std::task::{Context, Poll};

use futures::{future::FusedFuture, Future, FutureExt};

use crate::{
    cell::{Cons, ConsCell},
    list::set::ConsListSet,
    single::{ConsSingle, Single},
};

use super::FutureConsList;

/// `ConsList` of `Future`s that can be polled synchronously
pub trait FutureConsListPollSync: FutureConsList {
    fn poll_sync(&mut self, buf: &mut Self::Buffer, cx: &mut Context<'_>);
}

impl<F, CDR> FutureConsListPollSync for Cons<Single<F>, CDR>
where
    Self: FutureConsList<Buffer = Cons<Single<Option<<F as Future>::Output>>, CDR::Buffer>>,
    F: FusedFuture + Unpin,
    CDR: FutureConsListPollSync,
{
    fn poll_sync(&mut self, buf: &mut Self::Buffer, cx: &mut Context<'_>) {
        let (car, cdr) = self.destructure_mut();
        let car = car.car_mut();
        if car.is_terminated() {
            cdr.poll_sync(buf.cdr_mut(), cx);
        } else if let Poll::Ready(result) = car.poll_unpin(cx) {
            buf.set(Some(result));
            cdr.poll_sync(buf.cdr_mut(), cx);
        }
    }
}

impl<F> FutureConsListPollSync for Single<F>
where
    Self: FutureConsList<Buffer = Single<Option<<F as Future>::Output>>>,
    F: FusedFuture + Unpin,
{
    fn poll_sync(&mut self, buf: &mut Self::Buffer, cx: &mut Context<'_>) {
        let car = self.car_mut();
        if !car.is_terminated() {
            if let Poll::Ready(result) = car.poll_unpin(cx) {
                buf.set(Some(result))
            }
        }
    }
}
