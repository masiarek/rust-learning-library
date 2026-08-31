# Destructuring structs

**Level:** 101 → 201 · for newcomers

**One line:** A struct pattern names the fields you want and binds each to a variable — `let Point { x, y } = p;` — so the field name and the binding name are the same word unless you say otherwise, and `..` says *and I do not care about the rest*.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `let Point { x, y } = p;` — the shorthand, where `x` means `x: x`
- Renaming: `let Point { x: left, y: top } = p;`, and when that reads better than the shorthand
- `..` for the fields you are not naming, and why omitting it is an error rather than an implicit "ignore the rest"
- Nested patterns: a struct inside a struct inside a tuple, matched in one line
- Matching a **literal** in a field position — `Point { x: 0, y }` — which is where a struct pattern stops being irrefutable
- Tuple structs and unit structs take the same shapes: `Score(n)`, `Marker`
- Destructuring in a **function parameter**, which is where it quietly earns the most
- What it does to ownership: a struct pattern **moves** each bound field unless the field is `Copy` or you match on a reference

## The trap it exists for

Destructuring a struct that owns a `String` moves that field out and leaves the whole struct unusable — the error lands on the *next* use of the struct, not on the pattern. Matching `&p` (or binding with `ref`) is the fix, and the reason binding modes exist.

## See also

- [What a struct is](../../16_Structs/README.md) — the type being taken apart
- [Destructuring enums](../destructuring_enums/README.md) — the same syntax where it can fail
- [Irrefutable patterns](../irrefutable_patterns/README.md) — why a field literal changes where this pattern is allowed
- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — what a pattern moves
- [Struct update syntax](../../16_Structs/struct_update/README.md) — `..base`, which looks like the `..` here and does the opposite job
- [Comprehensive Rust: Destructuring Structs ↗](https://google.github.io/comprehensive-rust/pattern-matching/destructuring-structs.html)

## Po polsku

Wzorzec struktury wymienia pola, które chcesz wyjąć, i wiąże każde ze zmienną: `let Point { x, y } = p;`. Skrót `x` znaczy tu `x: x` — nazwa pola i nazwa zmiennej to jedno słowo, dopóki nie powiesz inaczej przez `x: lewo`. Dwukropek czyta się więc odwrotnie niż w przypisaniu: po lewej stoi pole struktury, po prawej nowa nazwa.

`..` mówi „reszta mnie nie obchodzi". Pominięcie go nie jest domyślnym zignorowaniem reszty, tylko błędem — Rust każe powiedzieć to wprost, żeby dopisanie pola do struktury zwróciło uwagę tam, gdzie ktoś rozbiera ją na części. Uwaga na podobieństwo, które myli: `..base` w składni aktualizacji struktury wygląda tak samo, a robi zadanie odwrotne — tam **dostarcza** brakujące pola, tutaj je **przemilcza**.

Najwięcej ta składnia zarabia w **parametrze funkcji**, i tam też czai się kłopot z własnością: wzorzec struktury **przenosi** każde związane pole, chyba że pole jest `Copy` albo dopasowujesz referencję. Struktura trzymająca `String` po takim rozbiorze przestaje nadawać się do użytku — a błąd wypada przy *następnym* jej użyciu, nie na samym wzorcu, więc szuka się przyczyny w złym miejscu. Lekarstwem jest dopasowanie `&p` albo wiązanie przez `ref`.

**Szukaj po polsku:** destrukturyzacja struktur · wzorce w Ruscie · przenoszenie pól struktury · `rust destructuring struct` · `rust partial move`
