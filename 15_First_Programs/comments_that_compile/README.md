# Comments that compile

**Level:** 101 → 201 · for newcomers

**One line:** Rust has six comment forms and only two of them are comments — the other four are parsed into `#[doc = "..."]` attributes, they must be attached to something, and the code inside them is compiled and run as part of your test suite.

Every language has a way to write text the compiler throws away. Rust has that, and it also has a second thing that *looks* identical — one extra slash — and behaves completely differently. Reading `///` as "a comment, but tidier" is the mistake, and it is the reason people write documentation that silently documents nothing.

---

## The six forms

| Form | Name | What it does | Thrown away? |
|---|---|---|---|
| `// text` | line comment | nothing at all | yes |
| `/* text */` | block comment | nothing at all — and these **nest** | yes |
| `/// text` | outer doc comment | becomes `#[doc = "text"]` on the item **below** it | **no** |
| `//! text` | inner doc comment | becomes `#[doc = "text"]` on the item it is **inside** | **no** |
| `/** text */` | outer **block** doc comment | the same as `///`, delimited instead of per-line | **no** |
| `/*! text */` | inner **block** doc comment | the same as `//!`, delimited instead of per-line | **no** |

The top two are lexical: the compiler removes them before parsing, so you can put anything inside one. The other four are syntax, in the same way `#[derive(Debug)]` is syntax.

