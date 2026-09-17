# Miri

**Level:** 301 · deep dive

**One line:** [Miri ↗](https://github.com/rust-lang/miri) runs your tests in an interpreter that checks each operation against the rules `unsafe` code must keep — out-of-bounds and freed memory, uninitialised reads, invalid values, misalignment, aliasing, data races — on the paths the tests take, and stops at the first foreign function it has no shim for.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Running it: a nightly component, so `rustup +nightly component add miri` and then `cargo +nightly miri test` or `cargo +nightly miri run`. How does that sit beside this library's pinned stable toolchain without [changing the machine's default](../../05_Tooling/nightly/README.md)?
- What it reported in four small probes: "constructing invalid value of type bool: encountered 0x02, but expected a boolean"; "memory is uninitialized" after a `set_len` whose filling panicked; "Data race detected between (1) non-atomic write on thread `main` and (2) non-atomic read"; and, for two `&mut` made from one raw pointer, "that tag does not exist in the borrow stack". Each message names a rule — which page of this library owns each rule?
- Aliasing: Stacked Borrows is the default and `-Zmiri-tree-borrows` switches to Tree Borrows; the README calls both experimental. Which program does one accept and the other reject, and what does that say about how settled the aliasing rules are?
- What it cannot see: foreign code. `abs` from libc stops the run with "unsupported operation: can't call foreign function `abs` on OS `macos`", while `getpid` runs, because Miri ships a shim for it. `-Zmiri-native-lib` experimentally calls real native code — with what checking lost?
- What it cannot prove: it runs the paths your tests execute and one schedule per seed; `-Zmiri-seed` and `-Zmiri-many-seeds` try more. Its README says it "fundamentally cannot ensure that your code is *sound*" — the difference between a passing Miri run and a proof.
- Cost: how much slower than a native `cargo test`, measured on one crate, and the `cfg(miri)` flag with `#[cfg_attr(miri, ignore)]` for tests too slow or too FFI-bound to run under it.
- Leaks: memory still allocated at exit and not reachable from a `static` is reported as an error. When is a deliberate `Box::leak` a false alarm?

## The trap it exists for

"Miri passed" read as "this `unsafe` code is sound". It ran the tests that exist, on the schedules it happened to pick, with no foreign code — and a test suite that never hands the code a panicking closure never walks the unwind path where the undefined behaviour was.

## Where this sits

- [Testing concurrent code](../testing_concurrent_code/README.md) covers Miri's scheduler beside `loom` and ThreadSanitizer; [When the type checker is wrong](../../20_Compilers/when_the_type_checker_is_wrong/README.md) covers soundness holes in `rustc` itself; [`rustup default nightly`](../../05_Tooling/nightly/README.md) covers the toolchain it needs. This page covers only running Miri over `unsafe` code and reading what it says.

## See also

- [`rustup default nightly`](../../05_Tooling/nightly/README.md) — where Miri lives, and how to reach it without switching the default
- [When the type checker is wrong](../../20_Compilers/when_the_type_checker_is_wrong/README.md) — eleven memory-corrupting soundness bugs in `rustc`, every one detected by Miri and AddressSanitizer
- [Validity invariants](../validity_invariants/README.md) — the rule behind the invalid-`bool` report
- [Panics in unsafe code](../panics_in_unsafe_code/README.md) — the rule behind the uninitialised-memory report
- [Testing concurrent code](../testing_concurrent_code/README.md) — the data-race report, and the tools beside it
- [Wrong, but not unsafe](../../14_Strings/wrong_but_not_unsafe/README.md) — two bugs Miri is silent on, because nothing undefined happened
- [TOOLCHAIN.md](../../TOOLCHAIN.md) — the toolchain map this tool belongs on

## If you are coming from another language

- **C and C++.** AddressSanitizer, MemorySanitizer and UndefinedBehaviorSanitizer instrument the real binary at compile time; Miri interprets instead, which makes it far slower and lets it check rules no sanitizer knows, such as Rust's aliasing rules and invalid `bool` values. The [C and C++ section](../../31_C_and_Cpp/README.md) records which of nine bugs each sanitizer caught. Valgrind is the closer relative in spirit — run the program on a checking virtual machine — and shares Miri's blind spot for code paths the run never took.
- **Go.** `go vet`'s `unsafeptr` analyzer and the `checkptr` instrumentation that `-race` switches on check `unsafe.Pointer` conversions — a narrow slice of what Miri checks for raw pointers, run on the real binary rather than an interpreter.
- **Python.** The interpreter already checks every operation, which is what Miri makes Rust do temporarily — and why a Python program never needs it until it calls into a C extension, the one thing Miri cannot follow either.

## Po polsku

Miri to interpreter pośredniej reprezentacji kompilatora (MIR), który podczas testów sprawdza każdą operację: odczyt poza zakresem lub ze zwolnionej pamięci, niezainicjalizowane dane, niepoprawne wartości, złe wyrównanie, łamanie reguł aliasowania (*Stacked Borrows*, *Tree Borrows*) i wyścigi danych. Działa tylko na nocnej wersji kompilatora (*nightly*) i tylko na ścieżkach, które testy faktycznie przechodzą, a zatrzymuje się na pierwszej funkcji obcej (FFI), dla której nie ma podmianki (*shim*). „Miri przeszło” nie znaczy więc „kod jest poprawny” — znaczy „te testy nie trafiły na niezdefiniowane zachowanie”.

**Szukaj po polsku:** niezdefiniowane zachowanie w Ruscie · reguły aliasowania · `cargo miri test` · `miri stacked borrows vs tree borrows` · `miri unsupported operation foreign function`
