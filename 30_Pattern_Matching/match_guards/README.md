# Match guards

**Level:** 201 · working knowledge

**One line:** A guard is an `if` bolted onto an arm — `Some(n) if n > 100 => …` — for the conditions a pattern cannot express, and it comes with one consequence worth knowing before you use it: the compiler **does not count a guarded arm towards exhaustiveness**.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The syntax, and that the guard runs **after** the pattern binds — so it can use the bindings the pattern just made
- Why a guard is needed at all: a pattern can test *shape* and *literal value*, but not a relation between two bindings, or a call
- Exhaustiveness: `Some(n) if n > 0` plus `None` is **not** exhaustive, and the error is right. The compiler cannot evaluate your condition, so it must assume the arm can decline
- The interaction with `|`: a guard applies to the **whole** alternation, not just the last branch — a rule that looks arbitrary until you have been bitten
- Guards and side effects: legal, and almost always wrong, because arm order then decides how many times your condition runs
- When a range pattern (`1..=9`) or a nested pattern says it better, which is often

## The trap it exists for

Splitting one case into a guarded arm plus a catch-all silently converts a `match` that the compiler was checking into one that it is not — you keep the shape of exhaustive matching and lose the guarantee. The tell is a `_ => unreachable!()` at the bottom: if the compiler could still prove it, you would not have needed to write it.

## See also

- [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) — ranges and `|`, which replace a guard more often than people expect
- [Destructuring enums](../destructuring_enums/README.md) — the exhaustiveness a guard opts out of
- [Zero wins is not zero games](../../17_Option_and_Result/wrong_guard/README.md) — a guard that looks careful and lets the wrong case through
- [Binding with `@`](../binding_at/README.md) — testing a value and keeping it, which is sometimes the guard you were reaching for
- [Match guards ↗](https://doc.rust-lang.org/reference/expressions/match-expr.html#match-guards) · [Comprehensive Rust: `match` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/match.html)

## Po polsku

Strażnik (*guard*) to `if` doczepiony do ramienia: `Some(n) if n > 100 => …`. Uruchamia się **po** związaniu wzorca, więc może korzystać z nazw, które wzorzec dopiero co utworzył — i po to istnieje, bo wzorzec potrafi sprawdzić kształt i literał, ale nie relację między dwiema nazwami ani wynik wywołania.

Cena jest jedna i lepiej poznać ją wcześniej niż później: **kompilator nie zalicza ramienia ze strażnikiem do kompletności dopasowania.** `Some(n) if n > 0` razem z `None` nie jest wyczerpujące i komunikat ma rację — kompilator nie umie policzyć twojego warunku, więc musi założyć, że to ramię może odmówić. W praktyce znaczy to, że rozbicie jednego przypadku na ramię ze strażnikiem plus łapacz `_` po cichu zamienia `match`, który był sprawdzany, w taki, który już nie jest. Sygnałem ostrzegawczym jest `_ => unreachable!()` na dole: gdyby kompilator wciąż potrafił to udowodnić, nie trzeba by tego pisać.

Dwie rzeczy na koniec. Strażnik obejmuje **całą** alternatywę po `|`, a nie tylko jej ostatnią gałąź — reguła wygląda na arbitralną dokładnie do chwili, w której się na niej przewrócisz. I bardzo często to, o co naprawdę chodziło, lepiej wyraża wzorzec zakresowy (`1..=9`) albo wzorzec zagnieżdżony, bez oddawania kompletności w zamian.

**Szukaj po polsku:** strażnik w `match` · kompletność dopasowania · wzorce zakresowe · `rust match guard exhaustiveness`
