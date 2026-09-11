# A `Duration` cannot be negative

**Level:** 201 · working knowledge

**One line:** `std::time::Duration` is whole seconds plus nanoseconds, unsigned, and one type for every unit — so there is no `-1s` to hold, and a subtraction that would make one is a `None` if you ask with `checked_sub` and a panic if you don't.

```rust
use std::time::Duration;

fn main() {
    let d = Duration::from_millis(1_999);
    println!("{d:?}");                     // 1.999s
    println!("{}", d.as_secs());           // 1
    println!("{}", d.as_millis());         // 1999

    let one = Duration::from_secs(1);
    let two = Duration::from_secs(2);
    println!("{:?}", one.checked_sub(two)); // None
}
```

## One type, whole seconds plus nanoseconds

A `Duration` is two fields — in std's source, `secs: u64` and `nanos`, with the comment *"Always 0 <= nanos < NANOS_PER_SEC"*. Every constructor fills the same two, so `from_secs(2)` and `from_millis(2_000)` are the same value and compare equal. A millisecond is not a different type from a second, only a different way of writing one down.

Reading it back is where the units return, and each accessor answers a different question:

| Method | Returns | For 1.999 s |
|---|---|---|
| [`as_secs` ↗](https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs) | `u64`, whole seconds | `1` — the fraction is dropped, not rounded |
| [`subsec_nanos` ↗](https://doc.rust-lang.org/std/time/struct.Duration.html#method.subsec_nanos) | `u32`, the fraction alone | `999000000` |
| [`as_secs_f64` ↗](https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs_f64) | `f64` | `1.999` |
| [`as_millis` ↗](https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_millis) | `u128`, whole milliseconds | `1999` |

`as_secs` is the trap in that table. Its docs say the value *"does not include the fractional (nanosecond) part of the duration"*, so a 1.999-second timing logged with it reads `1`. And `as_millis` returns a `u128` because a `u64` would not hold every answer: the output below prints `Duration::MAX` in milliseconds beside `u64::MAX`, and it is a thousand times larger.

There is no `Display`, either. The docs say that is deliberate — *"there are a variety of ways to format spans of time for human readability"* — and give you a `Debug` that picks the unit itself: `90s`, `1.5s`, `250µs`. The `µ` is not ASCII, which the docs mention in case your output goes somewhere that cannot take it.

## Below zero: ask, clamp, or panic

Unsigned means the subtraction has nowhere to put a negative answer, so std makes you say what you want instead:

| You write | When the result would be negative, you get |
|---|---|
| [`a.checked_sub(b)` ↗](https://doc.rust-lang.org/std/time/struct.Duration.html#method.checked_sub) | `None` — *"if the result would be negative or if overflow occurred"* |
| [`a.saturating_sub(b)` ↗](https://doc.rust-lang.org/std/time/struct.Duration.html#method.saturating_sub) | `Duration::ZERO` |
| [`a.abs_diff(b)` ↗](https://doc.rust-lang.org/std/time/struct.Duration.html#method.abs_diff) | the gap, whichever is larger (stable since 1.81) |
| `a - b` | a panic: `overflow when subtracting durations` |

The last row panics in a release build too, which makes it different from integer overflow — that [panics in debug and wraps in release](../../31_C_and_Cpp/signed_overflow/README.md). `Duration`'s `Sub` is written `self.checked_sub(rhs).expect("overflow when subtracting durations")`, and an `expect` does not care about the build profile:

```text title="Real run — the example below built with rustc 1.98.0 -O instead of the runner's unoptimized build: its last line"
   one - two                = panicked: overflow when subtracting durations
```

## The C++ side: a signed count and a unit

C++ makes the opposite choice on both counts. A `std::chrono::duration<Rep, Period>` is a count of type `Rep` — a signed integer in every standard alias — and a unit, `Period`, that is part of the type. `seconds` and `milliseconds` are different types, and `1s - 2s` is simply `-1s`:

```cpp
#include <chrono>
#include <cstdio>
using namespace std::chrono_literals;
namespace sc = std::chrono;

int main() {
    auto d = 1s - 2s;
    std::printf("1s - 2s                        = %llds\n", static_cast<long long>(d.count()));
    sc::milliseconds ms = 3s;
    std::printf("milliseconds ms = 3s           = %lldms\n", static_cast<long long>(ms.count()));
    auto s = sc::duration_cast<sc::seconds>(1999ms);
    std::printf("duration_cast<seconds>(1999ms) = %llds\n", static_cast<long long>(s.count()));
}
```

```text title="Real run — durations.cpp, Homebrew GCC 15.2.0 and Apple clang 21.0.0 print the same three lines"
1s - 2s                        = -1s
milliseconds ms = 3s           = 3000ms
duration_cast<seconds>(1999ms) = 1s
```

Converting to the finer unit is implicit, because nothing is lost. Converting to the coarser one does not compile until you ask:

```cpp
#include <chrono>
namespace sc = std::chrono;

int main() {
    sc::seconds s = sc::milliseconds{1500};
}
```

```text title="Real run — narrowing.cpp, Homebrew GCC 15.2.0, g++-15 -std=c++20 -c"
narrowing.cpp: In function 'int main()':
narrowing.cpp:5:25: error: conversion from 'duration<[...],ratio<[...],1000>>' to non-scalar type 'duration<[...],ratio<[...],1>>' requested
    5 |     sc::seconds s = sc::milliseconds{1500};
      |                         ^~~~~~~~~~~~~~~~~~
```

That refusal is the one thing C++ checks here that Rust does not. `duration_cast<seconds>` truncates 1999 ms to `1s` exactly as `as_secs` does, but C++ makes you write the cast to get there, and Rust's `as_secs` is one method call away with nothing in the type to slow you down. What Rust checks instead is the sign: C++'s `-1s` is a value you can carry around; Rust makes you decide, at the subtraction, what a negative answer should become.

## The verified output

<!-- output:a_duration_cannot_be_negative -->
*Verified output of [`a_duration_cannot_be_negative.rs`](examples/a_duration_cannot_be_negative.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One type: whole seconds plus nanoseconds
   Duration::from_millis(1_999) = 1.999s
   .as_secs()       = 1          whole seconds: the fraction is dropped
   .subsec_nanos()  = 999000000  the fraction, on its own
   .as_secs_f64()   = 1.999      an f64
   .as_millis()     = 1999       a u128
   from_secs(2) == from_millis(2_000)   true

2. {:?} picks the unit; there is no Display
   90s
   1.5s
   250µs
   7ns
   0ns

3. Why as_millis returns a u128
   Duration::MAX             = 18446744073709551615.999999999s
   Duration::MAX.as_millis() = 18446744073709551615999
   u64::MAX                  = 18446744073709551615

4. Below zero: ask, clamp, or panic
   one.checked_sub(two)     = None
   one.saturating_sub(two)  = 0ns
   one.abs_diff(two)        = 1s
   one - two                = panicked: overflow when subtracting durations
```
<!-- /output -->

## If you are coming from another language

- **C++.** The section above: a signed count with the unit in the type, against an unsigned count with the unit in the method name. The C++ side of this page is [A duration is a count and a unit ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/a_duration_is_a_count_and_a_unit/index.html).
- **Python.** `datetime.timedelta` can be negative, and normalises the sign into its days: `timedelta(seconds=-1)` is `datetime.timedelta(days=-1, seconds=86399)` — minus one day, plus 86,399 seconds — and `timedelta(seconds=1) - timedelta(seconds=2)` prints `-1 day, 23:59:59`. A plain difference of two `time.monotonic()` floats can be negative too. Rust has neither, which moves *"what if it is negative?"* from a value you might mishandle later to a `None` you handle now. `timedelta.total_seconds()` is `as_secs_f64`; there is no truncating `as_secs` in Python to be surprised by.
- **ABAP.** *(Not machine-checked — CI cannot run ABAP; this is from the ABAP keyword documentation.)* There is no duration type. The difference of two `GET RUN TIME FIELD` readings is an `i` counting microseconds, and it goes negative without complaint if you subtract in the wrong order; the unit lives in the variable's name, if anywhere — the convention C++ turned into a type and Rust turned into method names.

## See also

- [An `Instant` is not a `SystemTime`](../an_instant_is_not_a_system_time/README.md) — where the negative case goes instead: an `Err`, or a `None`
- [Signed overflow](../../31_C_and_Cpp/signed_overflow/README.md) — the integer version of *say which behaviour you meant*, with the same `checked_` and `saturating_` names
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — why `checked_sub` answers with an `Option`
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — the `catch_unwind` the example uses to show the panic without dying of it

## Po polsku

`Duration` to po polsku **czas trwania** (*duration*) i w Ruscie jest to jeden typ dla wszystkich jednostek: w środku są dwie liczby bez znaku (*unsigned*) — całe sekundy jako `u64` i nanosekundy jako część ułamkowa. Milisekunda nie jest osobnym typem, tylko innym sposobem zapisania tej samej wartości, więc `from_secs(2) == from_millis(2_000)`.

Skoro typ jest bez znaku, **ujemnego czasu trwania nie ma**. Odejmowanie, które dałoby wynik poniżej zera, trzeba nazwać: `checked_sub` zwraca `None`, `saturating_sub` — arytmetyka z nasyceniem (*saturating arithmetic*) — zwraca zero, `abs_diff` zwraca różnicę bez względu na kolejność, a zwykłe `a - b` panikuje z komunikatem `overflow when subtracting durations`. W odróżnieniu od przepełnienia liczb całkowitych ta panika zdarza się także w buildzie `--release`, bo implementacja to zwykłe `expect`.

C++ robi odwrotnie w obu sprawach: `std::chrono::duration<Rep, Period>` ma licznik ze znakiem i jednostkę w typie, więc `1s - 2s` to po prostu `-1s`, a zamiana milisekund na sekundy wymaga jawnego `duration_cast`. W Ruscie `as_secs` obcina część ułamkową bez żadnego rzutowania (*cast*) — 1,999 s daje `1` — i to jest pułapka warta zapamiętania przy logowaniu pomiarów.

**Szukaj po polsku:** czas trwania w Ruscie · arytmetyka z nasyceniem · `rust Duration checked_sub` · `rust Duration as_secs truncate` · `std::chrono::duration_cast`
