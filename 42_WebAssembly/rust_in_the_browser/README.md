# Rust in the browser

**Level:** 301 · deep dive

**One line:** A browser runs the `.wasm` module but only JavaScript can reach the page, so Rust in the browser is always Rust plus generated JavaScript glue — `wasm-bindgen` writes it — and every string that crosses is copied, and converted between UTF-8 and UTF-16, on the way.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What `#[wasm_bindgen]` generates on each side of the boundary, and why a module whose exports take and return only numbers needs no glue at all.
- Strings across the boundary, per the [`wasm-bindgen` guide ↗](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/str.html): `&str` and `String` are copied between JavaScript's garbage-collected heap and linear memory with `TextEncoder` and `TextDecoder`. JavaScript strings are UTF-16 and can hold unpaired surrogates, which become `U+FFFD` on the way into Rust. `js_sys::JsString` is the handle that avoids the copy — what can Rust still do with it?
- [`js-sys` ↗](https://docs.rs/js-sys) for JavaScript's built-ins and [`web-sys` ↗](https://docs.rs/web-sys) for the Web APIs — the DOM, `fetch`, canvas. Why does `web-sys` make you switch on a Cargo feature for each API you use?
- Calling JavaScript from Rust with an `extern` block under `#[wasm_bindgen]`, and the cost of each crossing: is a call into JavaScript cheap enough to make per pixel, per frame, per element?
- The tooling as of 2026-09. The `rustwasm` GitHub organisation was sunset ([Inside Rust, 2025-07-21 ↗](https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/)); [`wasm-bindgen` ↗](https://github.com/wasm-bindgen/wasm-bindgen) moved to its own organisation, and crates.io lists `wasm-pack` 0.15.0 under `wasm-bindgen/wasm-pack`. [Trunk ↗](https://github.com/trunk-rs/trunk) builds and serves an `index.html` app. Which of the three does a new project need?
- The lowest-level path, as Bevy 0.19.1's examples README gives it: `cargo build --release --target wasm32-unknown-unknown`, then `wasm-bindgen --target web` on the resulting `.wasm`, then a static file server.
- Panics: `panic="abort"` is the target default, so what does a panic look like in the browser console — and what does [`console_error_panic_hook` ↗](https://docs.rs/console_error_panic_hook) (last released in 2021; its GitHub repository is archived) add?
- Frameworks on top — Leptos and Dioxus — are on [Which Rust UI, and whether you want one](../../08_Interfaces/rust_ui_options/README.md); this page stops at the boundary they are built over.

## The trap it exists for

Treating the JavaScript boundary as a function call. Passing a string back and forth in a loop copies and transcodes it every time, and the one piece of text a user pasted with a lone surrogate arrives in Rust silently changed to `U+FFFD` — no error on either side.

## Where this sits

[What WebAssembly is](../what_webassembly_is/README.md) covers imports and exports in general and [Compiling to `wasm32`](../compiling_to_wasm32/README.md) the target; [Shipping a wasm page](../shipping_a_wasm_page/README.md) covers getting the files onto a server. This page covers only the Rust–JavaScript boundary inside the page.

## See also

- [Which Rust UI, and whether you want one](../../08_Interfaces/rust_ui_options/README.md) — Leptos, Dioxus and Tauri, built on this boundary
- [Four lengths, and which one the other system means](../../14_Strings/four_lengths/README.md) — the same text measured as UTF-8 bytes in Rust and UTF-16 units in JavaScript
- [STRINGS.md](../../STRINGS.md) — the map of the library's string pages
- [Calling C](../../09_Advanced/calling_c/README.md) — the other boundary where "the call is free, the data is not"
- [`str` is unsized](../../14_Strings/str_is_unsized/README.md) — why a `&str` travels as a pointer and a length
- [Books](../../10_Resources/books/README.md) — *Rust and WebAssembly*, listed there as dated in its tooling
- [UTF-16 and surrogates ↗](https://masiarek.github.io/encodings-learning-library/03_Encodings/utf16_and_surrogates/) — what an unpaired surrogate is
- [Why UTF-16 stayed ↗](https://masiarek.github.io/encodings-learning-library/09_History/why_utf16_stayed/) — why JavaScript's strings are still UTF-16

## If you are coming from another language

- **JavaScript.** You already know the host. What changes is that the module's linear memory is a buffer you view from JavaScript as bytes (`WebAssembly.Memory`'s `buffer`), and a Rust `String` inside it is bytes that mean nothing to JavaScript until the glue decodes them.
- **Java.** `char` and `String` are UTF-16 in Java exactly as in JavaScript, so the lone-surrogate problem is familiar — [A `char` is not a character ↗](https://masiarek.github.io/java-text-learning-library/01_Char_and_String/a_char_is_not_a_character/). Rust's `String` cannot hold a lone surrogate at all, which is why the conversion has to replace it rather than carry it.
- **Python.** Pyodide runs CPython inside the page and documents its own [type translations ↗](https://pyodide.org/en/stable/usage/type-conversions.html) between Python and JavaScript values. `wasm-bindgen` is the Rust equivalent, generated at build time, and its guide documents each conversion the same way — including the lossy one.

## Po polsku

Przeglądarka uruchomi moduł `.wasm`, ale do strony (DOM) dostęp ma tylko JavaScript, więc Rust w przeglądarce to zawsze Rust plus wygenerowany kod klejący — pisze go `wasm-bindgen`. Każdy łańcuch znaków przechodzący przez tę granicę jest kopiowany i przekodowywany: w Ruscie to UTF-8, w JavaScripcie UTF-16, a niesparowany surogat zamienia się po cichu na `U+FFFD`. Narzędzia się przetasowały — organizacja `rustwasm` została wygaszona, `wasm-bindgen` ma własną — więc starsze samouczki warto czytać z tą poprawką.

**Szukaj po polsku:** Rust w przeglądarce · WebAssembly i JavaScript · `wasm-bindgen strings utf-16` · `web-sys features` · `rustwasm organization archived` · `trunk rust wasm`
