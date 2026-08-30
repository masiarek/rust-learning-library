# What an attribute is

**Level:** 201 · working knowledge

**One line:** An attribute is metadata the compiler acts on — five families cover almost all of them, and the only piece of syntax to learn is that `#!` applies to the thing it is *inside* while `#[` applies to the thing *below* it.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Score(u8);

fn main() {
    let a = Score(4);
    println!("{a:?} {}", a == Score(4));   // Score(4) true
}
```

Eight traits, no bodies. Each one is a real `impl` block the compiler wrote for you.

## The five families

| Family | Examples | Does |
|---|---|---|
| **derive** | `#[derive(Debug, Clone)]` | generate an impl |
| **lints** | `#[allow]`, `#[warn]`, `#[deny]`, `#[forbid]` | change what a diagnostic does |
| **cfg** | `#[cfg(test)]`, `#[cfg(unix)]` | include or exclude the item entirely |
| **test** | `#[test]`, `#[bench]`, `#[ignore]` | mark a function for the test harness |
| **codegen / API** | `#[inline]`, `#[repr(C)]`, `#[must_use]`, `#[non_exhaustive]`, `#[deprecated]` | change layout, hints, or the contract |

A doc comment is one too: `///` is sugar for `#[doc = "…"]`, and `//!` for `#![doc = "…"]`. That is why a doc comment in the wrong place produces an attribute error rather than a comment error.

## Inner and outer

```rust
#![allow(clippy::needless_return)]   // applies to the whole file it opens

#[allow(dead_code)]                  // applies to the item below
fn never_called() -> u32 { 0 }
```

The `!` is the whole difference, and an inner attribute has to be the first thing in its file or block. `//!` and `///` follow the same rule for the same reason.

## The lint levels, and the one to avoid

| | |
|---|---|
| `allow` | say nothing |
| `warn` | print, keep compiling |
| `deny` | error |
| `forbid` | error, **and an inner `allow` is itself an error** |

They nest, innermost wins — except `forbid`, which cannot be relaxed. So `#![forbid(unsafe_code)]` at the crate root is a real promise worth making, and `#![forbid(warnings)]` is a trap: the day one generated macro trips a new lint, nobody can `allow` it locally. **Deny for style, forbid for safety properties.**

## `#[cfg]` deletes; `cfg!()` chooses

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() { assert_eq!(2 + 2, 4); }
}

fn main() {
    if cfg!(debug_assertions) { println!("debug build"); }
}
```

`#[cfg(test)]` means the module **does not exist** in a normal build — not that it is skipped at run time, which is why test code costs a release binary nothing. `cfg!(…)` is an expression that becomes a literal `true` or `false`, and both branches still have to type-check.

