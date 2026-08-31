# `if` expressions

**Level:** 101 · for newcomers

**One line:** `if` in Rust is used exactly like `if` in every other language — and then it does one more thing, which is **have a value**, so `let size = if n < 10 { "small" } else { "large" };` needs no ternary operator and no assignment in each branch.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The statement form first, since it is the familiar one: no parentheses around the condition, and braces are not optional
- The condition must be a `bool`. `if n` where `n` is an integer does not compile, and there is no truthiness anywhere in the language
- `if` as an expression: the last expression of the taken branch becomes the value of the whole thing
- **Both branches must have the same type** — which is where the beginner's `E0308` comes from, and the error is usually one stray `;`
- No `else` means the type must be `()`, so `let x = if c { 1 };` is an error with a *different* message than the mismatched-branch one
- The `;` at the end of `let x = if … { … };` — it is a `let` statement, and the statement still needs terminating
- When to reach for `match` instead: more than two cases, or a case analysis on a value rather than on a condition

## The trap it exists for

Adding `;` after `"small"` in a branch changes that branch's value from `&str` to `()`, so the two branches now disagree and the error is reported at the `if`, not at the semicolon. This is the same trap [a block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) describes, met a second time in the shape most people meet it first.

## See also

- [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) — why an `if` branch has a value at all
- [`match` expressions](../match_expressions/README.md) — the same idea for more than two cases, and the exhaustiveness the compiler checks
- [`if let`](../../17_Option_and_Result/if_let/README.md) — `if` against a *pattern*, once `Option` is in play
- [`if` expressions ↗](https://doc.rust-lang.org/reference/expressions/if-expr.html) · [Comprehensive Rust: `if` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/if.html)

## Po polsku

Warunek nie stoi tu w nawiasach, za to klamry są obowiązkowe nawet dla jednej instrukcji — co z definicji likwiduje klasyczny błąd z dopisaniem drugiej linii pod `if`-em bez klamer. Warunek musi być typu `bool`: `if n` dla liczby całkowitej się nie skompiluje, bo w Ruscie nigdzie nie ma „prawdziwościowości” (*truthiness*) znanej z Pythona czy C — zero nie jest fałszem, a pusty łańcuch znaków niczym szczególnym. Do tego `if` jest wyrażeniem i ma wartość, więc zamiast operatora warunkowego `?:` i zamiast przypisania w każdej gałęzi pisze się `let size = if n < 10 { "small" } else { "large" };` — pod jednym warunkiem: **obie gałęzie muszą mieć ten sam typ**. Stąd typowy `E0308` początkującego, którego zwykłą przyczyną jest jeden zabłąkany średnik, bo `"small";` zmienia wartość gałęzi z `&str` na `()`, a błąd zgłaszany jest przy `if`, nie przy średniku; uwaga — średnik kończący całe `let x = if … { … };` to zupełnie inny średnik i on ma tam być.

**Szukaj po polsku:** instrukcja warunkowa a wyrażenie · warunek musi być typu `bool` · `rust if expression both branches same type` · `rust E0308 if and else have incompatible types`
