# An enum as a state machine

**Level:** 201 · working knowledge

**One line:** Match on a *tuple* of two enums and the compiler enumerates the whole transition table for you — every state crossed with every event, and it names the cells you left out.

```rust
fn step(state: State, event: Event) -> State {
    match (state, event) {
        (Blank, Mark) => Marked,
        (Marked, Submit) => Cast,
        // ...
    }
}
```

Two enums — one for what the thing *is*, one for what just *happened* — and a function from the pair to the next state. That is the whole pattern, and the reason to write it this way rather than with `if` is in the next section.

---

## The compiler checks the table, not just the states

A `match` on `(State, Event)` is exhaustive over the **cross product**. Three states and three events means nine cells, and leaving one out is a build error that says which:

```text
error[E0004]: non-exhaustive patterns: `(State::Idle, Event::Stop)` and
              `(State::Running, Event::Start)` not covered
 --> sm.rs:6:11
  |
6 |     match (state, event) {
  |           ^^^^^^^^^^^^^^ patterns `(State::Idle, Event::Stop)` and
  |                          `(State::Running, Event::Start)` not covered
```

That is a design review you get for free. *What happens if a submit arrives before anything is marked?* is a question the compiler will not let you leave unanswered — and "nothing happens" is a perfectly good answer, it just has to be written down.

## The one line that throws it away

```rust
match (state, event) {
    (Blank, Mark) => Marked,
    (Marked, Submit) => Cast,
    _ => state,               // <- the audit ends here
}
```

Every table you find in the wild ends this way, and it converts the compiler's nine-cell checklist into two cells and a shrug. Add a fourth event and no build breaks; the machine simply ignores it. This is the same trade as [`if let`](../../17_Option_and_Result/if_let/README.md) and the same accident as [a typo becoming a binding](../a_typo_becomes_a_binding/README.md) — exhaustiveness is not something an enum gives you, it is something a `match` can decline.

The middle position, when nine arms really are too many, is a **partial** wildcard:

```rust
(Cast, _) => Cast,     // a cast ballot is final, whatever arrives
```

That still forces an arm for every new *state* while absorbing every new *event*. Which half you want checked is a real decision — write the one you mean rather than the one that is shortest.

## Grouping without losing the check

`|` collapses several cells into one arm and keeps them named, so the table stays complete and stays readable:

```rust
(Cast, Mark) | (Cast, Submit) | (Cast, Void) => Cast,
```

Three cells, one answer, and a fourth event still breaks the build. [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) is the page on that, including the two lints that catch a collapse done wrong.

## Why the states are a type and not a `bool`

Two booleans — `is_marked`, `is_cast` — describe four states, and one of them (`cast` but not `marked`) is nonsense that every function reading them has to defend against. Three named variants describe three. That is the [product-versus-sum](../variants_that_carry_data/README.md) argument again, and a state machine is where it bites hardest, because the impossible states are exactly the ones that produce the bug reports nobody can reproduce.

## What this pattern does not give you

`step` returns a `State`, so *"the event was refused"* and *"the event was accepted and changed nothing"* arrive as the same answer. If a caller needs to tell those apart, the return type has to say so — `Option<State>`, or a small `Transition` enum. The compiler checked the table's *completeness*; it has no opinion about whether the answers are right.

Nor does anything stop a caller constructing `State::Cast` directly and skipping the machine. Keeping the variants private to a module and exposing only `step` is what makes the transition table the only way through.

## If you are coming from another language

**Python.** The nearest idiom is a `dict` keyed by `(state, event)`, and it is genuinely close — the same table, written as data. What is missing is the completeness check: a missing key is a `KeyError` at runtime, in production, on the transition nobody tested, and `.get(key, state)` is the `_ => state` arm with the same consequence. `match` statements plus `enum.Enum` get you the shape; only a strict type checker gets you the audit.

**ABAP.** A nested `CASE` on a status field, or a Z-table of transitions read at runtime. Both work, and neither can tell you that a status was added last sprint and one of the nine branches was never written — that is a code inspection, done by a person, every time. This is the single clearest place where Rust's `CASE` equivalent does something ABAP's cannot.

**C.** A `switch` inside a `switch`, or a two-dimensional array of function pointers indexed by state and event. The array is the better version and is the direct ancestor of this pattern: it forces you to fill in every cell, because the array has a fixed size. Rust's `match` is that array with names instead of indices — and with the compiler, rather than a segfault, telling you when the dimensions change.

## Practice

**Match on a tuple and get the whole table.** Define four states and four events, and write `next(state, event) -> State` by matching on `(state, event)`. Write out every cell — no `_` arm anywhere.

