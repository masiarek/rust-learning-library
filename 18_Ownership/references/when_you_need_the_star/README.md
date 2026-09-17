# When you need the `*`

[References](../README.md) › **When you need the `*`**

**Level:** 101 → 201 · for newcomers

**One line:** Rust writes the `*` for you in five places — the dot, an operator with an impl for `&T`, formatting, a pattern, and a deref coercion between reference types — and nowhere else, so on a `flag: &bool`, `!flag` compiles and `if flag` does not.

```rust
let on = true;
let flag: &bool = &on;

if *flag { println!("on"); }   // on
println!("{}", !flag);         // false
// if flag { … }               // E0308: expected `bool`, found `&bool`
```

Neither `!flag` nor `if flag` has a `*`. The first compiles because std carries a separate `impl Not for &bool`; a condition has no impl to look for.

## Five places the `*` is written for you

| you write, with `flag: &bool` and `n: &i32` | why no `*` |
|---|---|
| `n.abs()`, `n.pow(2)` | the dot: [method resolution](../../../12_Traits/method_resolution/README.md) dereferences the receiver until a method fits |
| `!flag`, `flag & other`, `flag \| other`, `n + 1`, `-n` | std implements the operator on the reference as well: `impl Not for &bool`, `impl BitAnd<bool> for &bool`, `impl BitOr<bool> for &bool`, `impl Add<i32> for &i32`, `impl Neg for &i32` — see [operators are traits](../../../12_Traits/operators_are_traits/README.md) |
| `println!("{n}")` | `impl<T> Display for &T where T: Display + ?Sized` forwards to the `i32` |
| `match flag { true => …, false => … }` | [match ergonomics](../../../30_Pattern_Matching/match_ergonomics/README.md): a non-reference pattern matched against a reference dereferences it |
| `show(&r)` with `r: &bool`, and `show(&boxed)` with `boxed: Box<bool>`, for `fn show(flag: &bool)` | [deref coercion](../../../29_Conversion/coercion/README.md): at a coercion site a `&&bool` or a `&Box<bool>` becomes the `&bool` the parameter wants — a `*` inserted inside the `&`, never one that reaches `bool` |

`flag` is still a `&bool` in the first four rows; nothing converted it. The fifth converts, but only from one reference type to another. Each row works because something was written for the reference type specifically, which is why the list is short.

## A condition wants exactly `bool`

