mod async_fn;
mod fuse;
mod pin_mut;
mod poll_async;
mod poll_sync;

pub use async_fn::*;
pub use fuse::*;
use futures::Future;
pub use pin_mut::*;
pub use poll_async::*;
pub use poll_sync::*;

use crate::{cell::Cons, single::Single};

use super::option::OptionConsList;

pub trait FutureConsList {
    type Buffer: OptionConsList;
}

impl<F, CDR> FutureConsList for Cons<Single<F>, CDR>
where
    F: Future,
    CDR: FutureConsList,
{
    type Buffer = Cons<Single<Option<<F as Future>::Output>>, CDR::Buffer>;
}

impl<F> FutureConsList for Single<F>
where
    F: Future,
{
    type Buffer = Single<Option<<F as Future>::Output>>;
}
