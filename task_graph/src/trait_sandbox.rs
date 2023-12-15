use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
};

use cons::{
    list::futures::{FutureConsListPollAsync, FutureConsListPollSync},
    list::option::OptionConsList,
};

use futures::{future::FusedFuture, Future};

/// Wrapper for a `ConsList` of `Future`s that can be polled asynchronously
pub struct AsyncFutures<T>
where
    T: FutureConsListPollAsync,
{
    futures: T,
    buffer: Option<T::Buffer>,
}

impl<T> AsyncFutures<T>
where
    T: FutureConsListPollAsync,
{
    pub fn new(futures: T) -> Self {
        AsyncFutures {
            futures,
            buffer: Some(Default::default()),
        }
    }
}

impl<T> Deref for AsyncFutures<T>
where
    T: FutureConsListPollAsync,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.futures
    }
}

impl<T> DerefMut for AsyncFutures<T>
where
    T: FutureConsListPollAsync + Unpin,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.futures
    }
}

impl<T> Future for AsyncFutures<T>
where
    T: FutureConsListPollAsync + Unpin,
    T::Buffer: Unpin,
{
    type Output = <T::Buffer as OptionConsList>::Values;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let self_mut = self.get_mut();

        let mut buffer = self_mut.buffer.take().unwrap();
        self_mut.deref_mut().poll_async(&mut buffer, cx);

        if buffer.all_some() {
            Poll::Ready(buffer.unwrap())
        } else {
            self_mut.buffer = Some(buffer);
            Poll::Pending
        }
    }
}

impl<T> FusedFuture for AsyncFutures<T>
where
    T: FutureConsListPollAsync + Unpin,
    T::Buffer: Unpin,
{
    fn is_terminated(&self) -> bool {
        self.buffer.is_none()
    }
}

/// Wrapper for a `ConsList` of `Future`s that can be polled synchronously
pub struct SyncFutures<T>
where
    T: FutureConsListPollSync,
{
    futures: T,
    buffer: Option<T::Buffer>,
}

impl<T> SyncFutures<T>
where
    T: FutureConsListPollSync,
{
    pub fn new(futures: T) -> Self {
        SyncFutures {
            futures,
            buffer: Some(Default::default()),
        }
    }
}

impl<T> Deref for SyncFutures<T>
where
    T: FutureConsListPollSync,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.futures
    }
}

impl<T> DerefMut for SyncFutures<T>
where
    T: FutureConsListPollSync + Unpin,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.futures
    }
}

impl<T> Future for SyncFutures<T>
where
    T: FutureConsListPollSync + Unpin,
    T::Buffer: Unpin,
{
    type Output = <T::Buffer as OptionConsList>::Values;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let self_mut = self.get_mut();

        let mut buffer = self_mut.buffer.take().unwrap();
        self_mut.deref_mut().poll_sync(&mut buffer, cx);

        if buffer.all_some() {
            Poll::Ready(buffer.unwrap())
        } else {
            self_mut.buffer = Some(buffer);
            Poll::Pending
        }
    }
}

impl<T> FusedFuture for SyncFutures<T>
where
    T: FutureConsListPollSync + Unpin,
    T::Buffer: Unpin,
{
    fn is_terminated(&self) -> bool {
        self.buffer.is_none()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use cons::{
        list,
        list::futures::{AsyncFnConsList, FutureConsListFuse, FutureConsListPinMut},
    };

    async fn one() -> i32 {
        println!("One Start");
        async_std::task::sleep(Duration::from_secs_f64(2.0)).await;
        println!("One End");
        1
    }
    async fn two() -> f64 {
        println!("Two Start");
        async_std::task::sleep(Duration::from_secs_f64(1.0)).await;
        println!("Two End");
        2.0
    }
    async fn three() -> char {
        println!("Three Start");
        async_std::task::sleep(Duration::from_secs_f64(0.5)).await;
        println!("Three End");
        '3'
    }

    #[test]
    fn test_trait_sandbox() {
        println!("\nSync:");
        let sync_fn_list = list![one, two, three];
        let _proof: &dyn AsyncFnConsList<_, Run = _> = &sync_fn_list;

        let future_list = sync_fn_list.run();
        let _proof: &dyn FutureConsListFuse<Fuse = _> = &future_list;

        let mut future_list = future_list.fuse();
        let _proof: &dyn FutureConsListPinMut<PinMut = _> = &future_list;

        let future_list = future_list.pin_mut();
        let _proof: &dyn FutureConsListPollAsync<Buffer = _> = &future_list;

        let sync_futures = SyncFutures::new(future_list);

        let sync_result = async_std::task::block_on(sync_futures);
        println!("Result: {:?}", sync_result);

        println!("\nAsync:");
        let async_fn_list = list![
            async_std::task::spawn(one()),
            async_std::task::spawn(two()),
            async_std::task::spawn(three())
        ];

        let mut async_fn_list = async_fn_list.fuse();
        let _proof: &dyn FutureConsListPinMut<PinMut = _> = &async_fn_list;

        let future_list = async_fn_list.pin_mut();
        let _proof: &dyn FutureConsListPollAsync<Buffer = _> = &future_list;

        let async_futures = AsyncFutures::new(future_list);

        let async_result = async_std::task::block_on(async_futures);
        println!("Async Result: {:?}", async_result);
    }
}
