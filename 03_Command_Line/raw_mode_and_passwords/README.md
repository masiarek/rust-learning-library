# Raw mode and passwords

**Level:** 301 · deep dive

**One line:** Everything on the 101 page arrives one *line* at a time because the terminal driver, not your program, is holding the keystrokes until Enter — a single keypress, an arrow key, or a password that must not echo means asking the terminal to stop doing that, and `std` has no way to ask.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Canonical mode against raw mode: the kernel's line discipline buffers, echoes, and turns Ctrl-C into a signal, and `read_line` is only ever handed a finished line
- Why `std` cannot do it — `termios` on Unix and the console API on Windows are two different worlds — and what [`crossterm` ↗](https://docs.rs/crossterm) and [`rpassword` ↗](https://docs.rs/rpassword) each wrap
- A password prompt reads from `/dev/tty`, not from stdin, which is why `echo secret | sudo -S` needs a flag and a plain `sudo` in a pipeline still asks you
- Echo off is a terminal setting that outlives your process if it panics before restoring it — the `Drop` guard that puts the terminal back, and what `reset` is for when it did not
- An arrow key is three bytes (`1b 5b 41`), an escape sequence rather than a character, and the timeout that decides whether a lone `1b` was Escape or the start of one
- Why none of this can have an answer key in this library: the harness runs with no terminal, and every example here reads from a `Cursor`

## The trap it exists for

A program that switches the terminal to raw mode and then hits an `unwrap` leaves the shell you return to with echo off and Enter not moving the cursor. Nothing looks broken and nothing you type appears. The fix is one word typed blind — `reset` — and the lesson is that a terminal mode is a resource with a destructor, the same shape as a file handle or a lock.

## See also

- [Reading a line from standard input](../reading_stdin/README.md) — the cooked-mode behaviour this page turns off
- [Reading without blocking](../reading_without_blocking/README.md) — the other thing a keypress-driven program needs
- [`Drop` and RAII](../../12_Traits/drop_and_raii/README.md) — the guard that restores the terminal
- [Which Rust UI option](../../08_Interfaces/rust_ui_options/README.md) — the TUI crates that build on raw mode
- [Writing a file inspector](../writing_a_file_inspector/README.md) — where an interactive viewer would start

## Po polsku

Wszystko na stronie 101 przychodzi po jednym *wierszu*, bo to sterownik terminala, a nie program, przetrzymuje naciśnięcia klawiszy do chwili wciśnięcia Enter — tryb kanoniczny (*canonical mode*) buforuje, wyświetla echo i zamienia Ctrl-C w sygnał. Pojedynczy klawisz, strzałka albo hasło, którego nie wolno wyświetlać, wymagają wyłączenia tego trybu (*raw mode*), a `std` nie ma jak o to poprosić: `termios` w Uniksie i API konsoli w Windows to dwa różne światy, które opakowują dopiero `crossterm` i `rpassword`. Dwie rzeczy, które warto wiedzieć wcześniej niż później: zapytanie o hasło czyta z `/dev/tty`, nie ze standardowego wejścia — dlatego `sudo` w potoku i tak pyta ciebie — a wyłączone echo jest ustawieniem terminala, które przeżyje panikę programu; powłoka, do której wrócisz, nie pokaże ani jednego wpisanego znaku, a lekarstwem jest słowo `reset` wpisane na ślepo. Tryb terminala to zasób z destruktorem, jak uchwyt pliku.

**Szukaj po polsku:** tryb surowy terminala · odczyt pojedynczego klawisza · hasło bez echa · `rust crossterm raw mode` · `rust rpassword` · `termios canonical mode`
