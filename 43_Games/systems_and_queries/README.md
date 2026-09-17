# Systems and queries

**Level:** 201 · working knowledge

**One line:** A Bevy system is an ordinary function whose parameter types — `Query<&mut Transform, With<Enemy>>`, `Res<Time>` — declare what it reads and what it writes, which lets the scheduler run systems that do not overlap in parallel, and makes an overlap inside one system a panic rather than a compile error (as of Bevy 0.19).

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- A system is a function, registered with `App::add_systems(Update, move_enemies)`; its parameters are system parameters — `Query`, `Res`, `ResMut`, `Commands`, `Single` — and the body never sees the whole `World` (all as of Bevy 0.19).
- Reading a `Query` (as of Bevy 0.19): data such as `&T`, `&mut T` and `Entity`; filters `With`, `Without` and `Changed`; `iter` and `iter_mut` over every match, `get` for one entity, and `single` or the `Single` parameter for exactly one.
- The borrow rule moved to run time: `Query<&mut Transform, With<Enemy>>` next to `Query<&Transform, With<Player>>` in one system is Bevy error [B0001 ↗](https://bevy.org/learn/errors/b0001/), and Bevy's page says it *will panic*. Its two fixes: add `Without<Enemy>` to the player query so the two can never match the same entity, or wrap both in a `ParamSet`. [B0002 ↗](https://bevy.org/learn/errors/b0002/) is the same rule for `Res<T>` and `ResMut<T>`.
- Why `rustc` cannot catch B0001: two `Query` values are two parameters, and whether their entities overlap is a fact about the world, not about the types. [Borrowing](../../18_Ownership/borrowing/README.md) sees two distinct values, and it is right to.
- Scheduling: systems in one schedule with no conflicting access may run at the same time. `before`, `after` and `chain` (as of Bevy 0.19) set an order — what does Bevy report when two systems write the same component with no order between them, and does it matter?
- `Commands` are deferred. Bevy 0.19's docs say a command that changes the world is queued and applied later by the `ApplyDeferred` system — so is an entity spawned in one system visible to a query later in the same frame?
- Parallelism inside one system: `Query::par_iter_mut` (as of Bevy 0.19), and when splitting a thousand entities across threads costs more than it saves — [data parallelism ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/parallelism/data_parallelism/) with a measurement.
- A 0.19 change: resources are now entities, so broad queries such as `Query<EntityMut>` conflict with `Res` in the same system; the migration guide's fix is a `Without<IsResource>` filter.

## The trap it exists for

Writing the obvious system — move every enemy toward the player — with one mutable and one shared `Query` over `Transform`. `cargo build` passes. The app panics with B0001 when it runs, and the fix is not in the function body but in the filter that proves the two queries never meet.

## Where this sits

[Entities and components](../entities_and_components/README.md) covers the data these functions query; [Resources and plugins](../resources_and_plugins/README.md) covers `Res` and `ResMut` in full. This page covers only the function, its queries and how the scheduler runs it.

## See also

- [Borrowing: `&T`, `&mut T`, and where a borrow ends](../../18_Ownership/borrowing/README.md) — the compile-time rule B0001 enforces at run time
- [Sharing across threads: `Arc`](../../18_Ownership/sharing_across_threads/README.md) — what the scheduler spares you from writing
- [`Send` and `Sync`](../../09_Advanced/send_and_sync/README.md) — the bounds `Component` carries (`Send + Sync + 'static`, as of Bevy 0.19), so the thread pool may touch it
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — what `query.iter()` returns before anything runs
- [Entities and components](../entities_and_components/README.md) — the columns a query walks
- [Data race freedom ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/safety_in_languages/data_race_freedom/) — the guarantee the access declarations preserve
- [Bevy 0.18 to 0.19 migration guide ↗](https://bevy.org/learn/migration-guides/0-18-to-0-19/)

## If you are coming from another language

- **C++.** A job system can ask each job to list what it reads and writes, but nothing in the language checks that list against the job's code. In Bevy the list *is* the parameter list, so there is no undeclared access to forget, and a conflicting pair is reported with an error code.
- **Go.** Starting each system in its own goroutine would be easy and would race on shared components. Bevy's scheduler avoids that without a `Mutex` in your code: it does not run two systems at once when their declared access conflicts.
- **Python.** Picture type hints that the runtime obeys: Bevy reads each system's parameter types to decide what to pass in and what may run alongside it.

## Po polsku

System w Bevy to zwykła funkcja, a typy jej parametrów — `Query<&mut Transform, With<Enemy>>`, `Res<Time>` — deklarują, co czyta i co zapisuje. Dzięki temu harmonogram (*scheduler*) może uruchamiać równolegle systemy, które się nie nakładają. Reguła pożyczania działa tu jednak w czasie wykonania: dwa zapytania o `Transform`, jedno mutowalne, w jednym systemie to błąd B0001 i panika, bo kompilator nie wie, że gracz nigdy nie jest wrogiem — trzeba mu to powiedzieć filtrem `Without` albo użyć `ParamSet`.

**Szukaj po polsku:** systemy i zapytania w Bevy · równoległe systemy ECS · `bevy B0001 query conflict` · `bevy paramset without` · `bevy commands deferred apply_deferred`
