# *Rust in Action* §2.3.4, run

[Other number types](../README.md) › **Beside the pages** · read after: [Complex numbers](../complex_numbers/README.md)

**Level:** 101 → 201 · a companion to the book

**One line:** Section 2.3.4 of *Rust in Action* (McNamara, Manning 2021), "Rational, complex numbers, and other numeric types", introduces the `num` crate with listing 2.6. Here are twelve of its claims, checked against rustc 1.98.0, cargo 1.98.0 and num 0.4.3. The listing compiles and runs. Its printed output does not match, and neither does the `cargo-edit` step, how much `num` covers, or three sentences about literals and `new`.

The claims are paraphrased, and the numbering is this page's, not the book's.

| # | The book says | Verdict |
|---|---|---|
| 1 | std leaves out rationals, complex numbers, big integers, big floats and fixed-point decimals | **True** |
| 2 | the `num` crate gives you those types | **Three of the five** |
| 3 | *crate* is Rust's word for *package* | **Close; they are different things** |
| 4 | listing 2.6 prints `13.2 + 21.02i` | **It prints `13.2 + 21i`** |
| 5 | `use` brings crates into scope | **It binds a name; the crate was already in scope** |
| 6 | every type has a literal form, which is why Rust needs no constructors | **Only when you can see every field** |
| 7 | `new()` is a convention, not part of the language | **True** |
| 8 | a static method is a function on a type rather than on an instance | **Right idea; Rust calls it an associated function** |
| 9 | authors prefer `new()` because it sets defaults | **Not this `new`; the real reason is an invariant** |
| 10 | install `cargo-edit` to get `cargo add` | **Out of date since Cargo 1.62** |
| 11 | `cargo add num` adds `num v0.4.0` | **Today it is v0.4.3** |
| 12 | footnote: mechanical engineers write *j* for the imaginary unit | **Electrical engineers** |

## 1. What std leaves out — true

The item index of the rendered std docs for 1.98.0, `share/doc/rust/html/std/all.html`, has no item named `Complex`, `Ratio`, `Rational`, `BigInt` or `Decimal`. A case-insensitive search of that file for `complex`, `ratio`, `bigint` and `decimal` finds six items, all substring matches: four `GOLDEN_RATIO` constants, `time::Duration` and `prelude::v1::eii_declaration`. The overview page has [the full map](../README.md), with what Python ships for each.

## 2. What `num` covers — three of the five

`num` 0.4.3's `src/lib.rs` re-exports `num-bigint`, `num-complex`, `num-rational`, `num-integer`, `num-iter` and `num-traits`. That gives you big integers, complex numbers and rationals. Arbitrary-precision floats and decimals are not in it: they come from `bigdecimal`, `dashu-float`, `rug` and `rust_decimal`, which [Decimals](../decimal_numbers/README.md) runs side by side with Python's `Decimal`. None of those is fixed-point either, as the same page shows.

## 3. Crate vs package — close, but different

In Cargo's vocabulary, you publish and download a **package**, and a package contains one or more **crates**: at most one library and any number of binaries. `cargo-edit` 0.13.13, from claim 10, is one package whose `Cargo.toml` declares a `[lib]` and four `[[bin]]` targets (`cargo-add`, `cargo-rm`, `cargo-set-version`, `cargo-upgrade`), which makes five crates. `num` is one package with one library crate, so for the book's example the two words point at the same thing.

## 4. The printed output — `13.2 + 21i`

```text title="Real output — cargo run, cargo 1.98.0, num 0.4.3, abridged to the last line"
13.2 + 21i
```

