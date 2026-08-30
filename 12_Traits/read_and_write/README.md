# `Read` and `Write`

**Level:** 201 · working knowledge

**One line:** Two traits stand between your code and every byte source there is — a file, a socket, stdin, a `Vec<u8>` — which is why a function typed `fn load(r: impl Read)` can be tested against an in-memory string and shipped against a file, with no seam between the two.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The two required methods (`read`, `write`) and the large provided surface built on them — `read_to_string`, `read_to_end`, `write_all`, `lines`, `bytes`
- `&[u8]` implements `Read` and `Vec<u8>` implements `Write`, which is the whole testing story: no temp file, no mock, no trait of your own
- Taking `impl Read` / `&mut impl Write` as a parameter instead of naming `File` — the single change that makes an I/O function testable
- `BufRead` and why `BufReader` exists: a `read` is a syscall, and reading a line at a time without buffering is one syscall per byte
- `write!` and `writeln!` against a `Write`, and how they differ from `print!`
- The short read: `write` may write **fewer bytes than you gave it** and `read` may return fewer than you asked for. `write_all` and `read_exact` are the loops you would otherwise write
- `std::io::Write` versus `std::fmt::Write` — two traits, same method name, different error type, and the import error that follows
- Flushing, and what `Drop` on a `BufWriter` does and does not promise about errors

## The trap it exists for

A `BufWriter` flushes on drop and **discards the error if that flush fails** — there is nowhere for a `Drop` to return it. A program that writes a report and never calls `flush()` explicitly can exit 0 having written a truncated file. The fix is one line, and it is the reason `flush()` returns a `Result` at all.

## See also

- [Readers are fallible](../../02_Errors/readers_are_fallible/README.md) — the `io::Error` half, and the loop that keeps going
- [Reading lines efficiently](../../04_Files/reading_lines_efficiently/README.md) — `BufRead::lines` and what it allocates
- [Opening a file](../../04_Files/opening_a_file/README.md) — the `File` these traits abstract over
- [Standard error, and exit status](../../02_Errors/stderr_and_exit_status/README.md) — the two writers a program has by default
- [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md) — what you need *less* of once a function takes `impl Read`
- [`std::io::Read` ↗](https://doc.rust-lang.org/std/io/trait.Read.html) · [Comprehensive Rust: `Read` and `Write` ↗](https://google.github.io/comprehensive-rust/std-traits/read-and-write.html)

## Po polsku

Sedno `Read` i `Write` jest takie, że to **cechy** (*traits*), a nie typy: `&[u8]` implementuje `Read`, a `Vec<u8>` implementuje `Write`, więc funkcja przyjmująca `impl Read` daje się przetestować na danych trzymanych w pamięci i uruchomić na pliku bez żadnej różnicy w kodzie. Polskie materiały mówią zwykle o „strumieniach wejścia/wyjścia”, co podsuwa złą intuicję — tutaj nie ma klasy strumienia ani hierarchii dziedziczenia, są dwie cechy z jedną wymaganą metodą każda (`read`, `write`) i całą wygodną resztą (`read_to_string`, `write_all`, `lines`) dobudowaną na nich jako metody domyślne.

Pułapka, dla której ta strona istnieje, jest cicha: `BufWriter` opróżnia bufor przy wypuszczeniu zasobu (*drop*) i **gubi błąd**, jeśli to opróżnienie się nie powiedzie — `Drop` nie ma jak zwrócić `Result`. Program, który zapisze raport i nigdy nie wywoła jawnie `flush()`, potrafi zakończyć się kodem wyjścia 0, zostawiając ucięty plik. Druga rzecz do zapamiętania to `write` z krótkim zapisem — wolno mu zapisać mniej bajtów, niż mu podano — oraz to, że `std::io::Write` i `std::fmt::Write` są dwiema różnymi cechami o tych samych nazwach metod, więc błąd o brakującym imporcie potrafi wskazywać nie tę, o którą chodziło.

**Szukaj po polsku:** strumienie wejścia/wyjścia w Ruscie · buforowanie zapisu · `rust BufWriter flush on drop` · `rust impl Read for testing` · `rust io::Write vs fmt::Write`
