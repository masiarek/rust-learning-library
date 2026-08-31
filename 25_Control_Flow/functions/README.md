# Functions

**Level:** 101 · for newcomers

**One line:** `fn name(param: Type) -> Type { }` — every parameter and the return type are written out, always, and the body's **last expression without a semicolon** is what comes back, so most Rust functions contain no `return` at all.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The signature is never inferred, and why that is a feature rather than a gap — [type inference](../../15_First_Programs/type_inference/README.md) stops at the boundary on purpose
- No return type written means `-> ()`, and a function that ends in `;` returns `()` whether you meant it to or not
- `return` for early exit; the trailing expression for the normal one. Both are correct, and mixing them badly is what makes an `E0308` confusing
- Order does not matter: a function may call one defined below it, unlike a C file without a prototype
- No overloading, no default arguments, no named arguments — and the four things Rust does instead ([optional function arguments](../../17_Option_and_Result/optional_arguments/README.md) covers one of them)
- Where methods differ: `fn` inside an `impl` block, and the `self` receiver

## The trap it exists for

One extra semicolon on the last line changes the return type to `()`, and the error is reported against the function's declared return type — so the message names the signature you got right and not the line you got wrong. [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) is the same trap at block scale.

## See also

- [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) — why the last line is the return value
- [Type inference](../../15_First_Programs/type_inference/README.md) — solved inside a body, declared at the boundary
- [`impl` blocks](../../16_Structs/impl_blocks/README.md) — the same `fn`, with a receiver
- [What a closure is](../../23_Closures/what_a_closure_is/README.md) — the anonymous version, and what it can capture that a `fn` cannot
- [Optional function arguments](../../17_Option_and_Result/optional_arguments/README.md) — what to do without default arguments
- [Comprehensive Rust: Functions ↗](https://google.github.io/comprehensive-rust/control-flow-basics/functions.html)

## Po polsku

Sygnatura funkcji nigdy nie jest wnioskowana: każdy parametr i typ wyniku wypisuje się jawnie. To nie luka we wnioskowaniu typów, tylko decyzja projektowa — wnioskowanie zatrzymuje się na granicy funkcji, żeby zmiana w ciele nie przestawiła po cichu jej interfejsu. Wynikiem jest ostatnie wyrażenie **bez średnika**, dlatego większość funkcji w Ruscie nie zawiera w ogóle słowa `return`; zostaje ono do wcześniejszego wyjścia z funkcji. Stąd też pułapka, którą warto znać, zanim się w nią wdepnie: jeden dopisany średnik zmienia typ wyniku na `()` (typ jednostkowy, *unit*), a `E0308` zgłaszany jest wobec typu zadeklarowanego w nagłówku — komunikat wskaże więc linię napisaną poprawnie, a nie tę zepsutą. Dwie rzeczy, o które najczęściej pytają osoby przychodzące z C++ albo z Javy: kolejność definicji nie ma znaczenia (funkcja może wołać tę zdefiniowaną niżej, żadnych prototypów), a przeciążania funkcji, argumentów domyślnych ani nazwanych po prostu nie ma — ich rolę przejmują `Option`, cechy (*traits*) i osobne, mówiące nazwy funkcji.

**Szukaj po polsku:** sygnatura funkcji · wartość zwracana bez `return` · brak przeciążania funkcji w Ruscie · `rust missing return value semicolon E0308` · `rust default arguments alternative`
