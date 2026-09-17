# Entities and components

**Level:** 201 · working knowledge

**One line:** An entity is not an object — it is an id — and a component is a plain struct attached to that id, so a "player" is whatever set of components one entity happens to carry, and there is no class to inherit from.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- [`Entity` ↗](https://docs.rs/bevy/0.19.1/bevy/ecs/entity/struct.Entity.html) as of Bevy 0.19: its docs call it *just an id, not the entity itself*, and say the entity it names may no longer exist.
- Components as `#[derive(Component)]` structs — plain data with no required methods, so [What a struct is](../../16_Structs/what_a_struct_is/README.md) applies unchanged — and marker components with no fields at all, like the `struct Player;` and `struct Enemy;` in Bevy's own B0001 error page.
- Spawning with `Commands::spawn`, which takes any `Bundle` — a single component or a "tuple bundle" — and returns `EntityCommands`, with `insert` and `remove` on it (all as of Bevy 0.19). Why is "an enemy that can also fly" one more component rather than a subclass?
- Required components: does spawning a `Sprite` add a `Transform` you never wrote? What does the component declare to make that happen?
- Storage as of Bevy 0.19: entities with the same set of components share an `Archetype`, and a `Table` is *a column-oriented structure-of-arrays* — one type-erased column per component. `StorageType` has `Table` and `SparseSet`. What does adding or removing a component cost under each?
- Why the layout is fast: iterating one component is walking one column. Is the difference measurable against a `Vec` of structs in a std-only model — the same comparison as [What is a record, in memory?](../../16_Structs/representing_a_record/README.md)?
- Stale ids: Bevy 0.19's docs warn that *a later entity can be spawned at the exact same id* after a despawn, and that holding an `Entity` well after its despawn *can cause un-intuitive behavior*. What does the generation part of the id buy, and what does [Indices instead of references](../../18_Ownership/indices_instead_of_references/README.md) say about the same problem in a `Vec`?
- New in 0.19: a resource is stored on an entity too — see [Resources and plugins](../resources_and_plugins/README.md).

## The trap it exists for

Designing components like classes: a `Player` component with forty fields and methods that reach into other entities. It compiles, and it throws away both things the ECS offers — systems that touch only the columns they need, and behaviour assembled by adding or removing one small component at run time.

## Where this sits

[Systems and queries](../systems_and_queries/README.md) covers the functions that read and write components; [Indices instead of references](../../18_Ownership/indices_instead_of_references/README.md) covers the general ownership pattern. This page covers only what an entity and a component are, and where they are stored.

## See also

- [What a struct is](../../16_Structs/what_a_struct_is/README.md) — a component is exactly this
- [Indices instead of references](../../18_Ownership/indices_instead_of_references/README.md) — the pattern an entity id is an instance of
- [What is a record, in memory?](../../16_Structs/representing_a_record/README.md) — rows against columns, the layout question an archetype answers
- [An enum instead of a bool](../../13_Enums/an_enum_instead_of_a_bool/README.md) — the opposite pull: when a state belongs in one field rather than a marker component
- [Systems and queries](../systems_and_queries/README.md) — reading the components back
- [STRUCTS.md](../../STRUCTS.md) — the map of the library's struct pages
- [Component, in *Game Programming Patterns* ↗](https://gameprogrammingpatterns.com/component.html)

## If you are coming from another language

- **C++.** A hierarchy of game-object classes with behaviour inherited down the tree is the design an ECS replaces, and the data-oriented layout is available in C++ as well. What Rust adds is that a system's access to each column is part of its signature, which [Systems and queries](../systems_and_queries/README.md) turns into scheduling.
- **C#.** Unity's `GameObject` with attached `MonoBehaviour` components is composition, but each component is an object with its own methods and lifecycle. Bevy's components carry no behaviour at all; everything that happens is a system.
- **Java.** "Favour composition over inheritance" is advice in Java and the only option in an ECS: Rust has no struct inheritance to fall back on, so the design pressure the advice describes is the language itself.
- **Python.** A dictionary of component dictionaries keyed by entity id is a working ECS in a few lines, and a good way to see the idea. Bevy's version is typed, stored in columns, and reads from each system's signature which columns it touches.

## Po polsku

Encja (*entity*) nie jest obiektem, tylko identyfikatorem, a komponent to zwykła struktura doczepiona do tego identyfikatora. „Gracz” to więc po prostu zestaw komponentów, który akurat nosi dana encja — bez klas i bez dziedziczenia; nową zdolność dodaje się jednym komponentem, a nie podklasą. Bevy przechowuje encje o tym samym zestawie komponentów razem (archetyp) w tabelach kolumnowych, więc system czytający jeden komponent przechodzi po jednej kolumnie danych.

**Szukaj po polsku:** encja i komponent · system encja–komponent · kompozycja zamiast dziedziczenia · `bevy ecs archetype table` · `bevy entity id generation` · `entity component system explained`
