//! Iterator vs Stream: the same shape, with "not yet" added to the answer.
//!
//!   rustc --edition 2024 iterator_vs_stream.rs -o /tmp/ivs && /tmp/ivs
//!
//! Everything here is std-only and deterministic: no runtime, no timers, and a
//! `Waker::noop()` executor (stable since 1.85) that polls in a tight loop.

use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::task::{Context, Poll, Waker};

/// What `futures::Stream` is, written out. Not in std on stable: the eventual
/// std name is `AsyncIterator`, still unstable (tracking issue 79024).
trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

/// A source that is "not ready" once before each item — a stand-in for a socket
/// with nothing in the buffer yet.
struct Ticks {
    left: u32,
    stalled: bool,
    polls: &'static AtomicU32,
}

impl Stream for Ticks {
    type Item = u32;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<u32>> {
        let me = self.get_mut();
        me.polls.fetch_add(1, Relaxed);
        if me.left == 0 {
            return Poll::Ready(None);
        }
        if !me.stalled {
            me.stalled = true;
            // A real stream would register `cx.waker()` here and return.
            return Poll::Pending;
        }
        me.stalled = false;
        me.left -= 1;
        Poll::Ready(Some(me.left + 1))
    }
}

/// The `.next()` every Stream extension trait provides, spelled out: a future
/// that polls the stream once per poll of itself.
struct Next<'a, S>(&'a mut S);

impl<'a, S: Stream + Unpin> Future for Next<'a, S> {
    type Output = Option<S::Item>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        Pin::new(&mut *self.get_mut().0).poll_next(cx)
    }
}

fn next<S: Stream + Unpin>(s: &mut S) -> Next<'_, S> {
    Next(s)
}

/// The smallest executor that works: poll until Ready. A real one parks the
/// thread and lets the waker wake it; with `Waker::noop` there is nobody to
/// wake, so this spins — which is fine here and wrong in production.
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    let mut spins = 0u32;
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => {
                println!("   executor: {spins} Pending round(s) before the answer was Ready");
                return v;
            }
            Poll::Pending => spins += 1,
        }
    }
}

static POLLS: AtomicU32 = AtomicU32::new(0);

fn main() {
    println!("1. The two signatures, side by side");
    println!("   fn next(&mut self)                  -> Option<Item>");
    println!("   fn poll_next(Pin<&mut Self>, &mut Context) -> Poll<Option<Item>>");
    println!("   Same question — is there another item? — with a third answer.");
    println!("   Option says \"here it is\" or \"finished\". Poll<Option<_>> adds");
    println!("   \"not yet, wake me when there is\", and that one extra state is the");
    println!("   whole difference between the two traits.");

    println!();
    println!("2. Polling one by hand, so the Pending is visible");
    let mut ticks = Ticks { left: 3, stalled: false, polls: &POLLS };
    let mut cx = Context::from_waker(Waker::noop());
    for round in 1..=4 {
        let p = Pin::new(&mut ticks).poll_next(&mut cx);
        let shown = match p {
            Poll::Pending => "Pending".to_string(),
            Poll::Ready(Some(v)) => format!("Ready(Some({v}))"),
            Poll::Ready(None) => "Ready(None)".to_string(),
        };
        println!("   poll {round} -> {shown}");
    }

    println!();
    println!("3. The same stream, consumed by an async fn");
    POLLS.store(0, Relaxed);
    let items = block_on(async {
        let mut ticks = Ticks { left: 3, stalled: false, polls: &POLLS };
        let mut out = Vec::new();
        while let Some(v) = next(&mut ticks).await {
            out.push(v);
        }
        out
    });
    println!("   items {items:?}   stream polled {} times for 3 items", POLLS.load(Relaxed));
    println!("   `while let Some(v) = stream.next().await` is the async `for`.");
    println!("   There is no `for v in stream`: `for` desugars to IntoIterator,");
    println!("   which has no await point anywhere in it.");

    println!();
    println!("4. What an Iterator does inside async code");
    let sum = block_on(async {
        let mut total = 0;
        for n in [1, 2, 3] {
            total += n;
        }
        total
    });
    println!("   for n in [1, 2, 3] inside async -> {sum}");
    println!("   That loop is legal and has no await point, so it runs start to");
    println!("   finish without ever yielding. Harmless for three integers, and");
    println!("   the classic production bug when the body blocks: a synchronous");
    println!("   read inside async work stalls every other task on that thread,");
    println!("   because the executor only regains control at an `.await`.");

    println!();
    println!("5. Where the traits actually live");
    println!("   Iterator      std::iter::Iterator          stable since 1.0");
    println!("   Stream        futures::Stream / tokio_stream, NOT in std");
    println!("   AsyncIterator core::async_iter::AsyncIterator, unstable (#79024)");
    println!("   So async iteration on stable Rust means a crate, and the trait");
    println!("   above is what that crate defines — poll_next, and nothing else.");
}