Two axes, not six unrelated things. **Line or block** is how the form is delimited; **outer or inner** is which item it attaches to. The bottom two rows carry no meaning the two above them do not, and [the section below](#the-block-doc-forms-and-the-one-character-that-breaks-them) is the case for never using them.

## Which way each one points

This is the whole distinction between the two doc forms, and the names are worth taking literally:

- **`///` is *outer*** — it stands outside the thing it describes and points **down** at the next item.
- **`//!` is *inner*** — it sits inside the thing it describes and points **out** at its container.

So `//!` at the top of a file documents *the file*, which is why it has to come before any item: it is describing the thing it is inside, and once an item has started, the enclosing thing is no longer the file. That is why almost every `//!` you will ever see is in the first ten lines of a file.

## The proof

Nothing above needs to be taken on trust. A `macro_rules!` arm can capture attributes as `meta` fragments, and `stringify!` prints the tokens it captured — so we can ask the compiler what it turned a `///` into, and read the answer in its own words:

<!-- source:comments_that_compile -->
*[`comments_that_compile.rs`](examples/comments_that_compile.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Two of Rust's six comment forms are not comments.
//!
//! `//` and `/* */` are thrown away by the lexer: the compiler never sees them,
//! and you can write anything at all inside one. The other four — `///`, `//!`,
//! `/** */` and `/*! */` — are PARSED: they become `#[doc = "..."]` attributes,
//! they must be attached to something, and the code inside them is compiled and
//! run as a test.
//!
//! This very block is the third form. It documents the file it is inside, which
//! is why it has to come before any item — and why it is `//!` and not `///`.
//!
//!   rustc --edition 2024 comments_that_compile.rs -o /tmp/ctc && /tmp/ctc

/// Reveals what the compiler turned a doc comment INTO.
///
/// A `meta` fragment matches an attribute, and `stringify!` prints the tokens
/// it captured — so if `///` really does desugar to `#[doc = "..."]`, this
/// macro will say so in its own words rather than ours.
macro_rules! reveal_attrs {
    ($reveal:ident; $label:expr; $(#[$m:meta])* struct $name:ident;) => {
        #[allow(dead_code)]
        struct $name;
        fn $reveal() {
            println!("  {} carries:", $label);
            $( println!("      #[{}]", stringify!($m)); )*
        }
    };
}

reveal_attrs! {
    reveal_ballot;
    "`Ballot`";
    /// A ballot.
    /** The same thing, in the block form. */
    #[doc = "Written the long way."]
    struct Ballot;
}

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

fn main() {
    // ───────────────────────────────────────────────────────────── 1
    banner(1, "`//` is erased before the parser runs");
    // let x: = = = ;   <- not Rust, and not a problem
    /* fn ( ) ] } "unterminated                                       */
    println!("  The two lines above this one are syntactic garbage.");
    println!("  The program compiled anyway, so nothing ever parsed them.");
    println!("      That is what \"the compiler ignores comments\" means, and");
    println!("      it is true of exactly two of the six forms.");

    // ───────────────────────────────────────────────────────────── 2
    banner(2, "`///` is not erased — it becomes an attribute");
    reveal_ballot();
    println!("      The macro captured a `meta` fragment, so those are real");
    println!("      attributes, not text. `///` is sugar for `#[doc = \"...\"]`,");
    println!("      which is why the two forms sit side by side above and why");
    println!("      the leading space after the slashes is preserved verbatim.");

    // ───────────────────────────────────────────────────────────── 3
    banner(3, "Which way each one points");
    println!("  ///  documents the item BELOW it        (outer — points down)");
    println!("  //!  documents the item it is INSIDE    (inner — points out)");
    println!("      So the `//!` block at the top of this file documents the");
    println!("      file. Put it after an item and there is nothing enclosing");
    println!("      it to describe, which is a compile error, not a warning.");

    // ───────────────────────────────────────────────────────────── 4
    banner(4, "A misplaced doc comment: warning, or error, or neither");
    {
        // /// this one attaches to the println   <- uncomment for the warning
        println!("  There are three answers, not one, and the first surprises");
        println!("  people: a doc comment on a STATEMENT is only a warning —");
        println!("        warning: unused doc comment");
        println!("        ...rustdoc does not generate documentation for");
        println!("           macro invocations");
        println!("  It attached to something, so it parsed. Nothing will ever");
        println!("  read it, and the build still succeeds.");
        // /// nothing at all follows this one    <- uncomment for error[E0585]
    }
    println!("      Uncomment the LAST line of that block, where nothing");
    println!("      follows, and it stops being a warning:");
    println!("        error[E0585]: found a documentation comment that");
    println!("                      doesn't document anything");
    println!("        help: doc comments must come before what they document,");
    println!("              if a comment was intended use `//`");
    println!("      And an inner `//!` at item level, after an item has begun:");
    println!("        error[E0753]: expected outer doc comment");
    println!("      So the trap is not the error. The error tells you. The trap");
    println!("      is the WARNING — a `///` you wrote inside a function is not");
    println!("      documentation, it is a comment with extra steps, and the");
    println!("      only thing that says so is a line in the build log.");

    // ───────────────────────────────────────────────────────────── 5
    banner(5, "Block comments nest, unlike C's");
    /* outer /* inner */ still inside the outer one */
    println!("  /* outer /* inner */ still commented */ compiles here.");
    println!("      In C the first `*/` ends it and the rest is a syntax error,");
    println!("      so commenting out a region that already contains a comment");
    println!("      breaks. In Rust the lexer counts the pairs, so it works.");

    // ───────────────────────────────────────────────────────────── 6
    banner(6, "The block DOC forms exist, and one character breaks them");
    println!("  /** ... */  is `///` with a block delimiter (outer)");
    println!("  /*! ... */  is `//!` with a block delimiter (inner)");
    println!("      Step 2 above proves it: the `/** */` line desugared to the");
    println!("      same `#[doc = ...]` as the slashes did. rustdoc renders the");
    println!("      two identically, and even strips a leading `*` column, so");
    println!("      the Javadoc habit costs nothing on the rendered page.");
    println!("  What it costs is here — a doc example that mentions `*/`:");
    println!("        /** Strips a C comment.");
    println!("");
    println!("        ```");
    println!("        let s = \"/* hi */\";");
    println!("        assert!(s.ends_with(\"*/\"));");
    println!("        ```");
    println!("        */");
    println!("      The comment ENDS at the `*/` inside the string, four lines");
    println!("      early. Everything after it is parsed as code, and the errors");
    println!("      land wherever the wreckage happens to stop:");
    println!("        error: prefix `B` is unknown");
    println!("        error[E0765]: unterminated double quote string");
    println!("      — reported on a `println!` further down the file that has");
    println!("      nothing wrong with it. A `///` block cannot do this: it ends");
    println!("      at the newline, so no content can terminate it early.");
    println!("      That is the whole case for the line forms, and it is why");
    println!("      you will almost never see a `/** */` in real Rust.");

    // ───────────────────────────────────────────────────────────── 7
    banner(7, "The reason any of this matters: doc comments are tested");
    println!("  A fenced block inside `///` is compiled and run by `cargo test`:");
    println!("        /// ```");
    println!("        /// assert_eq!(doubled(3), 6);");
    println!("        /// ```");
    println!("      Write 7 there and the test suite fails. So the examples in");
    println!("      your documentation cannot rot into ones that no longer");
    println!("      compile — the one kind of comment that is checked at all.");
}
```
<!-- /source -->

<!-- output:comments_that_compile -->
*Verified output of [`comments_that_compile.rs`](examples/comments_that_compile.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: `//` is erased before the parser runs
  The two lines above this one are syntactic garbage.
  The program compiled anyway, so nothing ever parsed them.
      That is what "the compiler ignores comments" means, and
      it is true of exactly two of the six forms.

──── Step 2: `///` is not erased — it becomes an attribute
  `Ballot` carries:
      #[doc = r" A ballot."]
      #[doc = r" The same thing, in the block form. "]
      #[doc = "Written the long way."]
      The macro captured a `meta` fragment, so those are real
      attributes, not text. `///` is sugar for `#[doc = "..."]`,
      which is why the two forms sit side by side above and why
      the leading space after the slashes is preserved verbatim.

──── Step 3: Which way each one points
  ///  documents the item BELOW it        (outer — points down)
  //!  documents the item it is INSIDE    (inner — points out)
      So the `//!` block at the top of this file documents the
      file. Put it after an item and there is nothing enclosing
      it to describe, which is a compile error, not a warning.

──── Step 4: A misplaced doc comment: warning, or error, or neither
  There are three answers, not one, and the first surprises
  people: a doc comment on a STATEMENT is only a warning —
        warning: unused doc comment
        ...rustdoc does not generate documentation for
           macro invocations
  It attached to something, so it parsed. Nothing will ever
  read it, and the build still succeeds.
      Uncomment the LAST line of that block, where nothing
      follows, and it stops being a warning:
        error[E0585]: found a documentation comment that
                      doesn't document anything
        help: doc comments must come before what they document,
              if a comment was intended use `//`
      And an inner `//!` at item level, after an item has begun:
        error[E0753]: expected outer doc comment
      So the trap is not the error. The error tells you. The trap
      is the WARNING — a `///` you wrote inside a function is not
      documentation, it is a comment with extra steps, and the
      only thing that says so is a line in the build log.

──── Step 5: Block comments nest, unlike C's
  /* outer /* inner */ still commented */ compiles here.
      In C the first `*/` ends it and the rest is a syntax error,
      so commenting out a region that already contains a comment
      breaks. In Rust the lexer counts the pairs, so it works.

──── Step 6: The block DOC forms exist, and one character breaks them
  /** ... */  is `///` with a block delimiter (outer)
  /*! ... */  is `//!` with a block delimiter (inner)
      Step 2 above proves it: the `/** */` line desugared to the
      same `#[doc = ...]` as the slashes did. rustdoc renders the
      two identically, and even strips a leading `*` column, so
      the Javadoc habit costs nothing on the rendered page.
  What it costs is here — a doc example that mentions `*/`:
        /** Strips a C comment.

        ```
        let s = "/* hi */";
        assert!(s.ends_with("*/"));
        ```
        */
      The comment ENDS at the `*/` inside the string, four lines
      early. Everything after it is parsed as code, and the errors
      land wherever the wreckage happens to stop:
        error: prefix `B` is unknown
        error[E0765]: unterminated double quote string
      — reported on a `println!` further down the file that has
      nothing wrong with it. A `///` block cannot do this: it ends
      at the newline, so no content can terminate it early.
      That is the whole case for the line forms, and it is why
      you will almost never see a `/** */` in real Rust.

──── Step 7: The reason any of this matters: doc comments are tested
  A fenced block inside `///` is compiled and run by `cargo test`:
        /// ```
        /// assert_eq!(doubled(3), 6);
        /// ```
      Write 7 there and the test suite fails. So the examples in
      your documentation cannot rot into ones that no longer
      compile — the one kind of comment that is checked at all.
```
<!-- /output -->

Step 2 is the one to look at twice:

```text
  `Ballot` carries:
      #[doc = r" A ballot."]
      #[doc = r" The same thing, in the block form. "]
      #[doc = "Written the long way."]
```

The macro matched `#[$m:meta]` — an *attribute* fragment. A `//` comment could never have matched it, because by then it does not exist. All three of those did, so `/// A ballot.` is `#[doc = r" A ballot."]` and nothing else, and the `/** */` beside it is the same attribute reached by a different spelling. Note the leading space is preserved, and that rustc reached for a raw string to hold it.

## The block doc forms, and the one character that breaks them

`/** */` and `/*! */` are `///` and `//!` with a block delimiter. Nothing more: step 2 of the output above shows the `/** */` line desugaring to the same `#[doc = …]` the slashes produced, and rustdoc renders the two identically — it even strips a leading `*` column, so the Javadoc habit costs nothing on the rendered page.

What it costs is that a block comment ends at the first `*/`, and a doc comment's job is to contain **examples**:

````rust,ignore
/** Strips a C comment.

```
let s = "/* hi */";
assert!(s.ends_with("*/"));
```
*/
pub fn f() {}
fn main() { f(); println!("compiled B"); }
````

The comment ends four lines early, at the `*/` inside the string. Everything after it is parsed as code, and the diagnostics land wherever the wreckage happens to stop — in the real file, on a `println!` further down that has nothing wrong with it:

```text title="Abridged — real rustc output for block_doc_terminates_early.rs"
error: prefix `B` is unknown
 --> block_doc_terminates_early.rs:9:37
  |
9 | fn main() { f(); println!("compiled B"); }
  |                                     ^ unknown prefix

error[E0765]: unterminated double quote string
```

A `///` cannot do this. It ends at the newline, so no content can terminate it early — which is why you will almost never see `/** */` in real Rust, and why this page's other five sections are written entirely in terms of the line forms.

## Three answers to a misplaced doc comment

"Put it in the wrong place and you get an error" would be a tidier lesson than the truth, and the truth is more useful. There are three outcomes, and the dangerous one is not the error:

**A doc comment on a statement is a warning.** It attached to something, so it parsed — rustdoc simply has nowhere to put it:

```text
warning: unused doc comment
  --> comments_that_compile.rs:70:9
   |
70 |         /// this one attaches to the println
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ rustdoc does not generate documentation for macro invocations
   |
   = note: `#[warn(unused_doc_comments)]` (part of `#[warn(unused)]`) on by default
```

**A doc comment with nothing after it is an error.** Now there is no item to attach to at all:

```text
error[E0585]: found a documentation comment that doesn't document anything
   |
   = help: doc comments must come before what they document, if a comment was intended use `//`
```

**An inner doc comment at item level, after an item has begun, is a different error** — rustc even offers to convert it for you:

```text
error[E0753]: expected outer doc comment
   |
help: to annotate the function, change the doc comment from inner to outer style
   |
2 - //! inner doc, but not at the top
2 + /// inner doc, but not at the top
```

The two errors are the harmless cases, because the compiler stops and tells you. The **warning** is the trap: a `///` you wrote inside a function body is not documentation and never will be, the build succeeds, and the only thing that ever says so is one line in a build log that scrolls past.

## Why this is worth caring about: the examples are tested

Here is the part that makes doc comments earn their syntax. A fenced code block inside `///` is not a display of code — it is a **test**, compiled and run by `cargo test` along with everything else.

Take a small crate with two documented functions, where the second one's example is wrong on purpose:

```rust
/// Doubles a score.
///
/// ```
/// assert_eq!(doctest_demo::doubled(3), 6);
/// ```
pub fn doubled(s: u8) -> u8 { s * 2 }

/// Halves a score. The example below is WRONG on purpose.
///
/// ```
/// assert_eq!(doctest_demo::halved(10), 99);
/// ```
pub fn halved(s: u8) -> u8 { s / 2 }
```

In a Cargo project that is `cargo test`. Without Cargo — this repo's usual setting, per [Running a scratch program](../rustc_without_cargo/README.md) — it is `rustdoc --test` against the built rlib:

```sh
rustc --edition 2024 --crate-type lib doctest_demo.rs
rustdoc --test doctest_demo.rs --edition 2024 -L . --extern doctest_demo=libdoctest_demo.rlib
```

```text
running 2 tests
test doctest_demo.rs - doubled (line 5) ... ok
test doctest_demo.rs - halved (line 12) ... FAILED

---- doctest_demo.rs - halved (line 12) stdout ----
Test executable failed (exit status: 101).

stderr:
assertion `left == right` failed
  left: 5
 right: 99

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

The documentation was run, and the lie in it was caught with a line number. That is the argument for the extra slash: a comment can rot quietly for years, and this one cannot. It is the same instinct as the [`.out` answer keys](../../CONTRIBUTING.md) this repo checks every page against — a claim nobody re-runs is a claim nobody should believe.

(The two tests run in parallel, so which line appears first varies between runs.)

## If you are coming from another language

**Python.** `//!` is the **module docstring** — the string at the top of a `.py` file, before any `def`, which becomes `__doc__`. `///` is the docstring of a function or class, except that Rust puts it *before* the `fn` rather than inside the body. Python has doctests too, so the payoff above will look familiar — with two real differences. Python's are **opt-in** (`python -m doctest`, or a pytest flag) while Rust's run under plain `cargo test`, so in Rust the default is that your examples are checked. And a Python docstring is a **runtime value** you can read back with `obj.__doc__`; a Rust doc comment is compile-time only, consumed by rustdoc and absent from the binary. Nothing in a running Rust program can read its own documentation, which is why the example above needed a macro to see one.

**ABAP.** The bridge is unusually direct: ABAP Doc is `"!` before a declaration, and Rust's `///` is `//` plus one character in the same spirit — the `!` even shows up in Rust's *other* doc form. Both feed a documentation generator rather than the compiler proper, and both sit before the thing they describe. Ordinary comments line up too: ABAP's `*` in column 1 is a whole-line comment and `"` is a trailing one, both discarded exactly like Rust's `//`. What ABAP Doc has no counterpart for is the last section — nothing compiles or runs the examples you write in it, so an ABAP Doc snippet is a promise, while a Rust doc example is a test that fails the build.

## Practice

**Put each comment where it does the job its author intended.**

Three of the four doc comments below are misplaced. For each one, predict which of the three answers you get — a warning, `E0585`, or `E0753` — then move it so it does what was clearly meant:

```rust
struct Ballot { score: u8 }

fn main() {
    /// the program's own description
    let score: u8 = 3;
    println!("score = {score}");
    /// a note about main
}
```

Then prove the surviving doc comment actually landed, by printing the attribute back with the `reveal_attrs!` macro from the lesson. A fix you can see is worth more than one you assume.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:comments_that_compile_kata -->
*[`comments_that_compile_kata.rs`](examples/comments_that_compile_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Solution: the four comments, each moved to where it does its job.
//!
//! This inner doc comment is the first fix — in the broken version it was a
//! `///` sitting inside `main`, where it documented nothing and earned a
//! warning. A description of the whole file is what `//!` is for, and it has
//! to come before any item.

/// Prints the attributes an item actually carries, so a fix can be checked
/// rather than assumed. Same instrument as the lesson.
macro_rules! reveal_attrs {
    ($reveal:ident; $label:expr; $(#[$m:meta])* struct $name:ident { $field:ident : $ty:ty }) => {
        #[allow(dead_code)]
        struct $name { $field: $ty }
        fn $reveal() {
            println!("{} carries:", $label);
            $( println!("    #[{}]", stringify!($m)); )*
        }
    };
}

reveal_attrs! {
    reveal_ballot;
    "`Ballot`";
    /// One voter's filled-in paper.
    struct Ballot { score: u8 }
}

fn main() {
    reveal_ballot();

    // The second fix: this was a `///` before a `let`, which is a statement,
    // not an item — a warning, and documentation nobody would ever read. An
    // ordinary `//` is what a note-to-the-reader inside a function should be.
    let score: u8 = 3;

    println!("score = {score}");

    // The third fix: the trailing `///` with nothing after it was E0585.
    // Deleted, because it was a note about the function above it — and a note
    // about an item goes BEFORE the item, which is the whole rule.
}
```
<!-- /source -->

<!-- output:comments_that_compile_kata -->
*Verified output of [`comments_that_compile_kata.rs`](examples/comments_that_compile_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
`Ballot` carries:
    #[doc = r" One voter's filled-in paper."]
score = 3
```
<!-- /output -->

</details>

## See also

- [Running a scratch program](../rustc_without_cargo/README.md) — `rustc` alone, and where `cargo test` would have run the doctests for you
- [Formatting](../../05_Tooling/formatting/README.md) — `rustfmt` never touches a comment's contents, which is why comment style stays your problem
- [A shadow does not drop](../../18_Ownership/shadowing_does_not_drop/README.md) — the same trick as Step 2: make an invisible compiler behaviour print something

## Po polsku

Rust ma cztery formy komentarza i dwie z nich komentarzami nie są. `//` oraz `/* */` znikają w lekserze, zanim cokolwiek zdąży je sparsować — można w nich napisać dowolne śmieci składniowe. Natomiast `///` i `//!` to **składnia**: parser zamienia je na atrybut `#[doc = "..."]`, dokładnie tak, jak traktuje `#[derive(Debug)]`. Polski czytelnik zna zwykle „komentarz dokumentacyjny” (*doc comment*) z Javy (`/** */`) i to niezła pierwsza intuicja, ale różnica jest zasadnicza: tam była to konwencja czytana przez zewnętrzne narzędzie, tutaj jest to element języka, który ląduje w drzewie składniowym. Strona nie każe w to wierzyć na słowo — makro łapie fragment `meta` i wypisuje, co kompilator naprawdę zrobił: `#[doc = r" A ballot."]`, razem z zachowaną spacją po ukośnikach.

Nazwy obu form warto brać dosłownie. `///` jest **zewnętrzny** (*outer*): stoi na zewnątrz opisywanego elementu i wskazuje **w dół**, na to, co jest pod nim. `//!` jest **wewnętrzny** (*inner*): siedzi w środku i opisuje to, co go otacza — dlatego prawie każdy `//!`, jaki zobaczysz, znajduje się w pierwszych dziesięciu linijkach pliku, bo opisuje właśnie ten plik, a kiedy zacznie się pierwszy element, nie ma już czego opisywać. Przy okazji drobiazg, który ucieszy każdego, kto uczył się na C: komentarze blokowe w Ruście **się zagnieżdżają**. `/* zewnętrzny /* wewnętrzny */ nadal zakomentowany */` kompiluje się bez problemu, bo lekser liczy pary — zakomentowanie fragmentu kodu, w którym już jest komentarz, po prostu działa, zamiast rozsypywać składnię na pierwszym `*/`.

Najważniejsza rzecz z tej strony jest jednak inna: źle postawiony komentarz dokumentacyjny ma **trzy** różne zakończenia i najgroźniejsze z nich nie jest błędem. `///` napisany wewnątrz ciała funkcji przyczepia się do instrukcji, więc parsuje się poprawnie i daje tylko `warning: unused doc comment` — kompilacja się udaje, dokumentacja nie powstaje, a jedynym śladem jest linijka w logu budowania, która przewija się w ciągu sekundy. Dopiero gdy po komentarzu nie ma już nic, dostajesz `error[E0585]` z podpowiedzią, żeby użyć zwykłego `//`; a `//!` wstawiony po rozpoczęciu elementu to `error[E0753]`, przy którym rustc sam proponuje zamianę na `///`. Błędy są tu przypadkiem łagodnym, bo kompilator się zatrzymuje i mówi, o co chodzi. Pułapką jest ostrzeżenie.

Po co więc całe to zamieszanie z jednym ukośnikiem więcej? Bo blok kodu wstawiony wewnątrz `///` nie jest ozdobą, tylko **testem**: `cargo test` go kompiluje i uruchamia (bez Cargo robi to `rustdoc --test`), a przykład z błędem nie przechodzi i pokazuje numer linii — tutaj `assert_eq!` z celowo złą liczbą kończy się jako `left: 5, right: 99`. Stąd praktyczne pytanie, którego angielska wersja strony nie musi sobie zadawać: **w jakim języku pisać dokumentację?** Dla crate'a publikowanego na crates.io odpowiedź jest jednoznaczna — po angielsku, bo docs.rs nie ma wersji językowych, a szuka się i tak po angielsku. W kodzie wewnętrznym polski jest jak najbardziej w porządku (pliki źródłowe Rusta są z definicji w UTF-8, więc ogonki w `///` niczego nie psują), a co najważniejsze: język prozy nie zmienia nic w sprawie przykładów — kod w bloku i tak zostanie skompilowany i uruchomiony.

**Szukaj po polsku:** komentarz dokumentacyjny · dokumentowanie kodu w Ruście · `rust doc comments /// //!` · `rust doctest cargo test` · `rust E0585 doesn't document anything` · `rust unused doc comment warning`
