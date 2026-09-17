# The Ultimate Rust courses, mapped

**Level:** reference · the map

**One line:** Nathan Stocks teaches two released Rust video courses and has outlined nine more on single subjects — Bevy, contributing to Rust, *Rust for Rustaceans*, generics and lifetimes, macros, strings, tips and tricks, threads, WebAssembly — and this page lists every heading in those outlines next to the page here that covers it.

**What is released and what is a plan (checked 2026-09-16).** Two courses have public companion repositories: [Ultimate Rust Crash Course ↗](https://github.com/CleanCut/ultimate_rust_crash_course) and [Ultimate Rust 2: Intermediate Concepts ↗](https://github.com/CleanCut/ultimate_rust2). The crash course's README says the courses moved from Udemy to Ardan Labs. The other nine outlines below come from the author's list of planned courses. None has a companion repository under [github.com/CleanCut ↗](https://github.com/CleanCut), and this page has not confirmed whether any of them was released. The outlines give only headings, and the headings are what the tables below use.

Each row says where the topic lives here. *stub* marks an outline page with no checked example yet.

## Ultimate Bevy

Everything here depends on the Bevy version; the [Games with Bevy](../../43_Games/README.md) section is written against Bevy 0.19.

| Outline heading | Here |
|---|---|
| Getting started — game engines overview, hello world | [What a game engine is](../../43_Games/what_a_game_engine_is/README.md) *stub* |
| ECS — entities, components | [Entities and components](../../43_Games/entities_and_components/README.md) *stub* · [Indices instead of references](../../18_Ownership/indices_instead_of_references/README.md) *stub* |
| ECS — systems | [Systems and queries](../../43_Games/systems_and_queries/README.md) *stub* |
| ECS — resources, plugins | [Resources and plugins](../../43_Games/resources_and_plugins/README.md) *stub* |
| Input — keyboard, mouse, gamepad | [Keyboard, mouse and gamepad](../../43_Games/keyboard_mouse_and_gamepad/README.md) *stub* |
| Graphics — 2D sprites, 3D meshes | [Sprites and meshes](../../43_Games/sprites_and_meshes/README.md) *stub* |
| Audio — music, sound effects | [Music and sound effects](../../43_Games/music_and_sound_effects/README.md) *stub* |
| UI — text, a menu | [Text and menus](../../43_Games/text_and_menus/README.md) *stub* · [An enum as a state machine](../../13_Enums/an_enum_as_a_state_machine/README.md) |
| WebAssembly | [Shipping a wasm page](../../42_WebAssembly/shipping_a_wasm_page/README.md) *stub* |
| Game project — design, implementation, deployment | gap |

## Ultimate Rust Contributor

| Outline heading | Here |
|---|---|
| So you want to be a contributor — Rust organization overview, benefits | [The Rust project](../../20_Compilers/the_rust_project/README.md) *stub* |
| Getting started — hardware, prerequisites, getting the code, submodules, bootstrapping | [Building the compiler](../../20_Compilers/building_the_compiler/README.md) *stub* |
| Using your modified code | [Using a compiler you built](../../20_Compilers/using_a_compiler_you_built/README.md) *stub* · [rustup](../../05_Tooling/rustup/README.md) |
| Contribution process — identifying a need, code, tests and docs, the pull request | [A first contribution](../../20_Compilers/a_first_contribution/README.md) *stub* |

## Ultimate Rust for Rustaceans

This outline follows Jon Gjengset's book chapter by chapter, so its map is the book's: **[*Rust for Rustaceans*, chapter by chapter](../rust_for_rustaceans/README.md)**. Its headings — foundations, types, designing interfaces, error handling, project structure, testing, macros, async, unsafe, concurrency, FFI, `no_std`, the ecosystem — are that page's thirteen sections.

## Ultimate Rust Generics & Lifetimes

| Outline heading | Here |
|---|---|
| Lifetimes | [Lifetime annotations](../../18_Ownership/lifetime_annotations/README.md) · [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) · [What a lifetime does at the call site](../../18_Ownership/lifetimes_at_the_call_site/README.md) |
| Generic functions — function parameters | [What a generic is](../../22_Generics/what_a_generic_is/README.md) · [When the compiler cannot infer](../../22_Generics/when_the_compiler_cannot_infer/README.md) |
| … trait bounds | [Where the bound goes](../../22_Generics/where_the_bound_goes/README.md) |
| … lifetimes in generic functions | [Lifetime annotations](../../18_Ownership/lifetime_annotations/README.md) |
| … trait objects, monomorphization vs virtual dispatch | [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) · [Returning a trait](../../12_Traits/returning_a_trait/README.md) |
| Generic data types — structs, enums, builtins | [Generic enums](../../22_Generics/generic_enums/README.md) · [A generic recursive type](../../22_Generics/a_generic_recursive_type/README.md) · [Const generics](../../22_Generics/const_generics/README.md) *stub* |
| … `impl` blocks | [What a generic is](../../22_Generics/what_a_generic_is/README.md) (why `impl<T>` says it twice) · [Blanket impls](../../22_Generics/blanket_impls/README.md) *stub* |
| … lifetimes in generic data types | [Lifetime annotations: on a struct](../../18_Ownership/lifetime_annotations/README.md#on-a-struct-it-becomes-part-of-the-type) |

## Ultimate Rust Macros

| Outline heading | Here |
|---|---|
| Why macros? — when to use them, when not to | [Why `println!` is a macro](../../37_Procedural_Macros/why_println_is_a_macro/README.md) · [A function, `macro_rules!`, or a procedural macro](../../37_Procedural_Macros/function_macro_rules_or_proc_macro/README.md) *stub* |
| … expanding macros | [Expanding a macro](../../38_Declarative_Macros/expanding_a_macro/README.md) *stub* · [Printing the HIR](../../20_Compilers/printing_the_hir/README.md) |
| `macro_rules!` — patterns | [`macro_rules!` patterns](../../38_Declarative_Macros/macro_rules_patterns/README.md) *stub* |
| … repetition | [Repetition](../../38_Declarative_Macros/repetition/README.md) *stub* |
| … usage | [Hygiene](../../38_Declarative_Macros/hygiene/README.md) *stub* · [Exporting a macro](../../38_Declarative_Macros/exporting_a_macro/README.md) *stub* |
| Proc macros — concept, syntax, the `syn` crate | [Three kinds of procedural macro](../../37_Procedural_Macros/three_kinds_of_procedural_macro/README.md) · [Parsing with `syn`](../../37_Procedural_Macros/parsing_with_syn/README.md) · [Generating with `quote`](../../37_Procedural_Macros/generating_with_quote/README.md) |
| Derive macros | [Every struct and enum shape](../../37_Procedural_Macros/every_struct_and_enum_shape/README.md) · [A `StateMachine` derive](../../37_Procedural_Macros/a_state_machine_derive/README.md) · [Expanding `thiserror`](../../37_Procedural_Macros/expanding_thiserror/README.md) |
| Beyond macros | [std macros you have not met](../../38_Declarative_Macros/std_macros_you_have_not_met/README.md) *stub* |

## Ultimate Rust Strings

The [Strings](../../14_Strings/README.md) section and its map, [STRINGS.md](../../STRINGS.md), cover this outline almost row for row.

| Outline heading | Here |
|---|---|
| Wait, what? — the string types, owned vs borrowed | [Six kinds of string](../../14_Strings/six_kinds_of_string/README.md) · [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) |
| `String`, `str` | [The anatomy of a `String`](../../14_Strings/anatomy_of_a_string/README.md) · [`str` is unsized](../../14_Strings/str_is_unsized/README.md) |
| `OsString`, `OsStr`; `CString`, `CStr` | [Six kinds of string](../../14_Strings/six_kinds_of_string/README.md) · [Calling C](../../09_Advanced/calling_c/README.md) |
| `PathBuf`, `Path` | [`Path` and `PathBuf`](../../04_Files/path_and_pathbuf/README.md) *stub* |
| String processing — useful crates | [The string crates](../../14_Strings/string_crates/README.md) · [Internationalization](../../14_Strings/internationalization/README.md) *stub* |
| … optimizations | [When `String` is too slow](../../14_Strings/when_string_is_too_slow/README.md) *stub* |
| String-like and string-adjacent types | [The third owned form: `Box<str>`, `Rc<str>`, `Arc<str>`](../../14_Strings/boxed_str/README.md) · [`Cow`](../../18_Ownership/clone_on_write/README.md) · [Meet the `char`](../../14_Strings/meet_the_char/README.md) · [Meet the byte](../../19_Numbers/meet_the_byte/README.md) |

## Ultimate Rust Tips & Tricks

| Outline heading | Here |
|---|---|
| Advanced patterns | [Pattern matching](../../30_Pattern_Matching/README.md) · [Binding with `@`](../../30_Pattern_Matching/binding_at/README.md) *stub* · [`let else`](../../30_Pattern_Matching/let_else/README.md) *stub* |
| Divergent functions | [The never type `!`](../../15_First_Programs/the_never_type/README.md) |
| Compiler configurations for a fast compile or the best binary | [Compile times](../../05_Tooling/compile_times/README.md) · [Release profiles](../../05_Tooling/release_profiles/README.md) *stub* |
| Anonymous variables | [The wildcard `_`](../../30_Pattern_Matching/the_wildcard/README.md) · [Drop guards](../../12_Traits/drop_guards/README.md) *stub* |
| How to trigger deref coercion | [Coercion](../../29_Conversion/coercion/README.md) · [Method resolution](../../12_Traits/method_resolution/README.md) |
| std macros you never heard of | [std macros you have not met](../../38_Declarative_Macros/std_macros_you_have_not_met/README.md) *stub* |
| Utility crates | [Cargo subcommands worth knowing](../../05_Tooling/cargo_subcommands/README.md) *stub* · [The string crates](../../14_Strings/string_crates/README.md) |
| Attributes | [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) · [Conditional compilation](../../27_Modules/conditional_compilation/README.md) *stub* |
| Type aliases | [A type alias is not a new type](../../16_Structs/type_aliases/README.md) *stub* · [The `Result` alias](../../17_Option_and_Result/result_aliases/README.md) |

## Ultimate Rust Threads

| Outline heading | Here |
|---|---|
| Foundations — parallelism vs concurrency, memory model | [Concurrency or parallelism](../../09_Advanced/concurrency_or_parallelism/README.md) *stub* · [`Send` and `Sync`](../../09_Advanced/send_and_sync/README.md) *stub* |
| … spawning and joining threads | [Spawning a thread](../../09_Advanced/spawning_a_thread/README.md) |
| The hard way — `Mutex`, `RwLock`, `Arc` | [Lock poisoning](../../09_Advanced/mutex_poisoning/README.md) · [`RwLock` and atomics](../../09_Advanced/rwlock_and_atomics/README.md) *stub* · [Sharing across threads: `Arc`](../../18_Ownership/sharing_across_threads/README.md) |
| … atomics | [Atomic orderings](../../09_Advanced/atomic_orderings/README.md) *stub* · [Compare and exchange](../../09_Advanced/compare_and_exchange/README.md) *stub* |
| The easy way — channels, `mpsc` vs crossbeam | [Channels](../../09_Advanced/channels/README.md) · [Crossbeam channels](../../09_Advanced/crossbeam_channels/README.md) *stub* |
| Just plain cheating — rayon | [Rayon](../../09_Advanced/rayon/README.md) *stub* |

The same topics in six languages are in the [Concurrency library ↗](https://masiarek.github.io/concurrency-learning-library/), and in Go in the [Go library ↗](https://masiarek.github.io/go-learning-library/).

## Ultimate Rust WebAssembly

| Outline heading | Here |
|---|---|
| What is WebAssembly? | [What WebAssembly is](../../42_WebAssembly/what_webassembly_is/README.md) *stub* |
| Cross-compiling to WebAssembly | [Compiling to wasm32](../../42_WebAssembly/compiling_to_wasm32/README.md) *stub* · [Targets and triples](../../20_Compilers/targets_and_triples/README.md) *stub* |
| WebAssembly in the browser | [Rust in the browser](../../42_WebAssembly/rust_in_the_browser/README.md) *stub* |
| WebAssembly backend runtimes | [WebAssembly outside the browser](../../42_WebAssembly/wasm_outside_the_browser/README.md) *stub* |
| Deploying a game to WebAssembly | [Shipping a wasm page](../../42_WebAssembly/shipping_a_wasm_page/README.md) *stub* |

## See also

- [*Rust for Rustaceans*, chapter by chapter](../rust_for_rustaceans/README.md) — the book one of these outlines follows
- [Start here](../../00_Start_Here/README.md) — the free courses this library is a companion to
- [Books](../books/README.md) — the reading shelf
- [TOPICS.md](../../TOPICS.md) — every topic above, under its other names

## Po polsku

Nathan Stocks prowadzi dwa wydane kursy wideo o Ruście (Crash Course i Ultimate Rust 2) i rozpisał plany dziewięciu kolejnych, każdy o jednym temacie: Bevy, wkład w rozwój Rusta, *Rust for Rustaceans*, generyki i czasy życia, makra, napisy, sztuczki, wątki i WebAssembly. Ta strona zestawia każdy nagłówek z tych planów ze stroną biblioteki, która go omawia. Czy któryś z planowanych kursów się ukazał, nie zostało tu sprawdzone.

**Szukaj po polsku:** kurs Rusta wideo · Ultimate Rust Crash Course · `nathan stocks rust course` · `ultimate rust 2 intermediate concepts`
