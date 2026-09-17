# Sprites and meshes

**Level:** 201 · working knowledge

**One line:** Nothing is drawn until an entity has something visible *and* a camera exists to look at it — a `Sprite` for a 2D image, a `Mesh3d` with a `MeshMaterial3d` for 3D, a `Camera2d` or `Camera3d`, and a `Transform` on each saying where (as of Bevy 0.19).

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Cameras first: spawn a `Camera2d` or `Camera3d` (as of Bevy 0.19). What does a scene with sprites and no camera show — and does anything warn you?
- 2D: [`Sprite` ↗](https://docs.rs/bevy/0.19.1/bevy/prelude/struct.Sprite.html) as of Bevy 0.19 has the fields `image: Handle<Image>`, `texture_atlas`, `color`, `flip_x`, `flip_y`, `custom_size`, `rect` and `image_mode`. `AssetServer::load` hands back a `Handle` at once — what is drawn before the file has loaded?
- 3D: `Mesh3d` wrapping a handle from `Assets<Mesh>` (a primitive such as `Cuboid::new` turned into a mesh) and `MeshMaterial3d` wrapping a `StandardMaterial` from `Assets<StandardMaterial>` — plus a light, `PointLight` or `DirectionalLight`. What does a lit scene with no light look like?
- 2D meshes: `Mesh2d` with `MeshMaterial2d<ColorMaterial>` (as of Bevy 0.19) — when a shape is better than a sprite.
- [`Transform` ↗](https://docs.rs/bevy/0.19.1/bevy/prelude/struct.Transform.html) as of Bevy 0.19: `translation`, `rotation` (a quaternion) and `scale`, constructors such as `from_xyz`, and `GlobalTransform` computed through `ChildOf` and `Children`. What does a child's `Transform` mean relative to its parent?
- 2D coordinates: which way is +y, where is (0, 0), and does `translation.z` decide which sprite is drawn in front?
- Animation from a sprite sheet: a `TextureAtlas` index advanced by a `Timer` (as of Bevy 0.19), and what "frame" means when the animation rate and the frame rate differ.
- Why thousands of sprites sharing one image can be cheap: what does the renderer batch, and what breaks the batch?

## The trap it exists for

Spawning sprites, running the game, and seeing an empty window. Nothing is wrong with the sprites and nothing failed to compile — no camera was spawned, so nothing looks at them, and the fix is one entity that the code never mentions anywhere near the sprites.

## Where this sits

[Entities and components](../entities_and_components/README.md) covers components in general and [Text and menus](../text_and_menus/README.md) covers the UI layer drawn over the world; this page covers only things in the world and the camera that sees them.

## See also

- [Entities and components](../entities_and_components/README.md) — a sprite is a component on an entity
- [Keyboard, mouse and gamepad](../keyboard_mouse_and_gamepad/README.md) — turning a cursor position into a world position through the camera
- [What a game engine is](../what_a_game_engine_is/README.md) — the loop that redraws every frame
- [Writing a number down](../../19_Numbers/writing_a_number_down/README.md) — why `from_xyz(0.0, 1.0, 0.0)` needs the decimal points
- [Operators are traits](../../12_Traits/operators_are_traits/README.md) — how `transform.translation += velocity * dt` is spelled for vectors
- [Bevy 2D examples, 0.19.1 ↗](https://github.com/bevyengine/bevy/tree/v0.19.1/examples/2d)

## If you are coming from another language

- **Python.** pygame blits a surface to the screen yourself every frame. In Bevy you never draw: you spawn an entity with a `Sprite` and a `Transform`, and the renderer draws whatever exists, every frame, until you despawn it.
- **C#.** Unity's `SpriteRenderer` and `MeshRenderer` plus `Transform` on a `GameObject` are the same arrangement, with parent-child transforms working the same way. The difference is again that the Bevy components are data only.
- **JavaScript.** Drawing to a `<canvas>` with `drawImage` is the immediate style; a scene graph library such as three.js is the retained style Bevy uses — you describe objects, the renderer draws them.

## Po polsku

Nic się nie narysuje, dopóki encja nie ma czegoś widocznego **i** nie istnieje kamera, która na to patrzy. W 2D to `Sprite` i `Camera2d`, w 3D `Mesh3d` z `MeshMaterial3d` (np. `StandardMaterial`), `Camera3d` i światło. Położenie, obrót (kwaternion) i skalę każdej encji opisuje `Transform`. Najczęstszy problem początkujących to puste okno — sprite'y są poprawne, program się skompilował, tylko nikt nie dodał kamery.

**Szukaj po polsku:** sprite w Bevy · siatka i materiał 3D · kamera 2D Bevy · `bevy sprite camera2d` · `bevy mesh3d meshmaterial3d standardmaterial` · `bevy transform from_xyz`
