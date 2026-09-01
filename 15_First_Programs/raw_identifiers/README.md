# Raw identifiers `r#`

**Level:** 201 · working knowledge

**One line:** `r#name` lets a keyword be an ordinary name — and it exists because Rust's keyword list is a property of the **edition**, so adding a keyword to the language must not break a crate that already used the word.

```rust
fn r#match(r#type: u8) -> u8 {
    r#type + 1
}

struct Header {
    r#type: &'static str,
}

let h = Header { r#type: "text/csv" };
println!("{} {}", r#match(4), h.r#type);   // 5 text/csv
```

## The compiler suggests it before you know it exists

```text title="Abridged — real rustc output for keyword_as_name.rs"
error: expected identifier, found keyword `type`
 --> keyword_as_name.rs:1:17
  |
1 | fn main() { let type = 1u8; println!("{type}"); }
  |                 ^^^^ expected identifier, found keyword
  |
help: escape `type` to use it as an identifier
  |
1 | fn main() { let r#type = 1u8; println!("{type}"); }
  |                 ++
```

That `help` line is where most people meet `r#`. It is not obscure trivia — it is the first fix rustc offers.

## Why the language needs an escape at all

The keyword list is per-edition, and it grows. Measured with `rustc 1.98.0`, compiling the same one-line file at four `--edition` values:

| Word | 2015 | 2018 | 2021 | 2024 |
|---|---|---|---|---|
| `async` | a name | **keyword** | keyword | keyword |
| `gen` | a name | a name | a name | **keyword** |

A 2015 crate could perfectly well export `fn async()`. Editions are per-crate and a build mixes them freely, so a 2024 crate has to be able to call that function — and `r#async` is how. That is the design constraint the feature exists to satisfy: **new keywords must not be a breaking change.** Everything else about `r#` is a consequence.

## It escapes the parser, not the name

```rust
fn r#ordinary() -> &'static str { "one function" }

println!("{} {}", ordinary(), r#ordinary());   // one function one function
```

`r#` on a word that was never a keyword is legal and does nothing. There is one function here, callable by either spelling, because the raw form is not a different identifier — it is the same identifier written so the parser cannot mistake it for syntax. The distinction leaks in exactly one place, a macro re-printing the token it captured:

```rust
println!("{} {}", stringify!(r#type), stringify!(r#ordinary));   // r#type r#ordinary
```

`stringify!` prints what you wrote, prefix included. Name resolution does not care; a `macro_rules!` arm comparing strings does.

## Four names refuse it

```text
error: `crate` cannot be a raw identifier
```

`crate`, `self`, `Self` and `super` are rejected. They are not keywords in the ordinary sense — they are **path roots**, resolved by [the module system](../../27_Modules/modules_and_visibility/README.md) rather than parsed as names, so there is nothing an escape could turn them into. Every other keyword takes the prefix, `r#async`, `r#dyn`, `r#gen`, `r#union` and `r#macro_rules` included.

## Lifetimes take it too

```rust
fn longest<'r#fn>(a: &'r#fn str, b: &'r#fn str) -> &'r#fn str {
    if a.len() >= b.len() { a } else { b }
}
println!("{}", longest("Ada", "Cara"));   // Cara
```

The tick comes **first**: `'r#fn`, not `r#'fn`. What is being escaped is the identifier, and the `'` is the sigil that says a [lifetime](../../18_Ownership/README.md) follows. Raw lifetimes exist for the same reason as raw identifiers and for no other.

## Where you will actually type it

Almost never in your own code — you would just pick a different name. Where it earns its place is at a **boundary you do not control**:

- A serialisation format whose field really is called `type`. `#[derive(Deserialize)] struct Header { r#type: String }` maps to JSON's `"type"` with no rename attribute, because the field's name *is* `type`.
- A C or FFI binding whose function is called `new`, `move` or `match`.
- A crate published under an older edition, called from a newer one.
- Generated code, where the generator has no way to know that the name it was handed is a keyword — and where `r#` on a non-keyword being a harmless no-op is what makes "just always prefix it" a correct strategy.

## Not to be confused with the raw string

```rust
let path = r#"a "quoted" path: C:\tmp"#;
println!("{path}");   // a "quoted" path: C:\tmp
```

Same letter, unrelated feature. [A raw string](../../14_Strings/raw_strings_and_escapes/README.md) turns off escape processing inside a literal; a raw identifier turns off keyword recognition on a name. The tell is what follows the `#`: **a quote means string, a letter means identifier.**

## If you are coming from another language

**Python.** The convention you know is the trailing underscore — `class_`, `type_`, `id_` — recommended by PEP 8 precisely for this case, and it produces a *different name* from the keyword. `r#type` produces the **same** name, which is the point: a Python wrapper around an API with a `class` field has to rename it and then remember the mapping at every boundary (`getattr(obj, "class")`, `**{"class": v}`, `dict` literals instead of keyword arguments), while Rust's field genuinely is called `type` and serialises as `type` with nothing to remember. The other half does not transfer at all: Python's keyword list is a property of the *version*, and adding one — `async` and `await` in 3.7, `match` and `case` softened in 3.10 — really did break code, which is why `match` was made a *soft* keyword instead. Rust reached for editions rather than soft keywords, and `r#` is the escape hatch that made that choice affordable.

