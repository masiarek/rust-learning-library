# Keyboard, mouse and gamepad

**Level:** 201 · working knowledge

**One line:** Input in Bevy is state you read each frame, not a callback you register — `ButtonInput<KeyCode>` is a resource that knows which keys are held, which went down this frame and which came up — and a gamepad is an entity with a `Gamepad` component rather than a device you open (as of Bevy 0.19).

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `Res<ButtonInput<KeyCode>>` and its three questions, per the [`ButtonInput` docs ↗](https://docs.rs/bevy/0.19.1/bevy/input/struct.ButtonInput.html) as of Bevy 0.19: `pressed` is true between a press and a release; `just_pressed` and `just_released` are true for one frame after the event.
- The same docs' warning: when several systems check `just_pressed` but only one should react, clear the state with `clear_just_pressed`, or `clear` or `reset` after acting. What goes wrong if you do not?
- `KeyCode` against `Key` (as of Bevy 0.19): `KeyCode` is *the location of a physical key*, right for WASD movement on any layout; `Key` is the logical value the key produces, right for typed letters and for `+` and `-`, which sit in different places on different layouts.
- The mouse: `ButtonInput<MouseButton>` for buttons, and `Window::cursor_position()`, which returns `Option<Vec2>` in logical pixels and `None` outside the window (as of Bevy 0.19). Where is its origin, and how does `Camera::viewport_to_world_2d` turn it into a point in the game world?
- Mouse motion rather than position — `AccumulatedMouseMotion` (as of Bevy 0.19) — for a camera that turns with the mouse. Why is the cursor position the wrong input for that?
- Gamepads as entities: a `Query<&Gamepad>`, with `pressed(GamepadButton::South)`, `just_pressed`, and `left_stick()` returning a `Vec2` (as of Bevy 0.19). Why is a resting stick not exactly zero, and where does a dead zone belong?
- Input in `FixedUpdate`: the fixed schedule can run zero or two times in a frame. Can a `just_pressed` read there be missed, or seen twice?
- Actions against keys: does Bevy 0.19 ship a layer that maps "jump" to Space, a gamepad button and a touch, or is that a crate — and what does a game lose by reading `KeyCode::Space` in five systems?

## The trap it exists for

Jumping on `pressed`. The key is held for several frames, so the player jumps on every one of them — or, in the other direction, a menu that opens on `just_pressed` in one system and closes on `just_pressed` in another opens and closes in the same frame.

## Where this sits

[Resources and plugins](../resources_and_plugins/README.md) covers what a resource is; [What a game engine is](../what_a_game_engine_is/README.md) covers frames and the fixed timestep. This page covers only reading the devices.

## See also

- [What a game engine is](../what_a_game_engine_is/README.md) — the frame that "just" is measured in
- [Resources and plugins](../resources_and_plugins/README.md) — `ButtonInput<KeyCode>` is a resource like any other
- [Sprites and meshes](../sprites_and_meshes/README.md) — the camera that turns a cursor position into a world position
- [`Option` and `Result`](../../17_Option_and_Result/README.md) — `cursor_position()` returns `None` when the cursor is outside
- [An enum instead of a bool](../../13_Enums/an_enum_instead_of_a_bool/README.md) — `pressed`, `just_pressed` and `just_released` as three states rather than one flag
- [Raw mode and passwords](../../03_Command_Line/raw_mode_and_passwords/README.md) — reading keys one at a time in a terminal, the non-game version
- [Bevy input examples, 0.19.1 ↗](https://github.com/bevyengine/bevy/tree/v0.19.1/examples/input)

## If you are coming from another language

- **Python.** pygame offers both styles: an event queue you drain each frame, and `pygame.key.get_pressed()` for the held state. Bevy's `ButtonInput` is the second style with the edge detection — `just_pressed` — already done for you.
- **JavaScript.** The DOM delivers `keydown` events to callbacks, and `KeyboardEvent.code` against `KeyboardEvent.key` is exactly `KeyCode` against `Key`; the Bevy docs say `KeyCode` mostly conforms to the UI Events specification's `code`. What changes is the shape: no callback, one resource read in a system each frame.
- **C#.** Unity's legacy Input Manager has the same held-and-edge pair: `Input.GetKey`, and `Input.GetKeyDown`, which [returns true during the frame ↗](https://docs.unity3d.com/ScriptReference/Input.GetKeyDown.html) the key goes down — polled in `Update` just as `pressed` and `just_pressed` are read in a system. Unity now recommends its separate Input System package for new projects, an action layer the last bullet above asks about for Bevy.

## Po polsku

Wejście w Bevy to stan odczytywany w każdej klatce, a nie funkcja zwrotna (*callback*): zasób `ButtonInput<KeyCode>` odpowiada na trzy pytania — `pressed` (klawisz trzymany), `just_pressed` i `just_released` (prawdziwe przez jedną klatkę). `KeyCode` to fizyczne położenie klawisza (dobre do WASD na każdym układzie klawiatury), `Key` to znak, który klawisz wytwarza. Pad jest encją z komponentem `Gamepad`, a pozycję kursora daje `Window::cursor_position()` jako `Option<Vec2>`.

**Szukaj po polsku:** obsługa klawiatury w Bevy · pad do gier Rust · `bevy buttoninput just_pressed` · `bevy keycode vs key` · `bevy gamepad left_stick` · `bevy cursor_position viewport_to_world_2d`
