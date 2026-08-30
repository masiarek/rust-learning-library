# The elevator kata

**Level:** reference · a project, not a lesson

**One line:** Three well-known katas share the word *lift* and almost nothing else — pick one before you name a single type, because their data models do not merge.

It is the smallest problem that is about **state over time** rather than about a function, which is why every language's practice list has one. It is also where people quietly build three half-programs in one repository and stall.

## Three katas, not one

| | [Codewars "The Lift" ↗](https://www.codewars.com/kata/58905bfa1decb981da00009e) | [Lift-Kata ↗](https://kata-log.rocks/lift-kata) (Emily Bache) | [elevator-kata ↗](https://github.com/andreyors/elevator-kata) |
|---|---|---|---|
| Entry point | `the_lift(queues, capacity) -> Vec<u32>` | `LiftSystem::tick()` | `call(floor, dir)` + tick |
| Time | **none** — batch, runs to completion | a tick | a tick |
| Lifts | one | many (A–D) | many |
| People | queues of destinations | **none modelled** — calls and requests only | passengers |
| Capacity | central to the puzzle | absent | absent |
| Doors, DING, monitor | absent | central | central |
| Output | a `Vec<u32>` of stops | the rendered floor grid | an event sequence |
| Tests assert | equality on a vector | the printed grid, approved | that events happened in order |

Codewars has **people but no time**. Bache has **time but no people**. A `Lift` type that serves both ends up with a `capacity` field nothing reads and a `tick` that some callers must not call.

## The one thing they share

One pure function, with no time, no doors and no people in it:

```rust
// given where I am, where I am heading, and everywhere I still owe a stop —
// what is the next floor I stop at?
fn next_stop(at: u32, heading: Direction, pending: &BTreeSet<u32>) -> Option<u32> { … }
```

Every rule in every version of the kata is a rule about *that* answer. The lift never reverses while anything remains ahead of it; an empty lift continues in its current direction to the furthest call; nowhere left to go is `None`, which is [not the same as floor 0](../../../17_Option_and_Result/partial_functions/README.md).

Build it alone, with its own tests. Then Codewars is one shell around it (loop to completion, record the stops) and Bache is another (advance one tick). The ASCII floor grid is a third — a renderer that takes a snapshot and reaches into nothing.

## The four traps in the Codewars one

Each is a test you will fail without knowing why, so write a case for each *before* the algorithm:

1. **Consecutive duplicates collapse.** Stopping at floor 2 twice in a row is one entry in the output.
2. **A full lift still stops.** The stop is recorded even when nobody can board.
3. **A rejected passenger presses again.** So the same floor can legitimately reappear on a later pass — which collides with trap 1, and that collision is the kata.
4. **The trailing ground floor.** Return to 0 at the end, but do not emit `0` if you are already standing on it.

## An order to write it in

Time is the hard part, and it is optional for a long while. Stay in the batch model until the policy is right.

| Step | What is new | Stop when |
|---|---|---|
| 1 | one lift, one waiting passenger, infinite capacity | it goes there and comes home |
| 2 | several passengers, all travelling the same way | no reversals mid-journey |
| 3 | both directions | the *"never turn while anything is ahead"* rule holds |
| 4 | capacity | traps 2 and 3 above pass |
| 5 | the empty-lift rule | the whole Codewars suite is green |
| 6 | **now** introduce `tick()` | the batch tests still pass, unchanged |
| 7 | doors, monitor, DING | — |
| 8 | more than one lift | — |

Step 6 is the checkpoint that matters: if adding time breaks the policy tests, time leaked into the policy.

## Simulating time: a tick, or an event queue

A `tick()` advances every actor by one unit whether or not anything happens, which is why the kata prescribes it — it is trivial to write and trivial to test. It also spends most of its iterations doing nothing, and it forces every duration to be an integer multiple of the tick.

The alternative is **discrete-event simulation**: keep a priority queue of *(time, event)*, pop the earliest, jump the clock straight to it. Nothing is simulated between events. [Salabim ↗](https://www.salabim.org/) is the Python library for this, and its own documentation uses a bank queue and — inevitably — a lift.

Worth building the tick version first and then asking what the event-queue version would look like. The answer is short: `pending` stops being a set of floors and becomes a heap of scheduled arrivals, and the shell disappears.

## What it teaches, and where that lives here

The project is far too big to be [a kata in this library](../../../KATAS.md) — those are single-file, `std`-only, and compiled by CI. But it slices into pieces that are not:

| Slice | Lesson |
|---|---|
| `Direction` and `DoorState`, and `match (state, event)` | [An enum as a state machine](../../../13_Enums/an_enum_as_a_state_machine/README.md) — the lift is the canonical example |
| `Direction` rather than `bool going_up` | [An enum instead of a bool](../../../13_Enums/an_enum_instead_of_a_bool/README.md) |
| `use super::` vs `use crate::` from a nested test module | [The `use` declaration](../../../27_Modules/the_use_declaration/README.md) |
| A passenger-id counter that must not live inside a passenger | [`const` and `static`](../../../27_Modules/const_and_static/README.md) |
| `next_stop() -> Option<u32>` | [Partial functions](../../../17_Option_and_Result/partial_functions/README.md) |
| One file per concern, and what `mod` actually declares | [One module per file](../../../27_Modules/one_module_per_file/README.md) |

And three pages this library does not have yet, all of which the elevator motivates better than anything else would:

- **[`VecDeque` ↗](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)** — a floor queue is `pop_front`, and [`Vec`](../../../26_Collections/the_vec/README.md)`::remove(0)` is O(n). [Collections](../../../26_Collections/README.md) has no deque page.
- **[`BTreeSet` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)** — ordered, deduplicating, and `range(current..)` answers *"the next stop above me"* in one call. It is the elevator's core data structure.
- **Snapshot testing** — [Testing](../../../28_Testing/README.md) covers assertions, placement and doctests, and nothing on approving a rendered blob. Bache's kata is built around exactly that, and the floor grid is the artefact you approve.

## The shape on disk

```text
lift/
  Cargo.toml          [workspace]
  core/               the scheduling policy — no time, no doors, no people
  codewars/           the batch shell: queues + capacity -> Vec<u32>
  system/             the tick shell: doors, monitors, several lifts
  render/             snapshot -> the floor grid
```

Three things that cost more than they look:

- **Make the crate a library.** A `src/main.rs`-only crate cannot be imported by an integration test in `tests/` or by a doctest, so the whole suite has to live inside `src/`. Add `src/lib.rs` and reduce `main.rs` to a demo.
- **Do not silence the warnings.** `unused = "allow"` and `dead_code = "allow"` in `Cargo.toml` mute exactly the messages that tell you which parameter you have not wired up yet — on a kata built upward from a stub, that list [is the to-do list](../../../15_First_Programs/what_a_warning_is_asking/README.md).
- **Put clippy in CI, not just in a `#![warn]` line.** `cargo clippy -- -D warnings` and `cargo fmt --check`; see [strict lints](../../../05_Tooling/strict_lints/README.md).

## See also

- [Exercises](../README.md) — the four practice tracks, and which stage each is for
- [A tree of practice projects](../../../05_Tooling/practice_workspace/README.md) — where to keep this on disk
- [The long way round to a STAR count](../../../ROADMAP.md) — this library's own build-something-instead
- [masiarek/elevator_kata ↗](https://github.com/masiarek/elevator_kata) — the Rust attempt this page was written for
- [emilybache/Lift-Kata ↗](https://github.com/emilybache/Lift-Kata) — starting code in seven languages (C++, C#, Go, Java, JS, PHP, Python) — Rust not among them

## Po polsku

Po polsku winda jest jedna, a po angielsku dwie — brytyjski *lift* i amerykański *elevator* — i to jest część kłopotu opisanego na tej stronie. Szukając raz „lift kata”, raz „elevator kata”, trafia się na trzy różne zadania, które z daleka wyglądają na jedno, a nie dają się złożyć w jeden model danych: wersja z Codewars ma pasażerów i pojemność, ale **nie ma czasu** (liczy się wsadowo, do końca), wersja Emily Bache ma czas, drzwi i DING, ale **nie modeluje pasażerów** w ogóle. Typ `Lift`, który miałby obsłużyć obie, kończy z polem `capacity`, którego nikt nie czyta, i z metodą `tick()`, której części wywołujących nie wolno wywołać. Wybierz jedną wersję, zanim nazwiesz pierwszy typ.

Wspólna jest dokładnie jedna rzecz — czysta funkcja bez czasu, drzwi i ludzi: „jestem tu, jadę w tę stronę, mam jeszcze do obsłużenia te piętra — gdzie zatrzymam się następnym razem?”. Warto ją napisać osobno, z własnymi testami, bo każda reguła każdej wersji kata jest regułą o **tej** odpowiedzi. Dwie decyzje typologiczne, które od razu ustawiają rozwiązanie: kierunek to wyliczenie (*enum*) `Direction`, a nie `bool going_up` (bo `false` nie mówi, czy „w dół”, czy „nie wiadomo”), a wynik to `Option<u32>`, gdzie `None` znaczy „nie ma dokąd jechać”. W polskiej numeracji pięter tę drugą różnicę widać wyjątkowo dobrze: parter to nie jest „piętro 0”, tylko osobna nazwa — i tak samo `None` to nie jest zero. `Option` wymusza tę różnicę w typie, więc kompilator nie pozwoli o niej zapomnieć.

Kolejność budowania jest tu ważniejsza niż algorytm, bo czas jest najtrudniejszą częścią i najdłużej niepotrzebną. Zostań w modelu wsadowym aż polityka jazdy będzie poprawna, a `tick()` dodaj dopiero jako krok szósty — i traktuj to jako punkt kontrolny: **jeśli dodanie czasu psuje testy polityki, to znaczy, że czas wyciekł do polityki**. I jedna rzecz, którą łatwo zrobić odruchowo, a która kosztuje najwięcej: nie wyciszaj ostrzeżeń przez `dead_code = "allow"` w `Cargo.toml`. W projekcie budowanym od zaślepki w górę lista ostrzeżeń kompilatora *jest* listą rzeczy do zrobienia.

**Szukaj po polsku:** kata windy · symulacja zdarzeniowa · `elevator kata` · `lift kata` · `rust BTreeSet range`