**ABAP.** The nearest thing is the escape character `!` before a parameter name (`METHODS m IMPORTING !type TYPE string`), used for exactly this problem — a name that collides with an ABAP keyword — and it works the same way: the escape is not part of the name, and callers may write it or not. Two differences worth knowing. ABAP's reserved-word list is a property of the *release* and of the syntax check, not of a per-program setting, so there is no edition-style opt-in and no equivalent of calling a 2015 crate from 2024 code. And ABAP's dictionary-driven field names are frequently keywords by accident (`TYPE`, `KEY`, `CLIENT`), which is why the escape is common in generated ABAP and rare in hand-written ABAP — the same split you see in Rust, arrived at from the opposite direction.

## The verified output

<!-- output:raw_identifiers -->
*Verified output of [`raw_identifiers.rs`](examples/raw_identifiers.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: A keyword is not a name, and rustc says how to make it one
  let type = 1u8;
        error: expected identifier, found keyword `type`
        help: escape `type` to use it as an identifier
           |     let r#type = 1u8;
           |         ++
      The fix is in the error. `r#` is not obscure trivia — it is
      the first thing the compiler suggests.

──── Step 2: `r#` works on items, bindings, fields and lifetimes
  fn r#match(r#type: u8)      r#match(4) = 5
  struct Header { r#type }    text/csv (3 fields set)
  fn longest<'r#fn>(..)       "Cara"
      A raw lifetime is `'r#name`, not `r#'name` — the tick comes
      first, because what is being escaped is the identifier.

──── Step 3: `r#` is an escape, not part of the name
  fn r#ordinary() declared once, called two ways:
      ordinary()   -> same function, either way
      r#ordinary() -> same function, either way
      One function, two spellings, no ambiguity — so `r#` on a
      non-keyword is legal and does nothing. What it escapes is
      the PARSER, not the name.
  One place the distinction leaks: a macro re-printing the token
      stringify!(r#type)     = r#type
      stringify!(r#ordinary) = r#ordinary
      keeps the prefix, because it is printing what you wrote.

──── Step 4: Four names refuse it
  r#crate  r#self  r#Self  r#super
        error: `crate` cannot be a raw identifier
      These four are not keywords in the ordinary sense — they are
      PATHS, resolved by the module system rather than parsed as
      names, so there is nothing an escape could turn them into.
      Every other keyword takes `r#`, including `r#async`,
      `r#dyn`, `r#gen`, `r#union` and `r#macro_rules`.

──── Step 5: Do not confuse it with the raw STRING
  r#foo      a raw IDENTIFIER — a name that was a keyword
  r#"..."#   a raw STRING — a literal with no escape sequences
      a "quoted" path: C:\tmp
      Same letter, unrelated features. The tell is what follows
      the `#`: a quote means string, a letter means identifier.
```
<!-- /output -->

## See also

- [Variables](../variables/README.md) — the `let` this page is escaping
- [Raw strings and escapes](../../14_Strings/raw_strings_and_escapes/README.md) — the other `r#`, and the one you will use far more
- [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) — `crate`, `self` and `super`, the four names that refuse the prefix
- [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) — where this library's edition and compiler are set
- [Raw identifiers ↗](https://doc.rust-lang.org/reference/identifiers.html#raw-identifiers) · [Keywords ↗](https://doc.rust-lang.org/reference/keywords.html) · [Editions ↗](https://doc.rust-lang.org/edition-guide/)

## Po polsku

`r#nazwa` to **surowy identyfikator** (*raw identifier*): sposób na użycie słowa kluczowego jako zwykłej nazwy. Nie jest to ciekawostka — kompilator sam go podpowiada, gdy tylko napiszesz `let type = 1;`, w linijce `help: escape \`type\` to use it as an identifier`.

Powód istnienia tego mechanizmu jest jeden i warto go znać, bo tłumaczy całą resztę: **lista słów kluczowych zależy od edycji**. `async` było zwykłą nazwą do edycji 2015 włącznie, a słowem kluczowym stało się w 2018; `gen` było nazwą jeszcze w 2021, a w 2024 już nie. Edycje ustawia się osobno dla każdego pakietu i można je swobodnie mieszać w jednym programie, więc kod z 2024 musi umieć wywołać `fn async()` opublikowane pod 2015 — i robi to jako `r#async`. Dzięki temu dodanie nowego słowa kluczowego do języka **nie jest zmianą łamiącą zgodność**.

Trzy szczegóły, które oszczędzają zdziwienia. Po pierwsze, `r#` nie jest częścią nazwy: `ordinary()` i `r#ordinary()` to **ta sama funkcja**, więc prefiks na słowie niebędącym słowem kluczowym jest legalny i nic nie robi (to dlatego generatory kodu mogą go dopisywać zawsze). Wyjątkiem jest makro przepisujące token — `stringify!(r#type)` zwróci `r#type`, bo wypisuje to, co napisano. Po drugie, cztery nazwy odmawiają: `crate`, `self`, `Self` i `super`, bo nie są zwykłymi słowami kluczowymi, tylko **korzeniami ścieżek** rozwiązywanymi przez system modułów. Po trzecie, `'r#fn` — apostrof stoi **przed** `r#`, bo ucieczka dotyczy identyfikatora, a `'` tylko zapowiada czas życia.

Na koniec rozróżnienie, które myli najczęściej: `r#foo` to surowy **identyfikator**, a `r#"..."#` to surowy **napis** (literał bez sekwencji ucieczki). Ta sama litera, dwie niezwiązane funkcje języka. Rozstrzyga to, co stoi po `#`: cudzysłów oznacza napis, litera oznacza nazwę.

**Szukaj po polsku:** surowy identyfikator · słowa kluczowe · edycje Rusta · `rust raw identifier` · `rust r#type` · `rust edition keyword`
