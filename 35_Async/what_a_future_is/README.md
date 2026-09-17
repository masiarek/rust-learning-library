# What a future is

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A [`Future` ↗](https://doc.rust-lang.org/std/future/trait.Future.html) is a value with one method, `poll`, and creating one starts no work — the body of an `async fn` does not run until something polls the value it returned.

## What it has to cover

- The whole trait: `type Output` and `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`, and what `Poll::Ready` and `Poll::Pending` each promise the caller
- **Nothing runs until the first poll.** A std-only program shows it: build a future, print a line, then poll it once with `Context::from_waker(Waker::noop())` — the body's output appears only after the poll
- The warning rustc gives for a future nobody uses — *"futures do nothing unless you `.await` or poll them"* — and why it is a lint rather than an error
- What the `Waker` in `Context` is for: a future that returns `Pending` must arrange to be woken, or nothing will ever poll it again
- Why `poll` takes `Pin<&mut Self>`, to the depth this section needs — a future may hold a reference into itself, so it must not move once polled. The full story waits for [chapter 10](../building_minidb/async_read_and_async_write/README.md)
- A hand-written future (a countdown that is `Pending` three times) beside the `async` block that does the same thing

## The trap it exists for

Reading `let f = fetch();` as a call that has started. In JavaScript a `Promise` is already running when you hold it; in Rust the future is inert. Code that builds futures in a loop and awaits them later has done no work at all until the await, and a future that is dropped without being awaited never ran.

## If you are coming from another language

- **JavaScript:** a `Promise` is eager and a Rust future is lazy. `Promise.all` starts nothing new because everything had already started; the Rust equivalent (`join!`) is what starts them.
- **Python:** closest to a coroutine object — calling an `async def` returns one without running it, and the same *"was never awaited"* warning exists there.
- **C#:** a `Task` is hot (already running); Rust has no hot future in std — spawning onto a runtime is what makes work start on its own.

## See also

- [`async fn` and `.await`](../async_fn_and_await/README.md) — the syntax that writes the state machine for you
- [The Tokio runtime](../building_minidb/the_tokio_runtime/README.md) — who calls `poll` in a real program
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — the same "nothing until asked" rule, synchronous
- [Future and promise ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/future_and_promise/index.html) — the concept across languages, in the Concurrency library

## Po polsku

**Future** w Ruscie to wartość, a nie uruchomione zadanie: wywołanie funkcji `async` zwraca obiekt, w którym nic się jeszcze nie wydarzyło, i ciało funkcji wykona się dopiero przy pierwszym wywołaniu `poll`. To odwrotność `Promise` z JavaScriptu, który rusza od razu — i dlatego future, którego nikt nie odpytał ani nie „zaczekał” przez `.await`, po prostu nigdy nie zadziałał.

**Szukaj po polsku:** future w Ruscie · leniwe obliczenia · `rust future trait poll` · `rust futures are lazy`
