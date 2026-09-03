# `Option` fields: modelling what may be absent

**Level:** 101 · for newcomers

**One line:** In a struct, every field is required — `Option<T>` is how the type says *"this one may legitimately be missing"*, and then the compiler holds every reader to it.

The other Option pages are about what a function *returns*. This one is about `Option` in a type definition, which is where you meet it when modelling data or deserializing someone else's JSON.

---

## Required is the default

```rust
#[derive(Debug)]
struct Person {
    first: String,
    last: String,
    age: Option<u8>,
}
```

Leave a field out of the initializer and it does not compile. Supply the wrong shape and the compiler tells you exactly what to write:

```text title="Abridged — real rustc output for `age: 57`"
error[E0308]: mismatched types
7 |     let p = Person { first: "Ada".to_string(), age: 57 };
  |                                                     ^^ expected `Option<u8>`, found integer
  |
  = note: expected enum `Option<u8>`
             found type `{integer}`
help: try wrapping the expression in `Some`
  |
7 |     let p = Person { first: "Ada".to_string(), age: Some(57) };
  |                                                     +++++  +
```

That error is the lesson in miniature. There is no "unset" state to fall into: you write `Some(57)` or you write `None`, and writing `None` is you *stating* that the value is absent rather than leaving the question open. Compare a language where a missing field is `null` by default and every reader downstream has to guess whether that was deliberate.

## Reading the field

```rust
match &b.isbn {
    Some(i) => format!("{} — ISBN {i}", b.title),
    None    => format!("{} — no ISBN on record", b.title),
}
```

Note `match &b.isbn`, not `match b.isbn`. The second tries to *move* the `String` out of a struct you only borrowed, and the borrow checker refuses. This is the single most common stumble with `Option` fields, and the fix is almost always one `&` or one `.as_ref()`.

The three one-liners that cover most days:

| Call | Gives you | Use when |
|---|---|---|
| [`.as_deref()` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref) | `Option<&str>` from `Option<String>` | passing it on without cloning |
| [`.map_or(default, f)` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or) | one value either way | rendering for display |
| [`.unwrap_or_default()` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default) | `T::default()` when absent | absent and empty mean the same thing |

Use `.as_ref()` when you want to inspect the inside and keep the original — `known.isbn.as_ref().map(|s| s.len())` leaves `known.isbn` owned and intact, where `known.isbn.map(…)` would consume it.

## The question that decides whether a field should be `Option` at all

`Option<Vec<T>>` is usually wrong, because `Vec` can already say "nothing here" by being empty. It is right exactly when **absent and empty mean different things**:

```rust
struct Survey {
    question: String,
    /// None = not collected yet. Some(vec![]) = collected, and nobody answered.
    responses: Option<Vec<u32>>,
}
```

A plain `Vec` collapses those two states into one, and no amount of downstream code can recover the difference. So ask it directly: *is "we never asked" a different fact from "we asked and got nothing"?* If yes, keep the `Option`. If no, delete it — you are carrying a wrapper that only adds a `match` arm.

The same question governs `Option<bool>` (is "unanswered" different from "no"?) and `Option<String>` (is a missing name different from an empty one?). Most of the time the answer is no, and the plain type is better.

## `Option` fields and `Default`

`Option<T>` implements `Default` as `None` **for every `T`**, whether or not `T` itself has a default:

```rust
#[derive(Debug, Default)]
struct Config {
    name: Option<String>,
    retries: Option<u32>,
    verbose: bool,
}

let cfg = Config { retries: Some(3), ..Default::default() };
```

This is why config structs are so often made of `Option`s: you get `Default` for free, the struct-update syntax lets a caller set one field, and each `None` honestly records "the user did not say", leaving you free to apply the real default at the point of use with `.unwrap_or(1)`.

Note `verbose` is a plain `bool`, not `Option<bool>` — for a flag, `false` *is* the answer, so wrapping it would only invent a third state nobody needs.

## What it costs

```
Person       56 bytes
String       24   (each)
Option<u8>    2
u8            1
```

`u8` uses all 256 of its bit patterns, so there is no spare one to mean `None`, and `Option<u8>` must carry a separate tag byte — doubling it. That is the *expensive* case, and it is still one byte. Where the inner type has an unused pattern the wrapper is free; [`Option` is a one-item collection](../option_as_collection/README.md) has the sizes side by side.

---

## Practice

**Which fields may legitimately be missing?** Model a `Respondent` with a name, a middle name, an account id, and the ratings from their survey form. Make each field `Option` or not on purpose, and write a reader that tells *no form returned* apart from *form returned, nothing rated*.

