# Inside a `Split`

**Level:** 201 → 301 · deep dive

**One line:** `split` hands back a *plan* — a struct holding the string, the needle and a cursor — so `{:?}` on it prints the plan, and the pieces do not exist until something consumes it.

```rust
fn main() {
    let s = "a:b:c";

    let r: Vec<&str> = s.split(':').collect();
    println!("{r:?}");        // ["a", "b", "c"]

    for part in s.split(':') {
        println!("{part}");   // a, then b, then c
    }
}
```

`collect` is the usual answer and the `for` loop is the other one. This page is about what you get when you skip both:

```rust
fn main() {
    let s = "a:b:c";
    let r = s.split(":");
    println!("{r:?}");
}
```

```text title="rustc 1.98.0 — these are std's private internals, not a stable API"
Split(SplitInternal { start: 0, end: 5, matcher: StrSearcher { haystack: "a:b:c", needle: ":", searcher: TwoWay(TwoWaySearcher { crit_pos: 0, crit_pos_back: 1, period: 1, byteset: 288230376151711744, position: 0, end: 5, memory: 0, memory_back: 1 }) }, allow_trailing_empty: true, finished: false })
```

Nothing has gone wrong. `Split` is an ordinary struct with a `Debug` impl, so `{:?}` printed its fields — and its fields are a search that has not happened yet. Every number in there describes the *setup*: where the cursor is, what it is looking for, and how it intends to look. `"a"`, `"b"` and `"c"` appear nowhere, because at the moment of printing they had never been computed.

## Reading it

Field by field, for `"a:b:c".split(":")`:

