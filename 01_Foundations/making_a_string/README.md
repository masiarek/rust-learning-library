# Making a `String`

**Level:** 101 → 201 · working knowledge

**One line:** Five spellings produce a `String` from a `&str` and they are not interchangeable in *meaning*: `to_owned` is the borrowed-to-owned conversion, `String::from` is the same thing read as a constructor, `format!` builds, and `to_string()` is the universal one — because it comes free with `Display`, which is the only one of the five you ever implement yourself.

| you write | what it is | reach for it when |
|---|---|---|
| [`s.to_owned()`](../../12_Traits/to_owned/README.md) | the borrow → owned conversion | you have a `&str` and want the owned twin |
| [`String::from(s)` ↗](https://doc.rust-lang.org/std/convert/trait.From.html) | the same `From` impl, constructor-shaped | you prefer it to read as construction |
| [`s.to_string()` ↗](https://doc.rust-lang.org/std/string/trait.ToString.html) | goes through `Display` | the source is *anything* printable |
| [`s.into()` ↗](https://doc.rust-lang.org/std/convert/trait.Into.html) | `From`, backwards | the target type is already fixed by context |
| [`format!("{s}")`](../building_a_string/README.md) | allocates and runs the formatter | you are *building*, not converting |

---

## They all produce the same bytes

```rust
let literal = "equal vote";
let a = literal.to_string();
let b = literal.to_owned();
let c = String::from(literal);
let d: String = literal.into();
let e = format!("{literal}");
// a == b && b == c && c == d && d == e   // true
```

`into()` is the only one that needs help: it is `From` read backwards, so nothing in the expression says what to build. Drop the `: String` annotation and you get `E0282, type annotations needed`. The other four name their destination.

## `to_string()` is the universal one

```rust
42.to_string()        // "42"
3.5_f64.to_string()   // "3.5"
true.to_string()      // "true"
'x'.to_string()       // "x"
```

None of those are `&str`, and `to_owned()` would not help with any of them. `to_string()` works because it is defined once, for everything printable:

```rust
impl<T: Display + ?Sized> ToString for T { … }
```

## So implement `Display`, never `ToString`

Give a type a `Display` impl and `to_string()` arrives on its own, along with `{}` and `format!`:

```rust
struct Score(u8);

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} star{}", self.0, if self.0 == 1 { "" } else { "s" })
    }
}

Score(4).to_string()   // "4 stars"  — no ToString impl was written
```

Writing the `ToString` impl yourself is not merely redundant, it is refused:

```text
error[E0119]: conflicting implementations of trait `ToString` for type `Score`
 --> e0119.rs:6:1
  |
6 | impl ToString for Score {
  | ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: conflicting implementation in crate `alloc`:
          - impl<T> ToString for T
            where T: std::fmt::Display, T: ?Sized;
```

The blanket impl already covers your type, so yours would be a second one. This is the good kind of error: the language is telling you where the work goes. [`Debug` vs `Display`](../debug_vs_display/README.md) is the other half — `Debug` you derive, `Display` you write.

## The call that is not a conversion

```rust
let owned = String::from("already owned");
let copy = owned.to_string();     // a full clone. Second heap buffer.
```

`String` implements `Display`, so `to_string()` on a `String` compiles, says nothing, and allocates. It is the accidental `.clone()` — common in code that reaches for `.to_string()` reflexively to make a type error go away. If you wanted a view, `&owned` is free; if you wanted the value, move it; if you genuinely wanted a copy, write `.clone()` so the next reader can see that you meant it.

Same shape as `&str`'s: `.to_string()` and `.to_owned()` on a `&str` are equivalent today — the standard library specialises `str`'s `to_string` so it does not run the formatting machinery — so pick on readability, not speed.

## Coming back the other way

```rust
let n: i32 = "42".parse().unwrap();     // annotation says which type
let m = "42".parse::<i32>().unwrap();   // turbofish says it inline
"forty-two".parse::<i32>()              // Err(ParseIntError { kind: InvalidDigit })
```

`parse` is `FromStr`, the mirror of `Display` — and it returns `Result`, because the text is input and input lies. `.unwrap()` there turns a typo into a panic, which is fine in a scratch program and wrong in anything a user touches. [`unwrap_or`](../unwrap_or/README.md) and [What a panic costs](../what_a_panic_costs/README.md) are the ways out.

## If you have `X` and want a `String`

The wider conversion matrix, for the types the string family actually hands you:

| you have | you write |
|---|---|
| `&str` | `x.to_owned()` · `x.to_string()` |
| `char` / any number / `bool` | `x.to_string()` |
| `Vec<u8>` | `String::from_utf8(x)?` — can fail |
| `&[u8]` | `String::from_utf8_lossy(x).into_owned()` — never fails, substitutes `�` |
| `OsString` / `PathBuf` | `x.into_string()` — `Err` if not UTF-8 |
| `&OsStr` / `&Path` | `x.to_str()?.to_owned()` |
| `CString` | `x.into_string()?` |
| several pieces | `format!("{a}{b}")` |

The `?` in that column is the whole point of [Six kinds of string](../six_kinds_of_string/README.md): narrowing to a `String` is where a promise about the bytes gets *checked*, so those conversions return `Result` rather than a value.

## If you are coming from another language

**Python.** `str(x)` is `to_string()`, and `__str__` is `Display` — the correspondence is unusually exact, including the "define one method, get the rest" part.

| Python | | Rust |
|---|---|---|
| `str(x)` | stringify anything | `x.to_string()` |
| `def __str__` | you write it | `impl Display` |
| `def __repr__` | for the developer | `#[derive(Debug)]` |
| `f"{a}{b}"` | build a new string | `format!("{a}{b}")` |
| `int("42")` | raises on bad input | `"42".parse::<i32>()` — returns `Result` |

What changes: Python has no cost difference between naming a string and copying one, so `str(s)` on something already a `str` is a free no-op — it hands back the same object. `owned.to_string()` in Rust allocates a second buffer every time. The reflex that is harmless in Python is the accidental clone here.

**ABAP.** Conversion is mostly implicit — `lv_text = lv_number` just works, and the runtime picks a rule you did not write down.

| ABAP | | Rust |
|---|---|---|
| `lv_text = lv_number.` | implicit conversion | `n.to_string()` — always explicit |
| `|{ lv_a }{ lv_b }|` | string template | `format!("{a}{b}")` |
| `WRITE lv_date` | formatting from the runtime's rules | `impl Display` — rules you wrote |
| a bad `MOVE` | dumps at runtime | `parse()` returns `Result` |

What changes: ABAP's implicit conversions are the reason `'1,000'` and `'1.000'` behave differently by user setting. Rust makes every conversion a call you can see, and makes the fallible ones return `Result`, so the formatting rule for your own type lives in one `Display` impl instead of scattered across every `WRITE`.

---

## Practice

**Give a type a name, once.** Define a small struct, implement `Display` for it, and confirm you get four things without writing them: `{}` in a format string, `to_string()`, `format!`, and acceptance by a `fn label(x: impl Display) -> String`.

Then try to add `impl ToString` for the same type and read the `E0119`. Say in one sentence which crate the conflicting impl is in, and why that is the right design rather than an inconvenience.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:making_a_string_kata -->
*[`making_a_string_kata.rs`](examples/making_a_string_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: implement Display once, collect four abilities.
//!
//!   rustc --edition 2024 making_a_string_kata.rs -o /tmp/msk && /tmp/msk

use std::fmt;

#[derive(Debug)]
struct Ballot {
    voter: &'static str,
    scores: [u8; 3],
}

impl fmt::Display for Ballot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.voter, self.scores.map(|s| s.to_string()).join("/"))
    }
}

// Adding this to a type that already implements Display does not compile:
//
//   impl ToString for Ballot {
//       fn to_string(&self) -> String { … }
//   }
//
//   error[E0119]: conflicting implementations of trait `ToString` for type `Ballot`
//     |
//   6 | impl ToString for Ballot {
//     | ^^^^^^^^^^^^^^^^^^^^^^^^
//     |
//     = note: conflicting implementation in crate `alloc`:
//             - impl<T> ToString for T
//               where T: std::fmt::Display, T: ?Sized;

/// One signature that accepts anything printable — the payoff for implementing Display.
fn label(x: impl fmt::Display) -> String {
    x.to_string()
}

fn main() {
    let b = Ballot { voter: "Ada", scores: [5, 2, 0] };

    println!("One impl, four abilities:");
    println!("   {{}}            {b}");
    println!("   to_string()   {:?}", b.to_string());
    println!("   format!()     {:?}", format!("<{b}>"));
    println!("   impl Display  {:?}", label(&b));

    println!("\nThe same function serves every printable type:");
    println!("   label(42)          {:?}", label(42));
    println!("   label(true)        {:?}", label(true));
    println!("   label('x')         {:?}", label('x'));
    println!("   label(\"a literal\") {:?}", label("a literal"));
    println!("   label(3.5)         {:?}", label(3.5));
    println!("   label(&b)          {:?}", label(&b));

    println!("\nDebug is the other one, and it is NOT free:");
    println!("   {{:?}}   {b:?}   <- from #[derive(Debug)]");
    println!("   Display is for users and you write it; Debug is for you and you derive it.");

    println!("\nWhat you did NOT have to write: ToString, and it would not compile if you tried.");
}
```
<!-- /source -->

<!-- output:making_a_string_kata -->
*Verified output of [`making_a_string_kata.rs`](examples/making_a_string_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
One impl, four abilities:
   {}            Ada: 5/2/0
   to_string()   "Ada: 5/2/0"
   format!()     "<Ada: 5/2/0>"
   impl Display  "Ada: 5/2/0"

The same function serves every printable type:
   label(42)          "42"
   label(true)        "true"
   label('x')         "x"
   label("a literal") "a literal"
   label(3.5)         "3.5"
   label(&b)          "Ada: 5/2/0"

Debug is the other one, and it is NOT free:
   {:?}   Ballot { voter: "Ada", scores: [5, 2, 0] }   <- from #[derive(Debug)]
   Display is for users and you write it; Debug is for you and you derive it.

What you did NOT have to write: ToString, and it would not compile if you tried.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:making_a_string -->
*Verified output of [`making_a_string.rs`](examples/making_a_string.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Five ways to turn a &str into a String
   literal.to_string()   "equal vote"
   literal.to_owned()    "equal vote"
   String::from(literal) "equal vote"
   literal.into()        "equal vote"   <- needs the annotation to pick a target
   format!("{literal}")   "equal vote"   <- the only one that can also reshape
   all equal? true

2. They are not all the same call
   to_owned()   the borrowed -> owned conversion, defined on str itself
   String::from the From impl, same machinery, reads as a constructor
   to_string()  goes through Display — universal, and the one to reach for
                on ANY printable type, not just &str
   into()       From, backwards; fine when the target type is already known
   format!()    allocates and runs the formatter — use it when you are
                building, not merely converting

3. to_string() works on anything that prints
   42.to_string()        "42"
   3.5_f64.to_string()   "3.5"
   true.to_string()      "true"
   'x'.to_string()       "x"
   Score(4).to_string()  "4 stars"   <- our own type, no ToString impl written
   Score(1).to_string()  "1 star"

4. Why you never write `impl ToString`
   alloc already has:  impl<T: Display + ?Sized> ToString for T
   so writing your own is E0119: conflicting implementations.
   Implement Display. ToString, and `{}`, and format!, all follow.

5. The one that is not a conversion
   owned.to_string() on a String   "already owned"
   That is a full clone — a second heap buffer. It compiles, it is silent,
   and in a loop it is the allocation nobody meant to write. Wanted a view?
   &owned is free. Wanted the value? Move it.

6. Coming back the other way
   "42".parse::<i32>()        42      annotation or turbofish, pick one
   let n: i32 = text.parse()  42      same call, type from the binding
   "forty-two".parse::<i32>() Err(ParseIntError { kind: InvalidDigit })
   parse() returns Result, because text is input and input lies.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/making_a_string/examples/making_a_string.rs -o /tmp/ms && /tmp/ms
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [`String` vs `&str`](../string_vs_str/README.md) — which of the two a signature should ask for
- [`Debug` vs `Display`](../debug_vs_display/README.md) — the trait you derive and the trait you write
- [Six kinds of string](../six_kinds_of_string/README.md) — why half the conversions above return `Result`
- [`Copy` vs `Clone`](../copy_vs_clone/README.md) — what the accidental `to_string()` on a `String` actually costs
- [Building a `String`](../building_a_string/README.md) — `push_str`, `+` and `format!`, for when you are assembling rather than converting
- [`ToOwned`](../../12_Traits/to_owned/README.md) — the trait behind `to_owned()`, and why `str`'s owned twin is a different type
- [`ToString` ↗](https://doc.rust-lang.org/std/string/trait.ToString.html) · [`Display` ↗](https://doc.rust-lang.org/std/fmt/trait.Display.html) · [`FromStr` ↗](https://doc.rust-lang.org/std/str/trait.FromStr.html) · [Rust Language Cheat Sheet — String conversions ↗](https://cheats.rs/#string-conversions)
