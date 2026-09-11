# Two clocks: `Instant` and `SystemTime`

**Level:** 201 · working knowledge

**One line:** `Instant` is a stopwatch and `SystemTime` is a calendar — the first measures *how long* and never runs backwards, the second tells you *what time it is* and can be set — so you pick by the question you are asking, not by precision.

```rust
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn main() {
    // How long?  Ask the stopwatch.
    let start = Instant::now();
    let total: u64 = (1..=1_000).sum();
    let took = start.elapsed();                                     // a Duration

    // What time is it?  Ask the calendar.
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH); // a Result

    println!("{total} {}", since_epoch.is_ok());                    // 500500 true
    println!("{took:?}");                                           // a different Duration every run
}
```

| Clock | Answers | What `now()` reads | Can it go backwards? | C++ |
|---|---|---|---|---|
| [`Instant` ↗](https://doc.rust-lang.org/std/time/struct.Instant.html) | how long | `clock_gettime` with `CLOCK_MONOTONIC` on Unix and `CLOCK_UPTIME_RAW` on macOS; `QueryPerformanceCounter` on Windows | no | `steady_clock` |
| [`SystemTime` ↗](https://doc.rust-lang.org/std/time/struct.SystemTime.html) | what time | `clock_gettime`'s realtime clock; `GetSystemTimePreciseAsFileTime` or `GetSystemTimeAsFileTime` on Windows | yes | `system_clock` |

The *reads* column is std's own table for 1.98.0, which adds *"These system calls might change over time."* Precision differs by platform too: on Windows a `SystemTime` counts in 100-nanosecond steps, where Linux can represent single nanoseconds.

## `Instant`: only differences

An `Instant` holds no number you can read. Its docs call it *"Opaque and useful only with Duration"*: two can be compared, one can be subtracted from another, a `Duration` can be added to one, and that is the whole interface. There is no `as_secs`, because the zero point is whatever the operating system picked — on Linux, `CLOCK_MONOTONIC` counts from boot — and a raw reading would mean nothing on another machine, or on this one after a restart.

Its promise is a direction: *"monotonically nondecreasing"*, each instant *"no less than any previously measured instant"*. The example below takes a thousand readings in a row and checks. What it does not promise is a rate:

> Note, however, that instants are not guaranteed to be steady. In other words, each tick of the underlying clock might not be the same length (e.g. some seconds may be longer than others). An instant may jump forwards or experience time dilation (slow down or speed up), but it will never go backwards.

Nor does it promise anything about sleep: it is *"not specified whether system suspends count as elapsed time or not"*. On this Mac they do not count — `CLOCK_UPTIME_RAW` *"does not increment while the system is asleep"*, per `man clock_gettime` — and on Linux `CLOCK_MONOTONIC` *"does not count time that the system is suspended"* either. An `Instant` taken before you close a laptop and read after you open it has missed the time the lid was shut.

## `SystemTime`: a date, and it can be set

`SystemTime` reads the clock that NTP, an administrator or the user can set, and its docs put the consequence first: *"Distinct from the Instant type, this time measurement is not monotonic."* Save a file, save another, and the second can carry the earlier time. So every method that asks it for a length — `duration_since`, `elapsed` — returns a `Result` rather than a `Duration`. [The next page](../an_instant_is_not_a_system_time/README.md) is about that `Result`, and about the subtraction it replaces.

Its fixed point is [`UNIX_EPOCH` ↗](https://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html), defined as *"1970-01-01 00:00:00 UTC"* on all systems. `duration_since(UNIX_EPOCH)` is the POSIX timestamp, and `UNIX_EPOCH + duration` builds a `SystemTime` for any date — which is how the example below gets one that is the same on every run. Leap seconds are not counted.

## The same clock, a stronger promise in C++

C++'s `steady_clock` promises more than `Instant` does. Its `is_steady` is `true`, and the standard's clock requirements ([time.clock.req ↗](https://eel.is/c++draft/time.clock.req), Table 131 in the current draft) define `true` as *"`t1 <= t2` is always true and the time between clock ticks is constant"*. On Linux, libstdc++'s [`steady_clock::now()` ↗](https://github.com/gcc-mirror/gcc/blob/master/libstdc%2B%2B-v3/src/c%2B%2B11/chrono.cc) reads `CLOCK_MONOTONIC` — the clock `Instant` reads there — and Linux's [man page ↗](https://man7.org/linux/man-pages/man2/clock_gettime.2.html) says that clock *"is affected by frequency adjustments"*: NTP slews it, running it slightly fast or slow to steer it toward true time.

So one clock carries two statements. C++ promises constant ticks over a clock the kernel deliberately speeds up and slows down; Rust promises only the direction.

## No third clock

C++'s third clock, `high_resolution_clock`, *"may be a synonym for `system_clock` or `steady_clock`"* ([time.clock.hires ↗](https://eel.is/c++draft/time.clock.hires)), and the two standard libraries on this Mac chose differently:

```cpp
#include <chrono>
#include <cstdio>
#include <type_traits>
namespace sc = std::chrono;

int main() {
    std::printf("steady_clock::is_steady                         %d\n", sc::steady_clock::is_steady);
    std::printf("system_clock::is_steady                         %d\n", sc::system_clock::is_steady);
    std::printf("is_same_v<high_resolution_clock, steady_clock>  %d\n",
                std::is_same_v<sc::high_resolution_clock, sc::steady_clock>);
    std::printf("is_same_v<high_resolution_clock, system_clock>  %d\n",
                std::is_same_v<sc::high_resolution_clock, sc::system_clock>);
}
```

```text title="Real run — clocks.cpp, Homebrew GCC 15.2.0 (libstdc++), g++-15 -std=c++20"
steady_clock::is_steady                         1
system_clock::is_steady                         0
is_same_v<high_resolution_clock, steady_clock>  0
is_same_v<high_resolution_clock, system_clock>  1
```

```text title="Real run — clocks.cpp, Apple clang 21.0.0 (libc++), clang++ -std=c++20"
steady_clock::is_steady                         1
system_clock::is_steady                         0
is_same_v<high_resolution_clock, steady_clock>  1
is_same_v<high_resolution_clock, system_clock>  0
```

A C++ benchmark timed with `high_resolution_clock` therefore reads a clock that can be set backwards when it is built with GCC, and one that cannot when it is built with Clang — from the same source. Rust has no such clock: *how long* is `Instant`, and there is nothing else to pick.

std has no `Clock` trait either. C++ describes a clock as requirements any type can meet — a `now()`, a `time_point`, an `is_steady` — so `steady_clock`, `system_clock` and a fake clock in a test all satisfy them, and a function can be generic over "some clock". `Instant` and `SystemTime` share nothing, and code that wants to swap in a fake clock needs a trait of its own.

## The verified output

<!-- output:two_clocks -->
*Verified output of [`two_clocks.rs`](examples/two_clocks.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Instant: a stopwatch
   Instant::now() <= Instant::now()   true
   1,000 readings, each >= the last   true
   b - a        is a core::time::Duration
   a.elapsed()  is a core::time::Duration

2. SystemTime: a calendar
   now.duration_since(UNIX_EPOCH).is_ok()   true
   now.duration_since(UNIX_EPOCH) is a
       core::result::Result<core::time::Duration, std::time::SystemTimeError>
   now.elapsed() is a
       core::result::Result<core::time::Duration, std::time::SystemTimeError>

3. The epoch is a fixed point, so a SystemTime can be built from it
   (UNIX_EPOCH + 86_400 s).duration_since(UNIX_EPOCH) = Ok(86400s)
```
<!-- /output -->

The two long types in section 2 are the difference between the clocks, one line each: the stopwatch answers with a `Duration`, the calendar with a `Result`. They say `core::time::Duration` because `std::time::Duration` is `core`'s type, re-exported.

## If you are coming from another language

- **C++.** `steady_clock` is `Instant` and `system_clock` is `SystemTime`, down to the system call on Linux. The differences are the three sections above: C++ promises steady ticks where Rust promises only a direction, C++ has a third clock whose meaning depends on the standard library, and C++ has a clock *concept* where Rust has two unrelated types. The C++ side of this page is [Three clocks ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/three_clocks/index.html).
- **Python.** `time.monotonic()` and `time.perf_counter()` are `Instant`; `time.time()` is `SystemTime`. Python's docs give the two stopwatches the same opaqueness: *"The reference point of the returned value is undefined, so that only the difference between the results of two calls is valid."* On this Mac they are the same clock as `Instant`, too — `time.get_clock_info()` names `mach_absolute_time()` behind both (Python 3.14.7), and `man clock_gettime` says `CLOCK_UPTIME_RAW` is *"identical to the result of mach_absolute_time()"* once converted — while `time.time()` names `clock_gettime(CLOCK_REALTIME)`, the clock `SystemTime` reads. What changes is the type. All three return a `float`, so nothing stops `time.time() - time.monotonic()`: it runs, and produces a number of seconds that means nothing. The [next page](../an_instant_is_not_a_system_time/README.md) is Rust refusing to compile exactly that.
- **ABAP.** *(Not machine-checked — CI cannot run ABAP; this is from the ABAP keyword documentation.)* `GET RUN TIME FIELD` is the stopwatch: its first execution in an internal session sets the field to `0`, and each later one sets it to the microseconds elapsed since that first execution — a value with no meaning on its own, useful only as a difference, exactly like an `Instant`. `GET TIME STAMP FIELD` is the calendar: a UTC time stamp taken from the application server's system time and date, in a packed `TIMESTAMP`, or a `TIMESTAMPL` with seven decimal places of seconds. One difference worth knowing: the stopwatch's field is an `i`, and the documentation says not to measure sections longer than 1,000 seconds, so that its value range is not exceeded.

## See also

- [An `Instant` is not a `SystemTime`](../an_instant_is_not_a_system_time/README.md) — what happens when you subtract one from the other
- [A `Duration` cannot be negative](../a_duration_cannot_be_negative/README.md) — the type both clocks answer in
- [Timing a block](../timing_a_block/README.md) — the stopwatch in use
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — why the calendar's answer is a `Result`

## Po polsku

W Ruscie są dwa zegary i wybiera się je według pytania, a nie według precyzji. `Instant` odpowiada na pytanie „jak długo?” — to stoper. `SystemTime` odpowiada na pytanie „która godzina?” — to kalendarz. Po polsku `Instant` najlepiej oddaje słowo **chwila** (*instant*): punkt w czasie, którego nie da się odczytać jako liczby, tylko porównać z inną chwilą albo od niej odjąć. Zero tego zegara wybiera system operacyjny — na Linuksie jest to start systemu — więc sama wartość nic by nie znaczyła.

`Instant` czyta **zegar monotoniczny** (*monotonic clock*), czyli taki, który nigdy się nie cofa. Dokumentacja Rusta obiecuje jednak tylko kierunek, nie tempo: zegar może przyspieszać i zwalniać, bo NTP go koryguje, a to, czy liczy czas **uśpienia** (*suspend*), nie jest określone — na macOS nie liczy. C++ obiecuje więcej: `steady_clock::is_steady` znaczy według normy, że odstęp między tyknięciami jest stały, choć na Linuksie oba języki czytają ten sam `CLOCK_MONOTONIC`.

`SystemTime` to **czas systemowy** (*system time*, potocznie *wall-clock time*): data i godzina, które administrator, NTP albo użytkownik mogą przestawić, także wstecz. Dlatego każde pytanie o odstęp — `duration_since`, `elapsed` — zwraca `Result`, a nie `Duration`. Punktem odniesienia jest **epoka Uniksa** (*Unix epoch*), `UNIX_EPOCH`, czyli 1970-01-01 00:00:00 UTC, a sekundy przestępne (*leap seconds*) nie są liczone. Trzeciego zegara, jak `high_resolution_clock` w C++, Rust nie ma — i dobrze, bo w C++ ten sam kod dostaje z nim inny zegar zależnie od biblioteki standardowej.

**Szukaj po polsku:** zegar monotoniczny · czas uniksowy · epoka Uniksa · `rust Instant vs SystemTime` · `clock_gettime CLOCK_MONOTONIC`
