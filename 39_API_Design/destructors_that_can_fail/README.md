# Destructors that can fail

**Level:** 301 · deep dive

**One line:** `Drop::drop` takes `&mut self`, returns nothing and cannot be `async` on stable — so a close that can fail, such as a final flush or a commit, needs an explicit `close(self) -> Result<…>` beside a `Drop` that only cleans up whatever was left.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The signature and its limits: `fn drop(&mut self)` has no return value and no `async`; `core::future::AsyncDrop` exists on 1.98.0 only as a nightly-only experimental API (`async_drop`, issue #126482)
- std admitting it in its own docs: `BufWriter` says errors during the flush on drop are ignored and asks you to call `flush` first; `File` says errors detected on closing are ignored by its `Drop` and points to `sync_all` for anyone who needs them
- The explicit close taking `self` by value, so nothing can use the value afterwards — std's version is `BufWriter::into_inner`, which flushes and returns `Result<W, IntoInnerError<BufWriter<W>>>`
- The `Drop` that stays behind: a field (`Option<Inner>`, or a `closed` flag) that `close` clears, so `drop` can tell a closed value from an abandoned one — and what it should do with an abandoned one: flush and ignore, log, or `debug_assert!`?
- Panicking instead of ignoring: why a panic inside `drop` while the thread is already unwinding aborts the process, and the Guidelines' C-DTOR-FAIL, "Destructors never fail"
- Blocking in `drop` (C-DTOR-BLOCK): joining a thread or waiting on a socket in a destructor, why that is worse inside an async runtime, and the explicit alternative the guideline asks for
- `Drop` is not guaranteed to run — `mem::forget` and `Rc` cycles skip it ([`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md)) — so anything that must happen has to go through `close`, and `Drop` can only be the fallback
- *Rust for Rustaceans* ch. 3 → "Flexible" → "Fallible and Blocking Destructors" as the source

## The trap it exists for

Letting a `BufWriter` flush on drop at the end of `main`. If that last write fails — a full disk, a closed pipe — the error is discarded with the writer, the program exits 0, and the report on disk is short. One `writer.flush()?` before the end turns the silent truncation into an error message.

## Where this sits

[`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) covers drop order and what `Drop` cannot do; [Drop guards](../../12_Traits/drop_guards/README.md) covers the guard pattern built on it; [`Read` and `Write`](../../12_Traits/read_and_write/README.md) holds the same `BufWriter` trap from the I/O side. This page is the API a library offers when closing its resource can fail.

## See also

- [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) — what `Drop` cannot do, including fail
- [Drop guards](../../12_Traits/drop_guards/README.md) — cleanup on every exit path, and what a guard cannot report
- [`Read` and `Write`](../../12_Traits/read_and_write/README.md) — flushing, and the error a `BufWriter` throws away
- [Readers are fallible](../../02_Errors/readers_are_fallible/README.md) — I/O errors that arrive after the open succeeded
- [Making misuse a compile error](../making_misuse_a_compile_error/README.md) — `#[must_use]` and types that force the explicit close
- [`BufWriter` ↗](https://doc.rust-lang.org/std/io/struct.BufWriter.html) — the paragraph on flushing before drop, and `into_inner`
- [Rust API Guidelines — Dependability ↗](https://rust-lang.github.io/api-guidelines/dependability.html#c-dtor-fail) — C-DTOR-FAIL and C-DTOR-BLOCK

## If you are coming from another language

- **C++.** Destructors are implicitly `noexcept` since C++11, so a throw escaping one calls `std::terminate` — the same "do not fail here" rule, enforced at run time. `std::ofstream` also closes quietly in its destructor; calling `close()` and checking the stream state is the explicit path, as `close(self) -> Result` is in Rust.
- **Java.** `AutoCloseable.close()` is declared `throws Exception`, and try-with-resources calls it and reports the failure — attaching it as a suppressed exception when the body had already thrown. Java made the close explicit syntax rather than a destructor.
- **Go.** `defer f.Close()` discards the error `Close` returns. The usual fix is a named result that the deferred function overwrites if closing failed — an explicit close, written by hand every time.
- **Python.** An exception from `__exit__` propagates out of the `with` block; one raised in `__del__` is printed as "Exception ignored" and dropped — Python has both halves of this page, a reporting close and a swallowing destructor.

## Po polsku

`Drop::drop` nie zwraca `Result` i nie może być `async`, więc zamknięcie, które może się nie udać — ostatni zapis bufora, zatwierdzenie transakcji — musi mieć jawną metodę `close(self) -> Result<…>`, a `Drop` zostaje tylko do sprzątania tego, czego nikt nie zamknął. Sama biblioteka standardowa to przyznaje: `BufWriter` i `File` ignorują błędy przy zwalnianiu. Pułapka: program kończy się kodem 0, a plik na dysku jest ucięty, bo błąd ostatniego zapisu zniknął razem z `BufWriter`.

**Szukaj po polsku:** destruktor zwracający błąd · jawne zamykanie zasobu · `rust drop cannot return error` · `rust bufwriter flush on drop` · `rust async drop`