| field | here | what it is |
|---|---|---|
| `start` / `end` | `0` / `5` | the byte range still to be split — `end` is `s.len()`, in bytes |
| `matcher` | `StrSearcher` | the search machine, chosen by the *kind* of pattern you passed |
| `haystack` / `needle` | `"a:b:c"` / `":"` | both borrowed; the plan copies no text |
| `crit_pos`, `crit_pos_back`, `period` | `0`, `1`, `1` | constants of the [Two-Way algorithm ↗](https://en.wikipedia.org/wiki/Two-way_string-matching_algorithm) that std uses for `&str` needles |
| `byteset` | `288230376151711744` | a 64-bit fingerprint of the needle's bytes |
| `position` | `0` | how far the searcher has walked |
| `memory`, `memory_back` | `0`, `1` | indices into the needle before and after which a match is already known |
| `allow_trailing_empty` | `true` | keep a final empty piece — the single field that separates `split` from `split_terminator` |
| `finished` | `false` | nothing has been yielded yet |

### The pattern picks the machine

`split` takes anything implementing [`Pattern` ↗](https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html), and the four shapes are not four spellings of one search. Each builds a different searcher, which is why the dump for `split(":")` and the dump for `split(':')` do not even have the same field names:

```text
split(":")               -> StrSearcher
split(':')               -> CharSearcher
split(char::is_numeric)  -> CharPredicateSearcher
split(&['-', '_'][..])   -> CharSliceSearcher
```

A one-character `&str` needle drags in the full Two-Way machinery; the `char` gets a searcher that walks bytes looking for one UTF-8 encoding. Same pieces out, different work done — which is the practical reason to write `split(':')` rather than `split(":")` when one character is all you mean.

### `byteset` is a Bloom filter of one word

Its doc comment in std calls it *"a 64-bit fingerprint where each set bit `j` corresponds to a `(byte & 63) == j` present in the needle"* — a skip test, not part of Two-Way proper. So it is checkable by hand: `:` is byte 58, and

```text
byteset for ":"   288230376151711744
1 << (b':' & 63)  288230376151711744
```

`& 63` is the giveaway that this is a fingerprint rather than a set: bytes 64 apart land on the same bit, so the filter answers *"definitely absent"* or *"possibly present"*, and a false positive costs a comparison rather than a wrong answer.

### That huge number is a flag, not a length

`"the rain in spain".split("ain")` reports `memory: 18446744073709551615`, which is not a memory size:

```text
memory for "ain"  18446744073709551615
usize::MAX        18446744073709551615
```

std sets it there with the comment *"Dummy value to signify that the period is long"*, and reads it back as `let is_long = searcher.memory == usize::MAX`. It is a sentinel standing in for a boolean. This is the general hazard of reading a debug dump: the fields were named for the person maintaining the algorithm, and a value can be a flag, an index, an offset or an encoding without anything on the line saying which.

### `split_terminator` is the same struct with one bool flipped

```text
split            allow_trailing_empty: true
split_terminator allow_trailing_empty: false
```

Both are `SplitInternal`, both hold the same searcher. The whole documented difference between the two methods — whether `"a:b:".split(…)` ends with an empty piece — is that field. The [`str::split_terminator`](../str_methods/str_split_terminator/README.md) page describes the behaviour; this is the implementation of it.

### The plan is a cursor, and consuming moves it

```text
next()        start  position  finished
Some("a")         2         2     false
Some("b")         4         4     false
Some("c")         4         5      true
None              4         5      true
```

`start` is the front of the remaining text and `position` is where the searcher stopped, so the two leapfrog: after yielding `"b"` the cursor sits past the second colon but the searcher has not looked beyond it. `finished` flips on the call that yields the **last** piece, not on the call that returns `None` — the struct already knows there is nothing left before you ask.

That is also why an iterator cannot be rewound or read twice. It is not a view of a sequence; it is the state of a walk through one.

### Not every iterator hides its contents

```text
Chars(['a', ':', 'b', ':', 'c'])
```

`Chars` has a hand-written `Debug` (stable since 1.38) that consumes a *clone* of itself and prints the characters that are left. So `{:?}` on `s.chars()` shows you exactly the list you were hoping for, and `{:?}` on `s.split(":")` does not — the difference is one std author's choice, not a rule you can lean on. Print `.collect::<Vec<_>>()` and the question never arises.

### The compiler does not warn you here

Discarding an iterator is normally a lint:

```text title="Abridged — real rustc output, without the `#[warn]` note"
warning: unused `Chars` that must be used
 --> discarded.rs:3:5
  |
3 |     s.chars();
  |     ^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
help: use `let _ = ...` to ignore the resulting value
```

`s.split(":");` on the next line produces nothing at all. `Chars`, `Bytes`, `CharIndices`, `Lines` and every adapter (`Map`, `Filter`) carry `#[must_use = "iterators are lazy and do nothing unless consumed"]`; the split family does not, so a discarded split is silent. `Split`, `RSplit`, `SplitN`, `SplitTerminator` and `Matches` come out of one macro in `core::str::iter` whose invocations pass a doc comment and nothing more, and `SplitInclusive`, which is hand-written, omits the attribute too. Checked by hand on 1.98.0: one file, eleven discarded iterators, five warnings.

## None of this is an API

Field names, nesting and the choice of searcher are internal to `core`, and a future compiler may print something else entirely. Read a dump to understand a method or to see whether a cursor has moved; never `split_once` your way into one from a real program, and never assert on one in a test. The stable facts here are the two the page opened with: the value is a plan, and `collect` or a `for` loop is what turns it into pieces.

## If you are coming from another language

- **Python.** `"a:b:c".split(":")` is *eager* — it builds and returns a list, so printing it prints `['a', 'b', 'c']` and the question this page answers never comes up. Rust's `split` corresponds to `re.finditer` or an `itertools` object, and the Python 3 version of this exact surprise is `print(map(str.upper, xs))` showing `<map object at 0x7f…>`. Two differences worth carrying over. Python's repr of a lazy object tells you *nothing* — an address — where Rust's tells you everything, which is why this page can exist at all. And Python's laziness is a property of the function you called (`map` lazy, `str.split` eager, `sorted` eager, `reversed` lazy) with no rule to it, while in Rust it is a property of the *type*: if the return type is an iterator, nothing ran, and `Vec<&str>` on the left of the `=` is a promise that something did.
- **JavaScript.** `"a:b:c".split(":")` returns an `Array`, eagerly, same as Python. The nearest thing to what you saw is `console.log` on a generator, which Node prints as `Object [Generator] {}` — like Python's repr, it hides the state rather than dumping it. The mental hurdle in both directions is that `Array.prototype.map` and friends allocate a new array per step, so a JS chain of three `map`s walks the data three times and builds two arrays nobody wanted; the Rust chain builds one struct nested three deep and walks the data once, when `collect` finally asks. That struct-nested-three-deep is what `{:?}` would print.
- **ABAP.** `SPLIT text AT ':' INTO TABLE lt_parts` is eager and there is no lazy equivalent in the language — the internal table exists the moment the statement finishes, which is precisely the intermediate collection Rust is avoiding. The habit that transfers is the one from tuning a nested `LOOP`: you already know that building a table only to walk it once is waste, and that filtering in the `WHERE` beats a `CHECK` inside the loop. Rust's iterators make that the default rather than an optimisation. The habit that does *not* transfer is inspecting the result: `SPLIT` always gives you a table you can look at in the debugger, whereas the Rust value at the same point in the program has no pieces in it yet, and a debugger showing you `crit_pos` and `byteset` is showing you the truth.

## Example

<!-- source:inside_a_split -->
*[`inside_a_split.rs`](examples/inside_a_split.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// `split` returns a plan. This program prints the plan, then reads it.
//
// The field names below belong to std's private internals, not to a stable
// API: they are what rustc 1.98.0 prints, and a later compiler may print
// something else. Nothing here should be parsed by a real program.

/// Pull one scalar field out of a `{:?}` dump — `allow_trailing_empty: true,`
/// gives `"true"`, `matcher: StrSearcher {` gives `"StrSearcher"`.
fn field<'a>(dump: &'a str, name: &str) -> &'a str {
    let key = format!("{name}: ");
    let after = dump.split_once(key.as_str()).expect("field is present").1;
    after.split([',', ' ', '(']).next().expect("a value follows")
}

