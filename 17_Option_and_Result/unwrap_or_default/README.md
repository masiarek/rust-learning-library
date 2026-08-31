# `unwrap_or_default`: the fallback the type chose for you

**Level:** 201 · working knowledge

**One line:** It is `unwrap_or_else(T::default)` with the closure spelled as a trait bound — the shortest member of the family, and the only one whose fallback value is decided somewhere other than the call site.

On a `Result` it is also the most forgetful thing in the family: no closure, no error, no value at the call site. `T: Default` is the entire specification of what you get back, and `E` is not mentioned at all.

```rust
Option::unwrap_or_default(self) -> T   where T: Default
Result::unwrap_or_default(self) -> T   where T: Default   // E is ignored entirely
```

That is a genuine convenience roughly as often as it is a quiet decision made in another file. This page is about telling the two apart.

---

## What `Default` hands you

```text
u8                       0
f64                      0.0
bool                     false
char                     '\0'
String                   ""
Vec<u8>                  []
Option<u8>               None
BTreeMap<u8, u8>         {}
()                       ()
```

Nothing here is magic. `char`'s default is the null character rather than a space; `Option`'s own default is `None`, which is why a struct full of `Option` fields derives cleanly; and `unwrap_or_default()` on an `Option<u8>` is the same call as `unwrap_or_else(u8::default)`, one word shorter.

## A derived default is the type's zero, not your domain's

```rust
#[derive(Default)] struct DerivedQuorum(u32);          // 0
impl Default for HouseQuorum { fn default() -> Self { HouseQuorum(50) } }
```

```text
ballots cast: 40, and the config set no quorum
#[derive(Default)] -> DerivedQuorum(0)  => quorum met? true
impl Default (50)  -> HouseQuorum(50)   => quorum met? false
```

Same call site, opposite outcome, and the difference is a line of code in another file. `#[derive(Default)]` on a newtype is a claim that **zero is the sensible fallback** — true for a tally or a counter, false for a quorum, a threshold, a timeout, a seat count, a scale maximum.

The third option is the one worth remembering, because it does not look like an option: **implement `Default` for neither.** Then `unwrap_or_default()` stops compiling, and the compiler tells you exactly which decision you skipped.

```text
error[E0277]: the trait bound `Quorum: Default` is not satisfied
   |
4  |     let _ = q.unwrap_or_default();
   |               ^^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `Quorum`
note: required by a bound in `Option::<T>::unwrap_or_default`
```

A missing `Default` impl is a guard rail, not a gap. Deriving it "for completeness" on a domain type is how the guard rail gets removed by someone who was tidying.

The same reasoning runs the other way for `NonZeroU8`, which has no `Default` **because** it has no zero — the standard library declining to invent one for a type whose whole point is that the value is excluded.

## On an enum, the compiler makes you say it out loud

```text
error[E0665]: `#[derive(Default)]` on enum with no `#[default]`
  |
2 | enum Tiebreak { Lot, Margin }
  | ----------------------------- this enum needs a unit variant marked with `#[default]`
