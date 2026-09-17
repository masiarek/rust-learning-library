# Resources and plugins

**Level:** 201 · working knowledge

**One line:** A resource is the one value of its type in the world — `Res<Score>` to read it, `ResMut<Score>` to change it — a message carries "this happened" from one system to another, and a `Plugin` bundles resources, messages and systems into one `add_plugins` call (all as of Bevy 0.19).

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `#[derive(Resource)]`, `App::init_resource` and `App::insert_resource`, then `Res<T>` and `ResMut<T>` in a system — the [`Resource` ↗](https://docs.rs/bevy/0.19.1/bevy/ecs/resource/trait.Resource.html) docs (as of Bevy 0.19) call it *a type that can be inserted into a World as a singleton*. What does a system that asks for a resource nobody inserted get?
- New in 0.19, per the [migration guide ↗](https://bevy.org/learn/migration-guides/0-18-to-0-19/): `Resource` is a subtrait of `Component`, a resource is stored on an entity, one type can no longer derive both, and broad queries such as `Query<EntityMut>` now conflict with `Res` unless filtered with `Without<IsResource>`.
- Bevy's own state lives in the same mechanism: `Time` and `ButtonInput<KeyCode>` implement `Resource` (as of 0.19), inserted by Bevy's plugins exactly as a game inserts its `Score`.
- Two kinds of "something happened", as of Bevy 0.19. A buffered `Message` — `App::add_message`, `MessageWriter::write`, `MessageReader::read` — is stored in the `Messages<M>` resource and read at fixed points in the schedule, with each reading system tracking its own position. An `Event` is triggered with `Commands::trigger` and runs its observers immediately, receiving `On<E>`. Which fits damage numbers, and which a door opening?
- The rename that breaks tutorials: Bevy 0.16.1 had `EventReader` and `EventWriter` for the buffered kind; 0.17.3 has `MessageReader`, and 0.19.1 has no `EventReader` at all. How long does an unread message survive before it is dropped?
- `Plugin::build(&self, app: &mut App)` as the unit of composition: a game split into player, enemy and UI plugins, each adding its own resources, messages and systems. `DefaultPlugins` and `MinimalPlugins` are `PluginGroup`s — what does each contain, and which one does a headless test want?
- Global mutable state by another name: a resource is a typed singleton whose borrow rule is checked at run time — [B0002 ↗](https://bevy.org/learn/errors/b0002/) for `Res<T>` next to `ResMut<T>`. How does that compare with a `static` holding a `Mutex`?

## The trap it exists for

Putting everything in resources because they are the easiest thing to reach — the player's position, each enemy's health — and ending with a game whose systems all take `ResMut` of the same few types. Every one of them now conflicts with every other, the scheduler runs them one at a time, and the data that belonged on entities has no entity to belong to.

## Where this sits

[Systems and queries](../systems_and_queries/README.md) covers queries and the scheduler; [Entities and components](../entities_and_components/README.md) covers per-entity data. This page covers only the world-wide values, the messages between systems, and how a game is split into plugins.

## See also

- [Systems and queries](../systems_and_queries/README.md) — where `Res` and `ResMut` appear as parameters
- [Entities and components](../entities_and_components/README.md) — the data that should not be a resource
- [Sharing across threads: `Arc`](../../18_Ownership/sharing_across_threads/README.md) — shared state the std way
- [Interior mutability](../../09_Advanced/interior_mutability/README.md) — the other way to change something many places can see
- [Keyboard, mouse and gamepad](../keyboard_mouse_and_gamepad/README.md) — `ButtonInput<KeyCode>`, a resource you read every frame
- [Publish–subscribe ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/publish_subscribe/) — the pattern messages implement inside one process

## If you are coming from another language

- **Java.** A dependency-injection container hands a singleton to whoever declares it in a constructor. Bevy's system parameters are the same idea: declare `Res<Score>` and the world supplies it — and whether you asked for `Res` or `ResMut` is part of the declaration.
- **Python.** A module-level global is the default singleton, and any function can rebind it. A resource is a global you cannot touch without saying so in the function's signature, and cannot write without asking for `ResMut`.

## Po polsku

Zasób (*resource*) to jedyna wartość danego typu w świecie gry — `Res<Score>` do odczytu, `ResMut<Score>` do zmiany — a od Bevy 0.19 jest on przechowywany na encji, jak komponent. Wiadomości (`Message`, `MessageWriter`, `MessageReader`) przenoszą informację „coś się stało” między systemami; w starszych samouczkach sprzed 0.17 ten sam mechanizm nazywa się `EventReader`, a `Event` oznacza dziś zdarzenie obsługiwane od razu przez obserwatory. `Plugin` łączy zasoby, wiadomości i systemy w jedną jednostkę dodawaną przez `add_plugins`.

**Szukaj po polsku:** zasoby w Bevy · wtyczki w Bevy · komunikaty między systemami · `bevy resource res resmut` · `bevy message vs event 0.17` · `bevy plugin build`
