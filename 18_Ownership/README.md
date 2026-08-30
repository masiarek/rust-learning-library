# Ownership

**One line:** Every value has exactly one owner, moving it transfers responsibility rather than bytes, and a borrow lets somebody read it without taking that responsibility on — which together are what let Rust free memory with no garbage collector and no `free()` you can get wrong.

Three rules do the work, and the pages here try to make them *visible* rather than merely stated: a value that announces its own death shows exactly when a move happens, and `&x` printed before and after shows what actually changed (the three-word header, not the bytes).

The back half is names rather than values. Shadowing, scope, and lifetimes are three different questions that English collapses into one word — a name ends at its brace, a borrow ends at its last use, and a value drops on a schedule five ordinary things can move — so the section separates them and then puts shadowing back through the borrow checker to prove the difference.

| Lesson | Level | What it teaches |
|---|---|---|
| [Ownership and moves](ownership_and_moves/README.md) | 101 | A move transfers *responsibility*, not bytes — the three rules, made visible by a value that announces its own death |
| [There is no `Move` trait](no_move_trait/README.md) | 201 | Moving is the default, so there is no trait to implement — `Copy` is the opt-out that stops it, and the compiler says so as an absence: *"does not implement the `Copy` trait"* |
| [What an address shows](what_an_address_shows/README.md) | 201 | `&x` addresses the three-word header, not the text — so a move changes the number without relocating a byte, and a `Copy` does the same thing while nothing moves at all |
| [Stack and heap](stack_and_heap/README.md) | 101 → 201 | No keyword puts a value on the heap — the *type* does, and `size_of` shows it: a `String` is 24 bytes on the stack whether it holds 5 characters or 5,000. One table prices move, `Copy`, `clone` and `Arc::clone` against the heap side |
| [Borrowing](borrowing/README.md) | 101 → 201 | `&T` and `&mut T`, the many-readers-or-one-writer rule, and the last-use rule that decides which order compiles |
| [How to learn lifetimes](how_to_learn_lifetimes/README.md) | 201 | Is *"clone everything"* good advice? Mostly yes — with three amendments, the sharpest being that cloning to dodge a *mutation* error compiles and silently does nothing |
| [Lifetime annotations](lifetime_annotations/README.md) | 201 | `<'a>` names a relationship rather than granting a duration — `E0106` and what its help line is asking, the three elision rules that make most signatures need nothing, and why a second lifetime permits *more* programs than reusing one |
| [A name is not a place](a_name_is_not_a_place/README.md) | 201 | What separates a shadow from `mut`, proved with the borrow checker rather than with addresses: the shadow compiles and the `mut` spelling is `E0506`, because one is a declaration and the other is a write |
| [A shadow does not drop](shadowing_does_not_drop/README.md) | 201 | What shadowing does to the value underneath: nothing — it is still alive, still borrowable, and it drops *after* the shadow that hid it, with no name left to release it early |
| [When to shadow](when_to_shadow/README.md) | 201 | The judgement call the other two leave open: what shadowing buys that `mut` cannot, the five idioms worth copying, and the three bugs that compile — only one of which warns, and not about shadowing |
| [Nothing checks a shadow](nothing_checks_a_shadow/README.md) | 201 | The tooling, not the mechanism: `rustc` has no shadowing lint, the type error that gets mistaken for one, and the single clippy lint that catches the accumulator bug — by also banning the idiom |
| [Scope is about names, not values](scope_is_about_names/README.md) | 201 | One word, three questions: a name ends at its brace, a borrow ends at its last use, and a value dies on a schedule that five ordinary things can move — including the `_` that `rustc` denies outright on a lock |
| [`Cow`: borrow until somebody writes](clone_on_write/README.md) | 201 | Borrowed or owned, decided at run time by the data — `to_mut()` is the write that pays for the clone, and the tag costs nothing: `Cow<str>` is the same 24 bytes as `String` |
| [`Rc`: the clone that copies a pointer](reference_counting/README.md) | 201 | Several owners for one value, counted — `Rc::clone` duplicates a pointer and a number, never the data, which makes it the cheapest `.clone()` in Rust and the most commonly misread one |
| [Sharing across threads: `Arc`](sharing_across_threads/README.md) | 201 | The same counter made atomic — the difference is not a performance note but the reason one of the two compiles across a thread boundary, and `Arc<Mutex<T>>` is what shared *mutable* state costs |

The last three pages are the ways out of a copy the one-owner rule would otherwise force: borrow until somebody writes, or let several owners share one value and count them.

## Related sections

- [Strings](../14_Strings/README.md) — the worked example most of these pages already use
- [Structs](../16_Structs/copy_vs_clone/README.md) — `Copy` vs `Clone`, which decides what `=` means
- [`ToOwned`](../12_Traits/to_owned/README.md) — `Clone` generalized to borrowed data
- [`&'static str`](../14_Strings/static_str/README.md) — the longest lifetime, filed with the strings because that is where you meet it: the `E0597` that only bites off a non-literal, and why `T: 'static` the bound does not mean "lives forever"

- [Interior mutability](../09_Advanced/interior_mutability/README.md) — the one way to write through a `&T`, and what moving the borrow check to run time costs

[SHADOWING.md](../SHADOWING.md) is the full reading order for the names half.

## Po polsku

To jest ten dział, w którym Rust przestaje przypominać inne języki, i zarazem jedyny, dla którego istnieje porządny polski materiał: [Tour of Rust ↗](https://tourofrust.com/TOC_pl.html) ma przetłumaczony cały rozdział 5, „Koncepcje Własności i Pożyczania Danych”. Warto go przejść równolegle — ta biblioteka trzyma się jego terminologii.

Słownik, na którym opiera się reszta działu:

| English | Polski |
|---|---|
| ownership | własność (posiadanie danych) |
| owner | właściciel |
| move | przeniesienie własności |
| borrow, borrowing | pożyczanie |
| reference | referencja |
| mutable reference | referencja mutowalna |
| scope | zasięg |
| drop | wypuszczenie zasobu |
| lifetime | czas życia |
| shadowing | przesłanianie |
| stack / heap | stos / sterta |
| borrow checker | *borrow checker* (nie tłumaczymy) |

Trzy reguły własności, w formie do zapamiętania: **każda wartość ma dokładnie jednego właściciela; właściciel jest tylko jeden naraz; gdy właściciel wychodzi z zasięgu, wartość zostaje wypuszczona.** Reguła pożyczania jest jedna: **wielu czytających albo jeden piszący, nigdy jedno i drugie naraz.**

Dwa ostrzeżenia dla czytającego po polsku. Po pierwsze, „przeniesienie” nie oznacza, że dane wędrują w pamięci — przenosi się **odpowiedzialność za zwolnienie**, a bajty zostają na miejscu. Po drugie, sporo polskich materiałów opisuje jeszcze stan sprzed 2018 roku, w którym pożyczenie trwało do końca bloku. Od czasu NLL (*non-lexical lifetimes*) kończy się przy **ostatnim użyciu** referencji, i to zmienia odpowiedź na pytanie „dlaczego to się nie kompiluje” w bardzo wielu przykładach.

Druga połowa działu dotyczy **nazw**, nie wartości — przesłanianie, zasięg i czasy życia to trzy różne pytania, które polszczyzna (podobnie jak angielski) skleja w jedno „wychodzi z zasięgu”. Strona [Zasięg dotyczy nazw](scope_is_about_names/README.md) rozdziela je na trzy.

**Szukaj po polsku:** własność i pożyczanie w Ruscie · przenoszenie własności · kontroler pożyczeń · czasy życia · `rust ownership borrowing` · `rust NLL`
