# `unwrap_or`: the default you already have

**Level:** 201 · working knowledge

**One line:** `unwrap_or(d)` answers "and if there is nothing?" with a value you hand it up front — and because `d` is an ordinary argument, it is built every time, needed or not.

It is the gentlest member of the `unwrap` family: no panic, no closure, no trait bound. That is exactly why it is worth a page — the three things it costs are all invisible at the call site.

```rust
pub const fn unwrap_or(self, default: T) -> T
```

Read that signature slowly, because every trap below is already in it. `self` by value: it eats the `Option`. `default: T`, not `impl FnOnce() -> T`: the default is evaluated before the call. `-> T`: the result carries no memory of which branch produced it. (And `const` is not yet true on stable — see [below](#the-const-fn-in-the-signature-is-not-yet-true).)

---

## What it does

| | `Option` | `Result` |
|---|---|---|
| value present | `Some(5).unwrap_or(0)` → `5` | `Ok(5).unwrap_or(0)` → `5` |
| value absent | `None.unwrap_or(0)` → `0` | `Err(e).unwrap_or(0)` → `0`, **and `e` is dropped** |

On an `Option` there is nothing to lose: `None` carries no information, so replacing it with a default discards nothing. On a `Result` the same call throws away the one thing a `Result` exists to carry — the reason. If the reason is worth reporting (and in a parser it usually is: *which row, which column, which value*), reach for `unwrap_or_else(|e| …)`, which is handed the error, or keep the `Result` and let `?` carry it up.

## The eager trap

The standard library's own note is the whole warning: *arguments passed to `unwrap_or` are eagerly evaluated.* `opt.unwrap_or(f())` is an ordinary method call with an ordinary argument, so Rust evaluates `f()` first — on every call, including the ones where the value was there and the default is thrown away unused.

Nor is this something the optimizer quietly removes. The example builds a `Vec<String>`, and an allocation is a side effect the compiler must preserve. Five lookups with exactly one miss:

```text
unwrap_or(default_roster())     -> default_roster() ran 5 times
unwrap_or_else(default_roster)  -> default_roster() ran 1 time
```

**The rule is one question: does the default already exist?** A literal, a constant, a `Copy` value sitting on the stack — pass it, and `unwrap_or` is the clearest thing you can write. Anything you have to *build* — allocate, read a file, query, compute — belongs in a closure, because the happy path is the common one and you are paying for it on every trip.

The trap has a quiet variant worth naming: `unwrap_or(v.clone())` and `unwrap_or(s.to_string())` look like plain values and are function calls.

## It consumes the `Option`

`unwrap_or` takes `self`, so unless `T` is `Copy` the option is *moved* — and the error you get names a move, which reads oddly when all you asked for was a default:

```text
error[E0382]: borrow of moved value: `name`
  |
3 |     let shown = name.unwrap_or("(unnamed)".to_string());
  |                 ---- value moved here
4 |     println!("{shown} {name:?}");
  |                        ^^^^ value borrowed here after move
```

Three ways to keep the option alive, one per shape:

| You have | You want | Write |
|---|---|---|
| `Option<String>` | a `&str` fallback | `name.as_deref().unwrap_or("(unnamed)")` |
| `Option<&T>` where `T: Copy` | an owned `T` | `scores.get(9).copied().unwrap_or(0)` |
| anything else | to look without taking | `name.as_ref().map_or(0, \|s\| s.len())` |

`as_deref()` is the one to memorize: `Option<String>` → `Option<&str>`, which also fixes the *other* beginner error here, `expected String, found &str` — you cannot pass a `&'static str` as the default for an `Option<String>`, because the default's type **is** the return type.

## Which member of the family

| Call | Use it when |
|---|---|
| `unwrap_or(d)` | `d` already exists and is cheap |
| `unwrap_or_else(\|\| …)` | the default costs something to produce |
| `unwrap_or_else(\|e\| …)` (on `Result`) | the fallback should be computed *from* the error |
| `unwrap_or_default()` | `T: Default` and the default is the zero value |
| `map_or(d, f)` | you want to transform the value *and* have a fallback |
| `unwrap()` / `expect("…")` | absence is a bug, and crashing is the correct response |

## What the default erases

A fallback does not just fill a hole; it makes the hole unfindable. Two ballots, three candidates:

```text
scored 0   [Some(5), Some(3), Some(0)]  -> total 8, 3 of 3 marked
left blank [Some(5), Some(3), None]     -> total 8, 2 of 3 marked
```

Both total 8, and that is correct — a blank tabulates as zero. But after `unwrap_or(0)` the two are the same `[u8; 3]`, and no code downstream can tell a deliberate zero from an unmarked square. Turnout, "how many voters used the whole ballot", any question about *participation* rather than *score* is now unanswerable.

So: **apply the default at the boundary where the distinction stops mattering** — inside the sum — and never when loading the data. This is the same instinct as not writing `-1` for "missing" in a column of scores, and it is most of the reason `Option<T>` is a different type from `T` in the first place.

## The `const fn` in the signature is not yet true

The documented signature says `pub const fn unwrap_or(self, default: T) -> T where T: Destruct`, and the std docs illustrate it with an array-repeat expression that looks like proof you can call it in a `const` context:

```rust
let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
```

Two corrections, both checked against `rustc 1.97.1`.

**That line does not need a constant.** `[expr; N]` requires a const expression only when the element type is not `Copy`; `u8` is `Copy`, so the repeat is legal with an ordinary runtime expression and the `const`-ness never came into it. Swap in a `String` and you get `error[E0277]: the trait bound String: Copy is not satisfied` — not a const error.

**And you cannot actually call it in a `const` yet:**

```text
error: `Option::<T>::unwrap_or` is not yet stable as a const fn
error[E0658]: cannot call conditionally-const method `Option::<u8>::unwrap_or` in constants
```

The `const` in the rendered signature is the *conditionally-const* form (that is what the odd `T: Destruct` bound is doing there — it needs to know the discarded value can be dropped in a const context), still unstable. `unwrap()` and `expect()` **are** const-stable, since 1.83 — so a `const` is the one place where the eager member of the family is the only one available at all, a closure being unable to run there.

## If you are coming from another language

- **Python** — `d.get(k, default)` is the same call with the same eager trap: `d.get(k, expensive())` runs `expensive()` on every lookup, hit or miss. The idiom people reach for instead, `d.get(k) or fallback`, is lazy but wrong in a way `unwrap_or` cannot be — it also fires on `0`, `""`, and `[]`, whereas `Some(0).unwrap_or(5)` is `0`. What the compiler adds: you cannot reach the value at all without answering the absent case, and the default's type must match, so `d.get(k, "n/a")` on a column of ints is a type error rather than a surprise three functions later.
- **ABAP** — `VALUE #( itab[ key = k ] DEFAULT lv_fallback )` is `unwrap_or`, and `VALUE #( itab[ key = k ] OPTIONAL )` is `unwrap_or_default()`: no row found, take the initial value. The honest difference is the one from the section above — with `OPTIONAL` you cannot afterwards distinguish "no such row" from "a row whose field really was initial", exactly as `READ TABLE` forces you to consult `sy-subrc` before trusting the work area. `Option<T>` is that `sy-subrc` fused onto the value, and `unwrap_or` is the moment you deliberately spend it.
- **SQL** — `COALESCE(col, 0)`, with the same warning about what a defaulted `NULL` costs you in the next aggregate.

---

## The verified output

<!-- output:unwrap_or -->
*Verified output of [`unwrap_or.rs`](examples/unwrap_or.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: What it does — and what it throws away
  Some(5).unwrap_or(0)        -> 5
  None.unwrap_or(0)             -> 0
  Ok(5).unwrap_or(0)            -> 5
  Err(e).unwrap_or(0)           -> 0
  Err(e).unwrap_or_else(|e| ..) -> 0
      Both produced 0, but only the closure was told why: "row 7: '4x' is not a score".
      On a Result, unwrap_or DISCARDS the error — the one thing an
      Option had none of to lose. If the reason is worth reporting,
      unwrap_or is the wrong end of the family.

──── Step 2: The default is computed whether or not it is used
  5 lookups, exactly 1 of them missing:
    unwrap_or(default_roster())    -> default_roster() ran 5 times
    unwrap_or_else(default_roster) -> default_roster() ran 1 time
      `unwrap_or(f())` is a method call with an argument, so Rust
      evaluates f() first, every time, and then usually throws the
      result away. Nothing is optimized away either: default_roster
      allocates, and an allocation is a side effect the compiler must
      keep. The lazy form never even calls it on the happy path.

──── Step 3: It consumes the Option — reach for a borrowing form instead
  name.as_deref().unwrap_or("(unnamed)")     -> Ada
  missing.as_deref().unwrap_or("(unnamed)")  -> (unnamed)
  name is still usable afterwards            -> Some("Ada")
  scores.get(1).copied().unwrap_or(0)        -> 3
  scores.get(9).copied().unwrap_or(0)        -> 0
  name.as_ref().map_or(0, |s| s.len())       -> 3
      Three ways to keep the Option alive: as_deref (Option<String>
      -> Option<&str>), copied/cloned (Option<&T> -> Option<T>), and
      as_ref for everything else. The type error you get without them
      is a MOVE error, not a borrow error, which is why it reads as if
      you did something exotic when you only asked for a default.

──── Step 4: Which member of the family
  None.unwrap_or(0)                 -> 0   value already in hand
  None.unwrap_or_else(|| 6 * 7)     -> 42  costs something to build
  None.unwrap_or_default()          -> 0   T::default(), no argument
  None.map_or(-1, |v| v * 10)       -> -1  transform, or a default
  Some(4).map_or(-1, |v| v * 10)    -> 40  same call, value present
  Err(e).unwrap_or_else(|e| e.len())-> 18  the fallback FROM the error
      The rule is one question: does the default already exist? A
      literal, a constant, a copy of something on the stack — pass it,
      unwrap_or is clearer. Anything you have to BUILD — allocate,
      read, compute — goes in a closure, because the cost is real and
      the happy path is the common one.

──── Step 5: A default erases the difference it stood for
  scored 0   [Some(5), Some(3), Some(0)]
             unwrap_or(0) total = 8, but 3 of 3 marked
  left blank [Some(5), Some(3), None]
             unwrap_or(0) total = 8, but 2 of 3 marked
      Both total 8, which is correct: a blank tabulates as zero. What
      is not recoverable is the second number — after unwrap_or(0) the
      two ballots are the same array of u8 and no later code can tell
      them apart. So apply the default at the BOUNDARY where the
      distinction stops mattering (the sum), never when loading the
      data, or you have thrown away a fact you still needed.

──── Step 6: The `const fn` in the documentation is not the whole story
  compute_digest("Hello") -> [222, 254, 150]
  const START: u8 = ZERO.unwrap();  -> 42
      The docs show `pub const fn unwrap_or`, and the array-repeat
      example above looks like proof that you can use it in a const
      context. Two corrections. First, `[expr; N]` needs a constant
      only when the element is not Copy; u8 is Copy, so the repeat is
      legal with an ordinary runtime expression and the const never
      mattered. Second, writing `const D: u8 = ZERO.unwrap_or(0);`
      today is rejected: "`Option::<T>::unwrap_or` is not yet stable
      as a const fn". The signature is const-generic-over-Destruct on
      nightly. `unwrap` and `expect` ARE const-stable — which is the
      rare corner where the eager member of the family is the only one
      available, since a closure cannot run in a const context at all.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/unwrap_or/examples/unwrap_or.rs -o /tmp/uo && /tmp/uo
```

## See also

- [`Option` vs `Result`](../option_vs_result/README.md) — why discarding the error is a bigger decision on a `Result`
- [Initial values](../initial_values/README.md) — `unwrap_or(8080)` in its natural habitat, and the case where you want no `Option` at all
- [Partial functions](../partial_functions/README.md) — where the `None` you are defaulting away came from
- [`Option::unwrap_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or) · [`unwrap_or_else`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else) · [`unwrap_or_default`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default)