Then delete one arm and record exactly what the compiler says. Finish with the design question: why is a `_ => state` arm the wrong thing to write here, when it would compile today and save you eight lines?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:an_enum_as_a_state_machine_kata -->
*[`an_enum_as_a_state_machine_kata.rs`](examples/an_enum_as_a_state_machine_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: match on a tuple and the compiler writes out the whole table.
//!
//!   rustc --edition 2024 an_enum_as_a_state_machine_kata.rs -o /tmp/esm && /tmp/esm

#[derive(Debug, Clone, Copy, PartialEq)]
enum State { Idle, Loading, Ready, Failed }

#[derive(Debug, Clone, Copy)]
enum Event { Start, Loaded, Error, Reset }

fn next(state: State, event: Event) -> State {
    use Event::*;
    use State::*;
    match (state, event) {
        (Idle, Start) => Loading,
        (Loading, Loaded) => Ready,
        (Loading, Error) => Failed,
        (Failed, Reset) => Idle,
        (Ready, Reset) => Idle,

        // Every remaining cell, written out rather than swept up by `_`. The
        // compiler required them, which is the point: it enumerated four
        // states times four events and would not build until all sixteen were
        // accounted for.
        (Idle, Loaded) | (Idle, Error) | (Idle, Reset) => state,
        (Loading, Start) | (Loading, Reset) => state,
        (Ready, Start) | (Ready, Loaded) | (Ready, Error) => state,
        (Failed, Start) | (Failed, Loaded) | (Failed, Error) => state,
    }
}

fn main() {
    println!("THE TRANSITION TABLE, RUN");
    let mut s = State::Idle;
    for e in [Event::Start, Event::Loaded, Event::Reset, Event::Start, Event::Error] {
        let before = s;
        s = next(s, e);
        println!("  {:<8} + {:<7} -> {:?}", format!("{before:?}"), format!("{e:?}"), s);
    }
    println!();

    println!("THE WHOLE GRID");
    use Event::*;
    use State::*;
    print!("  {:<9}", "");
    for e in [Start, Loaded, Error, Reset] { print!("{:<9}", format!("{e:?}")); }
    println!();
    for st in [Idle, Loading, Ready, Failed] {
        print!("  {:<9}", format!("{st:?}"));
        for e in [Start, Loaded, Error, Reset] {
            print!("{:<9}", format!("{:?}", next(st, e)));
        }
        println!();
    }
    println!();

    println!("WHAT THE COMPILER DID FOR YOU");
    println!("  Matching on the TUPLE (State, Event) makes the match's subject");
    println!("  the cross product, so exhaustiveness checking covers all 4 x 4");
    println!("  = 16 cells. Delete any one arm and E0004 names the exact pair");
    println!("  you left out -- `(Ready, Error)` and so on.");
    println!();
    println!("  Add a fifth state and every incomplete match in the program");
    println!("  fails to build, with the missing cells listed. That is a design");
    println!("  review the compiler performs for free, every time.");
    println!();

    println!("THE ARM THAT THROWS IT AWAY");
    println!("  `_ => state` at the bottom would compile today and would also");
    println!("  silently absorb every cell you add later. On a state machine");
    println!("  that is exactly the code you do NOT want to write: the value of");
    println!("  the enum is that forgetting is a build error.");
    println!();
    println!("  Writing the ignored cells out as `(Idle, Loaded) | ...` keeps");
    println!("  that guarantee AND documents that they were considered -- which");
    println!("  a reader cannot tell from a wildcard.");

    assert_eq!(next(State::Idle, Event::Start), State::Loading);
    assert_eq!(next(State::Ready, Event::Error), State::Ready);
}
```
<!-- /source -->

<!-- output:an_enum_as_a_state_machine_kata -->
*Verified output of [`an_enum_as_a_state_machine_kata.rs`](examples/an_enum_as_a_state_machine_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
THE TRANSITION TABLE, RUN
  Idle     + Start   -> Loading
  Loading  + Loaded  -> Ready
  Ready    + Reset   -> Idle
  Idle     + Start   -> Loading
  Loading  + Error   -> Failed

THE WHOLE GRID
           Start    Loaded   Error    Reset    
  Idle     Loading  Idle     Idle     Idle     
  Loading  Loading  Ready    Failed   Loading  
  Ready    Ready    Ready    Ready    Idle     
  Failed   Failed   Failed   Failed   Idle     

WHAT THE COMPILER DID FOR YOU
  Matching on the TUPLE (State, Event) makes the match's subject
  the cross product, so exhaustiveness checking covers all 4 x 4
  = 16 cells. Delete any one arm and E0004 names the exact pair
  you left out -- `(Ready, Error)` and so on.

  Add a fifth state and every incomplete match in the program
  fails to build, with the missing cells listed. That is a design
  review the compiler performs for free, every time.

THE ARM THAT THROWS IT AWAY
  `_ => state` at the bottom would compile today and would also
  silently absorb every cell you add later. On a state machine
  that is exactly the code you do NOT want to write: the value of
  the enum is that forgetting is a build error.

  Writing the ignored cells out as `(Idle, Loaded) | ...` keeps
  that guarantee AND documents that they were considered -- which
  a reader cannot tell from a wildcard.
```
<!-- /output -->

</details>

## The verified output

[`examples/an_enum_as_a_state_machine.rs`](examples/an_enum_as_a_state_machine.rs) compiled and run — a ballot driven through six events, including two that are refused:

<!-- output:an_enum_as_a_state_machine -->
*Verified output of [`an_enum_as_a_state_machine.rs`](examples/an_enum_as_a_state_machine.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
from     event    to
----------------------------
Blank    Submit   Blank    no change
Blank    Mark     Marked
Marked   Void     Blank
Blank    Mark     Marked
Marked   Submit   Cast
Cast     Void     Cast     no change

final state: Cast
```
<!-- /output -->

---

## See also

- [What an enum is](../what_an_enum_is/README.md) — the declaration and exhaustiveness, from the start
- [Variants that carry data](../variants_that_carry_data/README.md) — why three variants beat two booleans
- [A typo becomes a binding](../a_typo_becomes_a_binding/README.md) — the other way a table stops being audited
- [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) — `|` groups, ranges, and their lints

## Po polsku

Cały wzorzec mieści się w jednym zdaniu: dwa wyliczenia — jedno na to, **czym rzecz jest** (stan), drugie na to, **co się właśnie stało** (zdarzenie) — plus funkcja z pary do następnego stanu. Sztuka polega na dopasowaniu do **krotki**: `match (state, event)` musi być wyczerpujący na iloczynie kartezjańskim obu wyliczeń, więc trzy stany i trzy zdarzenia dają dziewięć komórek, a kompilator wymienia z nazwy te, których nie wypełniłeś. Kto rysował kiedykolwiek tablicę przejść automatu skończonego, dostaje tutaj dokładnie tę tablicę, tylko sprawdzaną przy każdej kompilacji zamiast na tablicy w sali. Pytanie „co się stanie, jeśli karta zostanie oddana, zanim cokolwiek na niej zaznaczono?” przestaje więc być czymś, o czym da się zapomnieć — a „nic się nie dzieje” jest w pełni dobrą odpowiedzią, którą trzeba tylko zapisać.

Jedna linijka to wszystko unieważnia: `_ => state`. Tak kończy się właściwie każda tablica przejść, jaką spotkasz w cudzym kodzie, i zamienia dziewięciopunktową listę kontrolną w dwie komórki i wzruszenie ramion — dorzucone czwarte zdarzenie nie psuje już żadnej kompilacji, automat po prostu je zignoruje. Rozwiązaniem pośrednim, gdy dziewięć ramion naprawdę jest za dużo, jest **częściowy** symbol wieloznaczny: `(Cast, _) => Cast` wymusza ramię dla każdego nowego *stanu*, a pochłania każde nowe *zdarzenie*. To realna decyzja projektowa o tym, która połowa ma być dalej pilnowana, więc pisz tę, którą masz na myśli, a nie tę krótszą. Kilka komórek z tą samą odpowiedzią łączy się przez `|` i wtedy nic się nie traci, bo każda z nich dalej stoi w kodzie wymieniona z nazwy.

Warto na koniec wiedzieć, czego kompilator **nie** sprawdził, bo łatwo tu o nadmierną pewność siebie. Sprawdził kompletność tablicy, a nie sensowność odpowiedzi. Skoro `step` zwraca `State`, to „zdarzenie odrzucone” i „zdarzenie przyjęte, tylko nic nie zmieniło” docierają do wywołującego jako ta sama wartość — jeśli ma je rozróżniać, musi o tym powiedzieć typ zwracany (`Option<State>` albo małe wyliczenie `Transition`). Druga dziura jest po stronie widoczności: nic nie broni wywołującemu zbudować `State::Cast` wprost i ominąć automat. Dopiero warianty prywatne dla modułu i jedno publiczne `step` sprawiają, że tablica przejść jest jedyną drogą przez ten typ.

**Szukaj po polsku:** automat skończony · maszyna stanów · tablica przejść · `rust state machine enum match tuple` · `rust make illegal states unrepresentable`