`-1.2 + 22.2` is exactly 21 as an `f64`, and `{}` prints a whole float without `.0`. [Complex numbers](../complex_numbers/README.md#listing-26-from-rust-in-action) has the listing and a std-only version whose `{:?}` line shows the `21.0`.

## 5. What `use` does — binds a name

Delete the `use` line and write the full path. It still compiles, because `num` is already in scope as a dependency Cargo passed to rustc:

```rust title="src/main.rs — cargo 1.98.0, num 0.4.3"
fn main() {
    let b = num::complex::Complex::new(11.1, 22.2);
    println!("{b}"); // 11.1+22.2i
}
```

`use num::complex::Complex;` brings one name, `Complex`, into the module. It pulls nothing into the program, and deleting it removes only the short spelling. [Bringing names in with `use`](../../../27_Modules/the_use_declaration/README.md#what-is-in-scope-with-no-use-at-all) covers what is in scope with no `use` at all.

## 6. "Every type has a literal form" — only with visible fields

`Complex { re: 2.1, im: -1.2 }` works because `num-complex` made both fields `pub`. Two types that refuse the same form:

```text title="Real output — rustc 1.98.0 and cargo 1.98.0, error lines only"
error[E0451]: fields `numer` and `denom` of struct `Ratio` are private
error[E0451]: field `vec` of struct `String` is private
```

The first is `Ratio { numer: 1, denom: 2 }` with num 0.4.3. The second is `String { vec: Vec::new() }` on std. A type with private fields has a literal only inside its own module, so from outside, a function such as `new` is the only way to make one. [Rationals](../rational_numbers/README.md#no-literal-new-has-to-reduce) has the full transcript and the reason the fields are private. [Modules and visibility](../../../27_Modules/modules_and_visibility/README.md) explains why privacy stops at the module.

## 7. `new()` is a convention — true

Nothing in the language treats a function named `new` specially. [A type is not a constructor](../../../16_Structs/a_type_is_not_a_constructor/README.md#new-is-a-convention) makes the same point with a type whose constructor has another name.

## 8. "Static method" — Rust says associated function

The idea is right: `Complex::new` is called on the type and takes no `self`. The Rust term is **associated function**, and a **method** is an associated function whose first parameter is `self`. The distinction matters for the dot: `z.norm()` works because `norm` takes `self`, while `z.new(3.0, 4.0)` is `E0599`, with the note *"to be used as methods, functions must have a `self` parameter"*. [`impl` blocks](../../../16_Structs/impl_blocks/README.md#associated-function-vs-method) has both.

## 9. "`new()` sets defaults" — not this one

`num-complex` 0.4.6 defines it as:

```rust
pub const fn new(re: T, im: T) -> Self {
    Complex { re, im }
}
```

It sets nothing, and the literal on line 4 of the listing builds the same value. Where `new` earns its place is an **invariant** the literal cannot enforce. `Ratio::new(2, 4)` reduces to `1/2`, `Ratio::new_raw(2, 4)` skips that and prints `2/4`, and `Ratio::new(1, 0)` panics with `denominator == 0`. `num-complex` puts its default in the `Default` trait instead: `Complex::<f64>::default()` is `0+0i`.

## 10. `cargo install cargo-edit` — no longer needed

`cargo add` is built into Cargo: cargo 1.98.0 lists `add` among its built-in commands, with no install. `cargo-edit`'s own README says `cargo add` was merged into Cargo in 1.62. The current `cargo-edit`, 0.13.13, still ships a `cargo-add` binary, and its source (`src/bin/add/add.rs`) does one thing — exits with this message:

```text title="From cargo-edit 0.13.13, src/bin/add/add.rs"
`cargo add` has been merged into cargo 1.62+ as of cargo-edit 0.10, either
- Upgrade cargo, like with `rustup update`
- Downgrade `cargo-edit`, like with `cargo install cargo-edit --version 0.9.1`
```

The book's transcript installs `cargo-edit` v0.6.0, from before the merge. Of its other subcommands, `cargo rm` is built in too, as an alias of `cargo remove`. `cargo upgrade` and `cargo set-version` are not built into cargo 1.98.0.

## 11. `cargo add num` — v0.4.3 now

```text title="Real output — cargo 1.98.0, the first two lines"
    Updating crates.io index
      Adding num v0.4.3 to dependencies
```

It writes `num = "0.4.3"`. The hand-written `num = "0.4"` from the book's manual steps resolves to the same 0.4.3 in `Cargo.lock`, because both are caret ranges. [Adding a dependency](../../../05_Tooling/cargo_dependencies/README.md) explains what a caret range permits.

## 12. The footnote on *j* — electrical engineers

Electrical engineering writes *j* because *i* already means current. Python uses the same letter for its complex literal, `2.1-1.2j`, as [Complex numbers](../complex_numbers/README.md) shows.

## What the section gets right

The listing compiles and runs on a 2026 toolchain, five years after the book was published, with only its output line wrong. The overall message holds: std leaves these types out, and a crate from crates.io fills the gap. Claims 1 and 7 are true, and claim 8 is the right idea under another name.

## See also

- [Other number types](../README.md) — the map these pages hang off
- [Books](../../../10_Resources/books/README.md) — where *Rust in Action* sits among the other Rust books
- [What `Cow` explanations get wrong, run](../../../12_Traits/how_to_learn_to_owned/cow_claims_checked/README.md) — the same treatment for chapter 6 of this book and other `Cow` material
- [A throwaway that needs a crate](../../../05_Tooling/scratch_with_a_crate/README.md) — `cargo new`, `cargo add`, `cargo run`, and the error you get without them

## Po polsku

Sekcja 2.3.4 książki *Rust in Action* wprowadza crate `num` i listing 2.6 z liczbami zespolonymi. Program działa na rustc 1.98.0, ale kilka zdań z tej sekcji nie wytrzymuje uruchomienia. Wypisuje `13.2 + 21i`, a nie `13.2 + 21.02i`. `num` daje tylko trzy z pięciu obiecanych rodzajów liczb — liczb dziesiętnych i zmiennoprzecinkowych o dowolnej precyzji w nim nie ma. `use` nie „wciąga crate'a”, tylko wiąże nazwę (crate jest w zasięgu i bez tego). Nie każdy typ ma literał — `Ratio` i `String` mają prywatne pola (`E0451`). `Complex::new` niczego nie ustawia domyślnie — `new` ma sens tam, gdzie pilnuje niezmiennika (*invariant*), jak skracanie ułamka.

Część narzędziowa się zestarzała: `cargo add` jest wbudowane w Cargo od wersji 1.62, a obecny `cargo-edit` przy próbie `cargo-add` tylko wypisuje komunikat o scaleniu. Rustowa nazwa „metody statycznej” to *associated function* (funkcja powiązana). I drobiazg z przypisu: literą *j* piszą elektrycy, nie mechanicy — tak samo jak Python.

**Szukaj po polsku:** `rust in action num crate` · `cargo add built in 1.62` · `rust associated function vs method` · `rust E0451`

---

[Other number types](../README.md) › previous: [Decimals](../decimal_numbers/README.md)
