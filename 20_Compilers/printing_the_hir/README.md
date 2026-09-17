# Printing the HIR: what rustc turned your code into

**Level:** 201 · working knowledge

**One line:** `RUSTC_BOOTSTRAP=1 rustc --edition 2024 -Zunpretty=hir main.rs` prints your program after rustc has expanded its macros and rewritten `for`, `?` and friends into `loop` and `match` — the form type checking works on — and builds nothing.

```rust
fn main() {
    let s = String::from("foo");
    println!("{s}");   // foo
}
```

```bash
RUSTC_BOOTSTRAP=1 rustc --edition 2024 -Zunpretty=hir main.rs
```

```text title="rustc 1.98.0 — the whole output"
extern crate std;
#[attr = PreludeImport]
use std::prelude::rust_2024::*;
fn main() {
    let s = String::from("foo"); // foo
    {
        ::std::io::_print({
                super let args = (&s,);
                super let args = [format_argument::new_display(args.0)];
                unsafe { format_arguments::new(b"\xc0\x01\n\x00", &args) }
            });
    };
}
```

The `println!` is gone. In its place is a call to `std::io::_print` with a tuple holding `&s`, which is why [printing borrows](../../18_Ownership/printing_borrows/README.md) rather than moves. No `main` binary is written: the printout is the whole job.

