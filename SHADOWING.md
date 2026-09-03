# The shadowing map

**Level:** reference · the map

**One line:** Writing `let x` when `x` already exists makes a **second variable** wearing the first one's name — and this page is the door to every lesson about what that does, what it is for, and where it goes wrong.

Five pages, and they answer five different questions: *what is it*, *how does it differ from `mut`*, *what happens to the value it hid*, *should I use it here*, and *what would have caught me if I got it wrong*. Start wherever your question is; none of them assumes the others.

---

## What it is, before the reading order

If you landed here wanting the idea rather than the syllabus, it is this.

```rust
let spaces = "   ";
let spaces = spaces.len();
println!("{spaces}");  // 3
```

That is legal, and the second line is **not** an assignment. `spaces` is not being changed from a string into a number — a number cannot be put in a string's storage. A *new variable* is declared, of a different type, and it takes over the name for the rest of the scope. The old one is untouched.

Three consequences follow, and everything on the five pages below is one of them.

**It can change the type, so a name can survive a conversion.** This is the whole point. `input` stays `input` through `&str` → trimmed `&str` → `u32`, instead of becoming `input_raw`, `input_trimmed`, `input_num` — names that re-encode in text what the compiler already knows.

**It is not mutation, and that distinction is load-bearing.** `let mut x` promises the reader *"this may change anywhere below"*. A shadow promises *"this changed here, and the result is final"*. The second is a much stronger claim, and it is why so much idiomatic Rust manages without `mut` at all. The two are not interchangeable, either: a shadow cannot write to something that outlives the block it is in, which is the most common way shadowing is misused. The distinction is not a matter of taste, and there is a four-line test that proves it — see [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md).

**It takes away a name, not a value.** This is the one that surprises people. The shadowed value is still alive, still owned, still borrowable through any reference taken earlier — and because values drop in reverse declaration order, it dies *after* the shadow that hid it. So shadowing is not a way to release something early; it is the opposite, since it removes the handle you would have needed.

### The same idea in the languages you already have

- **Python.** `x = int(x)` looks like the same move and is not. Python **rebinds one variable**: the old object loses a reference and may be collected, so you can never have both alive under one name. Rust makes a **second variable** and keeps the first — which is why shadowing a lock in Rust leaves the lock held, a bug Python cannot have.
- **ABAP.** There is no analogue. A `DATA` name is one typed variable for the whole routine, and declaring `DATA(lv_x)` twice is a syntax error. So ABAP has to use `lv_input` / `lv_input_num`, and its `lv_` / `lt_` prefixes exist to carry the type in the name — precisely the job shadowing removes. What you give up in exchange is ABAP's free guarantee that one name in a routine means one thing.
- **C / C++.** Both shadow, but only in a *nested* block; a redefinition in the same block is an error, which is why C code opens braces to get the effect and why `-Wshadow` exists. Neither gives you the type change, because the declaration carries the type.

## The five lessons

| # | Lesson | Level | The question it answers |
|---|---|---|---|
| 1 | [Shadowing and `unwrap`](17_Option_and_Result/shadowing_and_unwrap/README.md) | 201 | What shadowing *is* — and why the popular explanation that ties it to `unwrap` is crediting it for something `Copy` is doing |
| 2 | [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md) | 201 | The mechanical difference from `mut` — one name and two places, proved by a borrow rather than by printing addresses, plus the one row the usual comparison table gets backwards |
| 3 | [A shadow does not drop](18_Ownership/shadowing_does_not_drop/README.md) | 201 | What happens to the value underneath: nothing. It outlives the shadow that hid it, and no name is left to free it early |
| 4 | [When to shadow](18_Ownership/when_to_shadow/README.md) | 201 | The judgement call — the design trade against `mut`, the five idioms, and the three bugs that compile |
| 5 | [Nothing checks a shadow](18_Ownership/nothing_checks_a_shadow/README.md) | 201 | What the tooling does about it — no lint in `rustc`, the type error mistaken for one, and the clippy lint that catches the bug only by banning the idiom |

