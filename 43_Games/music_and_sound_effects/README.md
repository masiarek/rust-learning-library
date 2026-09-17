# Music and sound effects

**Level:** 201 · working knowledge

**One line:** A sound in Bevy is an entity too — spawn one with an `AudioPlayer` holding a handle to the file and `PlaybackSettings` saying whether it plays once, loops, or cleans itself up when it finishes (as of Bevy 0.19).

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The shape from Bevy 0.19.1's [`bevy::audio` module docs ↗](https://docs.rs/bevy/0.19.1/bevy/audio/index.html): `commands.spawn((AudioPlayer::new(asset_server.load("background_audio.ogg")), PlaybackSettings::LOOP))`, with `AudioSource` as the loaded asset type.
- The four presets on [`PlaybackSettings` ↗](https://docs.rs/bevy/0.19.1/bevy/audio/struct.PlaybackSettings.html) as of Bevy 0.19: `ONCE` plays once, `LOOP` loops, `DESPAWN` plays once and despawns the entity, `REMOVE` plays once and removes the audio components. What does `ONCE` leave behind after a thousand footsteps, and which preset should a one-shot effect use?
- Volume as of Bevy 0.19: the `Volume` enum has `Linear` and `Decibels` variants, `PlaybackSettings::with_volume` sets it per sound and the `GlobalVolume` resource scales everything. Why do decibels add where linear factors multiply, and what does `Linear(0.5)` sound like next to `Decibels(-6.0)`?
- Controlling a sound that is already playing: the `AudioSink` component and the `AudioSinkPlayback` trait (as of Bevy 0.19) with `pause`, `play`, `stop`, `set_volume`, `set_speed` and `mute`. How does a system find the sink for "the music" among every other playing sound?
- Formats: which of `.ogg`, `.wav`, `.mp3` and `.flac` decode with Bevy's default Cargo features, and which need a feature switched on?
- Spatial audio: `PlaybackSettings::with_spatial` and `SpatialAudioSink` (as of Bevy 0.19) — where is the listener, and what moves it?
- Timing: a sound effect loaded the moment it is needed can start late. Does keeping its `Handle` in a resource from startup fix that?
- On the web: browsers restrict audio until the user interacts with the page. What happens to music a Bevy game starts in `Startup` — see [Shipping a wasm page](../../42_WebAssembly/shipping_a_wasm_page/README.md).

## The trap it exists for

Playing every sound effect with the default settings, which `bevy_audio` 0.19.1's source sets to `PlaybackSettings::ONCE`. `ONCE` plays and stops; it is `DESPAWN` that removes the entity. So each footstep can leave an entity behind, the world grows with every step, and nothing reports it — the finished page has to count them.

## Where this sits

[Resources and plugins](../resources_and_plugins/README.md) covers `AssetServer` as a resource; [Entities and components](../entities_and_components/README.md) covers spawning and despawning. This page covers only audio playback.

## See also

- [Entities and components](../entities_and_components/README.md) — a playing sound is an entity with components
- [Resources and plugins](../resources_and_plugins/README.md) — `GlobalVolume` and `AssetServer` are resources
- [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) — the cleanup that `DESPAWN` does at the level of entities
- [Shipping a wasm page](../../42_WebAssembly/shipping_a_wasm_page/README.md) — audio in the browser
- [Bevy audio examples, 0.19.1 ↗](https://github.com/bevyengine/bevy/tree/v0.19.1/examples/audio)

## If you are coming from another language

- **Python.** pygame keeps two mechanisms: `mixer.Sound` for effects, and `pygame.mixer.music`, *"for controlling streamed audio"*, for the background track. Bevy has one mechanism for both, and the difference between them is a `PlaybackSettings` value.
- **C#.** Unity's `AudioSource` is a component on a `GameObject`, and `PlayOneShot` plays a clip without cancelling what that source is already playing — the effect-versus-music split again, attached to an object the way `AudioPlayer` is attached to an entity.
- **JavaScript.** Browsers apply their autoplay rules to both media elements and the Web Audio API ([MDN's autoplay guide ↗](https://developer.mozilla.org/en-US/docs/Web/Media/Guides/Autoplay)), and a Bevy game built for the web runs under the same rules as any page.

## Po polsku

Dźwięk w Bevy to też encja: tworzy się ją z komponentem `AudioPlayer` (uchwyt do pliku) i `PlaybackSettings`, które mówią, czy dźwięk ma zagrać raz (`ONCE`), zapętlić się (`LOOP`), czy po zakończeniu usunąć encję (`DESPAWN`). Głośność opisuje enum `Volume` — liniowo albo w decybelach — a `GlobalVolume` skaluje wszystko naraz. Pułapka to efekty dźwiękowe z domyślnymi ustawieniami (`ONCE`): dźwięk gra i milknie, ale encję usuwa dopiero `DESPAWN`, więc świat może rosnąć z każdym krokiem postaci.

**Szukaj po polsku:** dźwięk w Bevy · muzyka w tle w grze Rust · `bevy audioplayer playbacksettings` · `bevy playbacksettings despawn` · `bevy volume decibels` · `bevy audiosink pause`