The `// foo` moved. It belonged to the `println!` line and is printed on the `let` line, [one of the traps below](#a-comment-can-land-on-the-wrong-line).

---

## The command, one word at a time

| Part | What it does | Leave it out |
|---|---|---|
| `RUSTC_BOOTSTRAP=1` | Sets an environment variable for this one command, and a stable `rustc` then acts as a nightly one, accepting `-Z` flags | *the option `Z` is only accepted on the nightly compiler* |
| `rustc` | The compiler itself, with no Cargo in between | — |
| `--edition 2024` | The edition to read the file as | rustc reads it as **2015**, its default |
| `-Zunpretty=hir` | Print the HIR, then stop | rustc compiles and links `./main` as usual |
| `main.rs` | The crate root: the file compilation starts from | — |

The `NAME=value command` prefix sets the variable for that one process only. bash, zsh and fish (checked on 4.3.2) all accept it.

Without the variable:

```text
error: the option `Z` is only accepted on the nightly compiler

help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: 1 nightly option were parsed
```

The `help` line suggests changing the default toolchain for every project on the machine to get one flag for one command — [the wrong trade](../../05_Tooling/nightly/README.md).

## What HIR is

rustc does not check the text you wrote. It parses it into a syntax tree, expands macros and resolves names, then **lowers** the result into the High-level Intermediate Representation. The [rustc dev guide ↗](https://rustc-dev-guide.rust-lang.org/overview.html) lists what runs on the HIR: type inference, trait solving and type checking. Borrow checking comes two stages later, on MIR, by way of THIR.

HIR still reads like Rust, with the sugar taken out. That makes it the stage worth printing when the question is *what did this macro, this loop, this `?` become?*

## What the HIR rewrites

```rust
fn double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

fn main() {
    let mut total = 0;
    for i in 0..3 {
        total += i;
    }
    let _ = (total, double("21"));
}
```

```text title="rustc 1.98.0, -Zunpretty=hir — the whole output"
extern crate std;
#[attr = PreludeImport]
use std::prelude::rust_2024::*;
fn double(s: &'_ str)
    ->
        Result<i32,
        std::num::ParseIntError> {
    let n: i32 =
        match branch(s.parse()) {
            Break {  0: residual } => #[allow(unreachable_code)]
                return from_residual(residual),
            Continue {  0: val } => #[allow(unreachable_code)]
                val,
        };
    Ok(n * 2)
}

fn main() {
    let mut total = 0;
    {
        let _t =
            match into_iter(Range { start: 0, end: 3 }) {
                mut iter =>
                    loop {
                        match next(&mut iter) {
                            None {} => break,
                            Some {  0: i } => { total += i; }
                        }
                    },
            };
        _t
    };
    let _ = (total, double("21"));
}
```

- **`0..3` is a struct literal:** `Range { start: 0, end: 3 }`.
- **`for` is a `loop` inside two `match`es.** `into_iter` turns the range into an iterator bound as `mut iter`, each turn calls `next(&mut iter)`, and the `None` arm holds the only `break`. [The Reference ↗](https://doc.rust-lang.org/reference/expressions/loop-expr.html#r-expr.loop.for.desugar) states the same equivalence, down to the block that binds the loop's result — `_t` here.
- **`?` is a `match` with an early `return`.** `branch` sorts the `Result` into `Continue`, which carries on with the value, or `Break`, which returns `from_residual(residual)`.
- **The elided lifetime is written out:** `&str` became `&'_ str`.

The bare names are trait methods. `-Zunpretty=hir,typed` prints them in full: `<std::ops::Range<i32> as IntoIterator>::into_iter`, `<std::ops::Range<i32> as Iterator>::next` and `<Result<i32, ParseIntError> as Try>::branch`.

## The rewrite, run

Stable code cannot call what the HIR calls: [`Try::branch` ↗](https://doc.rust-lang.org/std/ops/trait.Try.html) is nightly-only. The example writes both rewrites back out by hand — the `for` as printed, and the `?` in the form [the Reference ↗](https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.try) gives for a `Result`, where `Err(e)` returns `Err(From::from(e))` — and runs each beside the original:

<!-- output:printing_the_hir -->
*Verified output of [`printing_the_hir.rs`](examples/printing_the_hir.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── for, as written and as the HIR prints it
  end = 0   sum = 0   sum_desugared = 0   same: true
  end = 3   sum = 3   sum_desugared = 3   same: true
  end = 5   sum = 10  sum_desugared = 10  same: true

──── ?, as written and as a match
  "21"  Ok(42)
        Ok(42)   same: true
  "x"   Err(ParseIntError { kind: InvalidDigit })
        Err(ParseIntError { kind: InvalidDigit })   same: true
  ""    Err(ParseIntError { kind: Empty })
        Err(ParseIntError { kind: Empty })   same: true
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 20_Compilers/printing_the_hir/examples/printing_the_hir.rs -o /tmp/pth && /tmp/pth
```

## Other values of `-Zunpretty`

A value rustc does not know makes it list the ones it does:

```text
error: argument to `unpretty` must be one of `normal`, `expanded`, `expanded,identified`, `expanded,hygiene`, `ast-tree`, `ast-tree,expanded`, `hir`, `hir,identified`, `hir,typed`, `hir-tree`, `thir-tree`, `thir-flat`, `mir`, `stable-mir`, or `mir-cfg`; got bogus
```

The ones worth knowing, with line counts for the `double` file above on 1.98.0:

| Value | Prints | Lines |
|---|---|---|
| `expanded` | The source after macro expansion and nothing more — `for` and `?` are still there, and a `println!` stops at `format_args!` with no `&` in sight | 14 |
| `hir` | This page | 34 |
| `hir,typed` | The HIR with every expression cast to its type, as in `(s as &str).parse() as Result<i32, ParseIntError>` | 53 |
| `mir` | MIR, the borrow checker's input: numbered locals, basic blocks, and arguments marked `move` or `copy` | 140 |
| `mir-cfg` | The same MIR as a Graphviz graph | 48 |
| `ast-tree`, `hir-tree`, `thir-tree` | The compiler's own data structures, `Debug`-printed | 701, 1,645, 1,275 |

## The traps

### Leaving out `--edition` still prints a dump

rustc's default edition is 2015, so 2024 syntax is refused — and the HIR is printed anyway:

```rust
fn main() {
    let v = Some(3);
    if let Some(n) = v && n > 2 {
        println!("{n}");
    }
}
```

```text title="stderr of RUSTC_BOOTSTRAP=1 rustc -Zunpretty=hir main.rs"
error: let chains are only allowed in Rust 2024 or later
 --> main.rs:3:8
  |
3 |     if let Some(n) = v && n > 2 {
  |        ^^^^^^^^^^^^^^^

error: aborting due to 1 previous error
```

Stdout gets 15 lines of HIR, and the exit status is 1. Pipe stdout into a file or a pager and the error is easy to miss, leaving a dump that looks like a success. Its third line gives the edition away: `use ::std::prelude::rust_2015::*;`. `cargo rustc` does not have this trap, because Cargo passes the `edition` from `Cargo.toml`.

### The printout is not Rust

Save the first dump on this page as a `.rs` file and compile it:

```text title="Abridged — only the lines that start with error"
error: attribute value must be a literal
error: cannot find attribute `attr` in this scope
error[E0658]: `super let` is experimental
error[E0658]: `super let` is experimental
error[E0658]: use of unstable library feature `print_internals`: implementation detail which may disappear or be replaced at any time
error[E0433]: cannot find module or crate `format_argument` in this scope
error[E0433]: cannot find module or crate `format_arguments` in this scope
error: aborting due to 7 previous errors
```

It is for reading. [The example](#the-rewrite-run) is what a translation back into compilable Rust takes.

### A comment can land on the wrong line

Comments survive into the printout, but a comment after a macro call has no statement left to sit on: the macro became a block. It is printed before that block. At the top of this page that put `// foo` on the end of the `let` line, where it reads as if `String::from` printed something. When the line above already has a comment, the stray one gets a line of its own instead:

```rust
fn main() {
    let a = 1;   // one
    let b = 2;   // two
    println!("{a} {b}");   // 1 2
}
```

```text title="rustc 1.98.0, -Zunpretty=hir — abridged: from fn main to the start of the block"
fn main() {
    let a = 1; // one
    let b = 2; // two
    // 1 2
    {
```

`-Zunpretty=expanded` places them the same way.

### `RUSTC_BOOTSTRAP=1` unlocks more than `-Z`

It makes the compiler accept any `#![feature]`, which is why it belongs on one command and never in a shell profile:

```rust
#![feature(iter_intersperse)]

fn main() {
    let joined: String = ["a", "b", "c"].into_iter().intersperse("-").collect();
    println!("{joined}");   // a-b-c
}
```

```text title="rustc --edition 2024 main.rs, without the variable"
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> main.rs:1:1
  |
1 | #![feature(iter_intersperse)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0554`.
```

With the variable set, the same file compiles on stable 1.98.0 with no warning and prints `a-b-c`. [The unstable book ↗](https://doc.rust-lang.org/nightly/unstable-book/compiler-environment-variables/RUSTC_BOOTSTRAP.html) says using it *"opts you out of the normal stability/backwards compatibility guarantee"*.

### No promise about the output

`-Zunpretty` has no tracking issue, and [the unstable book ↗](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/unpretty.html) calls it likely internal to the compiler. The `mir` dump opens with its own warning that the format is subject to change without notice. For the three-line program at the top of this page, the HIR was byte-identical on 1.97.1, 1.98.0 and nightly 1.100.0 (2026-08-27) — a measurement of three compilers, not a guarantee about the next one.

## Other ways to run it

```bash
rustc +nightly --edition 2024 -Zunpretty=hir main.rs   # a nightly toolchain installed: no variable
RUSTC_BOOTSTRAP=1 cargo rustc -- -Zunpretty=hir         # inside a Cargo project: edition from Cargo.toml
```

`cargo rustc` without the variable fails with the same *only accepted on the nightly compiler* error. For macro expansion alone there is the [`cargo-expand` ↗](https://github.com/dtolnay/cargo-expand) crate, which prints the result of macro and `#[derive]` expansion and formats it with rustfmt when rustfmt is installed. It was not run for this page.

## If you are coming from another language

- **C and C++.** `clang -E` prints the source after the preprocessor, which is the counterpart of `-Zunpretty=expanded`: with `#define TWICE(x) ((x) * 2)`, a `return TWICE(21);` comes out as `return ((21) * 2);`. The counterpart of `hir` is [C++ Insights ↗](https://cppinsights.io/), written, in its author's words, to transform a range-based for-loop into the compiler-internal version: the `begin`/`end` iterator loop that plays the part of Rust's `into_iter` and `next`. What transfers is the question — what did the compiler actually see? What differs is that a C preprocessor pastes text, while a Rust macro's expansion is still checked as Rust, and HIR is a stage inside the compiler rather than a translation tool's reconstruction.
- **Python.** `ast.dump(ast.parse(...))` keeps a `for` as a `For` node, which is the counterpart of `ast-tree`. The rewrite shows up one level lower, in bytecode: on Python 3.14.7, [`dis.dis` ↗](https://docs.python.org/3/library/dis.html) shows a `for` over `range(3)` as `GET_ITER`, then a `FOR_ITER` that jumps past the loop when the iterator runs out. Those two opcodes do the jobs of `into_iter` and the `None` arm. Python has no stage that checks types between the two, and that stage is what HIR exists for.

## See also

- [Printing borrows](../../18_Ownership/printing_borrows/README.md) — one `println!` read through this printout, for the `&` that makes printing a borrow
- [What a compiler does before your program runs](../what_a_compiler_does/README.md) — the compile-time/run-time line this whole stage sits on the near side of
- [LLVM and its IR](../llvm_and_its_ir/README.md) — the IR rustc hands on to LLVM once MIR is done, where the optimizer works
- [`rustup default nightly`](../../05_Tooling/nightly/README.md) — the other way to get `-Z` flags, and why it is the wrong default
- [Macros](../../25_Control_Flow/macros/README.md) — what the `!` means, before looking at what it became
- [`for` loops](../../25_Control_Flow/for_loops/README.md) — the loop whose desugaring this page prints, taught from the user's side
- [*Rust in Action* §2.4, run](../../25_Control_Flow/flow_control_claims_checked/README.md#claim-21-read-from-the-hir) — the same printout for a `while` loop, which becomes a `loop` around an `if` rather than a `match`

## Documentation

- [`RUSTC_BOOTSTRAP` ↗](https://doc.rust-lang.org/nightly/unstable-book/compiler-environment-variables/RUSTC_BOOTSTRAP.html) — what it enables, the `=crate_name` and `=-1` forms, and why it exists at all
- [`-Z unpretty` ↗](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/unpretty.html) — the unstable book's entry, one paragraph long
- [rustc command-line arguments ↗](https://doc.rust-lang.org/rustc/command-line-arguments.html) — `--edition`, and its default of 2015
- The rustc dev guide: [overview ↗](https://rustc-dev-guide.rust-lang.org/overview.html), [the HIR ↗](https://rustc-dev-guide.rust-lang.org/hir.html), [lowering ↗](https://rustc-dev-guide.rust-lang.org/hir/lowering.html) and [MIR ↗](https://rustc-dev-guide.rust-lang.org/mir/index.html)
- The Reference: [`for` desugaring ↗](https://doc.rust-lang.org/reference/expressions/loop-expr.html#r-expr.loop.for.desugar) and [the `?` operator ↗](https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.try)
- [`IntoIterator` ↗](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html), [`Iterator::next` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next) and [`Range` ↗](https://doc.rust-lang.org/std/ops/struct.Range.html) — the three names the `for` rewrite uses
- [Editions ↗](https://doc.rust-lang.org/edition-guide/editions/index.html) — what `--edition` selects
- Python's [`ast` ↗](https://docs.python.org/3/library/ast.html) module, for the bridge above

## Po polsku

Polecenie `RUSTC_BOOTSTRAP=1 rustc --edition 2024 -Zunpretty=hir main.rs` pokazuje program w postaci, na której `rustc` naprawdę pracuje: po rozwinięciu makr i po **usunięciu lukru składniowego** (*desugaring*). Pętla `for` staje się `loop` wewnątrz dwóch `match`, operator `?` — `match` z wczesnym `return`, a `println!` — wywołaniem `_print` z krotką referencji `(&s,)`. HIR (*High-level Intermediate Representation*) to wysokopoziomowa reprezentacja pośrednia: na niej działają wnioskowanie typów i kontrola typów, a borrow checker pracuje dwa etapy dalej, na MIR. Nic się przy tym nie kompiluje — plik wykonywalny nie powstaje.

Każda część polecenia ma swoją pułapkę. Zmienna `RUSTC_BOOTSTRAP=1` sprawia, że stabilny kompilator zachowuje się jak nocny (*nightly*) i przyjmuje flagi `-Z` — ale także dowolne `#![feature]`, dlatego ustawia się ją dla jednego polecenia, nigdy w pliku startowym powłoki. Bez `--edition 2024` `rustc` czyta plik jako edycję 2015: składnia z 2024 daje błąd, a wydruk HIR i tak się pojawia. Sam wydruk nie jest poprawnym Rustem — zapisany jako plik `.rs` daje siedem błędów — komentarz stojący za wywołaniem makra potrafi trafić do innej linii, a format nie ma żadnej gwarancji stabilności między wersjami.

**Szukaj po polsku:** reprezentacja pośrednia HIR · lukier składniowy w Ruscie · `rustc -Zunpretty=hir` · `RUSTC_BOOTSTRAP` · `cargo expand`
