# Compiled, interpreted, or something between

**Level:** 101 · newcomer

**One line:** "Compiled language" is a category error — compilation is something an *implementation* does, at a moment of its choosing, and the interesting question is not whether it happens but **when**, and what is still around to help when it does.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The three moments: ahead of time (rustc, Clang), at first use (JVM's JIT, ART, V8), and never — a pure interpreter walking a tree
- CPython as the honest middle: it *does* compile, to bytecode, cached in `__pycache__`, and then interprets that. "Interpreted language" describes neither half well
- The JVM and ART: `javac` produces bytecode for a machine that does not exist, and the real machine's code is produced later — by a JIT at run time, or ahead of time at install
- What a **runtime** is, and what Rust having none means in practice: no VM to start, no garbage collector thread, no warm-up, and no ability to inspect a running program the way a VM can
- What each choice buys and costs, in one table: startup, peak speed, portability of the artifact, size, and how much the shipped thing reveals
- Where this shows up for a reverse engineer: bytecode decompiles nearly back to source, native code does not — which is why VM-based obfuscation exists and what it is imitating

## The trap it exists for

The claim "compiled languages are fast" attributes to the language what belongs to the moment of compilation and the quality of the optimizer. A JIT can beat an ahead-of-time compiler on the same source, because it knows which branch actually ran.

## See also

- [What a compiler does before your program runs](../what_a_compiler_does/README.md) — the ahead-of-time end of this spectrum, with the compile-time/run-time line drawn
- [Control-flow flattening](../control_flow_flattening/README.md) — what "the shipped thing reveals" is worth, and the passes that attack it
- [What the optimizer does](../what_the_optimizer_does/README.md) — the optimization that a JIT gets to do later, with more information

## Po polsku

Polska szkoła i większość kursów dzielą języki na „kompilowane” i „interpretowane” — i właśnie ten podział jest tu błędem kategorii, bo kompiluje **implementacja**, w wybranym przez siebie momencie, a nie język. Momenty są trzy: z wyprzedzeniem (`rustc`, `clang`), przy pierwszym użyciu (JIT w JVM, ART, V8) albo nigdy; CPython jest uczciwym środkiem, bo naprawdę kompiluje — do kodu bajtowego (*bytecode*), odkładanego w `__pycache__` — a dopiero potem go interpretuje, więc etykieta „język interpretowany” nie opisuje dobrze żadnej z tych połówek. O Ruscie mówi się natomiast, że nie ma środowiska uruchomieniowego (*runtime*): żadnej maszyny wirtualnej do wystartowania, wątku odśmiecacza ani rozgrzewki — ale to działa w obie strony, bo nie ma też czym zajrzeć do działającego programu tak, jak potrafi maszyna wirtualna. Ostrożnie więc ze zdaniem „języki kompilowane są szybkie”: przypisuje ono językowi to, co należy do momentu kompilacji i do jakości optymalizatora, a JIT potrafi wygrać z kompilacją z wyprzedzeniem właśnie dlatego, że wie, która gałąź naprawdę się wykonała.

**Szukaj po polsku:** języki kompilowane a interpretowane · kod bajtowy · środowisko uruchomieniowe · `rust has no runtime` · `JIT vs AOT compilation`
