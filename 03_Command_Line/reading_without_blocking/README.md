# Reading without blocking

**Level:** 301 · deep dive

**One line:** `read_line` does not return until a line arrives or the input ends, and there is no timeout to give it — so a program that must also do something else while it waits puts the read on a thread and hands the lines through a channel, which is exactly what `tokio::io::stdin` does underneath.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why a blocking read cannot be cancelled: the thread is inside the kernel, and the only ways out are data, end of input, or a signal — which is why Ctrl-C works and `Ctrl-C` handling in your program mostly does not need to
- The `std` answer: `thread::spawn` reading stdin forever, `mpsc::Sender` for each line, and `recv_timeout` on the other side — [Spawning a thread](../../09_Advanced/spawning_a_thread/README.md) and [Channels](../../09_Advanced/channels/README.md) are the two halves
- What happens to that thread at exit: it is still blocked in `read`, `main` returning kills it, and the last partial line is lost
- `select` and `poll` on fd 0 — Unix only, no `std` wrapper, and [`mio` ↗](https://docs.rs/mio) or [`nix` ↗](https://docs.rs/nix) if you want them; why they work on a pipe and a terminal and not on a regular file
- [`tokio::io::stdin` ↗](https://docs.rs/tokio/latest/tokio/io/fn.stdin.html), whose documentation says it is a blocking thread in a trench coat, and what that means for a `select!` that races it against a timer
- Why an example of any of this cannot be recorded here — the harness has no terminal and no clock it may trust — and what a deterministic stand-in looks like

## The trap it exists for

A program that reads stdin on a thread and prints a progress line every second works, and then the user pipes a file into it and every line arrives at once: the channel fills, the progress loop starves, and the output interleaves in an order that depends on the scheduler. Backpressure — a bounded channel — is the fix, and [when a bound is backpressure rather than a limit](../../09_Advanced/channels/README.md) is already a page.

## See also

- [Reading a line from standard input](../reading_stdin/README.md) — the blocking call this page works around
- [Raw mode and passwords](../raw_mode_and_passwords/README.md) — the other half of an interactive program
- [Sharing across threads](../../18_Ownership/sharing_across_threads/README.md) — what the reader thread may and may not touch
- [What MCP is](../../05_Tooling/what_mcp_is/README.md) — a server whose whole life is a blocking read loop on stdin, and is fine that way

## Po polsku

`read_line` nie wraca, dopóki nie przyjdzie wiersz albo nie skończy się wejście, i nie da mu się podać limitu czasu — wątek siedzi wtedy w jądrze, a wyjść stamtąd może tylko przez dane, koniec wejścia albo sygnał (dlatego Ctrl-C działa bez żadnej pomocy z twojej strony). Program, który w czasie czekania ma robić coś jeszcze, kładzie odczyt na osobnym wątku i przekazuje wiersze kanałem (`mpsc`), a po drugiej stronie używa `recv_timeout`; `tokio::io::stdin` robi dokładnie to samo pod spodem i mówi o tym w dokumentacji. `select` i `poll` na deskryptorze 0 istnieją tylko w Uniksie i tylko poza `std`. Pułapka: taki program działa z klawiaturą, a gdy użytkownik przekieruje do niego plik, wszystkie wiersze przychodzą naraz, kanał się zapełnia i wyjście przeplata się w kolejności zależnej od planisty — lekarstwem jest ograniczony kanał, czyli przeciwciśnienie (*backpressure*).

**Szukaj po polsku:** nieblokujący odczyt ze standardowego wejścia · odczyt z limitem czasu · `rust stdin thread channel recv_timeout` · `tokio stdin blocking thread`