help: make this unit variant default by placing `#[default]` on it
```

There is no first-variant rule and no zero to fall back on, so (since Rust 1.62) you mark the variant yourself. Notice the asymmetry with the section above: on a **struct**, `derive(Default)` makes a policy decision per field, silently; on an **enum** it refuses to make one at all. The attribute you are forced to write ends up in the type definition, where a reviewer reads it — which is where a decision like "ties are settled by lot" belongs.

## Empty is not absent

```text
no file at all         match -> no ballot file was provided; nothing to count
a file with no rows    match -> a real election in which nobody voted
no file at all         unwrap_or_default() -> 0 ballots, 0 points
a file with no rows    unwrap_or_default() -> 0 ballots, 0 points
```

One of those first two lines is a bug report — an input that never arrived — and after `unwrap_or_default()` the two are the same empty `Vec`. This is [the `Option<Vec<T>>` question](../option_fields/README.md) arriving at the point of use: if *missing* and *empty* mean the same thing to every caller, store a plain `Vec` and drop the `Option` from the type; if they do not, `unwrap_or_default()` is the line that throws the difference away, and it is one word long.

## Where it is exactly right

```rust
*counts.entry(name).or_default() += 1;             // build a tally
counts.get("Dan").copied().unwrap_or_default()     // a candidate nobody approved: 0
let drained = std::mem::take(&mut pending);        // swap the default IN, move the value OUT
```

A candidate nobody approved really did get zero approvals: here the type's zero **is** the domain's answer, and saying so in one word is an improvement. The pattern generalizes to any accumulation whose identity element is the default — counts, sums, empty strings being built up, `Vec`s being drained.

`mem::take` is the same trait used for its other half: it replaces a `&mut` field with `Default::default()` and hands you what was there, which is how you move a non-`Copy` value out of a struct you only borrowed. (Its `Option`-shaped sibling is [`take()`](../option_as_collection/README.md).)

## Three spellings, and where `Default` really earns its keep

| Call | Says |
|---|---|
| `unwrap_or(0)` | this call wants `0` |
| `unwrap_or_else(u32::default)` | this call wants the type's default, named at the call site |
| `unwrap_or_default()` | this call wants whatever the type says |

All three produce the same `0` and, for a `Copy` type, compile to the same instructions — so choose by what a reader should conclude, not by length.

The unambiguously good use of the trait is struct update syntax, which is the same `Default` doing the opposite job:

```rust
let cfg = Config { seats: 3, ..Default::default() };
// -> port 8080, seats 3, title "(untitled election)"
```

Every field you did not name comes from **one impl you can open and read**, and adding a field later does not break the call. That is defaults *stated in one place*; `unwrap_or_default()` at a call site is defaults *supplied at a place that never mentions them*.

None of the three, incidentally, works in a `const` — `unwrap_or_default` is conditionally-const like its siblings ("not yet stable as a const fn"), so [`unwrap` and `expect` remain the whole family there](../unwrap_or/README.md#the-const-fn-in-the-signature-is-not-yet-true).

## If you are coming from another language

- **ABAP** — this is the one that transfers almost too well: every ABAP variable is `unwrap_or_default` by construction. A `TYPE i` starts at `0`, a `TYPE string` at empty, `CLEAR` puts them back, and `IS INITIAL` cannot distinguish "never set" from "deliberately zero" — which is exactly the failure in *Empty is not absent*, promoted to a language rule. `VALUE #( itab[ key = k ] OPTIONAL )` is literally this method. What Rust changes is that the behaviour is **opt-in per type**: a type only has a default if someone wrote one, and asking for one it does not have is a compile error rather than a silent zero.
- **Python** — `collections.defaultdict(int)` and `Counter` are `entry().or_default()`, and they carry the same double edge: reading `d[k]` *creates* the entry, so a lookup mutates the map. `dict.get(k, 0)` is the explicit form, and `dataclasses.field(default_factory=list)` is `Default` for a struct field — including the reason it exists, which is that a shared mutable default is a bug in every language that allows one.
- **SQL** — `COALESCE(col, 0)` is the call-site version; a `DEFAULT` in the DDL is the type version. The difference between the two is this whole page.

---

## Practice

**The type's zero is not your domain's zero.** Derive `Default` on a `Ballot` of three scores, then call `unwrap_or_default()` on a ballot that was never handed in and compare the result with a ballot that scored everyone 0.