`if`, `while`, `&&` and `||` take a `bool` and consult no trait — `&&` and `||` [cannot be overloaded](../../../12_Traits/operators_are_traits/README.md#what-cannot-be-overloaded) — so there is nothing for a `&bool` to match:

```text title="Abridged — real rustc output for if_flag.rs"
error[E0308]: mismatched types
 --> if_flag.rs:4:8
  |
4 |     if flag {
  |        ^^^^ expected `bool`, found `&bool`
  |
help: consider dereferencing the borrow
  |
4 |     if *flag {
  |        +
```

`while flag`, `flag && other` and `other || flag` are the same `E0308`. So are `let b: bool = flag;` and passing `flag` to a `fn(bool)`: no [coercion](../../../29_Conversion/coercion/README.md) goes from a reference to the value behind it.

## `==` wants both sides at the same depth

std compares references with references — `impl<A, B> PartialEq<&B> for &A where A: PartialEq<B>` — and has no impl comparing `&bool` with `bool`:

```text title="Abridged — real rustc output for flag_eq_true.rs"
error[E0277]: can't compare `&bool` with `bool`
 --> flag_eq_true.rs:4:13
  |
4 |     if flag == true {
  |             ^^ no implementation for `&bool == bool`
  |
  = help: the trait `PartialEq<bool>` is not implemented for `&bool`
help: consider dereferencing here
  |
4 |     if *flag == true {
  |        +
```

Either side can move: `*flag == true` compares two `bool`, `flag == &true` compares two `&bool`. `assert_eq!(flag, true)` is the same `E0277`, because it compares with `==`.

`>` on a `&i32` is the same mistake under a different code:

```text title="Abridged — real rustc output for n_gt_one.rs"
error[E0308]: mismatched types
 --> n_gt_one.rs:4:12
  |
4 |     if n > 1 {
  |            ^ expected `&i32`, found integer
  |
help: consider dereferencing the borrow
  |
4 |     if *n > 1 {
  |        +
```

The code differs because the impl lists do. `&A` has two `PartialEq` impls, against `&B` and against `&mut B`, so rustc cannot settle the right-hand type and reports a missing impl. It has one `PartialOrd` impl, against `&B`, so rustc knows the right side must be `&i32` and reports a mismatch. `*n > 1` and `n > &1` both compile — while `n + 1` needs neither, because arithmetic does have an `Add<i32> for &i32`.

## The dot stops at the first receiver that fits

`n.abs()` works because `&i32` has no `abs`, so the search dereferences to `i32`. `max` is different: `Ord` is implemented for `&A` as well, so `n.max(…)` is found on `&i32` itself, and its argument has to be a `&i32`:

```text title="Abridged — real rustc output for max_on_a_reference.rs, a help and a note trimmed"
error[E0308]: mismatched types
 --> max_on_a_reference.rs:4:26
  |
4 |     println!("{}", n.max(3));
  |                      --- ^ expected `&i32`, found integer
  |                      |
  |                      arguments to this method are incorrect
  |
help: consider borrowing here
  |
4 |     println!("{}", n.max(&3));
  |                          +
```

rustc's `n.max(&3)` compiles and hands back a `&i32`; `(*n).max(3)` hands back an `i32`. [The search](../../../12_Traits/method_resolution/README.md#the-search) takes the first candidate, not the one you meant.

## Closures: count the `&`

`bits.iter()` yields `&bool`. [`any` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) passes that item on as it is; [`filter` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) passes a reference to it:

```rust
let bits = vec![true, false, true];
let any = bits.iter().any(|b| *b);                // true — b: &bool
let set = bits.iter().filter(|b| **b).count();    // 2    — b: &&bool
let same = bits.iter().filter(|&&b| b).count();   // 2    — both `&` taken apart in the pattern
let has = bits.contains(&true);                   // true — contains takes `x: &T`
```

One `*` short is the `if flag` error again: `.filter(|b| *b)` is `E0308`, expected `bool`, found `&bool`. [`contains` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.contains) goes the other way — `bits.contains(true)` is `E0308`, expected `&bool`, found `bool`. The second `&` in `filter` is [the extra `&` you did not ask for](../../../30_Pattern_Matching/match_ergonomics/README.md#the-trap-the-extra-you-did-not-ask-for).

## Writing through `&mut bool`

`*slot` names the place `slot` points at — [`*r` is a place, not a value](../../where_the_sigil_sits/README.md#r-is-a-place-not-a-value). Without the `*`, `slot = true` tries to re-point the reference itself:

```text title="Abridged — real rustc output for write_through_mut.rs"
error[E0308]: mismatched types
 --> write_through_mut.rs:4:12
  |
3 |     let slot: &mut bool = &mut on;
  |               --------- expected due to this type
4 |     slot = true;
  |            ^^^^ expected `&mut bool`, found `bool`
  |
help: consider dereferencing here to assign to the mutably borrowed value
  |
4 |     *slot = true;
  |     +

error[E0600]: cannot apply unary operator `!` to type `&mut bool`
 --> write_through_mut.rs:5:21
  |
5 |     let off: bool = !slot;
  |                     ^^^^^ cannot apply unary operator `!`
```

The second error is the operator row of the first table running out: std's `Not`, `BitAnd` and `BitOr` impls cover `bool` and `&bool`, not `&mut bool`. So it is `*slot = !*slot`, and `*slot |= true` — `slot |= true` is `E0368`.

## The whole table

| construct | needs `*`? | why |
|---|---|---|
| `n.abs()` | no | [the dot dereferences](#five-places-the-is-written-for-you) |
| `n.max(&3)` | no — but the argument needs `&` | [the dot stops at `&i32`](#the-dot-stops-at-the-first-receiver-that-fits), which has `Ord` |
| `!flag`, `flag & other`, `n + 1`, `-n` | no | [an impl for `&T`](#five-places-the-is-written-for-you) |
| `println!("{n}")` | no | `Display for &T` |
| `match flag { true => … }` | no | match ergonomics |
| `show(&r)`, `fn show(&bool)`, `r: &bool` | no | deref coercion, `&&bool` to `&bool` |
| `if flag`, `while flag`, `flag && other` | **yes** | [a condition takes `bool`](#a-condition-wants-exactly-bool); no trait, no coercion |
| `let b: bool = flag` | **yes** | no coercion from `&T` to `T` |
| `flag == true`, `n > 1` | **yes**, or `&` on the other side | [same depth](#wants-both-sides-at-the-same-depth) |
| `.any(\|b\| *b)` | one | the item is `&bool` |
| `.filter(\|b\| **b)` | two | [a `&` to the `&bool` item](#closures-count-the) |
| `slot = true` on `&mut bool` | **yes** | [`*slot` is the place](#writing-through-mut-bool) |
| `!slot` on `&mut bool` | **yes** | no `Not` for `&mut bool` |
| `bits.contains(true)` | no — it needs a `&` added | `contains` takes `&T` |

## Checkpoint

**Predict before you open the answer.** `if flag` is refused. Does `assert!(flag)` compile?

<details markdown="1">
<summary><strong>The answer</strong></summary>

<!-- output:when_you_need_the_star -->
*Verified output of [`when_you_need_the_star.rs`](examples/when_you_need_the_star.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The dot dereferences the receiver for you
   n.abs()                        -> 7
   n.pow(2)                       -> 49

2. An operator with an impl for &T needs no `*`
   !flag                          -> false
   flag & other                   -> false
   flag | other                   -> true
   n + 1                          -> -6
   -n                             -> 7

3. Formatting reads through the reference
   println!("{n}")                -> -7

4. A literal pattern matched against a reference
   match flag { true => .. }      -> on

5. Deref coercion: a `&bool` parameter accepts `&&bool` and `&Box<bool>`
   show(&r)                       -> true
   show(&boxed)                   -> false

6. A condition wants exactly `bool`: write the `*`
   if *flag                       -> taken
   *flag && !other                -> true

7. `==` and `>` want both sides at the same depth
   *flag == true                  -> true
   flag == &true                  -> true
   *n > 1                         -> false
   n > &1                         -> false

8. The dot stops at the first receiver that fits
   n.max(&3)                      -> 3, a &i32
   (*n).max(3)                    -> 3, an i32

9. A closure gets one `&` per adapter
   bits.iter().any(|b| *b)        -> true
   bits.iter().filter(|b| **b)    -> 2 kept
   bits.iter().filter(|&&b| b)    -> 2 kept
   bits.contains(&true)           -> true

10. Writing through `&mut bool` names the place with `*`
   *slot = true                   -> *slot = true
   *slot = !*slot                 -> *slot = false
   *slot |= true                  -> lamp = true

Checkpoint. `if flag` is refused. Does `assert!(flag)` compile?
   yes: assert!(flag) expands to `if !flag { panic!(..) }`,
   and `!` has an impl for &bool
```
<!-- /output -->

On 1.98.0 `assert!(flag)` expands to `if !flag { ::core::panicking::panic("assertion failed: flag") }`. The macro never writes `if flag`; it writes `!flag`, which is the first table's `impl Not for &bool`. `assert_eq!(flag, true)` goes through `==` instead, and is refused.

</details>

## If you are coming from another language

- **C.** `if (flag)` on a `bool *flag` compiles and tests the *pointer*: any non-null address is true, whatever the `bool` behind it holds. Apple clang 21 with `-Wall -Wextra` prints no warning for it. Rust's `E0308` on `if flag` is that bug made unwritable — a condition takes a `bool`, and a reference is a different type. What does not carry over is C's rule that the `*` is always yours to write: Rust writes it at the dot, and an operator like `!` accepts the pointer directly because std wrote an impl for it.
- **C++.** A `bool&` is an alias, so `if (flag)` on one reads the `bool`, and every operator works through it with no `*`. A Rust `&bool` is not an alias; it is a value of its own type, closer to a `bool*` that cannot be null or dangle. C++ lets a `bool*` into a condition through the pointer-to-`bool` conversion, and a `unique_ptr<bool>` through `explicit operator bool` — both test for non-null, not the `bool`. Rust has neither conversion; the operators that do accept a `&bool` are impls listed in the `bool` docs. The C++ reflex to unlearn is expecting a reference to behave like the thing it refers to everywhere.
- **Python.** `if x:` takes any object and asks it for truthiness, so `if [False]:` is taken — a non-empty list is true whatever it holds. Rust has no truthiness at all: `if 1 {}` is `E0308`, expected `bool`, found integer, the same error as `if flag`. Python has no `*` to forget because it has no reference type to dereference; every name already refers. The confusion Rust stops here — testing the container instead of its contents — is the one Python's `if [False]:` lets through.
- **ABAP.** There is no boolean type to be confused about: `abap_bool` is a one-character field holding `'X'` or space, so every condition is a comparison, `IF flag = abap_true.` — the shape of Rust's `*flag == true`. A data reference is read whole with `r->*` and a component with `r->comp`; the second is ABAP's version of the dot writing the `*` for you. Rust extends that convenience to the operators std implemented on references, such as `!flag`, and stops at the condition, where ABAP would also make you write `r->* = abap_true`.

## See also

- [Where the `&` sits decides what it does](../../where_the_sigil_sits/README.md) — `*r` as a place, and the `*` in a type that is not a dereference
- [Method resolution](../../../12_Traits/method_resolution/README.md) — the search the dot runs, and where it stops
- [Operators are traits](../../../12_Traits/operators_are_traits/README.md) — the traits behind `!`, `&`, `+` and `==`
- [Coercion](../../../29_Conversion/coercion/README.md) — the conversions the compiler does insert, and [method calls as a different rule](../../../29_Conversion/coercion/README.md#method-calls-are-a-different-rule-that-looks-the-same)
- [Match ergonomics](../../../30_Pattern_Matching/match_ergonomics/README.md) — the pattern side, and the `**n` closure trap
- [Reborrowing](../../reborrowing/README.md) — another `*` a call site writes for you, as `&mut *r`
- The Reference, [the dereference operator ↗](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-dereference-operator) · [`Deref` ↗](https://doc.rust-lang.org/std/ops/trait.Deref.html) · [`bool`'s trait implementations ↗](https://doc.rust-lang.org/std/primitive.bool.html#trait-implementations), where `Not for &bool` is listed

## Po polsku

Pytanie brzmi: dlaczego przy `bool` gwiazdka raz jest potrzebna, a raz nie? Odpowiedź: chodzi o to, **gdzie Rust sam dopisuje `*`, a gdzie nie**. Przy `flag: &bool` wyrażenie `!flag` się kompiluje, a `if flag` już nie — i nie ma w tym sprzeczności. `!flag` działa, bo biblioteka standardowa ma osobną implementację `impl Not for &bool`. `if` nie szuka żadnej cechy (*trait*): chce dokładnie `bool`, więc `&bool` kończy się błędem `E0308: expected bool, found &bool`, a poprawka to `if *flag`.

Pięć miejsc, w których gwiazdkę pisze kompilator: **kropka** (`n.abs()` — wyszukiwanie metody wyłuskuje odbiorcę), **operator z implementacją dla referencji** (`!flag`, `flag & other`, `n + 1`), **formatowanie** (`println!("{n}")`), **wzorzec** (`match flag { true => … }`) i **koercja deref** — `show(&r)` przy `fn show(flag: &bool)` i `r: &bool` zamienia `&&bool` w `&bool`, ale nigdy nie dochodzi do samego `bool`. Nigdzie indziej. W warunku `if`/`while`, w `&&` i `||`, przy przypisaniu `let b: bool = flag` — gwiazdkę piszesz sam.

Porównanie chce obu stron **na tej samej głębokości**: `flag == true` to `E0277` (*can't compare `&bool` with `bool`*), a działa `*flag == true` albo `flag == &true`. W domknięciach licz referencje: `iter()` daje `&bool`, `filter` podaje referencję do elementu, stąd `filter(|b| **b)`, a `any(|b| *b)`. Zapis przez `slot: &mut bool` to zawsze `*slot = true` — bez gwiazdki próbujesz przestawić samą referencję. Ciekawostka na koniec: `assert!(flag)` się kompiluje, bo makro rozwija się do `if !flag { panic… }`.

**Szukaj po polsku:** wyłuskanie referencji · operator gwiazdki w Ruscie · `rust expected bool found &bool` · `rust can't compare &bool with bool` · `rust auto deref operators` · `rust **n filter closure`
