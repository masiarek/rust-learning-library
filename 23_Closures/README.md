# Closures

**One line:** A closure is a function that can also see the variables around where it was written — so the compiler writes you a struct holding them, and the three questions that follow are what it captured, how, and how many times you may call it.

You have already written several. `unwrap_or_else(|| 0)`, `map(|s| s * 2)`, `sort_by_key(|r| r.len())` — all closures, all passed to a function that will call them, and none of the pages that use them says which of the three `Fn` traits they got or why the compiler asked.

The thing that makes Rust's version its own is that there is nothing to allocate. A closure is a struct sized to exactly what it captured — often **zero bytes** — and a call to it is a direct call. There is no garbage collector deciding when the captured `String` dies, which is the reason this language needs a `move` keyword at all, and the reason `Fn`, `FnMut` and `FnOnce` are three traits rather than one.

| Lesson | Level | What it teaches |
|---|---|---|
| [What a closure is](what_a_closure_is/README.md) | 101 → 201 | The two-pipe syntax, the capture that separates a closure from a `fn`, and the measured claim that its size is exactly what it captured — zero bytes for one that captured nothing, which is smaller than a `fn` pointer |
| [The three closure traits](three_closure_traits/README.md) | 201 | `Fn` / `FnMut` / `FnOnce` as a ladder rather than a menu; which bound to write; and the widely-repeated sentence about `move` and `FnOnce` that the run refutes in both directions |
| [The `move` keyword](the_move_keyword/README.md) | 201 | What it moves, the two errors that demand it, the `Copy` case that silently copies instead — and the field-granularity capture that edition 2021 introduced, measured in bytes |
| [Function pointers](function_pointers/README.md) | 201 | `fn` as a *type*: eight bytes pointing at code, the zero-sized `fn` item that is not the same thing, and why a bare `fn` parameter refuses callers an `Fn` bound would take |

Read them in that order: the declaration, then the classification the compiler cares about, then the keyword that gets blamed for that classification and does not decide it — and last the thing a closure is measured against, the plain `fn` pointer that carries nothing.

## The other half: iterators

[`24_Iterators/`](../24_Iterators/README.md) is where these get spent. Every adapter takes a closure, and the bound it takes tells you what it is allowed to do with your closure — `map` takes an `FnMut` so it may carry a running total, `unwrap_or_else` takes an `FnOnce` so its fallback may be an owned value moved out of the closure. The two sections were one until the iterator half outgrew it; if you are reading rather than looking something up, these three pages come first.

## Where the neighbouring sections take over

- **[Ownership](../18_Ownership/README.md)** — capturing a value is a move or a borrow, so most closure errors are ownership errors in a costume. [Ownership and moves](../18_Ownership/ownership_and_moves/README.md) and [borrowing](../18_Ownership/borrowing/README.md) are the rules; [how to learn lifetimes](../18_Ownership/how_to_learn_lifetimes/README.md) is the `'static` on `thread::spawn`.
- **[Traits](../12_Traits/README.md)** — `Fn`, `FnMut` and `FnOnce` are ordinary traits with nothing built into the language, and their ladder is [supertraits](../12_Traits/supertraits/README.md). [Returning a trait](../12_Traits/returning_a_trait/README.md) is the `impl Fn` / `Box<dyn Fn>` decision.
- **[Generics](../22_Generics/README.md)** — `fn apply<F: Fn(i32) -> i32>(f: F)` is a generic function with a trait bound, stamped out once per closure type.
- **[`Option` and `Result`](../17_Option_and_Result/README.md)** — [`unwrap_or_else`](../17_Option_and_Result/unwrap_or_else/README.md) and [`map_or`](../17_Option_and_Result/map_or/README.md) are where most people meet their first closure, several sections before this one.

## Not yet written

The gaps, listed rather than stubbed so they are visible: **closures that return closures** (and the `impl Fn` / `Box<dyn Fn>` decision that forces), **capturing in a struct field** — the lifetime parameter that arrives with it, and why `Box<dyn Fn>` is the usual escape — and **closures and `async`**, where the captured environment outlives the call by construction.

## Po polsku

Domknięcie (*closure*) to funkcja, która widzi zmienne z miejsca, w którym ją napisano. Samo słowo bywa mylące, bo w polskiej matematyce „domknięcie” znaczy coś zupełnie innego (domknięcie zbioru), więc w tekstach programistycznych spotkasz też „funkcję anonimową” albo po prostu *lambdę*. W kodzie i w komunikatach `rustc` jest zawsze `closure` — i to tego słowa szuka się w wyszukiwarce.

Kto przychodzi z Pythona albo JavaScriptu, ma już gotową intuicję domknięcia — i właśnie ona jest tutaj pułapką. Tam przechwycone zmienne żyją tak długo, jak długo żyje domknięcie, bo pilnuje tego odśmiecacz (*garbage collector*). W Ruscie odśmiecacza nie ma, więc kompilator generuje dla każdego domknięcia osobną **strukturę** przechowującą dokładnie to, co zostało przechwycone — często o rozmiarze **zera bajtów** — a wywołanie jest zwykłym, bezpośrednim wywołaniem, bez żadnej alokacji na stercie. Stąd biorą się trzy rzeczy, których w tamtych językach nie ma: słowo kluczowe `move`, trzy cechy (*traits*) `Fn` / `FnMut` / `FnOnce` oraz pytanie, ile razy wolno domknięcie wywołać.

Kolejność lekcji jest celowa: najpierw składnia z dwiema pionowymi kreskami i samo przechwytywanie, potem klasyfikacja, na którą patrzy kompilator, potem `move` — któremu tę klasyfikację się przypisuje, choć jej nie ustala — a na końcu goły wskaźnik na funkcję (`fn`), czyli to, z czym domknięcie się mierzy. Praktyczna wskazówka na start: **większość błędów z domknięciami to błędy własności w przebraniu**. Jeśli kompilator marudzi o przeniesieniu własności albo o pożyczaniu wewnątrz `|…|`, wracaj do rozdziału o własności, a nie szukaj czegoś specyficznego dla domknięć.

**Szukaj po polsku:** domknięcia w Ruscie · funkcje anonimowe · przechwytywanie zmiennych · `rust closure captures environment` · `rust Fn FnMut FnOnce`
