# What Rust is

**Level:** 101 · for newcomers

**One line:** Rust is a compiled systems language with no garbage collector, whose compiler proves memory safety and freedom from data races for every program it accepts outside `unsafe` — and most of what makes it feel different follows from that one decision.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The one-paragraph definition, and which words in it are claims a program can check: *compiled* (see [compiled or interpreted](../../20_Compilers/compiled_or_interpreted/README.md)), *no garbage collector*, *memory-safe without `unsafe`*
- Where memory management went instead of a GC — ownership, borrowing and `Drop` — as three sentences, each pointing at the section that proves it
- What "safe" does **not** promise: no leaks, no deadlocks, no logic errors, no panics — and which of those the library shows happening in safe code
- The release train and editions: a new stable compiler every six weeks, and an edition (2015, 2018, 2021, 2024) as the one place breaking syntax changes are allowed — why a 2015 crate still builds with today's compiler
- What Rust is used for today, as evidence rather than adjectives: the Linux kernel, Android, Windows, browsers, CLIs rewritten in it — each linked to its source
- Who makes it: the Rust Project's teams and RFCs versus the Rust Foundation — a pointer to [the Rust Project](../../20_Compilers/the_rust_project/README.md)

## The trap it exists for

Reading "memory-safe" as "cannot crash". A Rust program panics, leaks with `Box::leak` or an `Rc` cycle, and deadlocks on two `Mutex`es — all in safe code. What it rules out is a narrower and more valuable list: use-after-free, double free, data races, reading uninitialized memory, out-of-bounds access that silently continues.

## Where this sits

[Benefits of Rust](../benefits_of_rust/README.md) takes Google's list of twenty selling points apart claim by claim, and [measured claims](../measured_claims/README.md) checks the numbers people quote. This page is the definition those two argue about.

## See also

- [Benefits of Rust](../../00_Start_Here/benefits_of_rust/README.md) — the twenty selling points, sorted into three kinds of claim
- [C and C++ — the bugs Rust is a reply to](../../31_C_and_Cpp/README.md) — the bugs that "memory-safe" means, each one compiled in C and refused in Rust
- [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) — where the safety promise stops, and what still holds inside the block
- [Ownership](../../18_Ownership/README.md) — the idea that replaces the garbage collector
- [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) — the release train from the side of a project that has to pick one compiler
- [Glossary](../../GLOSSARY.md) — the vocabulary the definition uses

## If you are coming from another language

- **C and C++.** The same model of the machine — values on the stack, explicit heap, no runtime — with the lifetime rules you kept in your head moved into the type checker. The C library's [Strings chapter ↗](https://masiarek.github.io/c-learning-library/03_Strings/) is a tour of what that bookkeeping costs when nobody checks it.
- **Python.** The opposite trade on every axis: Python checks types and frees memory at run time, Rust at compile time. What transfers is the tooling experience — `cargo` feels closer to `uv` than to `make`.
- **Go.** Also compiled, also one binary, also built for concurrency — but Go keeps a garbage collector and a runtime scheduler, and lets a data race compile. The Go library's [race detector ↗](https://masiarek.github.io/go-learning-library/07_Testing_Concurrent_Code/the_race_detector/) page is the run-time check Rust moves into the build.
- **Java.** The type system will feel familiar and the absence of `null`, exceptions, inheritance and a GC will not. Generics are compiled per type rather than erased.

## Po polsku

Rust to język kompilowany, bez odśmiecania pamięci (*garbage collector*), którego kompilator dowodzi bezpieczeństwa pamięci i wątków dla każdego programu przyjętego poza blokiem `unsafe`. Najczęstsze nieporozumienie to utożsamienie „bezpieczny” z „nie może się wywrócić”: `panic!`, wyciek pamięci i zakleszczenie są w bezpiecznym Ruście jak najbardziej możliwe — wykluczone są za to błędy użycia po zwolnieniu, podwójnego zwolnienia i wyścigi danych.

**Szukaj po polsku:** co to jest Rust · bezpieczeństwo pamięci · edycje Rusta · `what is rust programming language` · `rust memory safety guarantees`
