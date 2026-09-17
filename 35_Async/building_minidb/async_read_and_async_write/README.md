# `AsyncRead` and `AsyncWrite`

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Under every `.await` on a socket are two traits whose methods are not `async` at all — `poll_read` and `poll_write` return `Poll`, and must arrange a wake-up before they return `Pending` — and implementing them is how a wrapper adds behaviour to every byte without changing a signature.

## What it has to cover

- [`AsyncRead` ↗](https://docs.rs/tokio/latest/tokio/io/trait.AsyncRead.html)'s one method, `poll_read(self: Pin<&mut Self>, cx, buf: &mut ReadBuf<'_>)`, and its contract: bytes added to `buf` and `Ready(Ok(()))`; nothing added and `Ready(Ok(()))` means end of stream; `Pending` only after the waker is registered
- [`ReadBuf` ↗](https://docs.rs/tokio/latest/tokio/io/struct.ReadBuf.html): its filled and initialised regions, and why the trait does not take a plain `&mut [u8]`
- [`AsyncWrite` ↗](https://docs.rs/tokio/latest/tokio/io/trait.AsyncWrite.html): `poll_write`, `poll_flush`, `poll_shutdown`, and a short write as a normal result
- Where `read`, `write_all` and `read_line` actually come from: the `AsyncReadExt` and `AsyncWriteExt` extension traits, whose futures call the poll methods
- **Pin projection** — reaching the inner stream through `Pin<&mut Self>` — to the depth the wrapper needs, and the `Pin` question left open in [chapter 1](../the_tokio_runtime/README.md)
- The wrapper: `CountingIo<T>` counts bytes in each direction for any `T: AsyncRead + AsyncWrite`, and drops into the handler from chapter 8 with no signature changed

## The trap it exists for

Returning `Pending` without registering the waker. Nothing fails and nothing is logged — the task is simply never polled again, and that connection hangs forever.

## What minidb gains

Per-connection byte counts, from a wrapper around the socket rather than edits to the handler.

## See also

- [`Read` and `Write`](../../../12_Traits/read_and_write/README.md) — the synchronous traits these mirror
- [What a future is](../../what_a_future_is/README.md) — the same `Poll` and `Context`, one level up
- [Polling ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/scheduling/polling/index.html) and [pinning ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/pinning/index.html) — in the Concurrency library

## Po polsku

Pod każdym `.await` na gnieździe są dwie cechy (*traits*), których metody wcale nie są asynchroniczne: `poll_read` i `poll_write` zwracają `Poll` i zanim zwrócą `Pending`, muszą zarejestrować `Waker`, bo inaczej nikt ich już nie wybudzi. Implementując je samodzielnie, można napisać **opakowanie** (*wrapper*), które np. liczy każdy przechodzący bajt i wstawia się do istniejącego kodu bez zmiany żadnej sygnatury.

**Szukaj po polsku:** asynchroniczne wejście-wyjście · `tokio asyncread poll_read` · `tokio readbuf` · `pin projection rust`