Read in that order they build: the first defines it, the second says what it *is* against `mut` and hands you the test that settles any argument about it, the third explains the mechanism the fourth page's worst bug rests on, the fourth is the one you will come back to, and the fifth says how much help you can expect while you get it wrong.

## Where it turns up elsewhere

Shadowing is not a topic you finish; it is a habit that shows up inside other lessons. These are the places it does real work, or does real damage:

| Lesson | What shadowing is doing there |
|---|---|
| [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md) | `unused variable` is the compiler's only genuine net against an accidental shadow — and `unused_mut` on an accumulator is a shadowing bug reported in words that never say so |
| [Ownership and moves](18_Ownership/ownership_and_moves/README.md) | Drop order in reverse declaration order, which is what makes a shadowed value outlive its shadow |
| [Borrowing](18_Ownership/borrowing/README.md) | Why a reference taken before a shadow keeps working after it |
| [Assignment drops the old value](18_Ownership/assignment_is_a_drop/README.md) | The nearest-miss comparison: `let e = …` twice keeps both values alive, `e = …` frees the first one on the spot — one keyword apart, and a completely different drop schedule |
| [`if let`](17_Option_and_Result/if_let/README.md) | Pattern bindings introduce a fresh name; `let … else` is the guard clause the unwrap-and-narrow idiom opens with |
| [Initial values](17_Option_and_Result/initial_values/README.md) | The other route away from `mut` — declare without initializing and let the compiler prove you assigned |
| [A score is not a number](16_Structs/newtype_score/README.md) | `let id = BallotId(id);` — narrowing into a newtype so the loose form becomes unreachable |
| [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md) | Where a shadow is given a deliberate *end* — and the nested-block snippet that circulates as "shadowing in Rust", which is the kind every language has |
| [Lock poisoning](09_Advanced/mutex_poisoning/README.md) | The guards that must never be shadowed, and what a held one costs a second thread |

## The one rule, if you only keep one

**Shadow when the new binding is the same concept in a new form, and keep it close to the one it replaces.** Reach for a second name when it is a different thing, and never shadow a value that holds a resource. [When to shadow](18_Ownership/when_to_shadow/README.md) is that sentence with the evidence attached.

## Practising it

Every page in the set ends in a `## Practice` exercise with a solution CI compiles and runs. The order to attempt them in is in [KATAS.md](KATAS.md); every kata is a row there, and the numbers live only in that table.

## Looking a term up

[GLOSSARY.md](GLOSSARY.md) defines the vocabulary these pages use — shadowing, pattern binding, drop order, `Copy`, dangling reference, `clippy::shadow_unrelated` — and every entry links to the page that explains it properly.

## Po polsku

Przesłanianie (*shadowing*) to napisanie `let x`, gdy `x` już istnieje: powstaje **druga zmienna** nosząca imię pierwszej. Ta strona jest mapą pięciu lekcji, odpowiadających na pięć różnych pytań — czym to jest, czym różni się od `mut`, co dzieje się z zasłoniętą wartością, czy wypada tak zrobić w danym miejscu i co by cię złapało, gdybyś się pomylił.

Polskie „przesłanianie" jest tu akurat słowem trafniejszym niż angielski oryginał, bo samo podpowiada sedno: **zasłania się nazwę, a nie wartość**. Wartość pod spodem żyje dalej, jest nadal posiadana i nadal można ją pożyczać — po prostu nie da się jej już nazwać. To właśnie odróżnia przesłanianie od `mut`, które pisze w to samo miejsce.

Uwaga terminologiczna, bo bywa myląca: w polskich materiałach o programowaniu obiektowym „przesłanianie" oznacza czasem nadpisanie metody klasy bazowej (*overriding*). To zupełnie inne zjawisko i nie ma z tym nic wspólnego.

**Szukaj po polsku:** przesłanianie zmiennych · `let` a `mut` · zasięg zmiennej · `rust shadowing` · `rust variable shadowing vs mutability`
