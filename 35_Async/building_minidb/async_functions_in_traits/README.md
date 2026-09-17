# Async functions in traits

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A trait may declare `async fn` [since Rust 1.75 ↗](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits/), but the future each method returns carries no `Send` promise — so a spawned actor that is generic over the trait is refused, until the trait states the bound itself with `-> impl Future<Output = …> + Send`.

## What it has to cover

- A `Durability` trait with an `async fn append`, and three implementations: the file log from chapter 9, an in-memory log for tests, and a no-op
- **The error.** Making the store actor generic over `D: Durability` and spawning it — the full compiler message, and why it points at `tokio::spawn` rather than at the trait
- Why the trait cannot promise `Send` by itself: each implementation's future is a different type, and the bound has to be written where callers can rely on it
- **The fix:** `impl Future<Output = …> + Send` in return position in the trait, and what it costs implementations that would not have been `Send`
- The other options, and when each is worth it: the `trait-variant` crate, a boxed future (`async-trait`), and naming the bound at the call site
- **What `dyn` does and does not allow.** A trait with `async fn` is not dyn compatible; `Box<dyn Durability>` needs boxed futures — or an enum of the three backends instead

## The trap it exists for

Reading the error as a problem with the spawn. The spawn is correct; the promise is missing from a trait in another file, and no amount of editing near `tokio::spawn` adds it.

## What minidb gains

Durability chosen at startup — file, memory or none — behind one trait, which chapter 15 also relies on.

## See also

- [Static vs dynamic dispatch](../../../12_Traits/static_vs_dynamic_dispatch/README.md) — `dyn` compatibility, and the enum as a third strategy
- [Returning a trait](../../../12_Traits/returning_a_trait/README.md) — `impl Trait` in return position outside a trait
- [`Send` and `Sync`](../../../09_Advanced/send_and_sync/README.md) — the bound that is missing
- [Tasks](../tasks/README.md) — where `Send + 'static` first appeared

## Po polsku

Od Rusta 1.75 trait może deklarować `async fn`, ale future zwracany przez taką metodę nie obiecuje, że jest `Send`. Aktor uogólniony po takim traicie i uruchomiony przez `tokio::spawn` zostaje więc odrzucony, a komunikat błędu wskazuje na `spawn`, nie na trait, w którym brakuje obietnicy. Rozwiązaniem jest zapisanie ograniczenia w samym traicie: `-> impl Future<Output = …> + Send`. Trait z `async fn` nie nadaje się też do `dyn` bez opakowywania future’ów w `Box`.

**Szukaj po polsku:** async fn w traitach · `rust async fn in trait send bound` · `trait_variant make send` · `async trait dyn compatible`
