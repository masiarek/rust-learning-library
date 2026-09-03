# Transforms instead of `match`

**Level:** 201 · working knowledge

**One line:** A `match` whose arms only put the value back has a method that does the same thing — and the reason to prefer the method is not that it is shorter, it is that its **name** says which of the four possible rewrappings you meant.

```rust
fn main() {
    let given: Option<u32> = Some(3);

    let by_match = match given {
        Some(n) => Some(n * 2),
        None => None,
    };
    let by_method = given.map(|n| n * 2);

    println!("{by_match:?} {by_method:?}");   // Some(6) Some(6)
}
```

Four lines against one, but count the *decisions* instead. The `match` version makes you re-state that `None` stays `None` — a line you must read to confirm it says nothing surprising. `map` states it once, in its name, and cannot say anything else.

## The question that picks the method

Every one of these methods answers a different sentence. Find your sentence and the method follows:

| What you want to say | `Option` | `Result` |
|---|---|---|
| change what is inside, keep the shape | `map` | `map` |
| change what is inside, **and it may go missing** | `and_then` | `and_then` |
| change the *error*, keep the value | — | [`map_err`](#map_err-the-same-value-a-different-error-type) |
| give me a value, or this default | [`unwrap_or`](../unwrap_or/README.md) | [`unwrap_or`](../unwrap_or/README.md) |
| …or this default, built only if needed | [`unwrap_or_else`](../unwrap_or_else/README.md) | [`unwrap_or_else`](../unwrap_or_else/README.md) |
| …or whatever the type calls empty | [`unwrap_or_default`](../unwrap_or_default/README.md) | [`unwrap_or_default`](../unwrap_or_default/README.md) |
| transform *and* fall back, in one call | [`map_or`](../map_or/README.md) | [`map_or`](../map_or/README.md) |
| absence is an error — here is which | `ok_or` / `ok_or_else` | — |
| I no longer care *why* it failed | — | `.ok()` |
| keep it only if it also satisfies this | `filter` | — |

The two blue boxes in [the section's diagram](../README.md) are those two types, and every label on an arrow is one row of this table.

## `map` or `and_then`? Ask what your closure returns

This is the only genuinely confusing choice in the list, and it has a mechanical answer — **look at the closure's return type, not at your intent**:

```rust
fn main() {
    let given: Option<u32> = Some(3);
    let half = |n: u32| if n % 2 == 0 { Some(n / 2) } else { None };

    println!("{:?}", given.map(|n| n * 2));   // Some(6)     closure returned u32
    println!("{:?}", given.map(half));        // Some(None)  closure returned an Option — nested
    println!("{:?}", given.and_then(half));   // None        flattened
}
```

`Some(None)` is the tell. If you find yourself reaching for `.flatten()` right after a `.map()`, the call you wanted was `and_then` — and if you have met `flat_map` on iterators, this is the same operation under the older name. `and_then` is the one allowed to introduce a *new* absence: the value was there, and after your closure it is not.

## `map_err` — the same value, a different error type

`map` reaches into `Ok`; `map_err` reaches into `Err`. The `Ok` case passes through untouched, which is what makes it the natural way to widen an error at a boundary:

```rust
fn main() {
    let bad: Result<u32, _> = "x".parse::<u32>();
    let good: Result<u32, _> = "12".parse::<u32>();

    println!("{:?}", bad.map_err(|e| format!("bad count: {e}")));    // Err("bad count: invalid digit found in string")
    println!("{:?}", good.map_err(|e| format!("bad count: {e}")));   // Ok(12)
}
```

Most of the time you will not write it, because [the `?` operator](../the_question_mark_operator/README.md) performs the same conversion through `From` without a call at the site. Reach for `map_err` when there is no `From` impl to write — a one-off message, or an error type you do not own.

## Crossing between the two types

`ok_or` turns an absence into a named failure. `.ok()` throws the name away:

```rust
fn main() {
    let given: Option<u32> = Some(3);
    let blank: Option<u32> = None;

    println!("{:?}", given.ok_or("no bio"));      // Ok(3)
    println!("{:?}", blank.ok_or("no bio"));      // Err("no bio")
    println!("{:?}", "x".parse::<u32>().ok());    // None — the reason is gone
}
```

The two directions are not equally safe. Going *up* to a `Result` adds information you had to supply. Going *down* with `.ok()` deletes information you already had, and [returning `None` on error](../none_on_error/README.md) is the page on why `input.parse().ok()` is usually a downgrade: an empty string, a negative number, a word and an overflow all arrive as the same `None`. Cross downward on purpose, never for tidiness.

## The `_else` suffix is not decoration

Every eager method has a lazy twin, and the difference is real rather than stylistic — an argument is evaluated **before** the call it is an argument to, so `unwrap_or(expensive())` builds the fallback even on `Some`:

```text
Some(3).unwrap_or(expensive_default())      expensive_default ran 1 time
Some(3).unwrap_or_else(|| expensive())      expensive_default ran 0 times
```

The value was discarded both times; only the second one avoided building it. Same split for `ok_or` / `ok_or_else`, `or` / `or_else`, `map_or` / `map_or_else`. The rule of thumb: if the fallback is a literal or an already-computed variable, take the eager one and keep the line readable; if it allocates, opens something or calls a function, take the `_else`. Clippy has a lint for the obvious cases and cannot see the rest.

## `as_ref()` — when `&self` owns nothing it may give away

This is the one that stops people, and it looks like a bug in `Option` rather than what it is:

```rust
struct Profile {
    bio: Option<String>,
}

impl Profile {
    fn shout(&self) -> String {
        // self.bio.unwrap_or("(no bio)".to_string())            // E0507
        self.bio.as_ref().map_or("(no bio)".to_string(), |b| b.to_uppercase())
    }
}

fn main() {
    let filled = Profile { bio: Some("writes code".into()) };
    println!("{}", filled.shout());   // WRITES CODE
}
```

```text title="Abridged — real rustc 1.98.0 output for profile.rs, without the file-and-line header"
error[E0507]: cannot move out of `self.bio` which is behind a shared reference
  |
8 |         self.bio.unwrap_or("(no bio)".to_string()).to_uppercase()
  |         ^^^^^^^^ move occurs because `self.bio` has type `Option<String>`, which does not implement the `Copy` trait
  |
help: consider cloning the value if the performance cost is acceptable
  |
8 |         self.bio.clone().unwrap_or("(no bio)".to_string()).to_uppercase()
  |                 ++++++++
```

`unwrap_or` takes `self` by value, so it wants to *consume* the `Option` — and a `&self` method has only borrowed it. [`as_ref()` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref) rebuilds the `Option` around a borrow: `&Option<String>` becomes `Option<&String>`, which the method may consume freely because consuming a reference costs nothing.

Read the compiler's suggestion twice. `.clone()` **works**, it is offered first, and it allocates a whole `String` on every call to avoid a move that never needed to happen. It is right that rustc offers the fix that always compiles; it is on you to notice the free one. The `&mut` version is `as_mut()`, and `as_deref()` goes one step further, handing you `Option<&str>`.

## Where a `match` is still the right answer

The advice is *stop writing the matches that only rewrap*, not stop writing `match`:

```rust
fn main() {
    let outcome: Result<u32, std::num::ParseIntError> = "7".parse();

    let sentence = match outcome {
        Ok(n) if n > 5 => format!("{n} is plenty"),
        Ok(n) => format!("{n} is tight"),
        Err(e) => format!("not a number: {e}"),
    };
    println!("{sentence}");   // 7 is plenty
}
```

Three outcomes, a guard, and different work per arm. No single transform expresses that, and a chain of them expressing it would be worse than the `match`. The signal to watch for is an arm whose body is `Some(…)`, `None`, `Ok(…)`, `Err(…)` or a bare `return` of the same — that arm is a method you have not named yet.

## If you are coming from another language

**Python.** The nearest thing is the walrus-and-`or` idiom and `dict.get(key, default)` — `unwrap_or` is `.get(k, d)` exactly, and `unwrap_or_else` is the reason `dict.setdefault` disappoints people, because its default argument is evaluated eagerly too. That trap is identical in both languages:

```python
d.setdefault("k", expensive())   # expensive() runs even when "k" is present
```

What Python has no counterpart for is the *typed* half. `x.map(f)` on a missing value is `None`, and Python's equivalent — `f(x) if x is not None else None` — restates the absence check at every call, which is why the pattern is usually written as a helper the third time it appears. Rust's version does not accumulate: `map` composes with `map` and `and_then` composes with both, and the compiler will not let you forget the sad arm because there is no arm to forget.

The other half worth naming: `None` in Python is a *value* of any type, so `f(None)` is a runtime question. `Option<T>` is a different type from `T`, so it is a compile-time one — see [`Option` vs `Result`](../option_vs_result/README.md).

**ABAP.** `sy-subrc` is the whole of `Result`, flattened into a global integer and checked by convention:

```abap
READ TABLE lt_votes INTO ls_vote WITH KEY name = 'Ada'.
IF sy-subrc = 0.
  " ls_vote is filled
ENDIF.
```

Three differences, in increasing order of how much they matter:

1. **The value and the status are separate.** `ls_vote` exists either way — it holds the previous read's data, or initial values, and nothing marks which. `Option<Vote>` cannot be in that state: there is no filled-in `Vote` to look at unless you unwrapped it.
2. **Nothing forces the check.** Skipping `IF sy-subrc = 0` compiles and runs, which is the classic ABAP defect. In Rust the check is the only door: `.map()` and `?` both go through the `Some`/`Ok` arm, and `Result` additionally carries `#[must_use]`, so ignoring one is a compiler warning.
3. **`sy-subrc` is overwritten by the next statement.** Its lifetime is one statement, so the check must be immediately after. An `Option` is a value you can store, pass, put in a table, and inspect an hour later.

The ABAP habit worth keeping is the one that transfers: `sy-subrc` is checked *once, immediately*, and this page's methods are that habit expressed in the type — `ok_or` is the `IF sy-subrc <> 0. RAISE …` you would have written, folded into the same line as the read.

## The verified output

<!-- output:transforms_instead_of_match -->
*Verified output of [`transforms_instead_of_match.rs`](examples/transforms_instead_of_match.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The shape: a `match` whose arms put the value back
   match { Some(n) => Some(n * 2), None => None } = Some(6)
   given.map(|n| n * 2)                           = Some(6)
   same answer: true
   The method is not shorter by accident — `map` NAMES the shape:
   change the payload, leave the Some/None decision alone.

2. `map` or `and_then`? Ask what your closure returns
   the closure returns u32          -> map      -> Some(6)
   the closure returns Option<u32>  -> map      -> Some(None)   nested!
   the closure returns Option<u32>  -> and_then -> None   flat
   Some(4).and_then(half) = Some(2)   Some(3).and_then(half) = None
   `and_then` is the one that may say "and now it is missing".

3. Crossing between the two types
   Option -> Result   given.ok_or("no bio")     = Ok(3)
                      blank.ok_or("no bio")     = Err("no bio")
   Result -> Option   parse("12").ok()          = Some(12)
                      parse("x").ok()           = None   the WHY is gone
   `.ok()` is a downgrade: four different parse failures all arrive
   as one indistinguishable None. Cross that way on purpose only.

4. `map_err` — the same value, a different error type
   parse("x").map_err(|e| format!("bad count: {e}")) = Err("bad count: invalid digit found in string")
   parse("12").map_err(..)                            = Ok(12)   untouched

5. Eager or lazy: the `_else` suffix is not decoration
   Some(3).unwrap_or(expensive_default())      builds = 1
   Some(3).unwrap_or_else(|| expensive())      builds = 0
   The value was never used either time. The eager form built it anyway,
   because an argument is evaluated before the call it is an argument to.
   Same split: ok_or / ok_or_else, or / or_else, map_or / map_or_else.

6. The rest of the vocabulary, on one line each
   filter   Some(3).filter(|n| n % 2 == 0)  = None   keep it only if
   or       blank.or(Some(9))               = Some(9)   first one present
   xor      Some(3).xor(Some(9))            = None   exactly one, or None
   zip      Some(3).zip(Some(9))            = Some((3, 9))   both, or None
   replace  slot.replace(9) -> Some(3), slot is now Some(9)
   take     slot.take()     -> Some(9), slot is now None

7. `as_ref()` — when `&self` owns nothing it may give away
   filled.shout() = WRITES CODE
   empty.shout()  = (no bio)
   Without as_ref(): E0507, cannot move out of `self.bio` which is
   behind a shared reference. as_ref() rewrites &Option<String> into
   Option<&String>, so the Option is rebuilt around a borrow.
   filled.name is still ours afterwards: Ada

8. Where a `match` is still the right answer
   7 is plenty
   Three outcomes, a guard, and different WORK per arm — no single
   transform expresses that. The advice is to stop writing the matches
   that only rewrap, not to stop writing `match`.
```
<!-- /output -->

## See also

- [The `?` operator](../the_question_mark_operator/README.md) — the transform that returns, and the `From` conversion it performs for free
- [`unwrap_or`](../unwrap_or/README.md) · [`unwrap_or_else`](../unwrap_or_else/README.md) · [`unwrap_or_default`](../unwrap_or_default/README.md) · [`map_or`](../map_or/README.md) — the fallback family, one page each
- [Returning `None` on error](../none_on_error/README.md) — why `.ok()` is usually the wrong direction
- [What a monad is](../what_a_monad_is/README.md) — `map` and `and_then` are the two operations that name the shape
- [`Option` vs `Result`](../option_vs_result/README.md) — which type you should have been transforming
- [Effective Rust, Item 3 ↗](https://effective-rust.com/transform.html) — the source of the diagram on the section page, and of this page's `as_ref()` example

## Po polsku

Ta strona nie namawia do pisania krócej, tylko do **nazywania** tego, co się robi. `match`, którego gałęzie odkładają wartość z powrotem do `Some`/`Ok`, nie podejmuje żadnej decyzji, a i tak każe czytelnikowi przejść obie gałęzie, żeby się upewnić, że nie kryje się w nich nic niespodziewanego. Nazwa metody mówi to samo raz i nie może powiedzieć nic innego. Polskie zdanie, które chcesz wypowiedzieć, wskazuje metodę:

- „zmień zawartość, kształt zostaw” → `map`
- „zmień zawartość, ale może jej zabraknąć” → `and_then`
- „zmień sam błąd, wartość zostaw nietkniętą” → `map_err`
- „brak jest błędem, a oto jakim” → `ok_or` / `ok_or_else`
- „nie obchodzi mnie już, dlaczego się nie udało” → `.ok()`

Sygnałem do zamiany jest gałąź, której całym ciałem jest `Some(…)`, `None`, `Ok(…)` albo `Err(…)`. Tam, gdzie w gałęziach stoi strażnik (`if`) i naprawdę różna praca, `match` zostaje na swoim miejscu.

Wybór między `map` a `and_then` ma odpowiedź mechaniczną: patrz na **typ zwracany przez domknięcie** (*closure*), a nie na własne intencje. Domknięcie oddające `u32` — `map`; domknięcie oddające `Option<u32>` — `and_then`. Jeśli w wyniku wyszło `Some(None)` albo sięgasz po `.flatten()` zaraz za `.map()`, chodziło o `and_then`. Osobom przychodzącym z Javy najszybciej wytłumaczyć to przez `Optional`: `map` to `map`, a `and_then` to `flatMap` (w iteratorach Rusta ta sama operacja nazywa się `flat_map`). W drugą stronę czujność przy `.ok()`: przejście z `Result` do `Option` **kasuje informację**, którą się już miało — pusty łańcuch znaków, liczba ujemna, słowo i przepełnienie docierają wtedy jako jedno, nierozróżnialne `None`. Idąc w górę przez `ok_or` dokładasz informację, schodząc w dół przez `.ok()` ją tracisz; rób to z rozmysłem, a nie dla porządku w kodzie.

Dwie pułapki łapią tu najczęściej. Pierwsza: sufiks `_else` nie jest ozdobnikiem. Argument jest obliczany **przed** wywołaniem, do którego należy, więc `unwrap_or(drogie())` zbuduje wartość zapasową także wtedy, gdy w środku siedzi `Some`, a leniwa wersja `unwrap_or_else(|| drogie())` nie zbuduje jej wcale. Ta sama para dotyczy `ok_or` / `ok_or_else`, `or` / `or_else`, `map_or` / `map_or_else`: przy literale albo gotowej zmiennej bierz wersję zachłanną i zostaw linijkę czytelną, przy alokacji lub wywołaniu funkcji — tę z `_else`. Druga: `unwrap_or` przyjmuje `self` przez wartość, czyli chce **przejąć własność**, a metoda z `&self` niczego nie posiada, tylko pożycza — stąd `E0507`, *cannot move out of `self.bio` which is behind a shared reference*. `as_ref()` przebudowuje `Option` wokół referencji (`&Option<String>` staje się `Option<&String>`), a skonsumowanie referencji nic nie kosztuje. Kompilator podpowie w tym miejscu `.clone()` i ta poprawka faktycznie działa — tyle że alokuje cały `String` przy każdym wywołaniu. Podpowiedź `rustc` jest najpewniejsza, niekoniecznie najlepsza.

**Szukaj po polsku:** metody `Option` i `Result` · leniwe wartościowanie · `rust map vs and_then` · `rust E0507 cannot move out of behind a shared reference` · `rust Option as_ref as_deref`
