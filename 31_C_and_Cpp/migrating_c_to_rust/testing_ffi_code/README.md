# Testing FFI code

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** No single tool covers a Rust–C boundary — tests check behaviour, sanitizers and Valgrind watch the running binary, [Miri ↗](https://github.com/rust-lang/miri) interprets Rust but cannot run the C side — so the skill is knowing what each one sees, where each one is blind, and which to reach for first.

## What it has to cover

- **Tests across the boundary:** calling the exported functions from Rust tests as C would, and a small C test program compiled and linked in CI, because a Rust test cannot catch a wrong header
- **Benchmarks:** measuring the boundary's cost with criterion, and cross-language LTO — [linker-plugin LTO ↗](https://doc.rust-lang.org/rustc/linker-plugin-lto.html) — so a small C function can be inlined into Rust and back
- **Sanitizers:** AddressSanitizer on both sides of one binary, which needs a nightly `-Zsanitizer` flag on the Rust side and a matching `-fsanitize` on the C side; what ASan, UBSan and TSan each report
- **Valgrind:** no recompilation needed, much slower, and effectively Linux-only — so it runs in the Docker column
- **Miri:** undefined behaviour in the unsafe Rust — aliasing, invalid values, out-of-bounds pointer use — but it interprets Rust and stops at a foreign function, so the C half has to be stubbed or skipped
- **A table of known bugs against the tools**, each one run: a double-free across the boundary, a mismatched signature, an invalid enum value from C, a data race in a wrapper wrongly marked `Sync` — and which tool caught which

## The trap it exists for

A clean Miri run taken as proof the FFI code is sound. Miri never executed a line of C, so every bug that needs the C side to happen — which, at a boundary, is most of them — was outside what it checked.

## See also

- [C and C++ — the honest limits](../../README.md#the-honest-limits) — which sanitizer caught which of the section's nine bugs on this machine, and which caught none
- [Nightly and its tools](../../../05_Tooling/nightly/README.md) — where Miri and `-Zsanitizer` live
- [Time and benchmarking](../../../33_Time_and_Benchmarking/README.md) — benchmarking the boundary's cost honestly
- [Other kinds of test](../../../28_Testing/other_kinds_of_test/README.md) — fuzzing an exported function
- [Sanitizers in the Unstable Book ↗](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html) and [Valgrind ↗](https://valgrind.org/)

## Po polsku

Żadne pojedyncze narzędzie nie obejmuje granicy Rust–C. Testy sprawdzają zachowanie, **sanitizery** (ASan, UBSan, TSan) i Valgrind obserwują działający program, a Miri interpretuje tylko kod Rusta i zatrzymuje się na funkcji obcej, więc nie widzi połowy w C. Trzeba wiedzieć, co każde narzędzie widzi, gdzie jest ślepe i po które sięgnąć najpierw — czysty przebieg Miri nie dowodzi, że kod FFI jest poprawny.

**Szukaj po polsku:** testowanie kodu FFI · sanitizery w Ruscie · `rust miri ffi limitations` · `rust address sanitizer c ffi` · `valgrind rust`
