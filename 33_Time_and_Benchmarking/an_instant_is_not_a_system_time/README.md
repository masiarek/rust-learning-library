# An `Instant` is not a `SystemTime`

**Level:** 201 · working knowledge

**One line:** In C++ a time point carries its clock in its type, so subtracting a `steady_clock` reading from a `system_clock` one does not compile; Rust gets the same refusal from two unrelated types — and goes one step further, refusing even `SystemTime - SystemTime`, because the answer might be negative.

```rust
use std::time::{Duration, Instant, UNIX_EPOCH};

fn main() {
    let t0 = Instant::now();
    let t1 = t0 + Duration::from_secs(2);
    println!("{:?}", t1 - t0);                   // 2s

    let a = UNIX_EPOCH + Duration::from_secs(100);
    let b = UNIX_EPOCH + Duration::from_secs(160);
    println!("{:?}", b.duration_since(a));       // Ok(60s)
    // println!("{:?}", b - a);                  // E0308 — see below
    // println!("{:?}", b - t0);                 // E0308 — see below
}
```

## The slide

Matt Godbolt's [C++Now 2026 keynote ↗](https://youtu.be/EU_nQh8wg5A) has a slide titled *Type safety* with three lines of C++ on it. Here they are as a whole file, with the `sc` alias the slide leaves off:

```cpp
#include <chrono>
namespace sc = std::chrono;

int main() {
    auto t1 = sc::steady_clock::now();
    auto t2 = sc::system_clock::now();
    auto diff = t2 - t1;
}
```

```text title="Real run — type_safety.cpp, Homebrew GCC 15.2.0, g++-15 -std=c++20 -c — abridged: the first error, 7 of 54 lines"
type_safety.cpp: In function 'int main()':
type_safety.cpp:7:20: error: no match for 'operator-' (operand types are 'std::chrono::time_point<std::chrono::_V2::system_clock, std::chrono::duration<long long int, std::ratio<1, 1000000000> > >' and 'std::chrono::time_point<std::chrono::_V2::steady_clock, std::chrono::duration<long long int, std::ratio<1, 1000000000> > >')
    7 |     auto diff = t2 - t1;
      |                 ~~ ^ ~~
      |                 |    |
      |                 |    time_point<std::chrono::_V2::steady_clock,[...]>
      |                 time_point<std::chrono::_V2::system_clock,[...]>
```

Two facts make that an error rather than a number.

