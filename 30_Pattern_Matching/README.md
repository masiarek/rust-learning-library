# Pattern matching

**One line:** A pattern is not a comparison — it is a **shape**, matched against a value, that pulls the pieces out by name as it goes. `match`, `if let`, `while let`, `let else`, every `let`, and every function parameter are all the same feature, and this section is that feature rather than the six syntaxes it wears.

The reason it is a section and not a page: patterns are taught six times over in most courses, once per keyword, and the thing being taught is the same each time. Learn `Some(n) => …` and you have also learned `let Some(n) = … else { return; }`, `if let Some(n) = …`, `while let Some(n) = …`, and `fn f((a, b): (i32, i32))`. What differs between them is only what happens when the shape does *not* fit.

| Lesson | Level | What it covers |
|---|---|---|
| [Irrefutable patterns](irrefutable_patterns/README.md) | 101 → 201 | Every `let` is already a pattern — and the reason `let Some(x) = opt;` is an error while `let (a, b) = pair;` is not |
| [The wildcard `_`](the_wildcard/README.md) | 201 | A pattern that binds **nothing** — so it moves nothing, and the same two lines release a lock immediately or not at all depending on whether the right-hand side was a temporary |
| [Destructuring structs](destructuring_structs/README.md) | 101 → 201 | Taking a struct apart by field name, `..` for the rest, and the shorthand that binds a field to its own name |
| [Destructuring enums](destructuring_enums/README.md) | 101 → 201 | The pattern each variant shape needs, and the exhaustiveness that makes adding a variant a compile error rather than a bug |
| [Match guards](match_guards/README.md) | 201 | `if` on an arm — a condition the pattern alone cannot express, and the reason a guard does **not** count towards exhaustiveness |
| [`let else`](let_else/README.md) | 201 | Bind or diverge: the shape that keeps the happy path unindented, and the one rule (`else` must not fall through) |
| [Binding with `@`](binding_at/README.md) | 201 → 301 | Testing a value *and* keeping it — `n @ 1..=9` — and the two other places a name can appear in a pattern |

**[The wildcard `_`](the_wildcard/README.md) is the only page here with a program behind it; the rest are stubs** — outlines with their boundaries and their trap written down, no runnable example yet. [CONTRIBUTING.md](../CONTRIBUTING.md) says what graduating one takes.

## Already taught elsewhere

Three pieces of this topic have lived on other pages since long before this section existed, and they stay there — they were written from the `Option` side, which is where a reader meets them:

- [`if let`](../17_Option_and_Result/if_let/README.md) — one pattern, no exhaustiveness, and the arm you deleted
- [`while let`](../17_Option_and_Result/while_let/README.md) — the loop that ends when the pattern stops fitting
- [`match` expressions](../25_Control_Flow/match_expressions/README.md) — the keyword itself: arm order, no fall-through, and `_`

And two pages are about what happens when a pattern is subtly wrong rather than about patterns as such:

- [A typo becomes a binding](../13_Enums/a_typo_becomes_a_binding/README.md) — a lowercase name in an arm is not a constant being compared, it is a **new binding** that matches everything. The single most expensive pattern-matching mistake in Rust, and it warns about almost nothing
- [One arm, many values](../17_Option_and_Result/one_arm_many_values/README.md) — `|` and ranges, collapsing twenty-six arms into five

## Planned

- **Exercise: expression evaluation** — a small `enum Expression` tree, evaluated by one recursive `match`. It is the exercise that makes the case for the whole section, because the recursion is a page long and contains no `if` at all
- **Slice and array patterns** — `[first, .., last]`, and the `rest @ ..` binding
- **Reference patterns and `ref`** — matching through a `&`, what binding modes do for you now, and why old code says `ref x`

## Where it goes next

Patterns pay off most on a type with a fixed set of shapes, which is [enums](../13_Enums/README.md) — and the two enums every Rust program already uses are [`Option` and `Result`](../17_Option_and_Result/README.md).

## Po polsku

Polska nazwa — **dopasowanie wzorców** (*pattern matching*) — sugeruje porównywanie i to jest pierwsza rzecz do przestawienia. Wzorzec niczego nie porównuje: opisuje **kształt**, a przy okazji sprawdzania rozbiera wartość na części i wiąże je pod nazwami. Dlatego znak `=` w `let Some(n) = …` nie jest przypisaniem, tylko pytaniem „czy ta wartość ma taki kształt?”, a `Some(n)` po lewej stronie nie jest wywołaniem funkcji, mimo że wygląda identycznie jak wywołanie o linijkę wyżej. Kto czyta wzorzec jak wyrażenie, potknie się na każdej stronie tej sekcji.

Druga rzecz to układ samej sekcji, bo różni się od tego, do czego przyzwyczajają kursy. Materiały — polskie i nie tylko — uczą wzorców sześć razy: osobno przy `match`u, osobno przy `if let`, osobno przy `while let`, osobno przy `let … else`, a o tym, że zwykły `let` i **parametr funkcji** też są wzorcami, zwykle nie mówią wcale. To jedna funkcja języka w sześciu składniach, a różni je wyłącznie to, co się dzieje, **gdy kształt nie pasuje**: `match` żąda pokrycia wszystkich przypadków, `if let` i `while let` po prostu nic nie robią, `let … else` musi przerwać przepływ sterowania, a zwykłemu `let` nie wolno nie pasować. Kto rozumie `Some(n) => …`, zna już pozostałe pięć zapisów — i to jest cała teza tej sekcji.

Na koniec uwaga nawigacyjna, bo rozdział jest celowo niekompletny: strony w tabelce powyżej są na razie szkicami (mają wyznaczone granice i opisaną pułapkę, ale nie mają jeszcze uruchamialnego przykładu), a trzy najczęściej używane kawałki tematu mieszkają w ogóle poza tą sekcją — `if let`, `while let` i sam `match` opisane są od strony `Option`, bo tam spotyka się je pierwszego dnia. Jedną rzecz warto jednak wiedzieć, zanim przeczyta się cokolwiek innego: nazwa wpisana w ramię małą literą nie jest porównaniem z wariantem, tylko **nowym wiązaniem**, które pasuje do wszystkiego — i kompilator nie powie o tym prawie nic.

**Szukaj po polsku:** dopasowanie wzorców · destrukturyzacja · wiązanie we wzorcu · `rust all the places patterns can be used` · `rust refutable irrefutable patterns`
