# Reading a line from standard input

**Level:** 101 · for newcomers

**One line:** `io::stdin().read_line(&mut line)` appends one line to a `String` you own — newline included — and answers with how many bytes arrived, where `Ok(0)` means nobody is typing any more; so the three habits are `clear()` before it, `trim()` after it, and a look at the number in between.

```rust
use std::io::{self, Write};

fn main() {
    print!("How many? ");
    io::stdout().flush().expect("stdout is open");   // print! ends without a newline, so push the prompt out
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("stdin is readable");
    let n: u32 = line.trim().parse().expect("a whole number");   // "42\n" -> 42
    println!("{n} it is");
}
```

Type `42` and Enter, and the program says `42 it is`. Every line of that is doing one job, and the rest of this page is those jobs one at a time — what came back from `read_line`, why the `trim()` is not optional, why the `flush()` is there, and what the number it returned is for.

---

## What one call does

`read_line` reads up to and including the next `\n`, **appends** those bytes to the `String` you passed, and returns how many it appended:

```rust
let mut line = String::new();
let n = io::stdin().read_line(&mut line).expect("stdin is readable");
// typed 42 and Enter:  n == 3,  line == "42\n"
```

Three bytes, not two: the newline is part of the line. Nothing strips it for you — which is the reason the very next thing anyone does with the result is `trim()`.

## Trim, then parse

`"42\n".parse::<u32>()` fails. The newline is not a digit, and [`parse`](../../14_Strings/parsing_a_string/README.md) is strict about the whole string:

```rust
let line = String::from("42\n");
let bad = line.parse::<u32>();          // Err: invalid digit found in string
let good = line.trim().parse::<u32>();  // Ok(42)
```

`trim()` removes whitespace from both ends — the `\n`, a Windows `\r\n`, and the spaces somebody typed before pressing Enter. The `expect` on the opening program's `parse` is the newcomer's honest choice: it names the assumption, and a wrong answer stops the program with that sentence rather than a wrong number. The loop that asks again instead is further down.

## The buffer is yours, so clearing it is too

Call `read_line` twice on the same `String` and the second line lands *after* the first:

```rust
let mut line = String::new();
io::stdin().read_line(&mut line).expect("stdin is readable");   // line == "42\n"
io::stdin().read_line(&mut line).expect("stdin is readable");   // line == "42\n  seven \r\n"
```

`read_line` never overwrites. That is deliberate — the buffer's allocation is reused across a whole file without a fresh `String` per line, which [`String::clear`](../../14_Strings/string_methods/string_clear/README.md) exists for — but it means a loop that forgets `line.clear()` at the top parses an ever-longer string and fails from the second turn on. Section 3 of the run below shows the growth.

## `Ok(0)` is the end

There are three ways for input to stop: the person at the keyboard presses **Ctrl-D** (Unix) or **Ctrl-Z then Enter** (Windows), the file that was redirected in runs out, or the program on the other end of the pipe closes it. All three look the same from inside: `read_line` returns `Ok(0)` and appends nothing.

Zero is a *success*. It is not an `Err`, and it will be returned again on every call after it. So a loop that keeps reading has one condition to write itself:

```rust
use std::io::{self, BufRead};

fn ask_number(input: &mut impl BufRead) -> io::Result<Option<u32>> {
    let mut line = String::new();
    loop {
        line.clear();
        let n = input.read_line(&mut line)?;
        if n == 0 {
            return Ok(None);                       // input ended before a number came
        }
        match line.trim().parse::<u32>() {
            Ok(v) => return Ok(Some(v)),
            Err(e) => println!("{:?} is not a number ({e}), asking again", line.trim()),
        }
    }
}
```

The `n == 0` check is the whole difference between a program that stops and one that spins forever on a closed pipe — [Endless iteration](../../02_Errors/endless_iteration/README.md) is the page for the loop that gets it wrong, and the kata below makes you write that loop and watch it. A last line typed without Enter still comes back on the call before the zero; section 4 of the run shows it.

## A prompt without a newline needs a `flush`

The opening program prints `How many? ` with `print!`, not `println!`, so the cursor stays on the prompt's line. That is the reason for the `flush()` on the next line. [`Stdout` ↗](https://doc.rust-lang.org/std/io/struct.Stdout.html) is *line-buffered* when it is a terminal: what you print is held back until a `\n` goes through, and a prompt has none. Without the flush, the program sits waiting for input with the prompt still in the buffer, and the person sees a blank line until *after* they have typed. `io::stdout().flush()` pushes it out; `println!` never needs one because the newline does it.

