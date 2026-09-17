# What a game engine is

**Level:** 201 · working knowledge

**One line:** A game is a loop — read input, advance the world by the time that passed, draw — and the practical difference between a game library, a framework and an engine is who writes that loop: you, or the code that calls yours from inside it.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The loop, once per frame: input, update, render. Why movement is `speed * delta` rather than `speed` per frame — `Time::delta_secs()` as of Bevy 0.19 — so a 144 Hz monitor does not make the game run faster.
- Delta time against a fixed timestep. As of Bevy 0.19, the `FixedUpdate` schedule may run zero, one or more times per frame, and `Time<Fixed>` defaults to 64 Hz — chosen, its docs say, because 60 Hz can alternate between two steps and zero per frame against a 60 Hz display. Glenn Fiedler's [Fix your timestep! ↗](https://gafferongames.com/post/fix_your_timestep/) is the classic account of the accumulator underneath.
- Who owns the loop. [Macroquad ↗](https://macroquad.rs/), "simple and easy to use game library", has you write `loop { … next_frame().await }`. [ggez ↗](https://ggez.rs/), "a lightweight game framework" after LÖVE, runs the loop in `event::run` and calls your `EventHandler`'s `update` and `draw`. Bevy's `App::run` (as of 0.19) hands the loop to a runner. [Fyrox ↗](https://fyrox.rs/) calls itself "a feature-rich game engine". Which of those differences matters for a first game?
- "Hello, world" as Bevy 0.19.1's own example writes it: `App::new().add_systems(Update, hello_world_system).run()`, with no `DefaultPlugins`. How many times does it print, and why? The `App::set_runner` docs (as of 0.19) say a main loop, if wanted, *is the responsibility of the runner function*.
- [`rusty_engine` ↗](https://github.com/CleanCut/rusty_engine): Nathan Stocks' 2D engine for people learning Rust, described in its README as a simplification wrapper over Bevy. Its 7.0.0 release on crates.io depends on `bevy ^0.18`, one release behind these pages, so its API is its own and not Bevy 0.19's.
- What an engine adds beyond the loop: windowing per platform, asset loading, a renderer, audio, input. Which of those does a small 2D game need on day one?
- Why Macroquad's `main` is `async`: its README says `async`/`await` is used for exactly one problem, organising the main loop across platforms — a blocking loop is hard to run on WASM. What does that say about [Shipping a wasm page](../../42_WebAssembly/shipping_a_wasm_page/README.md)?

## The trap it exists for

Moving an object a fixed distance every frame. It looks right on the machine it was written on, runs at double speed on a 120 Hz display and in slow motion whenever a frame takes too long — the distance has to be multiplied by elapsed time, or the logic moved to a fixed timestep.

## Where this sits

[Entities and components](../entities_and_components/README.md) and [Systems and queries](../systems_and_queries/README.md) cover what Bevy runs inside the loop; this page covers only the loop and who owns it.

## See also

- [Systems and queries](../systems_and_queries/README.md) — the functions the loop calls
- [Two clocks: `Instant` and `SystemTime`](../../33_Time_and_Benchmarking/two_clocks/README.md) — the monotonic clock under every delta time
- [Timing a block](../../33_Time_and_Benchmarking/timing_a_block/README.md) — measuring a frame the way the loop does
- [Entities and components](../entities_and_components/README.md) — the world the loop advances
- [Books](../../10_Resources/books/README.md) — *Hands-on Rust* and the game tutorials
- [Event loop ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/scheduling/event_loop/) — the same shape outside games
- [Game Loop, in *Game Programming Patterns* ↗](https://gameprogrammingpatterns.com/game-loop.html)

## If you are coming from another language

- **Python.** A pygame program writes its own `while` loop and calls `Clock.tick(framerate)` once per frame, which returns the milliseconds since the previous call — Macroquad's shape, with the delta time handed back to you. Bevy moves the loop out of your code and gives you `Time` as a resource instead.
- **C#.** Unity's `Update` runs once per rendered frame and `FixedUpdate` at a fixed interval — 0.02 s by default, so possibly zero, one or several times per frame, per [its documentation ↗](https://docs.unity3d.com/ScriptReference/MonoBehaviour.FixedUpdate.html). Bevy's `Update` and `FixedUpdate` schedules are the same split, and the same trap applies to input read in the fixed one.
- **Go.** Ebitengine's `Game` interface has `Update` and `Draw`, and its docs say `Update` is called TPS times per second — 60 by default — so a fixed step is the default rather than the option. In Bevy the default schedule is per frame and the fixed step is the one you opt into.

## Po polsku

Gra to pętla: odczytaj wejście, przesuń świat o czas, który upłynął, narysuj klatkę. Różnica między biblioteką (Macroquad), frameworkiem (ggez) a silnikiem (Bevy, Fyrox) sprowadza się w praktyce do pytania, **kto pisze tę pętlę** — ty czy kod, który wywołuje twój. Najczęstszy błąd początkujących to przesuwanie obiektu o stałą wartość w każdej klatce, przez co gra przyspiesza na monitorze 120 Hz; prędkość trzeba mnożyć przez czas klatki (*delta time*) albo przenieść logikę do stałego kroku czasowego (`FixedUpdate`, domyślnie 64 Hz w Bevy 0.19).

**Szukaj po polsku:** pętla gry · stały krok czasowy · silnik gier a biblioteka · `fix your timestep` · `bevy fixedupdate delta time` · `macroquad vs bevy vs ggez`
