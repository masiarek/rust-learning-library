# Control flow

**One line:** The `if`, `match` and three loops that every program is made of — familiar from every other language, with one difference running through all of them: in Rust these are **expressions**, so they have values.

That difference is the reason this section is not just a syntax reference. `let size = if n < 10 { "small" } else { "large" };` needs no ternary operator; `let x = loop { … break v; };` needs no result variable declared above the loop; a `match` arm's value is the `match`'s value. Once you have seen it three times it stops being a novelty and starts removing the `mut` bindings a C-shaped or Python-shaped version of the same function would have needed.

| Lesson | Level | What it covers |
|---|---|---|
| [`if` expressions](if_expressions/README.md) | 101 | No parentheses, no truthiness, both branches the same type — and the stray `;` that changes one of them |
| [`match` expressions](match_expressions/README.md) | 101 → 201 | First arm wins, no fall-through, and **exhaustiveness**: the compiler names the case you forgot |
| [`for` loops](for_loops/README.md) | 101 | Ranges and collections are the same mechanism, because `for` only knows about iterators — plus the `1..5` that stops at 4 |
| [`while` loops](while_loops/README.md) | 101 | The loop for when the count is unknown, and the `mut` it almost always needs |
| [`loop`](the_loop_keyword/README.md) | 101 → 201 | The only loop that can produce a value, via `break v` |
| [`break` and `continue`](break_and_continue/README.md) | 101 → 201 | Leaving early, and the label that leaves a *nested* loop without a flag variable |
| [Functions](functions/README.md) | 101 | Signatures are never inferred; the last expression is the return value |
| [Macros](macros/README.md) | 101 → 201 | What the `!` means, and the three things a macro can do that a function cannot |

**Every page in this section is currently a stub** — an outline with its boundaries and its trap written down, and no runnable example behind it yet. They graduate one at a time, and [CONTRIBUTING.md](../CONTRIBUTING.md) says what that takes.

## Blocks and scopes are already covered

The first slide of this topic in most courses is *blocks and scopes*, and this library teaches it in two places rather than repeating it here:

- [A block is an expression](../15_First_Programs/a_block_is_an_expression/README.md) — `{ }` does two jobs, and the second one is that it **has a value**: its last line without a semicolon. This is the rule everything above depends on, so read it first if you have not.
- [Variables](../15_First_Programs/variables/README.md#a-binding-is-scoped-to-its-block) — a binding is visible to the end of its block, and `E0425` when it is not.
- [Scope is about names, not values](../18_Ownership/scope_is_about_names/README.md) — what "goes out of scope" means once a value can be *moved* out of one, which is the version that matters later.

## Planned

- **Exercise: the Collatz sequence** — the length of the sequence that takes `n` to 1, halving evens and `3n + 1` on odds. It wants a `while` loop, a `match` or an `if` on the parity, and it overflows a `u32` for reasons that are not obvious from the problem statement, which makes it a good second meeting with [the width that runs out](../15_First_Programs/values/README.md#practice)
- **Exercise: the Collatz sequence** is the one still missing from this list; pattern matching proper now has [its own section](../30_Pattern_Matching/README.md) — bindings, tuples, guards, destructuring — which is where `match_expressions` hands off
- **`if let` and `let else` as control flow** — [`if let`](../17_Option_and_Result/if_let/README.md) and [`while let`](../17_Option_and_Result/while_let/README.md) exist already, taught from the `Option` side; the control-flow reading of them is a different page

## Where it goes next

Loops are the long way round to [iterators](../24_Iterators/README.md), which is where most of these `for` loops end up once the collection is real. And `match` is at its best on a type with a fixed set of cases, which is [enums](../13_Enums/README.md).

## Po polsku

W polskich kursach programowania `if` bywa nazywany **instrukcją** warunkową, a `switch` — instrukcją wyboru. Pierwsza rzecz do przestawienia w Ruscie leży dokładnie tutaj: `if`, `match` i `loop` są **wyrażeniami**, czyli mają wartość. Dlatego nie istnieje operator warunkowy `?:` — nie jest do niczego potrzebny, skoro można napisać `let size = if n < 10 { "small" } else { "large" };`. Z tego samego powodu nie trzeba deklarować zmiennej przed pętlą tylko po to, żeby wynieść z niej wynik: `let x = loop { … break v; };` załatwia sprawę. Praktyczny efekt jest taki, że znika sporo `mut`-ów, które wersja pisana odruchem z C albo z Pythona musiałaby wprowadzić.

Pod spodem pracuje jedna reguła — blok wyrażeniowy: wartością bloku `{ }` jest jego ostatnia linia **bez średnika**. Stąd bierze się najczęstsza wpadka początkującego: dopisany średnik zamienia wartość w `()`, a kompilator melduje `E0308` i typ `()` w miejscu, w którym spodziewaliśmy się liczby. Ta biblioteka uczy tej reguły osobno, przy pierwszych programach, i naprawdę warto ją mieć przerobioną przed czytaniem tej sekcji, bo wszystko powyżej z niej wynika.

Trzy pętle dzielą się prosto. `for` chodzi wyłącznie po iteratorze, dlatego zakres i kolekcja to dla niego ten sam mechanizm (a `1..5` kończy się na 4, bo prawy koniec jest wyłączony). `while` jest na sytuację, w której liczby powtórzeń nie znamy z góry. `loop` to jedyna pętla potrafiąca zwrócić wartość, przez `break v`. Z pętli zagnieżdżonej wychodzi się etykietą (`break 'outer`), co zastępuje znaną z C sztuczkę ze zmienną-flagą. `match` natomiast pokazuje pełnię swoich możliwości dopiero na wyliczeniu (*enum*): kompilator sprawdza wtedy kompletność dopasowania i sam wymienia przypadek, o którym zapomnieliśmy.

**Szukaj po polsku:** kontrola przepływu sterowania · wyrażenie a instrukcja · blok wyrażeniowy · `rust if is an expression` · `rust loop break with value`
