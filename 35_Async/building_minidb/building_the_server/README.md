# Building the server

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A TCP server is an accept loop that spawns one task per connection, and the design decision inside it is error scope — a failure on one connection ends that connection, and only a failure of the listener itself may end the loop.

## What it has to cover

- [`TcpListener` ↗](https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html): bind, accept, and one spawned task per accepted socket
- Framing with `BufReader` and `lines()`: `next_line` returning `Ok(Some(line))`, and `Ok(None)` when the client hung up
- A line protocol: parsing a line into a command enum, and the response for a line that does not parse
- Writing the response: `write_all`, and why a buffered writer needs a flush
- **Which errors end what.** A malformed command gets an error response; an IO error ends that connection's task; a transient `accept` error — running out of file descriptors — is logged and retried after a pause, not returned from `main`
- A limit on line length, because `lines()` will otherwise buffer whatever a client sends without a newline
- Talking to it with `nc`, and why that is not yet a test (chapter 8)

## The trap it exists for

A `?` inside the accept loop. It reads as ordinary error handling and turns one transient failure into the whole server exiting — every other client's connection with it.

## What minidb gains

A network protocol. Each connection still creates its own store, so two clients cannot see each other's writes — which is chapter 4's problem.

## See also

- [Who owns the state](../who_owns_the_state/README.md) — the next chapter
- [Reading lines efficiently](../../../04_Files/reading_lines_efficiently/README.md) — `BufRead::lines` on a file, the synchronous version of the same framing
- [Keep going or stop](../../../02_Errors/keep_going_or_stop/README.md) — the same error-scope decision in a program that is not a server
- [Codecs and framing](../codecs_and_framing/README.md) — where the line protocol is replaced

## Po polsku

Serwer TCP to pętla przyjmująca połączenia (*accept loop*), która dla każdego klienta uruchamia osobne zadanie. Najważniejsza decyzja projektowa to **zasięg błędu**: zła komenda dostaje odpowiedź z błędem, błąd wejścia-wyjścia kończy tylko to jedno połączenie, a chwilowy błąd `accept` — na przykład brak wolnych deskryptorów plików — trzeba zalogować i ponowić, zamiast przerywać całą pętlę operatorem `?`.

**Szukaj po polsku:** serwer TCP w tokio · protokół tekstowy · `tokio tcplistener accept loop` · `tokio bufreader lines`