Make `account_id` an `Option<u32>` first and count how many readers now have to handle a state that cannot occur. Then try storing the ratings as a plain `Vec<u8>` and see which of the two facts above you can no longer report.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:option_fields_kata -->
*[`option_fields_kata.rs`](examples/option_fields_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: which fields may legitimately be missing?
//!
//!   rustc --edition 2024 option_fields_kata.rs -o /tmp/ofk && /tmp/ofk

#[derive(Debug, Default)]
struct Respondent {
    /// Required. A person with no name is not a person with an empty name.
    name: String,
    /// Optional, and the type says so: plenty of people have no middle name.
    middle_name: Option<String>,
    /// NOT an Option. "Belongs to no account" is not a state that exists here;
    /// making it optional would push a check into every reader for nothing.
    account_id: u32,
    /// Optional for a real reason: the form may not be back yet. An empty Vec
    /// would say "returned, and rated nothing", which is a different fact.
    ratings: Option<Vec<u8>>,
}

fn describe(r: &Respondent) {
    let full = match &r.middle_name {
        Some(m) => format!("{} {} ", r.name, m),
        None => format!("{} ", r.name),
    };
    // Two different questions, and the type makes you ask them separately.
    let form = match &r.ratings {
        None => "no form returned".to_string(),
        Some(s) if s.is_empty() => "form returned, nothing rated".to_string(),
        Some(s) => format!("form returned, {} ratings, total {}", s.len(), s.iter().map(|n| *n as u32).sum::<u32>()),
    };
    println!("  {full:<18} account {:<4} {form}", r.account_id);
}

fn main() {
    let people = [
        Respondent { name: "Ada".into(), middle_name: Some("Q".into()), account_id: 7, ratings: Some(vec![5, 2, 0]) },
        Respondent { name: "Ben".into(), middle_name: None, account_id: 7, ratings: Some(vec![]) },
        Respondent { name: "Cara".into(), middle_name: None, account_id: 12, ratings: None },
    ];

    println!("Three respondents, and the two absences the type keeps apart:");
    for r in &people {
        describe(r);
    }

    let returned = people.iter().filter(|r| r.ratings.is_some()).count();
    println!("\n  forms returned: {returned} of {}", people.len());
    println!("      A Vec<u8> field with no Option could not have told you that.");

    println!("\nDefault gives None for every optional field, for free:");
    println!("  {:?}", Respondent::default());
}
```
<!-- /source -->

<!-- output:option_fields_kata -->
*Verified output of [`option_fields_kata.rs`](examples/option_fields_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Three respondents, and the two absences the type keeps apart:
  Ada Q              account 7    form returned, 3 ratings, total 7
  Ben                account 7    form returned, nothing rated
  Cara               account 12   no form returned

  forms returned: 2 of 3
      A Vec<u8> field with no Option could not have told you that.

Default gives None for every optional field, for free:
  Respondent { name: "", middle_name: None, account_id: 0, ratings: None }
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:option_fields -->
*Verified output of [`option_fields.rs`](examples/option_fields.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: Required by default; Option marks the exceptions
  known.isbn = Some("1-123-456")   tags: rust, reference
  draft.isbn = None   tags: 0 (none)
      You cannot forget a field: leaving `isbn` out is a compile error, and
      writing `None` is you SAYING it is absent rather than leaving it undefined.

──── Step 2: Reading the field: match on a REFERENCE to it
  Great book — ISBN 1-123-456
  Untitled draft — no ISBN on record
      `match &b.isbn`, not `match b.isbn` — the latter tries to move a String
      out of a borrowed struct, which the borrow checker refuses.

──── Step 3: The three one-liners you will actually use
  as_deref()          Some("1-123-456")
  map_or (known)      320 pages
  map_or (draft)      unknown
  unwrap_or_default   ""
  as_ref().map(len)   Some(9)   (known.isbn still owned: Some("1-123-456"))

──── Step 4: Option<Vec<T>> — usually wrong, occasionally exactly right
  question: "How was checkout?"
  not collected yet
  collected — nobody answered
  collected — 2 responses
      A plain Vec cannot tell those first two apart. If they mean the same thing
      in your domain, use Vec and delete the Option; if they differ, keep it.

──── Step 5: Option<T> defaults to None for every T
  Config::default() = Config { name: None, retries: None, verbose: false }
  one field set     = Config { name: None, retries: Some(3), verbose: false }
      Option<T>: Default is None regardless of T — so a config struct of Options
      derives Default even when the inner types cannot.
  retries.unwrap_or(1)      = 3
  name.unwrap_or("anon")    = anon
  verbose (a plain bool)   = false   <- not optional: false IS the answer

──── Step 6: What an optional field costs
  Ada Lovelace — age None
  Person                  56 bytes
  String (each)           24
  Option<u8>               2
  u8                       1
      u8 uses all 256 of its bit patterns, so there is no spare one for None
      and Option<u8> must carry a separate tag byte. Compare Option<Box<T>>,
      which is free because a null pointer was never a legal Box.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 17_Option_and_Result/option_fields/examples/option_fields.rs -o /tmp/of && /tmp/of
```

## See also

- [`Option` vs `Result`](../option_vs_result/README.md) — absence versus failure
- [`Option` is a one-item collection](../option_as_collection/README.md) — iteration, `take()`, and what the wrapper costs
- [`std::option` module docs ↗](https://doc.rust-lang.org/core/option/) — the canonical list of what `Option` is for

## Po polsku

W strukturze każde pole jest **obowiązkowe** — nie ma stanu „nieustawione”, w który dałoby się wpaść przez zapomnienie. `Option<T>` w definicji typu jest więc deklaracją: „to pole ma prawo nie istnieć”, a kompilator egzekwuje ją u każdego, kto tę strukturę czyta. Napisz `age: 57` zamiast `age: Some(57)`, a dostaniesz `error[E0308]: mismatched types` z gotową podpowiedzią *„try wrapping the expression in `Some`”*. Warto zobaczyć różnicę wobec języków, w których brakujące pole samo z siebie jest `null`-em: tam każdy późniejszy czytelnik musi zgadywać, czy brak był zamierzony. Tutaj `None` to zdanie oznajmujące, które ktoś świadomie napisał.

Najczęstsze potknięcie przy polach `Option` jest jedno i zawsze takie samo: `match b.isbn` zamiast `match &b.isbn`. To pierwsze próbuje przenieść `String` na własność ze struktury, którą masz tylko pożyczoną, a borrow checker na to nie pozwala. Poprawką jest zwykle jedno `&` albo jedno `.as_ref()` — i to jest dokładnie ten moment, w którym początkujący odruchowo sięga po `.clone()`. Nie trzeba: `.as_ref()` daje `Option<&String>` i zostawia oryginał nietknięty, a `.as_deref()` od razu `Option<&str>`, czyli to, co zwykle chcesz przekazać dalej bez kopiowania.

Osobne pytanie jest ważniejsze niż składnia: **czy to pole w ogóle powinno być `Option`em?** Tu polski czytelnik ma gotową intuicję z baz danych — regułę, że `NULL` to nie to samo co pusty łańcuch znaków ani zero. Ta sama reguła rozstrzyga sprawę: `Option<Vec<T>>` jest zwykle błędem, bo pusty wektor już potrafi powiedzieć „nic tu nie ma”, a sens ma tylko wtedy, gdy **brak i pustka to dwa różne fakty** — `None` znaczy „jeszcze nie zbieraliśmy odpowiedzi”, a `Some(vec![])` znaczy „zebraliśmy i nikt nie odpowiedział”. Zwykły `Vec` skleja te stany w jeden i żaden późniejszy kod ich nie rozdzieli. To samo pytanie zadaj przy `Option<bool>` (czy „nie odpowiedział” to co innego niż „nie”?) i przy `Option<String>`; najczęściej odpowiedź brzmi „nie” i goły typ jest lepszy.

Na koniec dwie rzeczy praktyczne. `Option<T>` implementuje `Default` jako `None` dla **każdego** `T`, niezależnie od tego, czy sam `T` ma wartość domyślną — dlatego struktury konfiguracyjne tak często składają się z `Option`ów: `#[derive(Default)]` przechodzi za darmo, składnia `..Default::default()` pozwala ustawić jedno pole, a każdy `None` uczciwie zapisuje „użytkownik nic nie podał”, więc prawdziwą wartość domyślną nakładasz dopiero w miejscu użycia, przez `.unwrap_or(1)`. Zwróć uwagę, że `verbose` zostaje gołym `bool`-em: dla flagi `false` **jest** odpowiedzią, a `Option<bool>` dorobiłby trzeci stan, którego nikt nie potrzebuje. Kosztu też nie ma co demonizować — najdroższy przypadek to `Option<u8>`, który waży 2 bajty zamiast 1, bo `u8` wykorzystuje wszystkie 256 wzorców bitowych i nie zostaje ani jeden wolny na `None`.

**Szukaj po polsku:** pola opcjonalne w strukturze · Option zamiast null · NULL a wartość pusta · `rust struct option field` · `rust cannot move out of borrowed content option` · `rust option as_ref as_deref`
