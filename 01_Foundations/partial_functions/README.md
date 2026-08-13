# Partial functions: why `Option` exists

**Level:** 201 · working knowledge

**One line:** A *partial* function has no answer for some of its inputs; returning `Option<T>` makes it *total*, because "no answer" becomes one of the answers.

This is the mathematical justification for `Option`, and it is the reason the [`std::option` docs](https://doc.rust-lang.org/core/option/) list *"return values for functions that are not defined over their entire input range"* among its jobs. If you have ever wondered why a language would bother inventing this type, this is the answer.

---

## Total and partial

A **total** function has an output for every input in its domain. `abs(x)` is total over the integers; every integer has an absolute value.

A **partial** function does not. Division has no answer when the divisor is zero. The real square root has none for a negative. `first()` has none for an empty list. These are not bugs or edge cases — the operation is genuinely undefined there, and no amount of careful coding invents a value.

A language then has three ways out:

1. **Crash** — panic, throw, abort. Honest, but the caller has no way to see it coming in the type.
2. **Lie** — return a sentinel: `-1`, `null`, `NaN`, an empty string. Cheap, and it poisons everything downstream that forgets to check.
3. **Widen the output** — return a type that can also express "no answer".

Rust's third option is `Option<T>`, and the effect is exact: **a partial function returning `T` becomes a total function returning `Option<T>`.** Nothing is undefined any more. Every input maps to an answer; some of those answers are `None`.

```rust
fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 { None } else { Some(dividend / divisor) }
}
```

`divide` now has a defined result for every pair of `f64` values that exists. That is the whole trick, and it is why the caller is forced to think — not because Rust is strict for its own sake, but because the second answer is genuinely there in the type.

## A correction worth making: floats do not panic

The usual way this example is introduced is *"instead of causing a runtime error or panic, Rust uses `Option` to handle it safely."* That is true for integers and **false for the floats the example is written in**:

```text
10.0_f64 / 0.0  = inf
 0.0_f64 / 0.0  = NaN
```

No panic, no error, no warning. IEEE 754 defines both results, and they propagate silently through every later calculation — `inf` compares oddly, and `NaN` is not even equal to itself, so a downstream `==` check quietly goes false and a `sort` can misbehave.

Which makes the case for `Option` **stronger**, not weaker. Against a panic, `Option` buys you a controlled failure instead of a crash. Against `inf`, it buys you something more valuable: a wrong number never escapes into your results at all. The guard is not there to prevent a crash — it is there to stop a plausible-looking non-answer from being mistaken for an answer.

Integers are the case that really does panic:

```rust
10 / 0                  // panics at runtime
10i32.checked_div(0)    // None
```

Same expression, two designs. One is partial and blows up; the other is total and hands you an answer you can act on.

## The standard library is full of them

Once you have the concept, `Option` returns stop looking arbitrary. Every one of these is an operation undefined somewhere in its input range, offered as a total function:

| Partial version | Total version | Undefined when |
|---|---|---|
| `a / b` | `a.checked_div(b)` | `b == 0` |
| `a + b` | `a.checked_add(b)` | the result overflows |
| `a - b` | `a.checked_sub(b)` | unsigned, and it would go below zero |
| `v[i]` | `v.get(i)` | `i` is out of bounds |
| — | `v.first()` / `v.last()` | the collection is empty |
| — | `map.get(k)` | the key is absent |

The `checked_*` family is the clearest illustration in the whole library: the same arithmetic, offered twice, differing only in whether the undefined case is a panic or a value.

## When `Option` is not enough

`None` says *"no answer"* and nothing more. That is sufficient when there is exactly one way to have no answer — a negative has one reason it lacks a real square root, a key has one way to be missing.

Once a function can be undefined for *several* reasons the caller might act on differently, `None` stops being an answer and you owe them a `Result`. That is the same "could the caller ask *why not?*" test from [`Option` vs `Result`](../option_vs_result/README.md), arrived at from the other direction: partiality tells you that you need a wider return type, and the number of reasons tells you *which* wider type.

---

## Practice

**The average of nothing.** Write `mean(xs: &[f64]) -> Option<f64>`, total over every slice a caller can hand you, and print it for a scored candidate and an unscored one.

Write the unguarded version first and print its answer for an empty slice. It does not panic, which is the point — it hands back `NaN` and lets it travel. Then say in one line what your `None` means, and check that it is *not* "the average is 0".

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:partial_functions_kata -->
*[`partial_functions_kata.rs`](examples/partial_functions_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: an average that has an answer for every input.
//!
//!   rustc --edition 2024 partial_functions_kata.rs -o /tmp/pfk && /tmp/pfk

/// The partial version: undefined on an empty slice, and it does not say so.
fn mean_naive(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

/// The total version: "no answer" is now one of the answers.
fn mean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        None
    } else {
        Some(xs.iter().sum::<f64>() / xs.len() as f64)
    }
}

fn report(label: &str, xs: &[f64]) {
    match mean(xs) {
        Some(m) => println!("  {label:<14} -> average score {m}"),
        None => println!("  {label:<14} -> nobody scored this candidate"),
    }
}

fn main() {
    let scored = [5.0, 3.0, 4.0];
    let unscored: [f64; 0] = [];

    println!("The naive version, on the input it has no answer for:");
    println!("  mean_naive(scored)   -> {}", mean_naive(&scored));
    println!("  mean_naive(unscored) -> {}", mean_naive(&unscored));
    println!("      0.0 / 0.0 is NaN. No panic, no warning — and NaN is not even");
    println!("      equal to itself, so a later == or sort quietly misbehaves.");

    println!("\nThe total version says so instead:");
    report("scored", &scored);
    report("unscored", &unscored);

    println!("\nWhat None means here is a decision, so write it down:");
    println!("  mean(unscored) is None because there is no such average —");
    println!("  NOT because the average is 0, which is a score a voter can give.");
    println!("  mean(&[0.0, 0.0]) -> {:?}", mean(&[0.0, 0.0]));
}
```
<!-- /source -->

<!-- output:partial_functions_kata -->
*Verified output of [`partial_functions_kata.rs`](examples/partial_functions_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The naive version, on the input it has no answer for:
  mean_naive(scored)   -> 4
  mean_naive(unscored) -> NaN
      0.0 / 0.0 is NaN. No panic, no warning — and NaN is not even
      equal to itself, so a later == or sort quietly misbehaves.

The total version says so instead:
  scored         -> average score 4
  unscored       -> nobody scored this candidate

What None means here is a decision, so write it down:
  mean(unscored) is None because there is no such average —
  NOT because the average is 0, which is a score a voter can give.
  mean(&[0.0, 0.0]) -> Some(0.0)
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:partial_functions -->
*Verified output of [`partial_functions.rs`](examples/partial_functions.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: The classic partial function, made total
  divide(10, 2) -> Result: 5
  divide(10, 0) -> Cannot divide by zero
      `divide` now has an answer for EVERY pair of f64. That is what
      'total' means, and it is the whole trick.

──── Step 2: But be careful WHY you are guarding: floats do not panic
  10.0_f64 / 0.0  = inf
   0.0_f64 / 0.0  = NaN
      No panic, no error — IEEE 754 says inf and NaN, and they propagate
      silently through every later calculation. So the Option is not
      preventing a crash; it is stopping a WRONG NUMBER from escaping.

──── Step 3: Integers are the case that really does panic
  10 / 0 -> panicked (caught here only to keep this demo running)
  10i32.checked_div(0) -> None
      Same expression, two designs: `/` is partial and panics;
      `checked_div` is total and hands back None.

──── Step 4: std is full of these — the checked_* family
  i32::MAX.checked_add(1)  -> None
  5u32.checked_sub(10)     -> None
  10i32.checked_rem(0)     -> None
  2i32.checked_pow(40)     -> None
      Every one is an operation that is undefined somewhere in its input
      range, offered as a total function returning Option.

──── Step 5: Collections do it too: indexing vs get()
  v.first()  -> Some(10)
  v.last()   -> Some(30)
  v.get(9)   -> None
  empty.first() -> None
      `v[9]` is the partial version and panics. `.get(9)` is the total one.
      Same data, same question — the return type is the entire difference.

──── Step 6: When None is not enough, the answer is Result
  sqrt(9.0)  -> Some(3.0)
  sqrt(-1.0) -> None
      Here None is fine: there is exactly one reason a real square root
      is undefined. Once a function can be undefined for SEVERAL reasons,
      None stops being an answer and you owe the caller a Result.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/partial_functions/examples/partial_functions.rs -o /tmp/pf && /tmp/pf
```

## See also

- [`Option` vs `Result`](../option_vs_result/README.md) — choosing between the two wider types
- [`Option` fields](../option_fields/README.md) — the same idea applied to data rather than functions
- [`Option` is a one-item collection](../option_as_collection/README.md) — what the wrapper costs, and why it is usually nothing
- [`i32::checked_div`](https://doc.rust-lang.org/std/primitive.i32.html#method.checked_div) and the rest of the `checked_*` family
