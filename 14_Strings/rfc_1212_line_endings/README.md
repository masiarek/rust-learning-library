# RFC 1212 — how `lines()` learned about `\r\n`

**Level:** 201 · working knowledge

**One line:** A one-line change to `str::lines`, argued for six weeks over a question the RFC never asked — and one residual surprise its three-word *Unresolved questions* section could not see.

```rust
let unix    = "name,qty\nbolt,12\n";
let windows = "name,qty\r\nbolt,12\r\n";
println!("{}", unix.lines().eq(windows.lines()));  // true
```

That has been true since **Rust 1.4.0** (2015-10-29), which lists it under *Breaking Changes*. Before it, the Windows string gave you two lines each carrying an invisible `\r` on the end — and the bug reached you only when somebody ran your program on the other operating system.

---

## What it asked for

Ralf Jung filed [RFC 1212 ↗](https://github.com/rust-lang/rfcs/blob/master/text/1212-line-endings.md) on 2015-07-15, years before Miri and Stacked Borrows made them a fixture of the project. It was their first — *"Yay, my first RFC got accepted :)"* — and its motivation is a sentence of experience rather than of theory:

> The editor has personally run into this issue when reading line-by-line from stdin, with the program suddenly failing on Windows.

The whole design fits in a paragraph. `BufRead::lines` and `str::lines` should both treat `\r\n` as a line ending, implemented by splitting on `\n` exactly as before and removing a trailing `\r` on the way out. `str::lines_any`, which already did that and was the only function in std that did, becomes redundant and is deprecated. Under *Unresolved questions*: **"None I can think of."**

## What was wrong was worse than "it only handles `\n`"

The RFC leads with surprise — other languages open files in a text mode, so a programmer arriving from one does not expect to handle this themselves. BurntSushi's comment in the thread is the sharper diagnosis, and it is not an argument about expectations at all. There are two coherent designs for a line iterator: hand back the line with its terminator, or hand it back without. Pre-1.4 `lines()` implemented **neither**.

| the line as written | what pre-1.4 `lines()` returned | what it removed |
|---|---|---|
| `"bolt,12\n"` | `"bolt,12"` | all of the terminator |
| `"bolt,12\r\n"` | `"bolt,12\r"` | half of it |

So it was not a Unix-only function that Windows users had to work around. It trimmed the terminator when the terminator was one byte and half of it when it was two, which is the behaviour that produces the bug rather than merely permitting it: a `\r` on the end of a `&str` is invisible in `println!`, and it makes `line == "bolt,12"` false with nothing on screen to say why. `{:?}` is the only thing that shows it.

## The change, run

<!-- source:rfc_1212_line_endings -->
*[`rfc_1212_line_endings.rs`](examples/rfc_1212_line_endings.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
#![allow(deprecated)]

use std::io::{BufRead, Cursor};

/// Pre-1.4 `str::lines` was exactly this: split on '\n', drop the empty
/// piece a trailing '\n' leaves behind, and hand back whatever else is
/// there — including the '\r'.
fn lines_before_1_4(s: &str) -> Vec<&str> {
    s.split_terminator('\n').collect()
}

fn main() {
    let unix = "name,qty\nbolt,12\n";
    let windows = "name,qty\r\nbolt,12\r\n";

    println!("WHAT RFC 1212 CHANGED");
    println!("  before  unix    {:?}", lines_before_1_4(unix));
    println!("  before  windows {:?}", lines_before_1_4(windows));
    println!("  today   unix    {:?}", unix.lines().collect::<Vec<&str>>());
    println!("  today   windows {:?}", windows.lines().collect::<Vec<&str>>());
    println!("  the two agree now: {}", unix.lines().eq(windows.lines()));

    println!();
    println!("BURNTSUSHI'S DIAGNOSIS: THE OLD BEHAVIOUR WAS NOT EVEN CONSISTENT");
    for (label, line) in [("lf  ", "bolt,12\n"), ("crlf", "bolt,12\r\n")] {
        let old = lines_before_1_4(line)[0];
        println!(
            "  {label} ended {:<6} -> {:<11} trimmed {}",
            format!("{:?}", &line[7..]),
            format!("{old:?}"),
            if old.ends_with('\r') { "the \\n only" } else { "all of it" }
        );
    }

    println!();
    println!("THE SURPRISE \"NONE I CAN THINK OF\" DID NOT SEE");
    let truncated = "name,qty\r\nbolt,12\r";
    println!("  no final \\n   {:?}", truncated.lines().collect::<Vec<&str>>());
    println!("  same, lf only {:?}", "name,qty\nbolt,12".lines().collect::<Vec<&str>>());
    let last = truncated.lines().next_back().unwrap();
    println!("  last line ends with \\r: {}", last.ends_with('\r'));
    println!("  and it fails the obvious test: last == \"bolt,12\" is {}", last == "bolt,12");
    println!("  a lone \\r splits nothing: {:?}", "old\rmac".lines().collect::<Vec<&str>>());

    println!();
    println!("BufRead::lines GOT THE SAME CHANGE; read_line DID NOT");
    let read = |s: &'static str| {
        Cursor::new(s).lines().map(|l| l.unwrap()).collect::<Vec<String>>()
    };
    println!("  lines      {:?}", read("name,qty\r\nbolt,12\r\n"));
    println!("  lines tail {:?}", read("name,qty\r\nbolt,12\r"));
    let mut buf = String::new();
    Cursor::new(windows).read_line(&mut buf).unwrap();
    println!("  read_line  {:?}   <- the terminator is still yours to strip", buf);

    println!();
    println!("BRSON'S QUESTION, ASKED IN THE THREAD AND NEVER ANSWERED BY THE RFC");
    let copied: String = windows.lines().map(|l| format!("{l}\n")).collect();
    println!("  read {} bytes, wrote {} bytes back, identical: {}",
             windows.len(), copied.len(), windows == copied);

    println!();
    println!("THE UNICODE SEPARATORS THE RFC WAS ASKED FOR AND REFUSED");
    println!("  char         lines()  is_whitespace  split_whitespace");
    for (name, c) in [
        ("VT   U+000B", '\u{0B}'), ("FF   U+000C", '\u{0C}'),
        ("NEL  U+0085", '\u{85}'), ("LS   U+2028", '\u{2028}'),
        ("PS   U+2029", '\u{2029}'),
    ] {
        let s = format!("a{c}b");
        println!("  {name}  {}        {:<5}          {:?}",
                 s.lines().count(), c.is_whitespace(),
                 s.split_whitespace().collect::<Vec<&str>>());
    }

    println!();
    println!("THE ALTERNATIVES THE RFC POINTED AT, AS THEY BEHAVE TODAY");
    println!("  split('\\n')        {:?}", windows.split('\n').collect::<Vec<&str>>());
    println!("  split_inclusive    {:?}", windows.split_inclusive('\n').collect::<Vec<&str>>());
    let raw: Vec<Vec<u8>> = Cursor::new("ab\r\ncd\r\n").split(b'\n').map(|v| v.unwrap()).collect();
    println!("  BufRead::split     {raw:?}");
    println!("                     ^ Vec<u8>, not String -- and the 13 is the \\r");
    println!("  lines_any          {:?}", windows.lines_any().collect::<Vec<&str>>());
}
```
<!-- /source -->

<!-- output:rfc_1212_line_endings -->
*Verified output of [`rfc_1212_line_endings.rs`](examples/rfc_1212_line_endings.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
WHAT RFC 1212 CHANGED
  before  unix    ["name,qty", "bolt,12"]
  before  windows ["name,qty\r", "bolt,12\r"]
  today   unix    ["name,qty", "bolt,12"]
  today   windows ["name,qty", "bolt,12"]
  the two agree now: true

BURNTSUSHI'S DIAGNOSIS: THE OLD BEHAVIOUR WAS NOT EVEN CONSISTENT
  lf   ended "\n"   -> "bolt,12"   trimmed all of it
  crlf ended "\r\n" -> "bolt,12\r" trimmed the \n only

THE SURPRISE "NONE I CAN THINK OF" DID NOT SEE
  no final \n   ["name,qty", "bolt,12\r"]
  same, lf only ["name,qty", "bolt,12"]
  last line ends with \r: true
  and it fails the obvious test: last == "bolt,12" is false
  a lone \r splits nothing: ["old\rmac"]

BufRead::lines GOT THE SAME CHANGE; read_line DID NOT
  lines      ["name,qty", "bolt,12"]
  lines tail ["name,qty", "bolt,12\r"]
  read_line  "name,qty\r\n"   <- the terminator is still yours to strip

BRSON'S QUESTION, ASKED IN THE THREAD AND NEVER ANSWERED BY THE RFC
  read 19 bytes, wrote 17 bytes back, identical: false

THE UNICODE SEPARATORS THE RFC WAS ASKED FOR AND REFUSED
  char         lines()  is_whitespace  split_whitespace
  VT   U+000B  1        true           ["a", "b"]
  FF   U+000C  1        true           ["a", "b"]
  NEL  U+0085  1        true           ["a", "b"]
  LS   U+2028  1        true           ["a", "b"]
  PS   U+2029  1        true           ["a", "b"]

THE ALTERNATIVES THE RFC POINTED AT, AS THEY BEHAVE TODAY
  split('\n')        ["name,qty\r", "bolt,12\r", ""]
  split_inclusive    ["name,qty\r\n", "bolt,12\r\n"]
  BufRead::split     [[97, 98, 13], [99, 100, 13]]
                     ^ Vec<u8>, not String -- and the 13 is the \r
  lines_any          ["name,qty", "bolt,12"]
```
<!-- /output -->

## The implementation was smaller than the discussion

[rust#28034 ↗](https://github.com/rust-lang/rust/pull/28034) opened **half an hour** after the RFC merged. Ralf Jung had offered to write it, and aturon's reply on the RFC was *"looks like @alexcrichton beat you to it"*; six files, and the core of it is one adapter moving between two functions:

```rust,ignore
// before
fn lines(&self)     -> Lines    { Lines(self.split_terminator('\n')) }
fn lines_any(&self) -> LinesAny { LinesAny(self.lines().map(LinesAnyMap)) }

// after
fn lines(&self)     -> Lines    { Lines(self.split_terminator('\n').map(LinesAnyMap)) }
fn lines_any(&self) -> LinesAny { LinesAny(self.lines()) }
```

`LinesAnyMap` — the closure that strips the `\r` — already existed and was already correct. The semantics-breaking change to a stable API was moving one `.map()` call up one line. `BufRead::lines` got three lines: after popping the `\n`, pop a `\r` too.

Two of the six files are call sites inside Rust's own toolchain — rustdoc's `unindent` and the lexer's doc-comment stripper both had to stop saying `lines_any()`. And brson asked the question with no good answer: how do you publicize a change that is *silently* breaking? aturon's reply is the part worth keeping — a semantic change like this "is very hard to check for breakage using crater". Tooling can find every call site that stops compiling. Nothing can find the ones that keep compiling and quietly start meaning something else.

## The question the thread argued about instead

The RFC's own topic was settled within a week, and the discussion moved to one it never raised: if `lines()` is going to know about line endings, should it know about *all* of them? Unicode names several more — `\v`, `\f`, U+0085 NEL, U+2028 LINE SEPARATOR, U+2029 PARAGRAPH SEPARATOR — and [UAX #18 ↗](https://www.unicode.org/reports/tr18/) says a conforming implementation should honour them. withoutboats proposed it, nagisa turned a 👎 into the condition for their vote, and aturon leaned that way too, on the grounds that it fit Rust's general embrace of Unicode.

What settled it was Simon Sapin — author of [RFC 69](../rfc_69_byte_literals/README.md), one page up — going and reading the standard rather than the report about it, and concluding: *"Let's stick with `\n` and `\r\n`."* withoutboats then argued their way out of their own proposal in public, which is the best comment in the thread: a protocol may define `\n` as its separator and permit arbitrary Unicode inside a line, and a Unicode-aware `lines()` would corrupt exactly that, rarely enough to reach production.

The decision is defensible and it left a seam you can measure, because `split_whitespace` was never part of the bargain. Those five characters are whitespace to Rust — `char::is_whitespace` is true, `trim()` removes them, `split_whitespace()` splits on them — and `lines()` does not treat any of them as a line. Two std functions, the same byte, different answers. Worth knowing before you point `lines()` at text you did not produce yourself.

## The surprise "None I can think of" did not see

Here is the case the three-word section does not cover, and it is not exotic. It is a CRLF file whose last line has no terminator — which is what every editor that does not add a final newline produces:

```rust
let truncated = "name,qty\r\nbolt,12\r";
println!("{:?}", truncated.lines().last());  // Some("bolt,12\r")
```

The rule is precisely the one the RFC wrote: split at `\n`, then strip a `\r` sitting in front of it. A `\r` with no `\n` after it was never in front of anything, so it stays — and it stays on the one line you are least likely to have a test for.

This was not found later and grudgingly documented. It is in the implementing PR's own test suite, changed on the day: `Cursor::new(&b"12\r"[..])` is asserted to yield `"12\r"`. Today [`str::lines` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.lines) says so in as many words — a carriage return not immediately followed by a line feed does not split a line, and is included in the line — so it is a documented decision, not a bug. But nothing in RFC 1212 predicts it, and the argument for the whole change was that people should not have to think about `\r`.

## brson's question, still unanswered

Also raised in the thread, and answered by nobody:

> If we make this change, then reading the lines of a file and outputting them again will produce a different file.

It does. Read a CRLF file with `lines()`, write each line back with a `\n`, and you have silently converted the file — 19 bytes in, 17 out in the run above. birkenfeld's reply pointed out that `lines()` was already lossy, since it never told you whether the last line had a terminator, and that a convenience API is allowed to be. Both halves of that are true. It still means **`lines()` is a parser, not a codec**: reach for it to *read* text, and never as half of a round trip. If bytes have to survive, [`split_inclusive`](../str_methods/str_split_inclusive/README.md) keeps the terminators attached and is the tool for that job — it arrived in 1.51, six years too late to be in the RFC's list of alternatives.

## `read_line` is the sibling that kept the terminator

RFC 1212 changed `str::lines` and `BufRead::lines`. It did not change `BufRead::read_line`, which is the loop you write when you want one buffer instead of one `String` per line — and which still hands you `"name,qty\r\n"`, terminator and all. Ralf Jung noted in the thread that `read_line` and `lines` ought to be in sync; they are, on the question the RFC was about, since both stop in the same place. They differ on what they give you, which is the whole point of `read_line`, and so the `\r` is still yours to deal with.

The obvious fix is the wrong one. `trim_end()` also eats trailing spaces and tabs, which are data in a TSV or a fixed-width record: `"  a,b  \r\n".trim_end()` is `"  a,b"`, and two significant spaces are gone. This is what `lines()` actually does, and it is worth writing out once:

```rust
fn strip_terminator(line: &str) -> &str {
    match line.strip_suffix('\n') {
        Some(rest) => rest.strip_suffix('\r').unwrap_or(rest),
        None => line,
    }
}
// "bolt,12\r\n" -> "bolt,12"      "  a,b  \r\n" -> "  a,b  "
```

Note the last arm: it reproduces the trailing-`\r` behaviour too, because it is the same rule. That is the price of agreeing with std rather than guessing.

## If you are coming from another language

**Python.** The RFC's motivation names it without naming it — *"Many languages open files in a 'text-mode' per default"* — and Python is the language it means. Python does **both** of the things this thread rejected.

| Python | | Rust |
|---|---|---|
| `open(path)` | universal newlines: `\r\n`, `\r` and `\n` all arrive as `\n` | no equivalent; a `File` gives you the bytes that are there |
| `open(path, newline='')` | turns the conversion off | the default, and the only mode |
| [`str.splitlines()` ↗](https://docs.python.org/3/library/stdtypes.html#str.splitlines) | splits on **eleven** line boundaries — `\v`, `\f`, `\x1c`–`\x1e`, `\x85`, U+2028 and U+2029 as well as the three obvious ones | [`str::lines`](../str_methods/str_lines/README.md) splits on two |
| `str.split("\n")` | mechanical; keeps the `\r`, adds a phantom last element | [`str::split`](../str_methods/str_split/README.md) — identical |
| `f.readline()` | keeps the `\n`, which text mode already converted | `read_line` keeps the `\r\n`, unconverted |

Row one is mitsuhiko's proposal from the thread — put the conversion at the I/O boundary, once, instead of in every function that says "line" — shipped and working for decades. Row three is withoutboats'. Rust took neither, and the reason it could not take the first is structural rather than a matter of taste: Python's text mode is a *decoding* step, bytes to `str`, and the newline conversion rides along inside it. Rust reads bytes and validates them separately, so there is no step in between for a conversion to live in. aturon made exactly that argument in the thread — Rust does not convert on the way in, it exposes functions that assume the data already is what it claims.

Porting, the practical difference is one line: a Python program that reads a CRLF file and compares `line == "bolt,12"` works, and its direct Rust translation using `read_line` does not. Translating with `lines()` works — except on the final line of a file that has no trailing newline.

**ABAP.** ABAP is squarely in mitsuhiko's camp and always has been: `OPEN DATASET … IN TEXT MODE ENCODING UTF-8` makes `READ DATASET` strip the line-end marker and `TRANSFER` append one, so the conversion sits at the I/O boundary and never appears in your code. There is no `lines()` to get wrong, because you never see a terminator.

The trap moves rather than disappearing, and it lands on the manual route. `SPLIT lv_text AT cl_abap_char_utilities=>cr_lf INTO TABLE lt_lines` is RFC 1212's bug in a mirror — hard-code CRLF, and a file that arrives with LF only comes back as one enormous line instead of a table. The two constants are `cl_abap_char_utilities=>newline` (LF) and `=>cr_lf` (the pair); what ABAP has no constant for is *either*, which is what `lines()` became in 1.4. The nearest honest equivalent is to normalise first — `REPLACE ALL OCCURRENCES OF cl_abap_char_utilities=>cr_lf IN lv_text WITH cl_abap_char_utilities=>newline` — and then split on `newline`, which is the RFC's own implementation strategy written out longhand.

## See also

- [`str::lines`](../str_methods/str_lines/README.md) — the reference page: signature, edge cases, the phantom empty line `split('\n')` adds
- [`str::lines_any`](../str_methods/str_lines_any/README.md) — the name this RFC deprecated, still compiling
- [`str::split_inclusive`](../str_methods/str_split_inclusive/README.md) — terminators kept, for when the bytes have to survive
- [`str::trim_end`](../str_methods/str_trim_end/README.md) — the fix that takes too much
- [RFC 69 — how Rust got `b'A'`](../rfc_69_byte_literals/README.md) — the other RFC this library reads line by line, by the person who settled the Unicode question here
- [Reading lines efficiently](../../04_Files/reading_lines_efficiently/README.md) — `read_to_string` vs `lines()` vs a reused buffer; a **stub** for now
- [RFC 1212 ↗](https://github.com/rust-lang/rfcs/blob/master/text/1212-line-endings.md) — one page; read it, it is shorter than this
- [rust-lang/rfcs#1212 ↗](https://github.com/rust-lang/rfcs/pull/1212) — the six-week thread, which is the interesting half
- [rust#28034 ↗](https://github.com/rust-lang/rust/pull/28034) — the implementation, `std: Account for CRLF in {str, BufRead}::lines`

## Po polsku

RFC 1212 to zmiana z 2015 roku, dzięki której `lines()` traktuje `\r\n` tak samo jak `\n`. Dla polskiego czytelnika jest to zwykle ważniejsze niż dla angielskiego, bo CRLF pojawia się tu wszędzie: Notatnik, eksport CSV z Excela, `core.autocrlf` w gicie, pliki wymiany z systemów SAP. Przed Rustem 1.4 `lines()` obcinał tylko `\n`, więc każdy wiersz z takiego pliku kończył się niewidocznym znakiem powrotu karetki — a `println!` go nie pokazuje, więc porównanie `wiersz == "tak"` po cichu dawało `false` i nic na ekranie tego nie tłumaczyło. Pokazuje go dopiero `{:?}`.

Dwie rzeczy warto zapamiętać poza samą zmianą. **Ostatni wiersz bywa wyjątkiem:** reguła brzmi „podziel po `\n`, potem usuń `\r` stojący tuż przed nim", więc w pliku CRLF bez końcowego znaku nowej linii ostatni wiersz **zachowuje** swoje `\r` — to jest udokumentowane zachowanie biblioteki standardowej, nie błąd, ale RFC go nie przewidziało. I **`read_line` nie został zmieniony:** oddaje wiersz razem z zakończeniem, więc `\r` zostaje. Kuszące `trim_end()` nie jest tu dobrą odpowiedzią, bo zjada też końcowe spacje i tabulatory, które w pliku TSV są danymi — właściwa jest funkcja `strip_terminator` z tej strony, czyli dokładnie to, co robi `lines()`.

Osobna ciekawostka: w wątku dyskusyjnym proponowano, by `lines()` uznawał również uniksowe i unikodowe separatory (`\v`, `\f`, U+0085, U+2028, U+2029). Propozycję odrzucono — ale te znaki nadal są w Ruscie „białymi znakami", więc `trim()` je usuwa, a `split_whitespace()` dzieli po nich. Dwie funkcje z tej samej biblioteki odpowiadają na to samo pytanie inaczej.

**Szukaj po polsku:** znaki końca wiersza · powrót karetki i wysuw wiersza · CRLF a LF · tryb tekstowy pliku · `rust lines crlf` · `rust read_line trailing carriage return`
