# `let else`

**Level:** 201 · working knowledge

**One line:** `let Some(n) = opt else { return; };` binds `n` for the **rest of the enclosing block** or leaves — so the happy path stays at one indent level and the failure is handled once, at the top, instead of wrapping everything below it in an `if let`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The shape, and the one hard rule: the `else` block must **diverge** — `return`, `break`, `continue`, `panic!`, or any other `!`-typed expression. It cannot fall through, because there would be nothing for `n` to be
- Why that is the whole feature: the binding escapes the block, which is exactly what `if let` cannot do
- The before-and-after that motivates it — a function with three fallible steps, once as nested `if let`s and once flat
- Where it is *not* the right answer: when the failure has a value to supply (`unwrap_or`), or when the error should propagate with context (`?`)
- Its relationship to `?`: same job, different situations — `?` needs a `Result`-shaped return type, `let else` works anywhere and can log
- Stabilised in Rust 1.65, so it is absent from older code and from most tutorials

## The trap it exists for

`let Some(n) = opt else { 0 };` looks reasonable and does not compile: an `else` that produces a value is exactly what this construct cannot do. The error names the `!` type, which is unhelpful the first time. What you wanted was `unwrap_or(0)`.

## See also

- [Irrefutable patterns](../irrefutable_patterns/README.md) — the `E0005` this construct exists to answer
- [`if let`](../../17_Option_and_Result/if_let/README.md) — the same test, when there is real work in the other branch
- [`unwrap_or`](../../17_Option_and_Result/unwrap_or/README.md) — when the failure has a value rather than an exit
- [Keep going, or stop](../../02_Errors/keep_going_or_stop/README.md) — the question `let else` is answering per call site
- [`let else` ↗](https://doc.rust-lang.org/reference/statements.html#let-statements) · [Comprehensive Rust: `let else` ↗](https://google.github.io/comprehensive-rust/pattern-matching/let-control-flow/let-else.html)

## Po polsku

`let ... else` wiąże zmienną **na resztę otaczającego bloku** albo wychodzi. Dzięki temu ścieżka powodzenia zostaje na jednym poziomie wcięcia, a niepowodzenie obsługuje się raz, na górze, zamiast zawijać cały dalszy kod w `if let`.

Jedna twarda reguła: blok `else` musi **przerwać wykonanie** — `return`, `break`, `continue`, `panic!` albo cokolwiek innego o typie `!`. Nie wolno mu przelecieć dalej, bo nie byłoby wtedy czym uczynić związanej nazwy. To nie kaprys składni, tylko cała istota konstrukcji: wiązanie **ucieka** poza blok, czego `if let` zrobić nie potrafi.

Pułapka wygląda niewinnie: `let Some(n) = opt else { 0 };` czyta się rozsądnie i nie kompiluje — `else`, który produkuje wartość, jest dokładnie tym, czego ta konstrukcja nie umie. Komunikat mówi o typie `!`, co za pierwszym razem niczego nie wyjaśnia; chodziło o `unwrap_or(0)`. Warto też wiedzieć, że `let else` ustabilizowano dopiero w Ruscie 1.65 — w starszym kodzie i w większości polskich materiałów po prostu go nie ma, więc jego brak w tutorialu nie znaczy, że jest niezalecany.

**Szukaj po polsku:** `let else` Rust · typ nigdy · wczesny powrót · `rust let else diverge` · `rust never type`
