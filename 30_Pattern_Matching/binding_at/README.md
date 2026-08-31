# Binding with `@`

**Level:** 201 → 301 · deep dive

**One line:** `n @ 1..=9` tests the value **and** keeps it — the pattern on the right decides whether the arm fires, the name on the left binds what matched, and without the `@` you have to choose between the two.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The problem it solves: `1..=9 => …` matched but has no name for the number; `n => …` has the name but tests nothing. `n @ 1..=9` is both
- Reading it aloud — *"call it `n`, and it must look like this"* — which is the whole syntax
- On a nested pattern: `msg @ Message::Resize { .. }`, keeping the whole value while testing its shape
- Its interaction with ownership: `@` binds by move unless the inner pattern is `Copy` or you are matching a reference — the same rule as every other binding
- `@` with `|` inside it (`n @ (1 | 9)`), stabilised later than the rest
- The other two places a name shows up in a pattern, so the three are not confused: a plain binding, and the field shorthand in a struct pattern

## The trap it exists for

Reaching for a **guard** where `@` is what you wanted — `n if (1..=9).contains(&n) => …` does the same job, opts the arm out of exhaustiveness checking, and reads worse. If the condition is expressible as a pattern, make it a pattern.

## See also

- [Match guards](../match_guards/README.md) — the tool this one frequently replaces, and what it costs
- [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) — the ranges and `|` that go on the right of the `@`
- [Destructuring enums](../destructuring_enums/README.md) — where `msg @ Variant { .. }` earns its keep
- [`@` bindings ↗](https://doc.rust-lang.org/reference/patterns.html#binding-modes) · [Comprehensive Rust: Pattern Matching ↗](https://google.github.io/comprehensive-rust/pattern-matching.html)

## Po polsku

Znak `@` rozwiązuje wybór, przed którym stawia zwykły wzorzec: `1..=9` sprawdza wartość, ale nie zostawia dla niej nazwy, a samo `n` nazywa, lecz niczego nie sprawdza — `n @ 1..=9` robi jedno i drugie, i czyta się to „nazwij to `n`, ale tylko jeśli ma taki kształt”. Tak samo działa na całym wariancie: `msg @ Message::Resize { .. }` bada kształt, a zachowuje wartość w komplecie. Nawyk wart wyrobienia jest jednak inny niż sama składnia: kiedy pojawia się ochota na strażnika (*guard*) w rodzaju `n if (1..=9).contains(&n)`, to prawie zawsze znak, że chodziło o `@` — strażnik robi to samo, czyta się gorzej, a przy okazji wypisuje ramię z kontroli kompletności dopasowania. Jeśli warunek da się wyrazić wzorcem, wyraź go wzorcem.

**Szukaj po polsku:** wiązanie we wzorcu · dopasowanie wzorców · `rust @ bindings pattern` · `rust bind and test value in match`
