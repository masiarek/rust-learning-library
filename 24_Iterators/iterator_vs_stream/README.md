# `Iterator` versus `Stream`

**Level:** 301 · deep dive

**One line:** A stream is an iterator whose `next` is allowed to answer *"not yet"* — one extra state in the return type, and everything else about it follows from that.

```rust
fn next(&mut self)                          -> Option<Item>;
fn poll_next(Pin<&mut Self>, &mut Context)  -> Poll<Option<Item>>;
```

Same question — *is there another item?* — with a third possible answer. `Option` says **here it is** or **finished**. `Poll<Option<_>>` adds **not yet, wake me when there is**, and that is the whole difference between the two traits.

## Where the trait lives, which is the first surprise

| Trait | Where | Status |
|---|---|---|
| `Iterator` | `std::iter` | stable since 1.0 |
| `Stream` | the `futures` crate, re-exported by `tokio-stream` | **not in std** |
| `AsyncIterator` | `core::async_iter` | **unstable**, [tracking issue 79024 ↗](https://github.com/rust-lang/rust/issues/79024) |

So async iteration on stable Rust means adding a crate, and the trait that crate defines is the four-line one above. The eventual std name is `AsyncIterator`; it has been unstable for years, mostly over how `next` should be spelled once `async fn` in traits landed.

## Polling one by hand

This library's examples compile with `rustc` alone, so the run below defines `Stream` itself, implements it for a source that stalls once before each item, and drives it with a `Waker::noop()` executor — stable since 1.85, and enough to see the machinery:

```text
poll 1 -> Pending
poll 2 -> Ready(Some(3))
poll 3 -> Pending
poll 4 -> Ready(Some(2))
```

A real stream registers `cx.waker()` before returning `Pending`, so the executor can park the thread instead of asking again. This one just returns, which is why the toy executor spins — correct for a demonstration, wrong in production, and exactly the distinction between a `Waker` that does something and `Waker::noop()`.

Consumed from an `async` block, the same three items cost **seven** polls: three `Pending`, three `Ready(Some(_))` and one `Ready(None)`.

## There is no `for x in stream`

`for` desugars to `IntoIterator::into_iter` and a `next` call, and there is no await point anywhere in that desugaring. The async form is a `while let` over a future:

```rust
while let Some(v) = stream.next().await {
    …
}
```

`.next()` there is not a trait method — `Stream` has only `poll_next`. It comes from an extension trait (`StreamExt`), and it is a small future that polls the stream once per poll of itself. The run writes that future out in full; it is nine lines, and knowing that it exists explains why you have to import `StreamExt` before `.next()` compiles.

The same is true of every adapter: `map`, `filter` and `fold` on a stream are `StreamExt` methods, mirroring `Iterator`'s but returning futures. `for_each` becomes `for_each(…).await`, and `collect` becomes `collect().await`.

## The trap: an `Iterator` inside async code does not yield

```rust
block_on(async {
    let mut total = 0;
    for n in [1, 2, 3] { total += n; }
    total
});
```

Legal, and it runs start to finish without ever returning to the executor — because there is no `.await` in it. For three integers that is exactly right. For a body that **blocks** — a synchronous file read, a `std::thread::sleep`, a CPU-heavy loop — it is the classic async production bug: the executor only regains control at an await point, so one blocking iteration stalls every other task sharing that thread.

The fix is not to make the loop a stream. It is to move the blocking work off the async thread entirely (`spawn_blocking` in tokio), or to use the async version of whatever is blocking. **An iterator over data already in memory is fine in async code**; it is I/O and long computation that are not.

## If you are coming from another language

- **Python.** This is `Iterator` versus `AsyncIterator` exactly: `__next__` versus `__anext__`, `for` versus `async for`, `StopIteration` versus `StopAsyncIteration`. Python built the async form into the language, so `async for x in stream` works with no import, where Rust's needs a crate and an extension trait — the same gap this page opens with. What Python hides and Rust shows is the poll loop: `await` in Python suspends a coroutine the event loop resumes, and you never see a `Pending`, a `Waker` or a `Context`. Reading the run below is a fair picture of what `asyncio` is doing underneath, and the blocking trap is identical — a synchronous `requests.get` inside an `async def` stalls the event loop for exactly the same reason.
- **ABAP.** There is no async at all in the language, and the honest translation of the whole page is *"this problem does not arise"*: a `LOOP` blocks, and concurrency is a `CALL FUNCTION ... STARTING NEW TASK` with a callback, which is closer to spawning a thread than to a stream. The one idea that transfers is the third state. `sy-subrc` after a `READ` answers "found" or "not found"; a stream's `poll_next` answers "found", "finished", or "ask me again later" — and if you have ever polled a qRFC queue or an SM58 entry in a loop waiting for something to arrive, you have written the `Pending` case by hand, with the wake-up replaced by a `WAIT UP TO n SECONDS`.
- **JavaScript.** `Symbol.asyncIterator` and `for await (const x of stream)` are the direct counterpart, and Node streams predate it. The structural difference is push versus pull: a Node `Readable` in flowing mode *pushes* data at your handler and needs `pause()`/`resume()` for backpressure, while a Rust `Stream` is **pulled** — nothing is produced until somebody polls — so backpressure is the default rather than a feature. That is the single best argument for the `poll_next` shape, and it is invisible until you have debugged a producer that outran its consumer.

---

## The verified output

<!-- output:iterator_vs_stream -->
*Verified output of [`iterator_vs_stream.rs`](examples/iterator_vs_stream.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The two signatures, side by side
   fn next(&mut self)                  -> Option<Item>
   fn poll_next(Pin<&mut Self>, &mut Context) -> Poll<Option<Item>>
   Same question — is there another item? — with a third answer.
   Option says "here it is" or "finished". Poll<Option<_>> adds
   "not yet, wake me when there is", and that one extra state is the
   whole difference between the two traits.

2. Polling one by hand, so the Pending is visible
   poll 1 -> Pending
   poll 2 -> Ready(Some(3))
   poll 3 -> Pending
   poll 4 -> Ready(Some(2))

3. The same stream, consumed by an async fn
   executor: 3 Pending round(s) before the answer was Ready
   items [3, 2, 1]   stream polled 7 times for 3 items
   `while let Some(v) = stream.next().await` is the async `for`.
   There is no `for v in stream`: `for` desugars to IntoIterator,
   which has no await point anywhere in it.

4. What an Iterator does inside async code
   executor: 0 Pending round(s) before the answer was Ready
   for n in [1, 2, 3] inside async -> 6
   That loop is legal and has no await point, so it runs start to
   finish without ever yielding. Harmless for three integers, and
   the classic production bug when the body blocks: a synchronous
   read inside async work stalls every other task on that thread,
   because the executor only regains control at an `.await`.

5. Where the traits actually live
   Iterator      std::iter::Iterator          stable since 1.0
   Stream        futures::Stream / tokio_stream, NOT in std
   AsyncIterator core::async_iter::AsyncIterator, unstable (#79024)
   So async iteration on stable Rust means a crate, and the trait
   above is what that crate defines — poll_next, and nothing else.
```
<!-- /output -->

---

## See also

- [Implementing `Iterator`](../implementing_iterator/README.md) — the synchronous trait this one mirrors, method for method
- [Iterators are lazy](../iterators_are_lazy/README.md) — pull-based already, which is why the async version is a small step
- [`while let`](../../17_Option_and_Result/while_let/README.md) — the loop form that replaces `for` here
- [Returning an iterator](../returning_an_iterator/README.md) — `impl Iterator` and `impl Stream` have the same lifetime story
- [Mutex poisoning](../../09_Advanced/mutex_poisoning/README.md) — the other place a blocking call on a shared thread ruins somebody else's day

## Sources

[`futures::Stream` ↗](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) is the trait in practice; [`core::async_iter::AsyncIterator` ↗](https://doc.rust-lang.org/std/async_iter/trait.AsyncIterator.html) is the unstable std one, and its [tracking issue ↗](https://github.com/rust-lang/rust/issues/79024) is where the naming argument lives. [`Waker::noop` ↗](https://doc.rust-lang.org/std/task/struct.Waker.html#method.noop) is what makes the executor in the run three lines long.