The same buffer is why [`process::exit` can lose output](../../02_Errors/stderr_and_exit_status/README.md#processexit-does-not-run-destructors) on the way out, and why a `BufWriter` you add for speed needs a flush of its own — [`Read` and `Write`](../../12_Traits/read_and_write/README.md).

## Windows, and the `\r`

A line from a Windows terminal, or a file saved there, ends in `\r\n`. `read_line` keeps both bytes. `trim()` takes both, so a program that trims is already fine; the one that bites is `trim_end()` against `trim()` when the leading spaces *matter*:

```rust
let line = "  seven \r\n";
line.trim_end();   // "  seven"  — the line ending is gone, the indent stays
line.trim();       // "seven"
```

[`trim_end`](../../14_Strings/str_methods/str_trim_end/README.md) is the one to reach for when indentation is data. The `lines()` iterator strips `\n` and `\r\n` for you and is the reason [RFC 1212](../../14_Strings/rfc_1212_line_endings/README.md) exists; `read_line`, its lower-level sibling, was left alone and still hands you the terminator.

## The one failure you will actually meet

`read_line` fills a `String`, and [a `String` promises UTF-8](../../04_Files/a_file_is_bytes/README.md). So a byte that is not UTF-8 — an `é` saved by Windows in code page 1250, a Latin-2 `ł` — is refused whole:

```text
Err: kind InvalidData, stream did not contain valid UTF-8
```

Section 7 of the run feeds exactly that byte and shows two things worth knowing: the buffer is left untouched, and the *next* `read_line` reads on past the bad line. The `expect("stdin is readable")` on the opening program is what turns that into a message and an exit; [Readers are fallible](../../02_Errors/readers_are_fallible/README.md) is the page for what else can go wrong that late. When the bytes are not text you promised anything about, ask for bytes — `read_until(b'\n', &mut Vec<u8>)` — which is where [Reading bytes](../reading_bytes/README.md) begins.

## Where the method lives, and when you need `lock()`

The opening program needed no `use` beyond `std::io`, because `read_line` is a method on [`Stdin` ↗](https://doc.rust-lang.org/std/io/struct.Stdin.html) itself, taking `&self` and locking the shared handle for the duration of one call. `io::stdin().lines()` is there too, since 1.62, and needs nothing extra either.

Take the lock yourself — `io::stdin().lock()` — for a loop that reads many lines, and the method comes from the [`BufRead` ↗](https://doc.rust-lang.org/std/io/trait.BufRead.html) trait instead, which has to be in scope:

```text title="Abridged — real rustc 1.98.0 output for lock_no_trait.rs, which calls lock().read_line without the import"
error[E0599]: no method named `read_line` found for struct `StdinLock<'a>` in the current scope
   |
5  |     io::stdin().lock().read_line(&mut line).expect("stdin is readable");
   |                        ^^^^^^^^^ method not found in `StdinLock<'static>`
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `BufRead` which provides `read_line` is implemented but not in scope; perhaps you want to import it
   |
1  + use std::io::BufRead;
```

The compiler names the fix. `use std::io::{self, BufRead};` is the import every loop on this page carries.

## How this page is checked

The tool that records every answer key in this library cannot type at a terminal, and an example that read the real stdin would wait forever in it. So every function in the example takes `impl BufRead`, and `main` feeds it a scripted transcript through [`io::Cursor` ↗](https://doc.rust-lang.org/std/io/struct.Cursor.html). `io::stdin().lock()` is a `BufRead` too: your program passes that instead, and the function does not change. That one line of difference is the whole of testing an input routine without a keyboard, and [`Read` and `Write`](../../12_Traits/read_and_write/README.md) is the page it is the opening move of.

## If you are coming from another language

**Python.** `input()` is the opening program with the trim built in: it prints the prompt, reads a line, strips the newline, and raises `EOFError` at end of input — three decisions Rust leaves to you. `sys.stdin.readline()` is the exact twin of `read_line`: it keeps the `\n` and returns `''` at the end, which is Python's spelling of `Ok(0)`. What does not transfer is the type of what came back. Python decoded the bytes into a `str` using the locale before you saw them, and a byte that does not decode raises inside `readline`; Rust's `read_line` does the same check, once, and the error says which kind it was. And `int("42\n")` works in Python — `int` strips whitespace itself — where `"42\n".parse::<u32>()` does not, so the `trim()` is the line a Python reader forgets.

**ABAP.** There is no standard input. A report's inputs come from a selection screen — `PARAMETERS p_n TYPE i.` is the prompt, the read and the parse in one declaration, with the type check done by the screen before your code runs — so the nearest shape to `read_line` is not interactive at all: `READ DATASET dset INTO line.` in a `DO` loop, where `sy-subrc = 4` is the end of the file and, like `Ok(0)`, it is a condition you test and not an exception you catch. What changes crossing over is where the newline went: `OPEN DATASET ... IN TEXT MODE` strips the line ending before you see the line, `read_line` hands it to you, and the `trim()` is yours.

---

## Practice

**Sum what you are given.** Numbers arrive on standard input, one per line, and the program prints their sum. Write `fn sum_lines(input: &mut impl BufRead) -> io::Result<Tally>` where `Tally` carries the sum, how many numbers were counted, and which lines were not numbers, then run it over scripted inputs the way this page's example does.

1. Blank lines are skipped, spaces around a number are fine, and a last line with no newline still counts. What does each of those need — `clear()`, `trim()`, or the `n == 0` check — and which one did you get for free?
2. A line that is not a number is reported with its line number and the sum goes on. Where does the line number come from, given that `read_line` does not count?
3. Feed it `"1\r\n2\r\n"`, as a Windows terminal would send it. Did anything need changing?
4. Now replace the exit check with `while let Ok(_) = input.read_line(&mut line)`. Bound the loop to five turns so the program finishes, and print what `read_line` returns on each. Why does the condition look correct, and what does the buffer hold on turn five?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:reading_stdin_kata -->
*[`reading_stdin_kata.rs`](examples/reading_stdin_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a sum over standard input that survives blank lines, spaces,
//! a bad line, Windows line endings and a last line with no newline — and the
//! loop that looks right and never ends.
//!
//!   rustc --edition 2024 reading_stdin_kata.rs -o /tmp/rsk && /tmp/rsk

use std::io::{self, BufRead, Cursor};

struct Tally {
    sum: u64,
    counted: usize,
    rejected: Vec<(usize, String)>, // (line number, what was there)
}

/// One number per line. Blank lines are skipped, spaces around a number are
/// fine, a line that is not a number is reported with its line number and the
/// sum goes on. The reader is any `BufRead`, so `main` can test it on a script.
fn sum_lines(input: &mut impl BufRead) -> io::Result<Tally> {
    let mut tally = Tally { sum: 0, counted: 0, rejected: Vec::new() };
    let mut line = String::new();
    let mut lineno = 0;
    loop {
        line.clear(); //                       (a) the buffer is reused
        if input.read_line(&mut line)? == 0 {
            break; //                          (b) Ok(0): nothing left
        }
        lineno += 1;
        let text = line.trim(); //             (c) the newline, \r\n or not, and the spaces
        if text.is_empty() {
            continue;
        }
        match text.parse::<u64>() {
            Ok(v) => {
                tally.sum += v;
                tally.counted += 1;
            }
            Err(_) => tally.rejected.push((lineno, text.to_string())),
        }
    }
    Ok(tally)
}

fn report(label: &str, typed: &str) {
    let t = sum_lines(&mut Cursor::new(typed)).unwrap();
    println!("   {label}: {typed:?}");
    println!("      sum {}  from {} numbers", t.sum, t.counted);
    for (n, what) in &t.rejected {
        println!("      line {n}: {what:?} is not a number, skipped");
    }
}

fn main() {
    println!("THE SUM, OVER FOUR THINGS THAT ARRIVE ON A REAL STDIN");
    report("plain    ", "3\n4\n5\n");
    report("messy    ", "3\n4\n\nfive\n 5 \n6");
    report("windows  ", "1\r\n2\r\n");
    report("nothing  ", "");

    println!();
    println!("THE LOOP THAT LOOKS RIGHT AND NEVER ENDS");
    println!("   while let Ok(_) = input.read_line(&mut line) {{ ... }}");
    println!("   Ok(0) is a success, so this condition is true at the end of the input,");
    println!("   and at the end of the input, and at the end of the input. Bounded to five");
    println!("   turns here so the page can show it:");
    let mut input = Cursor::new("7\n");
    let mut line = String::new();
    let mut turns = 0;
    while let Ok(n) = input.read_line(&mut line) {
        turns += 1;
        println!("      turn {turns}: Ok({n})   buffer {line:?}");
        if turns == 5 {
            println!("      ...still Ok(0), still looping. The guard is `n == 0`, and it is yours to write.");
            break;
        }
    }
    println!("   And without clear() the buffer kept the 7: read_line appends, even nothing.");
}
```
<!-- /source -->

<!-- output:reading_stdin_kata -->
*Verified output of [`reading_stdin_kata.rs`](examples/reading_stdin_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
THE SUM, OVER FOUR THINGS THAT ARRIVE ON A REAL STDIN
   plain    : "3\n4\n5\n"
      sum 12  from 3 numbers
   messy    : "3\n4\n\nfive\n 5 \n6"
      sum 18  from 4 numbers
      line 4: "five" is not a number, skipped
   windows  : "1\r\n2\r\n"
      sum 3  from 2 numbers
   nothing  : ""
      sum 0  from 0 numbers

THE LOOP THAT LOOKS RIGHT AND NEVER ENDS
   while let Ok(_) = input.read_line(&mut line) { ... }
   Ok(0) is a success, so this condition is true at the end of the input,
   and at the end of the input, and at the end of the input. Bounded to five
   turns here so the page can show it:
      turn 1: Ok(2)   buffer "7\n"
      turn 2: Ok(0)   buffer "7\n"
      turn 3: Ok(0)   buffer "7\n"
      turn 4: Ok(0)   buffer "7\n"
      turn 5: Ok(0)   buffer "7\n"
      ...still Ok(0), still looping. The guard is `n == 0`, and it is yours to write.
   And without clear() the buffer kept the 7: read_line appends, even nothing.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:reading_stdin -->
*Verified output of [`reading_stdin.rs`](examples/reading_stdin.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. ONE CALL: read_line APPENDS, KEEPS THE NEWLINE, RETURNS THE COUNT
   returned Ok(3)   buffer "42\n"
   Three bytes: '4', '2' and the newline. read_line does not strip it.

2. TRIM, THEN PARSE — AND WHAT parse SAYS IF YOU FORGET TO TRIM
   "42\n".parse()        -> Err: invalid digit found in string
   "42".parse()   -> Ok(42)
   The newline is an invalid digit. trim() takes it, and any spaces, off first.

3. A SECOND CALL WITHOUT clear(): THE BUFFER GROWS
   returned Ok(10)  buffer "42\n  seven \r\n"
   Both lines are in there. read_line appends to what you hand it; it never
   overwrites. A loop that forgets clear() reads every line into one String.

4. A LAST LINE WITH NO NEWLINE STILL COMES BACK; THEN Ok(0) IS THE END
   returned Ok(21)  buffer "last line, no newline"
   returned Ok(0)   buffer ""
   returned Ok(0)   buffer ""   (and again: still nothing)
   Zero bytes is not an error. It is the only signal that the input is over —
   Ctrl-D on Unix, Ctrl-Z Enter on Windows, or the end of whatever was piped in.

5. THE \r\n LINE: trim_end() KEEPS THE INDENT, trim() DOES NOT
   line            "  seven \r\n"
   line.trim_end() "  seven"   the line ending is gone, the two leading spaces stay
   line.trim()     "seven"   both ends
   lines() would have handed you "  seven " — it strips \n and \r\n for you.

6. THE LOOP: ASK UNTIL IT IS A NUMBER, OR UNTIL THERE IS NOTHING LEFT
   typed: abc, a blank line, then 17
      "abc" is not a number (invalid digit found in string), asking again
      "" is not a number (cannot parse integer from empty string), asking again
      -> Some(17)
   typed: only a wrong answer
      "forty-two" is not a number (invalid digit found in string), asking again
      -> None: the input ended first
   typed: nothing at all
      -> None: the input ended first

7. THE ONE FAILURE YOU WILL ACTUALLY MEET: BYTES THAT ARE NOT UTF-8
   Err: kind InvalidData, stream did not contain valid UTF-8
   buffer after the error: ""   (nothing was appended)
   the next call reads on: Ok(5) "next\n"
   read_line promises a String, and a String promises UTF-8, so one byte that
   is not UTF-8 is refused whole. A file saved by Windows in code page 1250 does
   this on its first accented letter. To read bytes you did not promise anything
   about, ask for bytes: read_until(b'\n', &mut Vec<u8>).

8. WHY EVERY READER ON THIS PAGE IS `impl BufRead`
   ask_number never named stdin. It takes any BufRead, so this program could feed
   it a Cursor over a string and record what it printed — and your program feeds
   it io::stdin().lock(). Same function, same body, one line different at the
   call site. That is the whole of testing an input routine without a keyboard.
```
<!-- /output -->

## See also

The input pages, in the order they build on each other:

- [A file or stdin](../a_file_or_stdin/README.md) — `tool FILE` and `cat FILE | tool` from the same function, the `-` convention, and knowing whether anyone is typing
- [Reading bytes](../reading_bytes/README.md) — when the input is not text you promised anything about: `Read`, short reads, and the sixteen-byte rows of a hex dump
- [Raw mode and passwords](../raw_mode_and_passwords/README.md) — a keypress without Enter, and a prompt that does not echo
- [Reading without blocking](../reading_without_blocking/README.md) — a read that can be given up on
- [Writing a file inspector](../writing_a_file_inspector/README.md) — the questions to ask before building the tool all of those pages lead to
- [Feeding stdin](../../11_Unix/feeding_stdin/README.md) — the shell side: `printf`, pipes, redirects, and how to see what your program is about to read

And the pages this one leaned on:

- [Randomness](../../15_First_Programs/randomness/README.md) — the Rust Book's guessing game, which is the opening program plus a loop
- [Endless iteration](../../02_Errors/endless_iteration/README.md) — the `while let Ok(_)` loop from the kata, at length
- [Readers are fallible](../../02_Errors/readers_are_fallible/README.md) — what a read can fail with after the file opened fine
- [Reading lines efficiently](../../04_Files/reading_lines_efficiently/README.md) — `read_line` against `lines()` against `read_to_string`, by what each allocates
- [Standard error, and exit status](../../02_Errors/stderr_and_exit_status/README.md) — the two output streams beside the one input stream
- [`Stdin::read_line` ↗](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line) · [`BufRead::read_line` ↗](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line) · [`Stdin::lines` ↗](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.lines) · [The Book's guessing game ↗](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

## Po polsku

`io::stdin().read_line(&mut line)` czyta ze standardowego wejścia (*standard input*, stdin) jeden wiersz i **dopisuje** go do łańcucha `String`, który sam przygotowałeś — razem ze znakiem nowej linii — a zwraca liczbę odczytanych bajtów. Stąd trzy nawyki, o które w tej lekcji chodzi: `clear()` przed wywołaniem, bo bufor nie jest nadpisywany, tylko rośnie; `trim()` po wywołaniu, bo `"42\n".parse::<u32>()` kończy się błędem *invalid digit found in string*, a znak nowej linii nie jest cyfrą; i spojrzenie na zwróconą liczbę, bo `Ok(0)` to jedyny sygnał końca wejścia — Ctrl-D w Uniksie, Ctrl-Z i Enter w Windows, koniec pliku przekierowanego potokiem — i nie jest to błąd, więc pętla, która pilnuje tylko `Err`, kręci się w nieskończoność. Czwarta rzecz, z którą polski czytelnik zderza się wcześniej niż angielski: `read_line` obiecuje `String`, a `String` obiecuje UTF-8, więc plik zapisany w Windows w stronie kodowej 1250 przewraca się na pierwszym „ł” z błędem `InvalidData` — bufor zostaje nietknięty, a kolejne wywołanie czyta dalej. Bajty, o których nic nie obiecywałeś, czyta się przez `read_until` do `Vec<u8>`, i to jest temat następnej strony.

Dwie drobne rzeczy, które kosztują kwadrans, jeśli nikt o nich nie powie. Zachęta bez znaku nowej linii (`print!("Ile? ")`) nie pojawi się na ekranie, dopóki nie wywołasz `io::stdout().flush()`, bo standardowe wyjście jest buforowane wierszami, a zachęta wiersza nie kończy. I `read_line` na samym `io::stdin()` nie wymaga żadnego importu poza `std::io`, ale ta sama metoda na `io::stdin().lock()` pochodzi z cechy (*trait*) `BufRead`, więc bez `use std::io::BufRead` kompilator odpowie `E0599` — i sam podpowie brakujący import.

**Szukaj po polsku:** czytanie ze standardowego wejścia · wczytywanie liczby z klawiatury · koniec pliku EOF · `rust read_line trim parse` · `rust stdin Ok(0) end of input` · `rust E0599 BufRead not in scope`