fn main() {
    let s = "a:b:c";

    // What you wanted: consume the iterator.
    let pieces: Vec<&str> = s.split(":").collect();
    println!("collected      {pieces:?}");

    // What `{:?}` on the iterator itself prints.
    println!("uncollected    {:?}", s.split(":"));

    // ---- the plan, one field per line ----
    println!("\n{:#?}", s.split(":"));

    // ---- the pattern you pass picks the searcher ----
    println!("\nthe four pattern shapes build four different machines:");
    for (call, dump) in [
        ("split(\":\")", format!("{:?}", s.split(":"))),
        ("split(':')", format!("{:?}", s.split(':'))),
        ("split(char::is_numeric)", format!("{:?}", s.split(char::is_numeric))),
        ("split(&['-', '_'][..])", format!("{:?}", s.split(&['-', '_'][..]))),
    ] {
        println!("  {call:<24} -> {}", field(&dump, "matcher"));
    }

    // ---- split vs split_terminator: one bool ----
    let split = format!("{:?}", s.split(":"));
    let term = format!("{:?}", s.split_terminator(":"));
    println!("\nsplit            allow_trailing_empty: {}", field(&split, "allow_trailing_empty"));
    println!("split_terminator allow_trailing_empty: {}", field(&term, "allow_trailing_empty"));

    // ---- byteset is a 64-bit fingerprint of the needle's bytes ----
    let fingerprint = |needle: &str| needle.bytes().fold(0u64, |set, b| set | (1u64 << (b & 63)));
    println!("\nbyteset for \":\"   {}", fingerprint(":"));
    println!("1 << (b':' & 63)  {}", 1u64 << (b':' & 63));
    println!("byteset for \"ain\" {}", fingerprint("ain"));

    // ---- memory: that huge number is a sentinel ----
    let long = format!("{:?}", "the rain in spain".split("ain"));
    println!("\nmemory for \"ain\"  {}", field(&long, "memory"));
    println!("usize::MAX        {}", usize::MAX);

    // ---- the plan is a cursor; consuming moves it ----
    let mut it = s.split(":");
    println!("\nthe same struct, after each next():");
    println!("  {:<12} {:>6} {:>9} {:>9}", "next()", "start", "position", "finished");
    for _ in 0..4 {
        let got = format!("{:?}", it.next());
        let d = format!("{it:?}");
        println!(
            "  {got:<12} {:>6} {:>9} {:>9}",
            field(&d, "start"),
            field(&d, "position"),
            field(&d, "finished"),
        );
    }

    // ---- not every iterator hides its contents ----
    println!("\n{:?}", s.chars());
}
```
<!-- /source -->

<!-- output:inside_a_split -->
*Verified output of [`inside_a_split.rs`](examples/inside_a_split.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
collected      ["a", "b", "c"]
uncollected    Split(SplitInternal { start: 0, end: 5, matcher: StrSearcher { haystack: "a:b:c", needle: ":", searcher: TwoWay(TwoWaySearcher { crit_pos: 0, crit_pos_back: 1, period: 1, byteset: 288230376151711744, position: 0, end: 5, memory: 0, memory_back: 1 }) }, allow_trailing_empty: true, finished: false })

Split(
    SplitInternal {
        start: 0,
        end: 5,
        matcher: StrSearcher {
            haystack: "a:b:c",
            needle: ":",
            searcher: TwoWay(
                TwoWaySearcher {
                    crit_pos: 0,
                    crit_pos_back: 1,
                    period: 1,
                    byteset: 288230376151711744,
                    position: 0,
                    end: 5,
                    memory: 0,
                    memory_back: 1,
                },
            ),
        },
        allow_trailing_empty: true,
        finished: false,
    },
)

the four pattern shapes build four different machines:
  split(":")               -> StrSearcher
  split(':')               -> CharSearcher
  split(char::is_numeric)  -> CharPredicateSearcher
  split(&['-', '_'][..])   -> CharSliceSearcher

split            allow_trailing_empty: true
split_terminator allow_trailing_empty: false

byteset for ":"   288230376151711744
1 << (b':' & 63)  288230376151711744
byteset for "ain" 72576357367808

memory for "ain"  18446744073709551615
usize::MAX        18446744073709551615

the same struct, after each next():
  next()        start  position  finished
  Some("a")         2         2     false
  Some("b")         4         4     false
  Some("c")         4         5      true
  None              4         5      true

Chars(['a', ':', 'b', ':', 'c'])
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/inside_a_split/examples/inside_a_split.rs -o /tmp/ias && /tmp/ias
```

## See also

- [`str::split`](../str_methods/str_split/README.md) — the method reference: what it returns, and why *n* matches give *n+1* pieces
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — the general rule this page is one worked instance of: adapters build, consumers run
- [Walking a `String`](../walking_a_string/README.md) — the three item types and the rest of the split family
- [`str::split_terminator`](../str_methods/str_split_terminator/README.md) — the one flipped bool, from the outside
- [`str` methods](../str_methods/README.md) — every method that returns one of these iterators
- [`std::str::pattern` ↗](https://doc.rust-lang.org/std/str/pattern/index.html) — the `Pattern` and `Searcher` traits, still unstable to implement · [`Split` ↗](https://doc.rust-lang.org/std/str/struct.Split.html)
