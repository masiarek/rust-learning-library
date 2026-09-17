# Games with Bevy

**One line:** A game is a loop that reads input, advances a world and draws it, many times a second — and Bevy organises that world as an entity–component system: entities that are only ids, components that are plain structs, and systems that are ordinary functions the engine schedules over them.

This section follows the Ultimate Bevy outline — getting started, the ECS, input, graphics, audio, UI, the web — and uses [Bevy ↗](https://bevy.org/), whose systems declare what they borrow in their parameter types — `Query<&mut Transform>` — so Rust's ownership rules shape the engine's own API. It is not a graphics-programming or shader course, and it is not a survey of engines beyond the first page's comparison. For a gentler start, Nathan Stocks' [`rusty_engine` ↗](https://github.com/CleanCut/rusty_engine) wraps Bevy in a small 2D API for learners; [What a game engine is](what_a_game_engine_is/README.md) says where it fits. Game books — *Hands-on Rust*, the Sokoban and roguelike tutorials — are on [Books](../10_Resources/books/README.md).

**Pinned to Bevy 0.19.** The latest stable release on crates.io is 0.19.1, a 0.20 release candidate is already published, and Bevy renames things between releases: the buffered `EventReader` of Bevy 0.16 is `MessageReader` by 0.17. Every API name on these pages is written *as of Bevy 0.19* and was looked up on docs.rs for 0.19.1; where a behaviour could not be checked, the page asks rather than states.

| Lesson | Level | What it covers |
|---|---|---|
| [What a game engine is](what_a_game_engine_is/README.md) | 201 | The game loop, delta time against a fixed timestep, who owns the loop, and Bevy's "Hello, world" — Stub |
| [Entities and components](entities_and_components/README.md) | 201 | An entity is an id, a component is plain data, and there is no inheritance — archetypes and tables underneath — Stub |
| [Systems and queries](systems_and_queries/README.md) | 201 | Functions whose parameters declare their access, parallel scheduling, and the borrow conflict Bevy reports at run time — Stub |
| [Resources and plugins](resources_and_plugins/README.md) | 201 | `Res` and `ResMut` singletons, messages and events, and `Plugin` as the unit of composition — Stub |
| [Keyboard, mouse and gamepad](keyboard_mouse_and_gamepad/README.md) | 201 | `ButtonInput<KeyCode>`, `pressed` against `just_pressed`, the cursor position, and gamepads as entities — Stub |
| [Sprites and meshes](sprites_and_meshes/README.md) | 201 | Cameras, 2D sprites, 3D meshes and materials, and `Transform` — Stub |
| [Music and sound effects](music_and_sound_effects/README.md) | 201 | `AudioPlayer`, one-shot against looping playback, and volume — Stub |
| [Text and menus](text_and_menus/README.md) | 201 | UI nodes, text, and `States` for a menu, playing and paused flow — Stub |

**Every page here is a stub** — an outline with its boundaries and its trap written down, and no runnable example yet. [CONTRIBUTING.md](../CONTRIBUTING.md) says what graduating one takes.

## How these pages graduate

Bevy is a crate with a window, a GPU and an audio device behind it, none of which bare `rustc` or CI has, so a page here graduates the way [Observability](../21_Observability/README.md) does — the mechanism modelled in std as a checked example (a `Vec` per component and a function over it, a state enum, a fixed-timestep accumulator), the real Bevy code in a `text` fence pinned to 0.19, and a dated "Real runs" fence from a Cargo project on a machine with a display.

## Where it goes next

[Shipping a wasm page](../42_WebAssembly/shipping_a_wasm_page/README.md) puts the finished game in a browser. [Indices instead of references](../18_Ownership/indices_instead_of_references/README.md) is the ownership idea an ECS is built on, and [An enum as a state machine](../13_Enums/an_enum_as_a_state_machine/README.md) is the pattern behind a menu. The parallel scheduler is [data parallelism ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/parallelism/data_parallelism/) with the borrow checker's rules applied at run time.

## Po polsku

**Silnik gier** (*game engine*) to po polsku termin, który obejmuje wszystko od prostej biblioteki do kombajnu z edytorem, więc warto od razu ustalić, o co chodzi tutaj. Gra to pętla: odczytaj wejście, przesuń świat o czas, który upłynął, narysuj klatkę. Bevy organizuje ten świat jako **ECS** (*entity–component system*): encja (*entity*) to tylko identyfikator, komponent to zwykła struktura z danymi, a system to zwykła funkcja, którą silnik uruchamia na wszystkich pasujących encjach — bez klas i bez dziedziczenia.

Jedna uwaga praktyczna, zanim sięgniesz po polskie albo angielskie samouczki: Bevy zmienia nazwy między wydaniami. Te strony są przypięte do wersji 0.19 (najnowsza stabilna to 0.19.1), a samouczek sprzed wersji 0.17 użyje `EventReader` tam, gdzie dziś jest `MessageReader`. Dla łagodniejszego startu jest `rusty_engine` Nathana Stocksa — prosty silnik 2D zbudowany na Bevy z myślą o nauce.

**Szukaj po polsku:** programowanie gier w Ruscie · silnik gier Bevy · system encja–komponent · `bevy 0.19 tutorial` · `bevy ecs explained` · `rusty_engine rust`