Compare them with `==` before you decide the derive was harmless. Then do the same one level up: `None` and `Some(vec![])` both default to an empty `Vec`, and the two facts they came from — *no list was given* and *a list was given and it was empty* — are now indistinguishable.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:unwrap_or_default_kata -->
*[`unwrap_or_default_kata.rs`](examples/unwrap_or_default_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the type's zero is not always your domain's zero.
//!
//!   rustc --edition 2024 unwrap_or_default_kata.rs -o /tmp/uodk && /tmp/uodk

/// Derived `Default` fills every field with its type's zero. Read the result as
/// a ballot and it says something false: a voter who scored everyone 0.
#[derive(Debug, Default, Clone, PartialEq)]
struct Ballot {
    ada: u8,
    ben: u8,
    cara: u8,
}

/// The same data with the domain's answer written down instead.
impl Ballot {
    /// A ballot that was handed in blank still exists — it is not scores of 0.
    fn blank() -> Option<Ballot> {
        None
    }
}

fn main() {
    let handed_in: Option<Ballot> = Some(Ballot { ada: 5, ben: 2, cara: 0 });
    let not_handed_in: Option<Ballot> = Ballot::blank();

    println!("unwrap_or_default fills in the TYPE's zero:");
    println!("  handed in     -> {:?}", handed_in.clone().unwrap_or_default());
    println!("  not handed in -> {:?}", not_handed_in.clone().unwrap_or_default());
    println!("      The second line is a ballot nobody cast, and it is now");
    println!("      indistinguishable from a voter who scored everyone 0:");
    println!("      equal? {}", not_handed_in.clone().unwrap_or_default() == Ballot { ada: 0, ben: 0, cara: 0 });

    println!("\nSo the count has to ask before it defaults:");
    let cast = [handed_in.clone(), not_handed_in.clone(), Some(Ballot { ada: 0, ben: 0, cara: 0 })];
    let turnout = cast.iter().filter(|b| b.is_some()).count();
    let zeroed = cast.iter().flatten().filter(|b| **b == Ballot::default()).count();
    println!("  ballots returned: {turnout} of {}", cast.len());
    println!("  of those, all-zero ballots: {zeroed}");

    println!("\nEmpty is not absent — the same trap one level up:");
    let no_list: Option<Vec<u8>> = None;
    let empty_list: Option<Vec<u8>> = Some(vec![]);
    println!("  None.unwrap_or_default()      -> {:?}", no_list.unwrap_or_default());
    println!("  Some(vec![]).unwrap_or_default() -> {:?}", empty_list.unwrap_or_default());
    println!("      Same value out, two different facts in: 'no list was given'");
    println!("      and 'a list was given and it was empty'. Default erases which.");

    println!("\nWhere it is exactly right — a counter with no entry yet:");
    let seen: Option<u32> = None;
    println!("  seen.unwrap_or_default() + 1 -> {}", seen.unwrap_or_default() + 1);
}
```
<!-- /source -->

<!-- output:unwrap_or_default_kata -->
*Verified output of [`unwrap_or_default_kata.rs`](examples/unwrap_or_default_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
unwrap_or_default fills in the TYPE's zero:
  handed in     -> Ballot { ada: 5, ben: 2, cara: 0 }
  not handed in -> Ballot { ada: 0, ben: 0, cara: 0 }
      The second line is a ballot nobody cast, and it is now
      indistinguishable from a voter who scored everyone 0:
      equal? true

So the count has to ask before it defaults:
  ballots returned: 2 of 3
  of those, all-zero ballots: 1

Empty is not absent — the same trap one level up:
  None.unwrap_or_default()      -> []
  Some(vec![]).unwrap_or_default() -> []
      Same value out, two different facts in: 'no list was given'
      and 'a list was given and it was empty'. Default erases which.

Where it is exactly right — a counter with no entry yet:
  seen.unwrap_or_default() + 1 -> 1
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:unwrap_or_default -->
*Verified output of [`unwrap_or_default.rs`](examples/unwrap_or_default.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: What `Default` hands you
  u8                       0
  i64                      0
  f64                      0.0
  bool                     false
  char                     '\0'
  String                   ""
  Vec<u8>                  []
  Option<u8>               None
  BTreeMap<u8, u8>         {}
  ()                       ()
  None.unwrap_or_default() 0   — the same call as unwrap_or_else(u8::default)
      Nothing here is magic: `unwrap_or_default()` is unwrap_or_else with
      the closure named by a trait bound, `T: Default`. Note char's — the
      null character, not a space — and that Option's own default is None,
      which is why a struct full of Options derives cleanly.

──── Step 2: A derived default is the type's zero, not your domain's
  ballots cast: 40, and the config set no quorum
  #[derive(Default)] -> DerivedQuorum(0)  => quorum met? true
  impl Default (50)  -> HouseQuorum(50)   => quorum met? false
      Same call site, opposite outcome, and the difference is a line of
      code in another file. `derive(Default)` on a newtype means 'zero is
      the sensible fallback' — true for a tally, false for a quorum, a
      threshold, a timeout, a seat count. The third option is the best one
      when there IS no sensible fallback: implement Default for neither,
      and `unwrap_or_default()` stops compiling (E0277). A missing impl is
      a guard rail, not a gap.

──── Step 3: On an enum, the compiler makes you say it out loud
  Tiebreak::default()            -> Lot
  configured.unwrap_or_default() -> Lot
    Lot  <- #[default]
    MostFirstPlaces
    Alphabetical
      `#[derive(Default)]` on an enum does not compile without `#[default]`
      on a variant (E0665: this enum needs a unit variant marked with
      #[default]). There is no first-variant rule and no zero to fall back
      on, so the language refuses to guess — and the attribute you are
      forced to write is a policy decision sitting in the type, where a
      reviewer can see it. Compare Step 2, where a struct's derive makes
      the same class of decision silently.

──── Step 4: Empty is not absent
  no file at all         match -> no ballot file was provided; nothing to count
  a file with no rows    match -> a real election in which nobody voted
  a file with 2 rows     match -> 2 ballots, 8 points
  no file at all         unwrap_or_default() -> 0 ballots, 0 points
  a file with no rows    unwrap_or_default() -> 0 ballots, 0 points
      The first two print the same line once the default is applied, and
      one of them is a bug report: an input that never arrived. This is
      the Option<Vec<T>> question — if 'missing' and 'empty' mean the same
      thing to every caller, store a plain Vec and skip the Option; if they
      do not, unwrap_or_default() is the line that throws the difference
      away, and it is one word long.

──── Step 5: Where it is exactly right: zero as an identity
  approvals: {"Ada": 3, "Ben": 2, "Cara": 1}
  counts.get("Ada").copied().unwrap_or_default() -> 3
  counts.get("Dan").copied().unwrap_or_default() -> 0
  mem::take(&mut pending) -> "row 3: '4x' is not a number", leaving ""
      A candidate nobody approved really did get zero approvals: here the
      type's zero IS the domain's answer, and `or_default` / `unwrap_or_
      default` say so in fewer characters than the alternative. mem::take
      is the same idea used for its other half — swap the default IN to
      move the real value OUT, which is how you take a field out of a &mut.

──── Step 6: Three spellings, and the place Default really earns its keep
  missing.unwrap_or(0)                 -> 0
  missing.unwrap_or_else(u32::default) -> 0
  missing.unwrap_or_default()          -> 0
  Config { seats: 3, ..Default::default() }
    -> port 8080, seats 3, title "(untitled election)"
      All three fallbacks produce the same 0, and for a Copy type they
      compile to the same thing — pick the one that says what you mean.
      Struct update syntax is where Default is unambiguously good: every
      field you did not name is filled from ONE impl you can read, and
      adding a field later does not break the call. That is the same trait
      doing the opposite job — stating the defaults in one place, instead
      of quietly supplying one at a call site that never mentions it.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 17_Option_and_Result/unwrap_or_default/examples/unwrap_or_default.rs -o /tmp/uod && /tmp/uod
```

## See also

- [`unwrap_or`](../unwrap_or/README.md) — the fallback written at the call site, and what it costs
- [`unwrap_or_else`](../unwrap_or_else/README.md) — the same call with the closure written out, and the only form handed the error
- [`Option` fields](../option_fields/README.md) — where the `Option<Vec<T>>` decision is made, one step before this one
- [`Option` is a one-item collection](../option_as_collection/README.md) — `take()`, the `Option`-shaped `mem::take`
- [`Option::unwrap_or_default` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default) · [`Default` ↗](https://doc.rust-lang.org/std/default/trait.Default.html) · [`mem::take` ↗](https://doc.rust-lang.org/std/mem/fn.take.html)

## Po polsku

`unwrap_or_default()` to po prostu `unwrap_or_else(T::default)` z domknięciem zapisanym jako ograniczenie cechy (*trait bound*) `T: Default`. Różnica praktyczna jest jedna, za to poważna: to jedyny człon tej rodziny, którego wartości zapasowej **nie widać w miejscu wywołania** — stoi ona w jakimś `impl Default` albo w `#[derive(Default)]`, zwykle w innym pliku. Na `Result` metoda jest przy tym najbardziej zapominalska z całej rodziny: w jej sygnaturze nie pojawia się nawet typ błędu `E`. Warto też zapamiętać dwie wartości domyślne, które potrafią zaskoczyć: domyślnym znakiem (`char`) jest `'\0'`, a nie spacja — w Ruscie to zwyczajny, poprawny znak, który wyląduje w środku `String` i niczego nie zakończy, inaczej niż w C; a domyślną wartością samej `Option` jest `None`, dzięki czemu struktura złożona z pól `Option` derywuje się bez kłopotu.

Sedno strony mieści się w jednym zdaniu: `Default` daje **zero typu, a nie zero twojej dziedziny**. Przy `#[derive(Default)] struct DerivedQuorum(u32)` brak kworum w konfiguracji daje `0`, więc czterdzieści oddanych kart „spełnia kworum”; przy ręcznym `impl Default` zwracającym `HouseQuorum(50)` te same czterdzieści kart go nie spełnia. To samo miejsce wywołania, przeciwny wynik, a różnicą jest jedna linijka w innym pliku. `derive(Default)` na strukturze opakowującej jest więc twierdzeniem, że zero jest sensowną wartością zapasową — prawdziwym dla licznika czy sumy, fałszywym dla kworum, progu, limitu czasu i liczby mandatów. Jest jeszcze trzecie wyjście, o którym łatwo zapomnieć, bo nie wygląda na wyjście: **nie implementuj `Default` wcale**. Wtedy `unwrap_or_default()` przestaje się kompilować — `E0277`, *the trait bound `Quorum: Default` is not satisfied* — a kompilator wskazuje palcem decyzję, którą się pominęło. Brak implementacji jest barierką ochronną, nie luką, a derywowanie jej „dla porządku” to dokładnie ten ruch, którym barierkę się usuwa. Biblioteka standardowa trzyma się tej samej zasady: `NonZeroU8` nie ma `Default`, bo nie ma zera.

Na wyliczeniu (*enum*) kompilator każe powiedzieć to samo na głos: `#[derive(Default)]` bez `#[default]` na którymś wariancie to `E0665`. Nie ma reguły „pierwszy wariant” ani zera, do którego można by uciec, więc język odmawia zgadywania. Warto zauważyć asymetrię — na strukturze `derive` podejmuje decyzję za każde pole po cichu, na wyliczeniu nie podejmuje żadnej. Atrybut, który trzeba wtedy dopisać, ląduje w definicji typu, czyli tam, gdzie czyta go recenzent kodu, i to jest właściwe miejsce dla ustalenia w rodzaju „remisy rozstrzyga losowanie”. Uwaga dla korzystających ze starszych materiałów: `#[default]` na wariancie działa dopiero od Rusta 1.62, więc dawniejsze poradniki pokazują w tym miejscu ręcznie pisany `impl Default`.

Zostaje różnica między **brakiem a pustką** — po polsku te dwa słowa nie są wymienne, a to właśnie ją kasuje `unwrap_or_default()`. „Nie było pliku z kartami” oraz „był plik, w którym nikt nie zagłosował” to dwa różne fakty, a po tym wywołaniu oba są tym samym pustym `Vec`-em, mimo że pierwszy z nich jest zgłoszeniem błędu. Jeśli dla każdego odbiorcy brak i pustka znaczą to samo, trzymaj zwykły `Vec` i wyrzuć `Option` z typu; jeśli znaczą co innego, `unwrap_or_default()` jest tą jednosłowną linijką, która różnicę kasuje. Bywa jednak dokładnie na miejscu: kandydat, którego nikt nie poparł, naprawdę ma zero poparć — wtedy zero typu **jest** odpowiedzią dziedziny, podobnie jak w `*counts.entry(name).or_default() += 1` albo w `mem::take`, które wstawia wartość domyślną po to, żeby wyjąć prawdziwą. Najbezpieczniejsze zastosowanie samej cechy `Default` jest zaś zupełnie gdzie indziej — w składni aktualizacji struktury: `Config { seats: 3, ..Default::default() }` bierze wszystkie niewymienione pola z **jednej implementacji, którą można otworzyć i przeczytać**, czyli zapisuje wartości domyślne w jednym miejscu, zamiast podsuwać je w wywołaniu, które o nich nie wspomina.

**Szukaj po polsku:** cecha `Default` w Ruscie · wartość domyślna typu · `rust derive Default enum #[default]` · `rust E0277 trait bound Default is not satisfied` · `rust struct update syntax Default::default()`
