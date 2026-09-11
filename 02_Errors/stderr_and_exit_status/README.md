# Standard error, and exit status

**Level:** 101 → 201 · for newcomers

**One line:** A program has two output streams and one number: the streams separate *the answer* from *everything you want to say about it*, and the number is the only part another program reads.

```rust
fn main() {
    eprintln!("reading 4 rows");   // stderr — what you want to say about the answer
    println!("total 6");           // stdout — the answer
}
```

One letter apart, and it decides what a caller can use. `prog > total.txt` puts `total 6` in the file and leaves `reading 4 rows` on your screen; `prog | wc -l` counts one line, not two. Neither half is lost, and neither is in the other's way.

## Two streams

| | Carries | You write it with | Who reads it |
|---|---|---|---|
| **stdout**, fd 1 | the answer | [`println!` ↗](https://doc.rust-lang.org/std/macro.println.html), [`print!` ↗](https://doc.rust-lang.org/std/macro.print.html), [`io::stdout()` ↗](https://doc.rust-lang.org/std/io/fn.stdout.html) | the next program in the pipeline, a redirect, a file |
| **stderr**, fd 2 | everything else — warnings, progress, what failed | [`eprintln!` ↗](https://doc.rust-lang.org/std/macro.eprintln.html), [`eprint!` ↗](https://doc.rust-lang.org/std/macro.eprint.html), [`io::stderr()` ↗](https://doc.rust-lang.org/std/io/fn.stderr.html) | a person |

The name is the misleading part: stderr is not *for errors*, it is for everything that is not the answer. Progress, a warning about one bad row, a prompt — all of it belongs there, and the test is a single question. **Should the next program in the pipeline count this as data?** If not, it goes on stderr.

The shell reaches the two separately, and the last two rows are not two spellings of one thing:

| Spelling | Where stdout goes | Where stderr goes |
|---|---|---|
| `prog > f` | the file | the screen |
| `prog 2> f` | the screen | the file |
| `prog > f 2>&1` | the file | the file |
| `prog 2>&1 > f` | the file | **the screen** |

`2>&1` means *send stderr where stdout is pointing right now*, so putting it first copies the screen, and the later `> f` moves only stdout — measured, and the file holds `answer` while `note` is still on the terminal.

## Which stream a tool uses is not guessable

Each program chose once, possibly decades ago, and nothing announces the choice. Where it bites most often is a tool's *version* line, which is the thing scripts scrape:

```text title="Measured 2026-09-07 — macOS 26, OpenJDK 25.0.4.1 (Homebrew)"
$ java -version > version.txt
openjdk version "25.0.4.1" 2026-08-18
OpenJDK Runtime Environment Homebrew (build 25.0.4.1)
OpenJDK 64-Bit Server VM Homebrew (build 25.0.4.1, mixed mode, sharing)

$ wc -c version.txt
       0 version.txt
```

The redirect worked perfectly. `java -version` writes on stderr, so the file is empty and the three lines you were trying to capture went to the terminal instead. `java --version` — the two-dash long form, added in JDK 9 — writes the same information on **stdout**, in a slightly different format, and captures fine. So the fix is usually the other flag rather than `2>&1`.

Java is the outlier: on the same machine `python3`, `rustc`, `node`, `ruby`, `go`, `bash` and `perl` all put `--version` on stdout. But "usually stdout" is not a rule you can lean on, and the failure is silent — an empty file, exit status 0, nothing to catch. Check the stream before you scrape a program's output, and check it for *that* program. GNU `grep` and BSD `grep` [disagree about it for the same notice ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/grep/index.html), which is the same trap one layer down.

## The trap: a diagnostic on the wrong stream

Section 3 of the run below is one program with one word changed — `println!` where `eprintln!` belonged — and nothing about it looks wrong. It exits 0, it prints a warning that is accurate, and a person reading the terminal sees exactly what they saw before.

What changed is that the warning is now indistinguishable from data. A caller counting rows gets 4 instead of 3. A caller summing the last column gets **9 instead of 6**, because the warning ends in a row number and `awk '{s+=$NF}'` cannot tell a sentence from a record. That is the shape of the bug: not a crash, a plausible wrong number, in a pipeline that reports success.

## The number a script reads

A person reads your message. A shell script, a [`Makefile`](../../20_Compilers/makefiles/README.md) and a CI step read one byte: the exit status. Zero is success, non-zero is failure, and that is nearly the whole convention — so a program that prints `error: no such file` and exits 0 has reported success to everything except a human.

| What `main` does | Status | What reaches stderr |
|---|---|---|
| returns `()` | 0 | nothing |
| returns `Err(e)` | 1 | `Error: ` then the **`Debug`** form of `e` |
| [`process::exit(n)` ↗](https://doc.rust-lang.org/std/process/fn.exit.html) | `n`, truncated to 8 bits | nothing |
| panics | 101 | the panic message, plus the backtrace note |

Two of those rows are worth a second look. The `Err` row prints `Debug`, not `Display`, which is why a hand-written error type shows its struct fields where you expected a sentence — [`main` can return a `Result`](../main_returns_result/README.md) is the page for that. And `exit(n)` really is truncated: `exit(256)` measured as status **0**, a failure reported as success by arithmetic.

`prog && next` runs `next` only after a zero, `set -e` stops the script on a non-zero, and `$?` is where a shell keeps the last one. None of them will ever read your sentence.

## `process::exit` does not run destructors

It ends the process immediately — no unwinding, no `Drop`, no flush of anything you were buffering yourself:

```rust
use std::io::{BufWriter, Write};

fn main() {
    let mut w = BufWriter::new(std::io::stdout().lock());
    writeln!(w, "total 6").expect("the write only fills a buffer");
    w.flush().expect("stdout is open");   // without this line, nothing is printed
    std::process::exit(0);
}
```

Delete the `flush` and the run prints nothing at all while still exiting 0 — measured in section 5 below. `println!` is safe here because `Stdout` is line-buffered by std, but the moment you wrap it in a [`BufWriter` ↗](https://doc.rust-lang.org/std/io/struct.BufWriter.html) for speed, the buffer is yours and no runtime cleanup knows it exists. Returning from `main` — or `return Err(...)` — flushes it; `exit` does not.

## If you are coming from another language

- **Python** — `print(msg, file=sys.stderr)` and `sys.exit(1)`, the same two ideas under different spellings. Two differences worth holding. Python gives you a *default* Rust does not: an uncaught exception prints a traceback on stderr and exits 1 for free, whereas in Rust an ignored `Result` is a warning, and a program that never checks it exits 0. And the `-> Result` return in `main` is the closest Rust has to that default: it is the one shape that gets you the message-plus-non-zero pair without writing either.
- **ABAP** — there is no pair of streams to separate; list output (`WRITE`) and a message (`MESSAGE`) go to different places, but both go to a person, and a background job's log is not a pipeline anybody parses for data. What transfers is the *number*: `sy-subrc` is an exit status you have to test at the call site, and it has the same polarity — `0` is success, and the failure is silent if nobody looks. What changes crossing over is scope. `sy-subrc` is a global the next statement overwrites, so it must be read immediately; an exit status is one byte the operating system carries to whoever called your program, so it survives leaving the process entirely — which is exactly why a shell script can branch on it and no ABAP caller can branch on somebody else's `sy-subrc`.
- **The shell itself** — `2>&1` is worth learning as a phrase rather than a symbol: *make fd 2 point where fd 1 points*. Everything about redirect ordering follows from reading it that way.

## The verified output

<!-- output:stderr_and_exit_status -->
*Verified output of [`stderr_and_exit_status.rs`](examples/stderr_and_exit_status.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── 1. One run, seen from outside
   stdout  alpha beta -> 2
           gamma -> 1
           delta epsilon zeta -> 3
   stderr  warning: skipping unreadable row 3
   status  0
   Three rows counted, one row complained about, and the complaint is
   not one of the three. Nothing here is a convention the compiler
   enforces: `println!` writes to the first, `eprintln!` to the second.

──── 2. What a redirect keeps
   prog > rows.txt
   rows.txt gets    51 bytes  the 3 answer lines
   the screen keeps 35 bytes  the warning
   Neither half is lost and neither is in the other's way. A tool that
   puts its diagnostics on stdout offers you both or neither.

──── 3. The same warning, one word different
   stdout  alpha beta -> 2
           gamma -> 1
           warning: skipping unreadable row 3
           delta epsilon zeta -> 3
   stderr  (nothing)
   status  0

   a caller counting rows            3 -> 4
   a caller summing the last column  6 -> 9
   Both runs exited 0 and neither printed anything a person would call
   wrong. The warning became data because nothing distinguishes them
   once they are on the same stream.

──── 4. The number a script reads
   silent  stderr: error: row 3 is unreadable, giving up    status 0
   honest  stderr: error: row 3 is unreadable, giving up    status 1
   The sentence is identical; the number is not. `prog && next` runs
   `next` after the first and stops at the second, and a CI step
   passes the first. Zero is success, non-zero is failure, and that is
   nearly the whole convention.

──── 5. process::exit does not run destructors
   wrote "total 6" into a BufWriter, then exit(0)
   without flush  stdout ""  status 0
   with flush     stdout "total 6\n"  status 0
   The buffer is yours, so no runtime cleanup knows about it, and
   `exit` does not unwind — the `Drop` that would have flushed it never
   runs. A correct message, reported as success, printed by nobody.
```
<!-- /output -->

## See also

- [`main` can return a `Result`](../main_returns_result/README.md) — the shortest path to a non-zero status, and the `Debug` form it prints on the way
- [Testing a command](../../03_Command_Line/testing_a_command/README.md) — asserting the status and the stream from an integration test
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — where the 101 comes from, and what is left half-done underneath it
- [`unwrap` is a TODO you forgot to remove](../unwrap_is_a_todo/README.md) — the most common way a program exits 101 by accident
- [Spawning a thread](../../09_Advanced/spawning_a_thread/README.md) — a panicked thread does *not* set the process status; only the main thread's does

## Po polsku

Nazwy obu strumieni zostają po angielsku, bo tak nazywa je system: `stdout` (standardowe wyjście) i `stderr` (standardowe wyjście błędów). Druga nazwa myli i warto ją od razu poprawić — `stderr` nie jest „dla błędów", tylko dla **wszystkiego, co nie jest odpowiedzią**: ostrzeżeń, postępu, pytań do użytkownika. Test jest jeden i mieści się w zdaniu: czy następny program w potoku ma to policzyć jako dane? Jeśli nie, to idzie na `stderr`. W Ruscie różnica to jedna litera — `println!` pisze na `stdout`, `eprintln!` na `stderr` — i nic więcej za tym nie stoi.

Pułapka jest w przekierowaniu. `prog > plik` zabiera **tylko** `stdout`, więc program piszący komunikaty na `stderr` zostawi ci pusty plik i wypisze wszystko na ekran — dokładnie to robi `java -version`, którego dwukreskowy odpowiednik `java --version` pisze już na `stdout`. Ratunkiem jest `2>&1`, które czyta się nie jako symbol, lecz jako zdanie: *skieruj deskryptor 2 tam, gdzie w tej chwili wskazuje deskryptor 1*. Stąd bierze się cała reszta, łącznie z tym, że **kolejność ma znaczenie**: `prog > plik 2>&1` wkłada do pliku obie rzeczy, a `prog 2>&1 > plik` — pozornie to samo — kopiuje najpierw ekran, więc `stderr` zostaje na ekranie, a do pliku trafia sam `stdout`.

Osobno stoi kod wyjścia i jest w nim jedna rzecz odwrotna do intuicji: **zero znaczy sukces**, choć w wartościach logicznych zero to fałsz. Skrypt powłoki nie czyta twojego komunikatu — czyta tę jedną liczbę, więc program wypisujący `error: nie ma takiego pliku` i kończący się zerem melduje sukces wszystkim poza człowiekiem. W Ruscie: `main` zwracające `()` daje 0, zwrócenie `Err` daje 1 i wypisuje na `stderr` postać **`Debug`** błędu (nie `Display`, dlatego widać wtedy pola struktury zamiast zdania), `panic!` daje 101, a `process::exit(n)` daje `n` obcięte do ośmiu bitów — `exit(256)` to zmierzone **0**, czyli porażka zgłoszona jako sukces przez samą arytmetykę.

Na koniec zasadzka, która potrafi zjeść poprawnie wypisany komunikat: `process::exit` kończy proces **natychmiast**, bez odwijania stosu i bez uruchomienia destruktorów. `println!` jest bezpieczne, bo `Stdout` w std buforuje liniowo, ale gdy owiniesz je własnym `BufWriter`-em dla szybkości, bufor jest twój i żadne sprzątanie po programie o nim nie wie — zdanie oddane do takiego bufora nigdy nie dotrze na ekran, a program i tak zakończy się zerem. Wyjście z `main` opróżnia bufor, `exit` nie.

**Szukaj po polsku:** standardowe wyjście błędów · przekierowanie strumieni w powłoce · kod wyjścia programu · `rust eprintln vs println` · `bash 2>&1` · `rust process::exit no destructors`
