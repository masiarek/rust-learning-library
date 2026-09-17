# Text and menus

**Level:** 201 · working knowledge

**One line:** A menu is a game state, not a screen you draw — a `States` enum such as menu, playing and paused, whose `OnEnter` and `OnExit` schedules spawn and remove UI entities, while `run_if(in_state(…))` keeps gameplay systems from running behind the menu (as of Bevy 0.19).

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- UI as entities, as of Bevy 0.19: `Node` for layout, `Text` for a label, `Button`, and the `Interaction` component whose variants are `Pressed`, `Hovered` and `None`. How is `Node`'s layout expressed — flexbox, grid, both?
- `Text` on the UI layer against `Text2d` in the game world (as of Bevy 0.19): which moves with the camera?
- States, per the [`States` docs ↗](https://docs.rs/bevy/0.19.1/bevy/prelude/trait.States.html) as of Bevy 0.19: derive `Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States` on an enum, mark the starting variant `#[default]`, register it with `App::init_state`, and request a change through `ResMut<NextState<GameState>>` and `set`. The docs say transitions typically run in the `OnEnter` and `OnExit` schedules when the `StateTransition` schedule runs — so in which frame does the menu appear?
- Cleaning up: a `DespawnOnExit` component (as of Bevy 0.19) ties a UI tree's lifetime to a state. What happens to menu entities without it when the game starts twice?
- `run_if(in_state(GameState::Playing))` on gameplay systems, so pausing stops them rather than hiding them — and what `Time<Virtual>` pausing does differently.
- [An enum as a state machine](../../13_Enums/an_enum_as_a_state_machine/README.md) makes the compiler enumerate every transition; `NextState::set` accepts any variant from any state. Where does a game put the transition rules Bevy does not check?
- Sub-states and computed states, `App::add_sub_state` and `App::add_computed_state` (as of Bevy 0.19), for "paused only exists while playing".
- Fonts and beyond: the default font against a loaded `.ttf`, and what the Bevy 0.19 release notes' *Text Input* and *Accessible Label Component* sections add to a menu.

## The trap it exists for

Drawing the pause menu on top of a game that is still running. The menu looks right, and behind it the enemies keep moving and the timer keeps counting, because showing a UI changes nothing about which systems run — only a state and a run condition do.

## Where this sits

[An enum as a state machine](../../13_Enums/an_enum_as_a_state_machine/README.md) covers the state machine as plain Rust; [Sprites and meshes](../sprites_and_meshes/README.md) covers things drawn in the world. This page covers only the UI layer and Bevy's `States`.

## See also

- [An enum as a state machine](../../13_Enums/an_enum_as_a_state_machine/README.md) — the transition table the compiler checks, and Bevy's `States` does not
- [What an enum is](../../13_Enums/what_an_enum_is/README.md) — the type every Bevy state is
- [The comparison traits](../../12_Traits/comparison_traits/README.md) — `PartialEq` and `Eq`, two of the derives the `States` example lists
- [Keyboard, mouse and gamepad](../keyboard_mouse_and_gamepad/README.md) — the Escape key that opens the menu
- [Resources and plugins](../resources_and_plugins/README.md) — `State<T>` and `NextState<T>` are resources
- [Bevy state examples, 0.19.1 ↗](https://github.com/bevyengine/bevy/tree/v0.19.1/examples/state) and [UI examples ↗](https://github.com/bevyengine/bevy/tree/v0.19.1/examples/ui)

## If you are coming from another language

- **C#.** Unity's [`Time.timeScale` ↗](https://docs.unity3d.com/ScriptReference/Time-timeScale.html) set to zero makes the application act *as if paused* — but only, its docs say, *if all your functions are frame rate independent*. Bevy offers both halves separately: pausing `Time<Virtual>` stops the clock, and a `run_if(in_state(…))` condition stops the systems whether they read the clock or not.
- **JavaScript.** A single-page app's router is a state machine whose states mount and unmount components — the same shape as `OnEnter` spawning a UI tree and `DespawnOnExit` removing it.
- **Python.** A pygame loop with `if state == "menu": … elif state == "playing": …` is the hand-written version. Bevy's `in_state` run condition is that `if`, attached to each system instead of wrapped around all of them.

## Po polsku

Menu to stan gry, a nie ekran do narysowania: enum z `#[derive(States)]` (menu, gra, pauza), którego harmonogramy `OnEnter` i `OnExit` tworzą i usuwają encje interfejsu (`Node`, `Text`, `Button`), a warunek `run_if(in_state(…))` wyłącza systemy rozgrywki na czas menu. Najczęstszy błąd to menu pauzy narysowane nad grą, która dalej działa — sam interfejs niczego nie zatrzymuje, robi to dopiero stan i warunek uruchomienia. Bevy nie sprawdza przy tym, które przejścia są dozwolone: `NextState::set` przyjmie dowolny wariant.

**Szukaj po polsku:** menu w grze Bevy · stany gry · interfejs użytkownika w Bevy · `bevy states onenter onexit` · `bevy run_if in_state` · `bevy ui node text button`