**The clock is part of the type.** A C++ reading is a `time_point<Clock, Duration>`, so `t1` is a `time_point<steady_clock, …>` and `t2` a `time_point<system_clock, …>` — two different types, as the last two lines of the transcript say. And the standard declares the subtraction of two time points with **one** `Clock` parameter for both operands ([time.point.nonmember ↗](https://eel.is/c++draft/time.point.nonmember)):

```cpp
template<class Clock, class Duration1, class Duration2>
  constexpr common_type_t<Duration1, Duration2>
    operator-(const time_point<Clock, Duration1>& lhs, const time_point<Clock, Duration2>& rhs);
```

Deduction wants `Clock` to be `system_clock` from the left operand and `steady_clock` from the right, cannot have both, and drops the candidate. Nothing else matches, so: *no match for 'operator-'*.

**The refusal is the feature.** The two clocks count from different zeros. `system_clock` measures *"time since 1970-01-01 00:00:00 UTC excluding leap seconds"* ([time.clock.system.overview ↗](https://eel.is/c++draft/time.clock.system.overview)); `steady_clock`'s epoch is unspecified, and in practice it is when the machine booted. If `t2 - t1` compiled, it would come to about fifty-six years minus the machine's uptime: a number with the right unit and no meaning.

## Rust's mirror

The same three lines, with the subtraction commented out so the snippet builds:

```rust
use std::time::{Instant, SystemTime};

fn main() {
    let t1 = Instant::now();
    let t2 = SystemTime::now();
    // let diff = t2 - t1;
}
```

Uncomment line 6 and rustc refuses:

```text title="Real rustc 1.98.0 output — mixed.rs, the snippet above with line 6 uncommented"
error[E0308]: mismatched types
 --> mixed.rs:6:21
  |
6 |     let diff = t2 - t1;
  |                     ^^ expected `Duration`, found `Instant`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

The same refusal, reached a different way. Rust has no `time_point<Clock>` template: `Instant` and `SystemTime` are two unrelated structs, and an operator is only a trait impl ([operators are traits](../../12_Traits/operators_are_traits/README.md)). The one `Sub` that `SystemTime` implements is `Sub<Duration>` — move a calendar reading back by a length — so, looking for something to put on the right of `t2 -`, the compiler expects a `Duration`. That is why the code is `E0308`, *mismatched types*, and not `E0369`, which is what rustc reports when there is no `Sub` impl at all (*cannot subtract `Foo` from `Foo`*).

## The subtraction C++ allows and Rust does not

Two readings of the *same* C++ clock subtract without complaint, into a signed duration that may be negative:

```cpp
#include <chrono>
#include <cstdio>
namespace sc = std::chrono;

int main() {
    sc::system_clock::time_point a{sc::seconds{100}};
    sc::system_clock::time_point b{sc::seconds{160}};
    auto diff = a - b;
    std::printf("a - b = %lld s\n", static_cast<long long>(sc::duration_cast<sc::seconds>(diff).count()));
}
```

```text title="Real run — two_readings.cpp, Homebrew GCC 15.2.0 and Apple clang 21.0.0 print the same line"
a - b = -60 s
```

Rust will not build the same thing:

```rust
use std::time::SystemTime;

fn main() {
    let a = SystemTime::now();
    let b = SystemTime::now();
    // let d = b - a;
}
```

```text title="Real rustc 1.98.0 output — wall.rs, the snippet above with line 6 uncommented"
error[E0308]: mismatched types
 --> wall.rs:6:17
  |
6 |     let d = b - a;
  |                 ^ expected `Duration`, found `SystemTime`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Same error, same reason: `SystemTime` implements `Add<Duration>` and `Sub<Duration>`, and nothing that takes another `SystemTime`. The subtraction it offers instead is a method, and its return type is the lesson. [`duration_since` ↗](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.duration_since) returns `Result<Duration, SystemTimeError>`, because *"measurements taken earlier are not guaranteed to always be before later measurements (due to anomalies such as the system clock being adjusted either forwards or backwards)."* A `Duration` [cannot be negative](../a_duration_cannot_be_negative/README.md), so the negative case needs somewhere else to go, and that is the `Err`. The error still carries the size of the gap: `SystemTimeError::duration()` is *"how far forward the second system time was from the first."*

## `Instant - Instant` compiles, and saturates

`Instant` does implement `Sub<Instant>`, with `Output = Duration`, because a later reading of a stopwatch cannot be earlier. So what does the wrong order give you? The answer has changed over the years:

> Previous Rust versions panicked when other was later than self. Currently this method saturates. Future versions may reintroduce the panic in some circumstances.

*Saturates* means it clamps: `t0 - t1` with `t1` the later one is `0ns`, silently, and so is `t0.duration_since(t1)`. The docs give the reason — some platforms have monotonicity bugs, and saturating papers over them — and name the cost in the same breath: *"This workaround obscures programming errors where earlier and later instants are accidentally swapped."* When the order is a question rather than a certainty, ask it: [`checked_duration_since` ↗](https://doc.rust-lang.org/std/time/struct.Instant.html#method.checked_duration_since) returns an `Option`, and `None` means the wrong way round.

## The verified output

<!-- output:an_instant_is_not_a_system_time -->
*Verified output of [`an_instant_is_not_a_system_time.rs`](examples/an_instant_is_not_a_system_time.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. SystemTime: duration_since is a question with two answers
   b.duration_since(a)   = Ok(60s)
   a.duration_since(b)   = Err(SystemTimeError(60s))
   err.duration()        = 60s
   err, displayed        = second time provided was later than self

2. Instant: `-` compiles, and never goes below zero
   t1 - t0                        = 2s
   t0 - t1                        = 0ns
   t0.duration_since(t1)          = 0ns
   t0.checked_duration_since(t1)  = None
   t1.checked_duration_since(t0)  = Some(2s)
```
<!-- /output -->

Every time in it is a fixed point plus a `Duration`, so it prints the same on every run. Section 1 is what `duration_since` hands back when NTP has stepped the clock back between two real readings; section 2 is a pair of stopwatch readings subtracted the wrong way round.

## If you are coming from another language

- **C++.** Rust keeps the *Type safety* slide's refusal, but gets it from two separate types instead of a template parameter — and adds the refusal C++ does not make: two readings of the calendar do not subtract into a signed duration, they subtract into a `Result`. The C++ side of this page is [A time point knows its clock ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/a_time_point_knows_its_clock/index.html).
- **Python.** Nothing here is refused, because every clock returns a `float`: `time.time() - time.monotonic()` runs and hands back a meaningless number, and `time.time() - earlier` can come out negative, since the docs say `time.time()` *"can return a lower value than a previous call if the system clock has been set back between the two calls."* The `datetime` module has the typed version: two `datetime`s subtract into a `timedelta`, which can be negative — `datetime(2026, 1, 1) - datetime(2026, 1, 2)` prints `-1 day, 0:00:00`. Rust's `Result` makes you handle that case at the subtraction, where Python lets it flow on as data.
- **ABAP.** *(Not machine-checked — CI cannot run ABAP; this is from the ABAP keyword documentation.)* Both statements hand back plain numbers — `GET RUN TIME FIELD` an `i`, `GET TIME STAMP FIELD` a packed number — so ABAP will subtract one from the other without complaint, which is the mixed-clock subtraction above with nothing to stop it. The time stamp has a trap of its own, and the documentation names it — time stamps *"are not suited for direct calculations"*: the short form holds the digits `yyyymmddhhmmss` as one number, so subtracting two of them with `-` is a subtraction of digits, not of seconds — one second across a minute boundary, 12:00:00 minus 11:59:59, comes out as 4041. The system class `CL_ABAP_TSTMP` does time-stamp arithmetic properly, which makes it ABAP's `duration_since` — by convention rather than by type.

## See also

- [Two clocks](../two_clocks/README.md) — what each of the two types reads, and what it promises
- [A `Duration` cannot be negative](../a_duration_cannot_be_negative/README.md) — why the wrong-order case has to go somewhere other than the `Duration`
- [Operators are traits](../../12_Traits/operators_are_traits/README.md) — `type Output`, and why `Instant - Instant` is allowed to be a `Duration`
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — the two shapes the wrong-order case comes back in

## Po polsku

Slajd *Type safety* z wykładu Godbolta pokazuje, że w C++ punkt w czasie (*time point*) niesie swój zegar w typie: `time_point<steady_clock, …>` i `time_point<system_clock, …>` to dwa różne typy, a operator odejmowania dwóch punktów w czasie ma w normie **jeden** parametr szablonu `Clock` dla obu argumentów. Kompilator nie potrafi go wydedukować, więc `t2 - t1` się nie kompiluje — i dobrze, bo oba zegary liczą od innego zera: `system_clock` od 1970 roku, `steady_clock` zwykle od startu maszyny. Wynik byłby liczbą z poprawną jednostką i bez żadnego sensu.

Rust dochodzi do tej samej odmowy inną drogą. Nie ma tu szablonu z parametrem zegara: `Instant` i `SystemTime` to dwie niezwiązane struktury, a operator `-` to po prostu implementacja cechy (*trait*) `Sub`. `SystemTime` ma tylko `Sub<Duration>`, więc kompilator oczekuje po prawej stronie `Duration` i zgłasza `E0308` (*mismatched types*), a nie `E0369`.

Rust idzie przy tym o krok dalej niż C++: odmawia nawet `SystemTime - SystemTime`. C++ zwraca w takiej sytuacji czas trwania ze znakiem, który może być ujemny; w Ruscie czas trwania (*duration*) ujemny być nie może, więc odejmowanie jest metodą `duration_since`, która zwraca `Result` — zegar systemowy mógł zostać cofnięty. `Instant - Instant` się kompiluje, ale przy odwrotnej kolejności **nasyca się** (*saturates*) do zera; kto chce to wykryć, używa metody `checked_duration_since`, która zwraca wtedy `None`.

**Szukaj po polsku:** bezpieczeństwo typów · odejmowanie czasu w Ruscie · `rust duration_since SystemTimeError` · `rust Instant saturating_duration_since` · `std::chrono time_point operator-`
