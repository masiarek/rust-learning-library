# Shipping a wasm page

**Level:** 301 · deep dive

**One line:** A Rust web app is static files — `index.html`, generated JavaScript and one `.wasm` — so any static host can serve it, provided the server labels the module `application/wasm`, which `WebAssembly.instantiateStreaming` needs before it will compile the module while it downloads.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What gets deployed: `index.html`, the JavaScript `wasm-bindgen` generated, the `.wasm`, and the assets — and the consequence that no Rust runs on the server at all.
- The MIME type: [MDN's `instantiateStreaming` page ↗](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static) notes that for it to work, `.wasm` files should be returned with the `application/wasm` MIME type, and the type is [registered with IANA ↗](https://www.iana.org/assignments/media-types/application/wasm). What does the fallback — `fetch`, `arrayBuffer`, `WebAssembly.instantiate` — cost, and how does `curl -I` show what a given host sends?
- [GitHub Pages ↗](https://docs.github.com/en/pages) as the host: which `Content-Type` does it send for `.wasm`, and what breaks when the site lives under `/<repository>/` rather than at the root — the same situation as this library's own site?
- A size budget, as the table the finished page measures: debug, release, `opt-level = "z"`, `wasm-opt -Oz`, each raw and compressed — and what the largest of them costs on a phone connection before the first frame.
- Caching: a new deploy served with a cached `.wasm` from the old one and new JavaScript glue, or the reverse. What does the mismatch look like, and do content-hashed file names fix it?
- Deploying a Bevy game: Bevy 0.19.1's [examples README ↗](https://github.com/bevyengine/bevy/blob/v0.19.1/examples/README.md) builds with `cargo build --release --target wasm32-unknown-unknown`, runs `wasm-bindgen --target web` on the result, and serves the directory. Bevy 0.19's `App::run` documentation warns that it *will never return on iOS and Web* — what does that change about code written after it? And do assets load over HTTP the same way they load from disk?
- Audio and input on the web: can a game's sound start before the player has clicked anything, and what does Bevy do with the sounds requested before that?
- Threads in the browser need cross-origin isolation headers. Can a plain static host such as GitHub Pages send them, and what does a Bevy build do without threads?

## The trap it exists for

Testing with a local development server that sends the right headers, then deploying to a host that labels `.wasm` as something else. The page worked on every machine it was tried on; in production the streaming compile refuses the response, and the symptom is an error in a console that the visitor never opens.

## Where this sits

[Rust in the browser](../rust_in_the_browser/README.md) covers what the files contain; [Games with Bevy](../../43_Games/README.md) covers the game. This page covers only getting the built files in front of a visitor.

## See also

- [Rust in the browser](../rust_in_the_browser/README.md) — the glue and the module being shipped
- [Compiling to `wasm32`](../compiling_to_wasm32/README.md) — the size settings the budget is measured against
- [Games with Bevy](../../43_Games/README.md) — the game this page deploys
- [Music and sound effects](../../43_Games/music_and_sound_effects/README.md) — the audio that the browser may hold back
- [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) — which compiler the deploy's build used
- [Bevy examples ↗](https://bevy.org/examples/) — Bevy's own examples, running in the browser

## If you are coming from another language

- **JavaScript.** A single-page app bundle is the same deployment with the same caching problem. The addition is the MIME type, which a `.js` file never makes anyone think about because every server already knows it.
- **Python.** A Pyodide page ships the CPython interpreter itself as wasm, so its size is mostly the interpreter's. A Rust page ships only the program, which is why the budget is yours to measure and yours to cut.
- **Go.** A `GOOS=js GOARCH=wasm` build is served with the `wasm_exec.js` loader that Go ships in `$(go env GOROOT)/lib/wasm/` (Go 1.25.5), and needs the same `application/wasm` type; the module carries the Go runtime, which sets its starting size.

## Po polsku

Aplikacja Rusta w przeglądarce to tylko pliki statyczne — `index.html`, wygenerowany JavaScript i jeden plik `.wasm` — więc wystarczy dowolny hosting statyczny, np. GitHub Pages. Warunek jest jeden, łatwy do przeoczenia: serwer musi wysyłać `.wasm` z typem MIME `application/wasm`, bo inaczej `WebAssembly.instantiateStreaming` odmawia kompilacji w trakcie pobierania. Gra w Bevy trafia na stronę tą samą drogą: `cargo build --target wasm32-unknown-unknown`, potem `wasm-bindgen --target web`, a w przeglądarce `App::run` nigdy nie wraca.

**Szukaj po polsku:** publikacja aplikacji wasm · GitHub Pages WebAssembly · typ MIME application/wasm · `instantiateStreaming application/wasm` · `bevy wasm github pages` · `wasm-bindgen --target web`
