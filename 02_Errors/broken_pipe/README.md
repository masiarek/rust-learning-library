# Broken pipe

**Level:** 201 · working knowledge

**One line:** `tool | head` closes the pipe after ten lines, Rust's runtime has already set `SIGPIPE` to ignored, and so the eleventh `println!` gets `EPIPE` back and **panics** — `failed printing to stdout: Broken pipe (os error 32)` — which is why a command-line tool writes with `writeln!` and treats `ErrorKind::BrokenPipe` as a quiet exit.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What a C program does here: `SIGPIPE`'s default action kills the process silently, which is why `yes | head` has never printed an error in fifty years
- What Rust does instead: the runtime ignores `SIGPIPE` before `main` so that a write can *return* an error, and `println!` turns that error into a panic because it has no `Result` to give you — measured for [the inspector page](../../03_Command_Line/writing_a_file_inspector/README.md) on 1.98.0
- The fix in three sizes: `writeln!(out, …)` with the error matched on `ErrorKind::BrokenPipe` and an early `return`; a `BufWriter` whose final `flush` is where the error actually surfaces; and `-Zon-broken-pipe=kill`, the unstable flag that restores the C behaviour
- Exit status: `head` has already exited 0, the shell reports the pipeline's *last* command by default, and `set -o pipefail` is what makes your 101 visible
- The same event on the read side — a writer that closed early is just end of input, `Ok(0)`, and needs nothing — so this page is only about writing
- Python's `BrokenPipeError` and the `signal(SIGPIPE, SIG_DFL)` idiom, for the reader who has met this before

## The trap it exists for

Every test passes and every file dumps cleanly; the panic appears only when a *user* pipes the tool into `head`, `less` that they quit early, or `grep -m1`. It is the first bug report most Rust command-line tools receive, and it is not a bug in the tool's logic anywhere — it is a default the runtime chose so that a network server would not die when a client hung up.

## See also

- [Standard error, and exit status](../stderr_and_exit_status/README.md) — the stream the panic message went to, and the number a script reads
- [`Read` and `Write`](../../12_Traits/read_and_write/README.md) — the `Result` `writeln!` gives you and `println!` does not
- [`unwrap` is a TODO you forgot to remove](../unwrap_is_a_todo/README.md) — the same shape: a panic where a decision belonged
- [Writing a file inspector](../../03_Command_Line/writing_a_file_inspector/README.md) — the tool that meets this on its first `| head`

## Po polsku

`tool | head` zamyka potok po dziesięciu wierszach. Program w C ginie wtedy cicho, bo domyślna akcja sygnału `SIGPIPE` kończy proces — dlatego `yes | head` od pięćdziesięciu lat nie wypisuje żadnego błędu. Rust robi inaczej: środowisko uruchomieniowe **ignoruje** `SIGPIPE` jeszcze przed `main`, żeby zapis mógł *zwrócić* błąd, a `println!` nie ma jak oddać `Result`, więc zamienia ten błąd w panikę: `failed printing to stdout: Broken pipe (os error 32)`. Lekarstwo ma trzy rozmiary — `writeln!` z dopasowaniem `ErrorKind::BrokenPipe` i cichym wyjściem, `BufWriter`, którego końcowy `flush` jest miejscem, gdzie błąd naprawdę wypływa, i niestabilna flaga `-Zon-broken-pipe=kill` przywracająca zachowanie z C. Pułapka jest w tym, że każdy test przechodzi: panika pojawia się dopiero, gdy *użytkownik* przekieruje narzędzie do `head`, przedwcześnie zamkniętego `less` albo `grep -m1` — i to jest pierwsze zgłoszenie błędu, jakie dostaje większość narzędzi wiersza poleceń napisanych w Ruście.

**Szukaj po polsku:** zerwany potok · sygnał SIGPIPE · `rust println panic broken pipe` · `rust ErrorKind::BrokenPipe` · `rust -Zon-broken-pipe`
