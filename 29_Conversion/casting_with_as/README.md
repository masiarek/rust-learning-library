# Casting with `as`

**Level:** 201 · working knowledge

**One line:** `as` always succeeds, which is the problem — it truncates, reinterprets, saturates and rounds without a `Result`, a panic or a warning, so every one of its four losses is silent.

```rust
fn main() {
    println!("{}", 300u32 as u8);     // 44
    println!("{}", -1i32 as u32);     // 4294967295
    println!("{}", 3.9f64 as i32);    // 3
    println!("{}", f64::NAN as i32);  // 0
}
```

None of those is a bug in Rust. Each is defined behaviour, chosen deliberately, and each is a wrong answer if you did not mean it.

## The four losses

| Cast | What happens | Example |
|---|---|---|
| **narrowing** | keeps the low bits | `300u32 as u8` → `44` |
| **signedness** | same bits, read the other way | `-1i32 as u32` → `4294967295` |
| **float → int** | truncates toward zero, then **saturates**; `NaN` → 0 | `1e10f64 as i32` → `2147483647` |
| **int → float** | rounds to the nearest representable | `16_777_217i64 as f32` → `16777216` |

Narrowing is not an arithmetic overflow, so it does not panic in a debug build — this is a defined truncation, not `a + b`. Float-to-int saturation has been the defined behaviour since **Rust 1.45**; before that it was undefined and produced whatever LLVM felt like.

