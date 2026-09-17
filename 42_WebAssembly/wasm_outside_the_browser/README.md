# WebAssembly outside the browser

**Level:** 301 · deep dive

**One line:** WASI gives a module a standard set of host functions — arguments, clocks, files it was granted — so `rustc hello.rs --target wasm32-wasip1` followed by `wasmtime hello.wasm` prints *Hello, world!* with no JavaScript anywhere, and the component model turns that idea into typed interfaces between modules.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The hello-world path exactly as [wasmtime.dev ↗](https://wasmtime.dev/) prints it: `rustup target add wasm32-wasip1`, `rustc hello.rs --target wasm32-wasip1`, `wasmtime hello.wasm`. What does the module import to make `println!` work?
- WASI's three milestones as [wasi.dev ↗](https://wasi.dev/) lists them — 0.1, 0.2 and 0.3, also written Preview 1/2/3 or P1/P2/P3, with 0.3 adding native async to the component model — and the Rust target for each on 1.98.0: `wasm32-wasip1` and `wasm32-wasip2` at Tier 2, `wasm32-wasip3` at Tier 3.
- Preview 1 against preview 2, per the rustc book: `wasm32-wasip1` produces a core module importing functions from `wasi_snapshot_preview1` and is now kept *for historical compatibility*; `wasm32-wasip2` produces a component, and the runtime must support components to run it. What can a component express that a core module cannot?
- Capabilities instead of ambient authority: a WASI program opens only what the runtime granted it. What does `fs::read("/etc/hosts")` return inside `wasmtime` without a `--dir` grant, and with one?
- The [component model ↗](https://component-model.bytecodealliance.org/): WIT files describing interfaces with strings, records and lists, and [`cargo-component` ↗](https://github.com/bytecodealliance/cargo-component) — replacing the pointer-and-length conventions of [Rust in the browser](../rust_in_the_browser/README.md) with a typed contract.
- Wasm as a plugin system: embedding the [`wasmtime` ↗](https://docs.rs/wasmtime) crate (48.0.2 on crates.io) so a third-party module gets CPU and memory and nothing else. What stops a plugin's infinite loop or unbounded allocation — which limits does the host have to set? [Extism ↗](https://extism.org/) packages the same idea as a framework.
- Threads: `wasm32-wasip1-threads` and what shared memory needs from the runtime.
- Against a container: start-up time, isolation and portability claims — as measurements the finished page makes, or as questions it leaves open.

## The trap it exists for

Porting a command-line tool to `wasm32-wasip1`, watching it run under `wasmtime`, and assuming it now has the machine's filesystem. It has what the runtime was told to grant; everything else fails at run time, and the build gave no sign of the difference.

## Where this sits

[Compiling to `wasm32`](../compiling_to_wasm32/README.md) covers choosing among the targets; [Rust in the browser](../rust_in_the_browser/README.md) covers the JavaScript host. This page covers only hosts that are not a browser.

## See also

- [Compiling to `wasm32`](../compiling_to_wasm32/README.md) — `wasm32-wasip1` and `wasm32-wasip2` among the six
- [What WebAssembly is](../what_webassembly_is/README.md) — imports, exports and the sandbox the capabilities rest on
- [Calling C](../../09_Advanced/calling_c/README.md) — the other way to load someone else's code into your process, without a sandbox
- [The right to post is a value](../../09_Advanced/one_account_one_review/README.md) — permission as something held, the same idea as a pre-opened directory
- [Standard error, and exit status](../../02_Errors/stderr_and_exit_status/README.md) — the streams and the number a WASI command hands back to its host
- [Process ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/units_of_execution/process/) — the isolation boundary a wasm instance is compared with

## If you are coming from another language

- **Java.** A portable bytecode on a host runtime is the closest precedent, and the Security Manager was Java's in-process way of restricting what loaded code could touch — deprecated for removal in Java 17 ([JEP 411 ↗](https://openjdk.org/jeps/411)) and later permanently disabled. WASI puts the restriction in the host interface itself: a module has no access until the runtime grants some.
- **Go.** `GOOS=wasip1 GOARCH=wasm` builds for the same WASI preview 1 host (`go tool dist list`, Go 1.25.5), so a Go module and a Rust module can run under the same `wasmtime`. That list has no preview 2 port, which is where `wasm32-wasip2` and components go further.

## Po polsku

Poza przeglądarką moduł WebAssembly dostaje od gospodarza standardowy zestaw funkcji — **WASI** (*WebAssembly System Interface*): argumenty, zegar i tylko te pliki, do których runtime dał mu dostęp. `rustc hello.rs --target wasm32-wasip1` i `wasmtime hello.wasm` wypisują „Hello, world!” bez grama JavaScriptu. Wersje WASI to 0.1, 0.2 i 0.3 (Preview 1/2/3); od 0.2 opierają się na modelu komponentów (*component model*) z interfejsami opisanymi w plikach WIT, a osadzony w programie `wasmtime` pozwala wykonywać cudze wtyczki w piaskownicy.

**Szukaj po polsku:** WASI w Ruscie · WebAssembly na serwerze · wtyczki w piaskownicy · `wasmtime wasi rust` · `webassembly component model wit` · `wasm32-wasip2 rust`
