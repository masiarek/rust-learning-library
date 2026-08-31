# Destructuring enums

**Level:** 101 → 201 · for newcomers

**One line:** Each variant has a shape, and the pattern for it is that shape with names in the holes — which is why `match` on an enum is the one place the compiler can tell you that you forgot a case.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- One pattern per variant shape: unit (`Stop`), tuple (`Move(x, y)`), struct-like (`Resize { w, h }`)
- Exhaustiveness, and `E0004` printing the variant you missed — the property that makes an enum plus a `match` a design tool rather than a switch statement
- Why `_` is a liability here, and the alternative when you genuinely need one: `#[non_exhaustive]`, or matching the variants you handle and letting the compiler come find you
- Nesting: `Some(Ok(n))`, and how far to take it before a nested `match` reads better
- The same ownership rule as structs — the bound fields move unless you match a reference
- Where `Option` and `Result` fit: they are ordinary enums, so everything here is what `if let Some(n)` was doing all along

## The trap it exists for

A lowercase name in an arm is not a constant being compared against — it is a **new binding**, and it matches everything, so the arms below it are dead. The compiler warns about the unreachable arm but not about the mistake, and the whole story is [a typo becomes a binding](../../13_Enums/a_typo_becomes_a_binding/README.md).

## See also

- [What an enum is](../../13_Enums/what_an_enum_is/README.md) · [Variants that carry data](../../13_Enums/variants_that_carry_data/README.md) — the type this page takes apart
- [A typo becomes a binding](../../13_Enums/a_typo_becomes_a_binding/README.md) — the trap above, in full, with the two lints that catch it
- [`Some` and `None`](../../17_Option_and_Result/some_and_none/README.md) — the enum you have been destructuring since page one
- [Six kinds of zero](../../17_Option_and_Result/six_kinds_of_zero/README.md) — exhaustiveness used as a domain model
- [`match` expressions](../../25_Control_Flow/match_expressions/README.md) — the keyword and its arm-order rules
- [Comprehensive Rust: Destructuring Enums ↗](https://google.github.io/comprehensive-rust/pattern-matching/destructuring-enums.html)

## Po polsku

Destrukturyzacja wyliczenia to zapisanie wzorca (*pattern*) w kształcie samego wariantu — `Stop`, `Move(x, y)`, `Resize { w, h }` — i wstawienie nazw w dziury. Nazwy po lewej nie odnoszą się do niczego, co już istnieje: to **nowe wiązania**, tworzone w chwili dopasowania.

Najcenniejsza jest tu jednak nie wygoda, tylko **kompletność dopasowania** (*exhaustiveness*): kompilator zna wszystkie warianty wyliczenia, więc potrafi wskazać ten, który pominąłeś, i robi to jako `E0004`, wymieniając brakujący wariant z nazwy. To właśnie zamienia parę „wyliczenie + `match`" w narzędzie projektowe, a nie w odpowiednik `switch`. Dlatego `_` na dole jest tu obciążeniem, a nie udogodnieniem: raz dopisany, na zawsze uciszy pytanie „a co z wariantem, który dojdzie za rok?".

Pułapka, przez którą przechodzi każdy: **nazwa pisana małą literą w ramieniu nie jest porównaniem ze stałą, tylko nowym wiązaniem** — a takie wiązanie pasuje do wszystkiego, więc ramiona pod nim są martwe. Kompilator ostrzeże o nieosiągalnym ramieniu, ale nie o przyczynie, bo z jego punktu widzenia napisałeś dokładnie to, co chciałeś.

**Szukaj po polsku:** dopasowanie wzorców · destrukturyzacja wyliczeń · kompletność dopasowania · `rust match exhaustiveness` · `E0004`