## The trap: derived `Ord` reads your field order

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ByName  { name: &'static str, score: u32 }

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ByScore { score: u32, name: &'static str }
```

Same two fields, same derive, and sorting the same three records gives `["Ada", "Ben", "Cara"]` for one and `["Cara", "Ben", "Ada"]` for the other. **Derived comparison is lexicographic in declaration order**, so moving a field up is a one-line diff that reads as cosmetic and silently reorders every `sort`, every `BTreeMap` and every `binary_search` over the type.

If the order matters, write the `impl Ord` — then the diff that changes it is the one that says so.

Two more derive rules worth knowing before they bite: `#[derive(Default)]` on an enum needs a variant marked `#[default]` (`E0665`), and every derive adds a bound on the type parameters, so `#[derive(Clone)]` on `Wrapper<T>` produces `impl<T: Clone> Clone for Wrapper<T>` — which is why a `Wrapper<NotClone>` sometimes refuses to clone for reasons that are not in your code.

## If you are coming from another language

- **Python.** Decorators are the closest thing, and the resemblance is real for `#[test]` — but `@decorator` is a *function called at run time* that returns a new object, while an attribute is read by the compiler and can delete the item entirely. `#[cfg(test)]` has no decorator equivalent at all; the nearest Python idiom is an `if TYPE_CHECKING:` block or a `# pragma: no cover`, both of which are conventions read by other tools rather than by the interpreter. `@dataclass` is the honest analogue of `#[derive]`: both generate the methods you would have typed, and both generate `__eq__`/`__lt__` comparing fields **in declaration order** — so the trap above is a `@dataclass(order=True)` trap too, with the same fix.
- **ABAP.** Pragmas (`##NEEDED`, `##NO_TEXT`) and pseudo-comments (`"#EC NOTEXT`) are the lint family exactly: metadata that silences a specific check at a specific place, read by the Code Inspector rather than by the runtime. `#[allow(dead_code)]` is `##NEEDED`. What ABAP has no counterpart for is `derive` — a `REDEFINITION` of `IS_EQUAL` or a `GET_TEXT` has to be written out — and none for `#[cfg]`, since there is no compile-time variant selection; the nearest thing is a `IF sy-sysid = 'PRD'` check, which is a run-time branch and stays in the shipped code.
- **Java / C#.** Annotations and attributes, and the naming is nearly the same. The difference is what they do: a Java annotation is metadata read reflectively at run time (or by an annotation processor at build time), while Rust's are compiler instructions with no run-time trace. `@Deprecated` and `#[deprecated]` are the closest exact match.
- **C / C++.** `#ifdef` is `#[cfg]`, and `#pragma`/`[[attributes]]` are the rest. Rust's version type-checks the excluded branch's *syntax* — it must still parse — where `#ifdef` can hide anything at all.

---

## The verified output

<!-- output:what_an_attribute_is -->
*Verified output of [`what_an_attribute_is.rs`](examples/what_an_attribute_is.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. `derive` writes the impl you would have written
   Debug:      Score(4)
   Copy:       a is still usable after `let b = a`: Score(4) Score(4)
   PartialEq:  a == b is true
   Ord:        sorted [4, 1, 3] -> [Score(1), Score(3), Score(4)]
   Default:    Score::default() = Score(0)
   Eight traits, no bodies. Each one is a real impl block the
   compiler generated, and each has a rule: derived Ord compares
   fields in DECLARATION ORDER, so reordering a struct's fields is
   a behaviour change.

2. Inner and outer, and how to tell them apart
   #[attr]   applies to the item BELOW it       (outer)
   #![attr]  applies to the item it is INSIDE   (inner)
   The `!` is the whole difference, and an inner attribute has to be
   the first thing in its file or block. `//!` and `///` follow the
   same rule, because a doc comment IS an attribute — `#[doc = "…"]`.

3. The lint family: allow, warn, deny, forbid
   #[allow(dead_code)] on never_called() is why this program has no
   warnings. The four levels differ in what they do and in whether
   an inner scope may override them:
     allow   say nothing
     warn    print, keep compiling
     deny    error
     forbid  error, and an inner allow is itself an error
   They nest, so a crate-level `#![deny(warnings)]` can be relaxed
   on one function — which is the whole reason to prefer deny over
   forbid.

4. `cfg`: compiled or not compiled at all
   #[cfg(test)] on the tests module above means it does not exist in
   this binary — not that it is skipped at run time.
   cfg!(test) here = false   <- this binary was not built with --test
   `#[cfg(…)]` removes code; `cfg!(…)` is an expression that becomes
   a literal true or false. The second still type-checks both sides,
   which is why a `cfg!(windows)` branch cannot rot on a Mac.
   (Nothing here prints the target OS on purpose: every example in
   this library has a recorded answer key, and `env::consts::OS` says
   "macos" on the author's machine and "linux" in CI. That is the
   determinism rule, and it is easier to break than it looks.)

5. The rest, in one line each
   #[test]           this fn is a test; rustc --test collects it
   #[derive(…)]      generate these impls
   #[non_exhaustive] downstream crates may not match it exhaustively
                     or build it with a struct literal: Star
   #[inline]         a hint, not an instruction: doubled(21) = 42
   #[must_use]       warn if the return value is dropped
   #[repr(C)]        lay this type out the way C would
   #[deprecated]     warn at every use, with a message
```
<!-- /output -->

## Practice

**The derive that changed behaviour when a field moved.** Write the same two-field record twice, with the fields in opposite orders, derive `Ord` on both, and sort three of each. Explain the two different answers in one sentence, and say what you would do if the sort order is part of the type's contract.

Then three smaller ones. Derive `Default` for an enum and find out what it needs. Put `#[must_use]` on a function, discard its result, and read the warning and the fix rustc offers. And say why `#![deny(warnings)]` at the crate root is reasonable while `#![forbid(warnings)]` is not.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_an_attribute_is_kata -->
*[`what_an_attribute_is_kata.rs`](examples/what_an_attribute_is_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the derive that changed behaviour when a field moved.
//!
//!   rustc --edition 2024 what_an_attribute_is_kata.rs -o /tmp/wak && /tmp/wak

/// Field order: name first.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct ByName {
    name: &'static str,
    score: u32,
}

/// The same two fields, swapped. Nothing else differs.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct ByScore {
    score: u32,
    name: &'static str,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Config {
    seats: u32,
    method: Method,
    strict: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Method {
    #[default]
    Star,
    Approval,
}

#[must_use]
fn tally(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

fn main() {
    println!("1. Derived Ord compares fields in declaration order");
    let mut a = vec![
        ByName { name: "Cara", score: 1 },
        ByName { name: "Ada", score: 9 },
        ByName { name: "Ben", score: 5 },
    ];
    let mut b = vec![
        ByScore { score: 1, name: "Cara" },
        ByScore { score: 9, name: "Ada" },
        ByScore { score: 5, name: "Ben" },
    ];
    a.sort();
    b.sort();
    println!("   name first : {:?}", a.iter().map(|r| r.name).collect::<Vec<_>>());
    println!("   score first: {:?}", b.iter().map(|r| r.name).collect::<Vec<_>>());
    println!("   Same data, same derive, opposite answers. Moving a field up is a");
    println!("   one-line diff that reads as cosmetic and silently reorders every");
    println!("   sort, every BTreeMap and every binary search over the type.");
    println!("   If the order matters, write the impl:");
    println!("     impl Ord for ByName {{ fn cmp(&self, o: &Self) -> Ordering {{ … }} }}");
    println!("   and then the diff that changes it is the one that says so.");

    println!();
    println!("2. `derive(Default)` on an enum needs `#[default]`");
    println!("   Config::default() = {:?}", Config::default());
    let approving = Config { method: Method::Approval, ..Config::default() };
    println!("   with one field changed: {approving:?}");
    println!("   Without `#[default]` on a variant, deriving Default for an enum is");
    println!("   E0665, \"`#[derive(Default)]` on enum with no `#[default]`\" — the");
    println!("   compiler cannot guess which variant is the zero. For a struct it");
    println!("   can: every field's own Default.");

    println!();
    println!("3. `#[must_use]` turns a discarded value into a warning");
    let total = tally(&[5, 3, 0]);
    println!("   tally(&[5, 3, 0]) = {total}");
    println!("   Writing `tally(&scores);` on its own line warns: \"unused return");
    println!("   value of `tally` that must be used\", and offers `let _ = ...` as");
    println!("   the way to say you meant it. It is the mechanism behind");
    println!("   the warning you have already met on Result and on iterator");
    println!("   adapters — `#[must_use]` sits on the TYPE there, not the method.");

    println!();
    println!("4. Attributes that change what is compiled, not how");
    println!("   #[cfg(test)]         the item does not exist in a normal build");
    println!("   #[cfg(unix)]         nor on Windows");
    println!("   #[cfg(feature = \"x\")] nor unless the feature is on");
    println!("   cfg!(unix) compiled on BOTH platforms — the branch not taken");
    println!("   still had to type-check, and only then was replaced by a literal.");
    println!("   #[cfg] deletes; cfg!() chooses. Which is why a cfg! branch cannot");
    println!("   rot and a #[cfg] one can: nobody compiles the deleted arm.");

    println!();
    println!("5. The four lint levels, and the one to avoid");
    println!("   allow / warn / deny / forbid, innermost wins — except `forbid`,");
    println!("   which cannot be relaxed by an inner `allow` (that is itself an");
    println!("   error). So a crate-wide `#![forbid(unsafe_code)]` is a real");
    println!("   promise, and a crate-wide `#![forbid(warnings)]` is a trap: the");
    println!("   day one generated macro trips a new lint, nobody can allow it");
    println!("   locally. Use deny for style, forbid for safety properties.");
}
```
<!-- /source -->

<!-- output:what_an_attribute_is_kata -->
*Verified output of [`what_an_attribute_is_kata.rs`](examples/what_an_attribute_is_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Derived Ord compares fields in declaration order
   name first : ["Ada", "Ben", "Cara"]
   score first: ["Cara", "Ben", "Ada"]
   Same data, same derive, opposite answers. Moving a field up is a
   one-line diff that reads as cosmetic and silently reorders every
   sort, every BTreeMap and every binary search over the type.
   If the order matters, write the impl:
     impl Ord for ByName { fn cmp(&self, o: &Self) -> Ordering { … } }
   and then the diff that changes it is the one that says so.

2. `derive(Default)` on an enum needs `#[default]`
   Config::default() = Config { seats: 0, method: Star, strict: false }
   with one field changed: Config { seats: 0, method: Approval, strict: false }
   Without `#[default]` on a variant, deriving Default for an enum is
   E0665, "`#[derive(Default)]` on enum with no `#[default]`" — the
   compiler cannot guess which variant is the zero. For a struct it
   can: every field's own Default.

3. `#[must_use]` turns a discarded value into a warning
   tally(&[5, 3, 0]) = 8
   Writing `tally(&scores);` on its own line warns: "unused return
   value of `tally` that must be used", and offers `let _ = ...` as
   the way to say you meant it. It is the mechanism behind
   the warning you have already met on Result and on iterator
   adapters — `#[must_use]` sits on the TYPE there, not the method.

4. Attributes that change what is compiled, not how
   #[cfg(test)]         the item does not exist in a normal build
   #[cfg(unix)]         nor on Windows
   #[cfg(feature = "x")] nor unless the feature is on
   cfg!(unix) compiled on BOTH platforms — the branch not taken
   still had to type-check, and only then was replaced by a literal.
   #[cfg] deletes; cfg!() chooses. Which is why a cfg! branch cannot
   rot and a #[cfg] one can: nobody compiles the deleted arm.

5. The four lint levels, and the one to avoid
   allow / warn / deny / forbid, innermost wins — except `forbid`,
   which cannot be relaxed by an inner `allow` (that is itself an
   error). So a crate-wide `#![forbid(unsafe_code)]` is a real
   promise, and a crate-wide `#![forbid(warnings)]` is a trap: the
   day one generated macro trips a new lint, nobody can allow it
   locally. Use deny for style, forbid for safety properties.
```
<!-- /output -->

</details>

---

## See also

- [Comments that compile](../../15_First_Programs/comments_that_compile/README.md) — `///` and `//!`, which are attributes in disguise
- [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) — the diagnostics `#[allow]` silences, and when silencing is the wrong fix
- [Strict clippy](../../05_Tooling/strict_lints/README.md) — a crate-level lint policy, and living with it
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — two of the derives above, and why one is a marker
- [`const` and `static`](../const_and_static/README.md) — the other item-level metadata
- [Marker traits](../../12_Traits/marker_traits/README.md) — what `derive(Eq)` actually generates, which is nothing

## Sources

[Attributes ↗](https://doc.rust-lang.org/rust-by-example/attribute.html) in Rust by Example; the Reference's [attributes ↗](https://doc.rust-lang.org/reference/attributes.html) chapter for the full list, and [derivable traits ↗](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) in The Book. Both error transcripts were produced by compiling the broken version.