The non-numeric casts are the safe ones: `'A' as u32` cannot lose anything (a `char` is a Unicode scalar value), `65u8 as char` always fits, and `true as i32` is 1. Going the other way needs [`char::from_u32` ↗](https://doc.rust-lang.org/std/primitive.char.html#method.from_u32), which returns `Option` because `0xD800` is a surrogate and not a scalar value.

## What to write instead

| You are doing | Write | Because |
|---|---|---|
| widening, cannot fail | `u64::from(n)` | there is a [`From`](../from_and_into/README.md) impl, and it breaks if the type changes |
| narrowing, might fail | `u8::try_from(n)?` | [`TryFrom`](../tryfrom_and_tryinto/README.md) reports what `as` swallows |
| truncation is intended | `n as u8` | with a comment saying so |
| float → int, rounded | `n.round() as i64` | say which rounding before you cast |

The rule of thumb: **if you cannot say out loud what `as` does to the out-of-range case, you wanted `try_from`.**

The first row is worth more than it looks. `f64::from(n)` compiles only while the conversion is lossless, so the day somebody widens `n` from `u32` to `u64` the build breaks and you go and look. `n as f64` keeps compiling and starts losing precision above 2⁵³.

## The trap: `as` cannot repair a value already lost

```rust
fn main() {
    let voted: u32 = 4;
    let eligible: u32 = 6;
    println!("{:.1}", (voted / eligible) as f64 * 100.0);              // 0.0
    println!("{:.1}", (f64::from(voted) / f64::from(eligible)) * 100.0); // 66.7
}
```

Integer division ran first, `4 / 6` was 0, and the cast preserved that zero perfectly. This is the most common `as` bug in real code, and it reads as a rounding problem rather than an ordering one.

The second most common has a sharper edge:

```rust
fn main() {
    let scores = [5u8, 3, 0];
    let position: i32 = -1;
    // if position < scores.len() as i32 { scores[position as usize] }  // guard passes, index panics
    println!("{:?}", usize::try_from(position));   // Err(TryFromIntError(NegOverflow))
}
```

`-1 < 3` is true, so the guard lets it through; `position as usize` is then 18446744073709551615 and the index panics. Comparing in the *unsigned* world instead — or keeping the index a `usize` from the start, so a negative one is unrepresentable — is the fix. `.get(i)` also refuses it, which is one more reason to prefer `.get`.

## Arithmetic has the same choice, spelled out

`250u8 + 10` **panics in a debug build and wraps in release** — the one case where the two profiles disagree about the answer. Three named methods each pick a behaviour and say so:

| | `250u8` + 10 |
|---|---|
| [`saturating_add(10)` ↗](https://doc.rust-lang.org/std/primitive.u8.html#method.saturating_add) | `255` |
| [`wrapping_add(10)` ↗](https://doc.rust-lang.org/std/primitive.u8.html#method.wrapping_add) | `4` |
| [`checked_add(10)` ↗](https://doc.rust-lang.org/std/primitive.u8.html#method.checked_add) | `None` |

Only the last hands the decision back to the caller, which makes it the one to reach for when the overflow means something.

## If you are coming from another language

- **C.** `as` is C's cast, with two of the sharpest edges filed off: signed overflow on cast is defined rather than UB, and float-to-int saturates rather than being undefined. Everything else transfers, including the habit that matters — a cast in C is a *claim*, and code review looks at every one. Rust makes that easier by giving the safe conversions a different spelling (`From`), so a remaining `as` is a place somebody made a decision.
- **Python.** `int(3.9)` is `as` (truncation), `int("abc")` is `TryFrom` (it raises), and integers have no width at all, so the entire narrowing column has no Python counterpart — which is exactly why it catches Python programmers out. The float rows do transfer: `float(2**53 + 1)` loses the same bit for the same reason, and `0.1 + 0.2 != 0.3` is the same IEEE-754 arithmetic under both languages.
- **ABAP.** The implicit `MOVE` conversions are the closest thing, and they are `as` without the syntax: a `CHAR10` into a `CHAR5` truncates, packed-to-integer rounds, and none of it announces itself. Two habits transfer well. ABAP developers already count money in the smallest unit or in `TYPE p DECIMALS 2` rather than floats, which is the same conclusion the money section of the practice reaches. And `sy-subrc` after an arithmetic operation is the ancestor of `checked_add` returning `Option` — the difference is that Rust makes ignoring it visible, since a discarded `Option` warns and a discarded `sy-subrc` does not.
- **Java / C#.** `(byte) 300` is the same truncation and the same silence. C#'s `checked` / `unchecked` blocks are the nearest thing to Rust's debug/release split, with the difference that Rust's default in debug is `checked` and nobody has to remember to write it.

---

## The verified output

<!-- output:casting_with_as -->
*Verified output of [`casting_with_as.rs`](examples/casting_with_as.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Narrowing keeps the low bits and drops the rest
   300u32 as u8   = 44   (300 = 0b1_0010_1100; the low 8 bits are 0b0010_1100)
   256u32 as u8   = 0
   1000u32 as u8  = 232
   No panic, no warning, no Result. In a debug build too: this is
   not an arithmetic overflow, it is a defined truncation.

2. Signedness is a reinterpretation of the same bits
   -1i32 as u32   = 4294967295
   -1i8 as u8     = 255
   200u8 as i8    = -56
   255u8 as i8    = -1
   Two's complement, unchanged, read the other way round. A length
   that went negative and was cast to usize becomes about 18
   quintillion, which is how a bounds check gets passed by accident.

3. Float to integer saturates, and NaN becomes zero
   3.9f64 as i32     = 3   (truncates toward zero, never rounds)
   -3.9f64 as i32    = -3
   1e10f64 as i32    = 2147483647   <- saturates at i32::MAX
   -1e10f64 as i32   = -2147483648   <- and at i32::MIN
   f64::NAN as i32   = 0
   Saturation has been the defined behaviour since Rust 1.45; before
   that this was undefined and produced whatever LLVM felt like.

4. Integer to float loses precision without saying so
   16_777_217i64 as f32 = 16777216
   back again:            16777216
   f32 has 24 bits of mantissa, so 2^24 + 1 is not representable and
   the nearest value is 2^24. The cast is silent in both directions.
   i64::MAX as f64 as i64 = 9223372036854775807
   i64::MAX               = 9223372036854775807

5. The non-numeric casts, which are the safe ones
   'A' as u32   = 65
   '€' as u32   = 8364
   65u8 as char = A   <- only u8 may be cast to char, and always fits
   true as i32  = 1, false as i32 = 0
   A `char` is a Unicode scalar value, so char -> u32 never loses
   anything. The reverse needs char::from_u32, which returns Option:
   char::from_u32(0xD800) = None   <- a surrogate is not a scalar value

6. What to write instead
   widening, cannot fail   u64::from(n)         From
   narrowing, might fail   u8::try_from(n)?     TryFrom
   truncation is intended  n as u8              as, with a comment
   float to int, rounded   n.round() as i64     say which rounding
   The rule of thumb: if you cannot say out loud what `as` does to
   the out-of-range case, you wanted try_from.
   u8::try_from(200i32)  = Ok(200)
   u8::try_from(300i32)  = Err(TryFromIntError(PosOverflow))
```
<!-- /output -->

## Practice

**Four silent losses, each with the fix beside it.** Compute a turnout percentage from two `u32`s four different ways — including `(voted / eligible) as f64 * 100.0` — and say which two are wrong and why they are the *same* bug.

Then three more. Cast `-1i32` to `usize` and find the guard that lets it index an array (there is one that passes and then panics). Add 10 to `250u8` with the three named methods and say which one is the only one that lets the caller decide. And add `0.1 + 0.2` in `f64`, compare it to `0.3`, and say what a program that handles money should be counting in instead.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:casting_with_as_kata -->
*[`casting_with_as_kata.rs`](examples/casting_with_as_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: four silent losses, each with the fix beside it.
//!
//!   rustc --edition 2024 casting_with_as_kata.rs -o /tmp/ck && /tmp/ck

fn main() {
    println!("1. Turnout, computed four ways, and only two of them right");
    let voted: u32 = 4;
    let eligible: u32 = 6;
    println!("   voted = {voted}, eligible = {eligible}");
    println!("   voted / eligible * 100                = {}", voted / eligible * 100);
    println!("   voted * 100 / eligible                = {}", voted * 100 / eligible);
    println!("   (voted as f64 / eligible as f64) * 100 = {:.1}",
             (f64::from(voted) / f64::from(eligible)) * 100.0);
    println!("   (voted / eligible) as f64 * 100        = {:.1}",
             (voted / eligible) as f64 * 100.0);
    println!("   The first and last are the same bug: integer division happened");
    println!("   BEFORE the widening, so 4/6 was 0 and the cast preserved it");
    println!("   perfectly. `as` cannot repair a value that was already lost.");
    println!("   Note the third line uses f64::from, not `as` — for a widening");
    println!("   that cannot fail there is a From impl, and using it means the");
    println!("   compiler rejects the day someone changes u32 to u64.");

    println!();
    println!("2. The index that went negative");
    let scores = [5u8, 3, 0];
    let position: i32 = -1;
    let as_index = position as usize;
    println!("   position = {position}, position as usize = {as_index}");
    println!("   scores.get(that) = {:?}   <- .get still refuses", scores.get(as_index));
    println!("   The guard that lets it through is the one written in the SIGNED");
    println!("   world and cast afterwards:");
    println!("     if position < scores.len() as i32 {{ scores[position as usize] }}");
    println!("   -1 < 3 is {}, so the guard passes, and the index is then the",
             position < scores.len() as i32);
    println!("   20-digit number above.");
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let boom = std::panic::catch_unwind(|| {
        if position < scores.len() as i32 { scores[position as usize] } else { 0 }
    });
    std::panic::set_hook(hook);
    println!("   running exactly that: {}", if boom.is_err() { "panicked" } else { "returned" });
    println!("   Two fixes. Compare in the unsigned world — usize::try_from(position)");
    println!("   = {:?} — or keep the index a usize from the start", usize::try_from(position));
    println!("   and let the type make a negative one unrepresentable.");

    println!();
    println!("3. The count that stopped counting");
    let mut total: u8 = 250;
    for _ in 0..3 {
        total = total.saturating_add(5);
    }
    println!("   250u8, three saturating_adds of 5 -> {total}   (u8::MAX = 255)");
    let wrapped = 250u8.wrapping_add(10);
    println!("   250u8.wrapping_add(10)            -> {wrapped}");
    let checked = 250u8.checked_add(10);
    println!("   250u8.checked_add(10)             -> {checked:?}");
    println!("   `250 + 10` on a u8 PANICS in a debug build and wraps in release,");
    println!("   which is the one case where debug and release disagree about the");
    println!("   answer. The three named methods each pick one behaviour and say");
    println!("   so, and one of them is the only one that lets the caller decide.");

    println!();
    println!("4. The money that did not add up");
    let cents: i64 = 1_00 + 2_00 + 3_00;
    let as_float = 0.1_f64 + 0.2_f64;
    println!("   in cents (i64):   {cents} -> {}.{:02}", cents / 100, cents % 100);
    println!("   0.1 + 0.2 in f64: {as_float}");
    println!("   0.1 + 0.2 == 0.3: {}", as_float == 0.3);
    println!("   Not a casting bug, but the reason the first line exists: money");
    println!("   is counted in the smallest unit as an integer, and converted to");
    println!("   a decimal string only for display.");

    println!();
    println!("5. The rule, as a table you can apply without thinking");
    println!("   u32 -> u64     f64::from / u64::from    cannot fail, so From");
    println!("   i64 -> u8      u8::try_from(n)?         can fail, so TryFrom");
    println!("   f64 -> i64     n.round() as i64         say which rounding first");
    println!("   u64 -> f64     n as f64                 lossy above 2^53, and");
    println!("                                           there is no From for it");
    println!("   The last row is the honest exception: some conversions are lossy");
    println!("   and unavoidable, and `as` is what you have. Write the comment.");
}
```
<!-- /source -->

<!-- output:casting_with_as_kata -->
*Verified output of [`casting_with_as_kata.rs`](examples/casting_with_as_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Turnout, computed four ways, and only two of them right
   voted = 4, eligible = 6
   voted / eligible * 100                = 0
   voted * 100 / eligible                = 66
   (voted as f64 / eligible as f64) * 100 = 66.7
   (voted / eligible) as f64 * 100        = 0.0
   The first and last are the same bug: integer division happened
   BEFORE the widening, so 4/6 was 0 and the cast preserved it
   perfectly. `as` cannot repair a value that was already lost.
   Note the third line uses f64::from, not `as` — for a widening
   that cannot fail there is a From impl, and using it means the
   compiler rejects the day someone changes u32 to u64.

2. The index that went negative
   position = -1, position as usize = 18446744073709551615
   scores.get(that) = None   <- .get still refuses
   The guard that lets it through is the one written in the SIGNED
   world and cast afterwards:
     if position < scores.len() as i32 { scores[position as usize] }
   -1 < 3 is true, so the guard passes, and the index is then the
   20-digit number above.
   running exactly that: panicked
   Two fixes. Compare in the unsigned world — usize::try_from(position)
   = Err(TryFromIntError(NegOverflow)) — or keep the index a usize from the start
   and let the type make a negative one unrepresentable.

3. The count that stopped counting
   250u8, three saturating_adds of 5 -> 255   (u8::MAX = 255)
   250u8.wrapping_add(10)            -> 4
   250u8.checked_add(10)             -> None
   `250 + 10` on a u8 PANICS in a debug build and wraps in release,
   which is the one case where debug and release disagree about the
   answer. The three named methods each pick one behaviour and say
   so, and one of them is the only one that lets the caller decide.

4. The money that did not add up
   in cents (i64):   600 -> 6.00
   0.1 + 0.2 in f64: 0.30000000000000004
   0.1 + 0.2 == 0.3: false
   Not a casting bug, but the reason the first line exists: money
   is counted in the smallest unit as an integer, and converted to
   a decimal string only for display.

5. The rule, as a table you can apply without thinking
   u32 -> u64     f64::from / u64::from    cannot fail, so From
   i64 -> u8      u8::try_from(n)?         can fail, so TryFrom
   f64 -> i64     n.round() as i64         say which rounding first
   u64 -> f64     n as f64                 lossy above 2^53, and
                                           there is no From for it
   The last row is the honest exception: some conversions are lossy
   and unavoidable, and `as` is what you have. Write the comment.
```
<!-- /output -->

</details>

---

## See also

- [`TryFrom` and `TryInto`](../tryfrom_and_tryinto/README.md) — the conversion that reports what `as` swallows
- [`From` and `Into`](../from_and_into/README.md) — the one to use when the conversion cannot fail
- [What a float actually stores](../../19_Numbers/what_a_float_stores/README.md) — why 2²⁴ + 1 is not representable in an `f32`
- [Meet the byte](../../19_Numbers/meet_the_byte/README.md) — the bits that narrowing keeps
- [What a type annotation does](../../15_First_Programs/what_an_annotation_does/README.md) — where a bare `10` gets its width from, before any cast is involved
- [Strict clippy](../../05_Tooling/strict_lints/README.md) — `cast_possible_truncation` and friends turn every one of these into a warning you have to answer

## Sources

[Types: Casting ↗](https://doc.rust-lang.org/rust-by-example/types/cast.html) in Rust by Example, and the Reference's [type cast expressions ↗](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions), which is where the saturation and truncation rules are actually specified.
