# C and C++ — the bugs Rust is a reply to

**Level:** 201 · for C and C++ programmers

**One line:** Every compile-time guarantee on [Google's benefits list](../00_Start_Here/benefits_of_rust/README.md) is a reply to a specific C or C++ bug, and the list is far more convincing once you have watched the bug happen.

[Benefits of Rust](../00_Start_Here/benefits_of_rust/README.md) sorts twenty claims by the evidence behind each one. This section supplies the other half: for each claim, the program the claim is about — compiled with the C compiler already on your machine, run, and its real output printed. Nine short programs, nine failures, and the Rust that will not build them.

| The bug | Kind | What it did here | Rust's reply |
|---|---|---|---|
| [Uninitialized reads](uninitialized_reads/README.md) | initialization | the same stale number every run at `-O0`, a different one every run at `-O2` | `E0381` — the read, not the declaration, is what is checked |
| [Double-free](double_free/README.md) | temporal | `SIGABRT`, *"Double free of object"* | one value, one owner, so one free |
| [Use-after-free](use_after_free/README.md) | temporal | printed `6382657`, which is the string `"Ada"` a later `malloc` put there | a reference may not outlive what it points at |
| [Null dereference](null_dereference/README.md) | null | `SIGSEGV`, no output at all | absence is `Option`, a type you must open |
| [Forgotten unlock](forgotten_unlock/README.md) | none — a deadlock | printed two lines and then hung forever | the guard is a value; dropping it unlocks |
| [Data races](data_races/README.md) | data race | counted to 1,149,618 instead of 2,000,000 — and to exactly 2,000,000 once optimized | `Send`, `Sync`, and a borrow checker that spans threads |
| [Iterator invalidation](iterator_invalidation/README.md) | temporal | silently kept one of the two elements it was asked to delete | `E0502` — you cannot read and mutate in one breath |
| [Buffer overruns](buffer_overruns/README.md) | spatial | summed three small numbers to `1433338037`, differently each run | every index checked; `.get()` when you would rather ask |
| [Signed overflow](signed_overflow/README.md) | integer overflow | `x + 1 > x` was false at `-O0` and **true** at `-O2` | overflow is defined, and you name which definition |

*Kind* is the taxonomy the C++Now 2026 talk below opens with: spatial, temporal, type, initialization and data-race safety, with null and integer-overflow safety as goals of their own. None of the nine is a **type** confusion — the one class this section does not show yet.

Chromium's security team puts numbers on why this matters: around 70% of their high-severity security bugs are memory-unsafety problems, and half of *those* are use-after-free — [measured across 912 high or critical bugs since 2015 ↗](https://www.chromium.org/Home/chromium-security/memory-safety/).

## What C++ is doing about it

The nine pages show what C and C++ allow. The next two show C++'s own replies — the plan Yitzhak Mandelbaum laid out for Google at C++Now 2026 in [*A Path to Practically Safe C++* ↗](https://www.youtube.com/watch?v=fi6csDXvve0) — each measured on Clang 21 and 23, beside the Rust it resembles.

| The reply | Kind | What it adds to C++ | How close it gets to Rust |
|---|---|---|---|
| [Safe Buffers](safe_buffers/README.md) | spatial | a length on every buffer, a check on every index, raw-pointer indexing flagged | the same three rules — as a warning and a build mode rather than a default |
| [Lifetime safety in Clang](lifetime_safety_in_clang/README.md) | temporal | a flow-sensitive check that no pointer is read after its object died | the borrow checker's first rule, down to the three-point diagnostic — without its second |

## Why these pages have no answer key

Every other page in this library ends with a transcript that CI recompiles, re-runs and diffs against a recorded file. The C halves of these pages cannot work that way, and the reason is the subject:

> **Undefined behaviour** — a program the language standard declines to define at all, so no output is the "right" one. See [the glossary](../GLOSSARY.md).

A recorded answer key is a claim that a program has one correct output. All nine programs here are undefined behaviour, so none of them has one. Several are reproducible anyway — the double-free aborts every single time — but that is a fact about this allocator and this compiler, not about the program, and it is exactly the kind of fact that changes when you upgrade something. So the C transcripts are labelled with the compiler, target and optimization level that produced them, and nothing further is claimed. **The Rust half of every page is verified normally**, which is the asymmetry the section is about.

The compilers used, for anything you want to reproduce:

```text
Apple clang version 21.0.0 (clang-2100.1.1.101), target x86_64-apple-darwin25.5.0
rustc 1.x stable, --edition 2024, no -O (so overflow checks are on)
clang version 23.1.0 on Compiler Explorer, for the two pages on C++'s replies, whose newest checks Apple's clang 21 lacks
```

## How to read a page here

Each one runs the same five steps: **the program** (short enough to read in one go), **what it did** (a real run, with the command that produced it), **why the standard allows it**, **what Rust does instead** (a verified transcript), and **the refusal** — the actual `rustc` message for the same mistake. The two pages on C++'s replies swap *what it did* for *what the check says*, and end on where the C++ check and `rustc` part ways.

They are independent, so start with whichever bug you have personally shipped. If you have shipped none of them, [use-after-free](use_after_free/README.md) is the one with the most instructive output.

## The honest limits

- **All of this means *safe* Rust.** `unsafe` reopens every door — see [what `unsafe` turns off](../09_Advanced/what_unsafe_turns_off/README.md). The guarantee is not that the doors are locked, it is that they are labelled.
- **C and C++ have tools for most of these, and they are good.** Run against these nine programs on this machine: AddressSanitizer catches three outright — the double-free, the use-after-free, the buffer overrun — and annotates the null dereference's segfault after it has already happened. ThreadSanitizer catches the data race. UndefinedBehaviorSanitizer catches the signed overflow. Three are caught by none of them: the [uninitialized read](uninitialized_reads/README.md), because MemorySanitizer is the one that would and `clang` will not build it for `x86_64-apple-darwin`; the [forgotten unlock](forgotten_unlock/README.md), where ThreadSanitizer printed nothing in the four seconds before the hung process was killed; and the [iterator invalidation](iterator_invalidation/README.md), because nothing invalid is ever touched — only the answer is wrong. The difference from a build error is *when* and *whether*: a sanitizer finds a bug on the path a test happened to take, at a slowdown you cannot ship.
- **Modern C++ closes some of these by convention.** `unique_ptr` makes double-free hard, `lock_guard` makes a forgotten unlock hard, and both are advice a codebase can decline to take on any given line. That is the actual contrast: the same idea, enforced rather than recommended. The [two replies above](#what-c-is-doing-about-it) are C++ starting to check instead of recommend — as warnings and build modes a codebase opts into, which is the part of the gap that remains.

## See also

- [Benefits of Rust](../00_Start_Here/benefits_of_rust/README.md) — the twenty claims these nine pages are the evidence for
- [Ownership](../18_Ownership/README.md) — the mechanism behind five of the nine
- [`Option` and `Result`](../17_Option_and_Result/README.md) — behind two more
- [What every C programmer should know about undefined behaviour ↗](https://blog.llvm.org/2011/05/what-every-c-programmer-should-know.html) — the LLVM series, from the optimizer's side
- [Behaviour considered undefined ↗](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) — the same list for Rust, which is the list `unsafe` makes reachable
- [Time and benchmarking](../33_Time_and_Benchmarking/README.md) — the library's other chapter written against C++: `std::time` beside `std::chrono`, each page the twin of one in the [C++ learning library ↗](https://masiarek.github.io/cpp-learning-library/)

## Po polsku

Ten dział nie uczy C ani C++ — pokazuje dziewięć konkretnych błędów, na które te języki pozwalają, uruchomionych naprawdę i z prawdziwym wydrukiem, a obok nich Rusta, który tych samych programów po prostu nie zbuduje. Strony są niezależne, więc zaczynaj od tego błędu, który sam kiedyś wypuściłeś na produkcję; jeśli żadnego, to [use-after-free](use_after_free/README.md) ma najbardziej wymowne wyjście — wypisał `6382657`, czyli bajty łańcucha `"Ada"`, które późniejszy `malloc` zdążył wstawić w zwolnione miejsce.

Rzecz, przez którą te strony jako jedyne w całej bibliotece nie mają zapisanego wzorca wyjścia: wszystkie dziewięć programów to **zachowanie niezdefiniowane** (*undefined behaviour*). Tu warto uważać na polskie nazewnictwo, bo norma C rozróżnia trzy rzeczy, które w polskich kursach zlewają się w jedną: *zachowanie niezdefiniowane* (norma nie mówi nic, wolno wszystko), *zachowanie nieokreślone* (*unspecified* — norma podaje zbiór dopuszczalnych wyników, kompilator wybiera jeden i nie musi mówić który) oraz *zachowanie zdefiniowane przez implementację* (*implementation-defined* — kompilator wybiera, ale ma obowiązek to udokumentować). Tylko pierwsze z nich znaczy „nie istnieje poprawny wynik”, i dlatego CI nie ma tu czego porównywać: `1433338037` z przepełnienia bufora jest faktem o tym kompilatorze i tej maszynie, nie o programie. Rustowa połowa każdej strony jest weryfikowana normalnie i ta asymetria jest właściwie tematem całego działu.

Dwa zastrzeżenia, żeby nie wyszło zbyt gładko. Po pierwsze, mowa o **bezpiecznym** Ruscie — `unsafe` otwiera te drzwi z powrotem; gwarancją nie jest to, że są zamknięte, tylko że są oznaczone. Po drugie, C i C++ mają na to narzędzia i są one dobre: na tych dziewięciu programach AddressSanitizer łapie trzy błędy, ThreadSanitizer wyścig danych, UndefinedBehaviorSanitizer przepełnienie ze znakiem — ale trzech nie łapie nikt, a różnica wobec błędu kompilacji dotyczy **kiedy** i **czy**: sanitizer znajduje błąd na tej ścieżce, którą akurat wykonał test, przy spowolnieniu, którego nie wypuścisz na produkcję. Tak samo `unique_ptr` i `lock_guard` w nowoczesnym C++ — ta sama idea, tyle że zalecana, a nie wymuszona.

Dwie ostatnie strony pokazują drugą stronę medalu — co robi samo C++. [Safe Buffers](safe_buffers/README.md) to odpowiedź Clanga na przepełnienia bufora: długość zawsze razem z danymi, sprawdzany każdy indeks, surowy wskaźnik tylko za ostrzeżeniem. [Lifetime safety](lifetime_safety_in_clang/README.md) to analiza czasów życia (*lifetime analysis*) wzorowana na rustowym borrow checkerze, działająca od Clanga 23. Obie są opcjonalne i obie kończą się tam, gdzie Rust wymusza więcej — i właśnie tę granicę te strony mierzą.

**Szukaj po polsku:** zachowanie niezdefiniowane w C · bezpieczeństwo pamięci · wyścig danych · `undefined behavior C standard` · `rust memory safety` · `-fsanitize=address` · `C++ Safe Buffers` · `-Wlifetime-safety`
