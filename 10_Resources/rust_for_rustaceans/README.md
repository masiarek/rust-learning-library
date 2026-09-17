# *Rust for Rustaceans*, chapter by chapter

**Level:** reference · the map

**One line:** Jon Gjengset's book is the usual "second Rust book", and this page lists its thirteen chapters section by section, with the page in this library that covers each section — a lesson, a stub with its boundaries written down, or a gap named as one.

[*Rust for Rustaceans* ↗](https://nostarch.com/rust-rustaceans) (No Starch Press, 2021, first edition) is reviewed on the [Books](../books/README.md) shelf. The book is not free, so this page quotes nothing from it. It gives chapter and section names, which stay stable when page numbers move between printings, and points at pages you can read here. Nathan Stocks outlined a video course that follows the same chapter order; [the Ultimate Rust courses](../ultimate_rust_courses/README.md) page maps that outline.

**Reading the tables.** *lesson* means a checked page with a program behind it. *stub* means an outline listing the questions the page must answer. *gap* means nothing here yet. Checked 2026-09-16.

## 1. Foundations

| Section | Here | Status |
|---|---|---|
| Talking About Memory — terminology, variables in depth, memory regions | [Stack and heap](../../18_Ownership/stack_and_heap/README.md) · [A name is not a place](../../18_Ownership/a_name_is_not_a_place/README.md) · [The call stack](../../18_Ownership/the_call_stack/README.md) · [`const` and `static`](../../27_Modules/const_and_static/README.md) | lessons |
| Ownership | [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) · [The drop flag](../../18_Ownership/the_drop_flag/README.md) | lessons |
| Borrowing and Lifetimes — shared and mutable references | [Borrowing](../../18_Ownership/borrowing/README.md) · [Reborrowing](../../18_Ownership/reborrowing/README.md) · [What `&'a T` claims](../../18_Ownership/what_a_reference_claims/README.md) | lessons |
| … interior mutability | [Interior mutability](../../09_Advanced/interior_mutability/README.md) | stub |
| … lifetimes | [Lifetime annotations](../../18_Ownership/lifetime_annotations/README.md) · [What a lifetime does at the call site](../../18_Ownership/lifetimes_at_the_call_site/README.md) · [Temporary lifetime extension](../../18_Ownership/temporary_lifetimes/README.md) | lessons; variance is a gap |

## 2. Types

| Section | Here | Status |
|---|---|---|
| Types in Memory — alignment, layout, complex types | [Type layout](../../09_Advanced/type_layout/README.md) · [What is a record, in memory?](../../16_Structs/representing_a_record/README.md) · [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) | stub + lessons |
| … dynamically sized types and wide pointers | [Wide pointers](../../36_Pointers/wide_pointers/README.md) · [`str` is unsized](../../14_Strings/str_is_unsized/README.md) · [Step 2: some types have no size](../../12_Traits/how_to_learn_to_owned/types_with_no_size/README.md) | lessons |
| Traits and Trait Bounds — compilation and dispatch | [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) | lesson |
| … generic traits | [Associated types or type parameters?](../../22_Generics/associated_types_or_type_parameters/README.md) | stub |
| … coherence and the orphan rule | [`From` and `Into`](../../29_Conversion/from_and_into/README.md) · [Extension traits](../../12_Traits/extension_traits/README.md) · [Blanket impls](../../22_Generics/blanket_impls/README.md) | lessons + stub |
| … trait bounds | [Where the bound goes](../../22_Generics/where_the_bound_goes/README.md) | lesson |
| … marker traits | [Marker traits](../../12_Traits/marker_traits/README.md) · [Phantom types](../../12_Traits/phantom_types/README.md) | lessons |
| Existential Types | [Returning a trait](../../12_Traits/returning_a_trait/README.md) · [Returning an iterator](../../24_Iterators/returning_an_iterator/README.md) | lessons |

## 3. Designing Interfaces

The whole chapter is the [API design](../../39_API_Design/README.md) section, which follows the book's four headings.

| Section | Here | Status |
|---|---|---|
| Unsurprising — naming, common traits, ergonomic impls, wrapper types | [Naming conventions](../../39_API_Design/naming_conventions/README.md) · [Traits every type should consider](../../39_API_Design/traits_every_type_should_consider/README.md) · [Blanket impls for references](../../39_API_Design/blanket_impls_for_references/README.md) · [Wrapper types](../../39_API_Design/wrapper_types/README.md) | stubs |
| Flexible — generic arguments, object safety, borrowed vs owned, destructors | [Generic or concrete parameters](../../39_API_Design/generic_or_concrete_parameters/README.md) · [Dyn compatibility](../../39_API_Design/dyn_compatibility/README.md) · [String parameters worth copying](../../14_Strings/string_api_design/README.md) · [Destructors that can fail](../../39_API_Design/destructors_that_can_fail/README.md) | stubs + lesson |
| Obvious — documentation, type system guidance | [Documenting an interface](../../39_API_Design/documenting_an_interface/README.md) · [Making misuse a compile error](../../39_API_Design/making_misuse_a_compile_error/README.md) · [An enum instead of a bool](../../13_Enums/an_enum_instead_of_a_bool/README.md) | stubs + lesson |
| Constrained — type modifications, trait implementations, hidden contracts | [Semver hazards](../../39_API_Design/semver_hazards/README.md) | stub |

## 4. Error Handling

| Section | Here | Status |
|---|---|---|
| Representing Errors — enumeration, opaque errors | [What makes a type an error](../../02_Errors/the_error_trait/README.md) · [`thiserror` vs `anyhow`](../../02_Errors/thiserror_vs_anyhow/README.md) · [Not every error is an `io::Error`](../../02_Errors/not_every_error_is_io_error/README.md) | lessons + stub |
| … special error cases | [The never type `!`](../../15_First_Programs/the_never_type/README.md) · [Returning `None` on error](../../17_Option_and_Result/none_on_error/README.md) | lessons |
| Propagating Errors | [The `?` operator](../../17_Option_and_Result/the_question_mark_operator/README.md) · [`anyhow` and context](../../02_Errors/anyhow_and_context/README.md) | lesson + stub |

## 5. Project Structure

| Section | Here | Status |
|---|---|---|
| Features | [Cargo features](../../05_Tooling/cargo_features/README.md) | stub |
| Workspaces | [Workspaces](../../05_Tooling/workspaces/README.md) · [A tree of practice projects](../../05_Tooling/practice_workspace/README.md) | stub + lesson |
| Project Configuration — crate metadata, build configuration | [Publishing a crate](../../05_Tooling/publishing_a_crate/README.md) · [Release profiles](../../05_Tooling/release_profiles/README.md) · [Compile times](../../05_Tooling/compile_times/README.md) | stubs + lesson |
| Conditional Compilation | [Conditional compilation](../../27_Modules/conditional_compilation/README.md) | stub |
| Versioning — MSRV, minimal dependency versions, changelogs | [MSRV](../../05_Tooling/msrv/README.md) · [Two versions of one crate](../../05_Tooling/two_versions_of_one_crate/README.md) · [`Cargo.lock`](../../05_Tooling/cargo_lock/README.md) | stub + lessons |

## 6. Testing

| Section | Here | Status |
|---|---|---|
| Rust Testing Mechanisms — the harness, `#[cfg(test)]`, doctests | [How `cargo test` runs your tests](../../28_Testing/how_cargo_test_runs/README.md) · [Where a test goes](../../28_Testing/where_a_test_goes/README.md) · [The example that is a test](../../28_Testing/doc_tests/README.md) | lessons + stub |
| Additional Testing Tools — linting, test generation and augmentation | [Other kinds of test](../../28_Testing/other_kinds_of_test/README.md) · [Clippy](../../34_Templates/clippy/README.md) · [cargo-nextest](../../05_Tooling/nextest/README.md) · [Miri](../../09_Advanced/miri/README.md) | stubs + lessons |
| … performance testing | [Benchmark harnesses](../../33_Time_and_Benchmarking/benchmark_harnesses/README.md) · [`black_box` is a hint](../../33_Time_and_Benchmarking/black_box_is_a_hint/README.md) | stub + lesson |

## 7. Macros

| Section | Here | Status |
|---|---|---|
| Declarative Macros | [Declarative macros](../../38_Declarative_Macros/README.md) · [Macros](../../25_Control_Flow/macros/README.md) | stubs |
| Procedural Macros | [Procedural macros](../../37_Procedural_Macros/README.md) · [A function, `macro_rules!`, or a procedural macro](../../37_Procedural_Macros/function_macro_rules_or_proc_macro/README.md) · [Three kinds of procedural macro](../../37_Procedural_Macros/three_kinds_of_procedural_macro/README.md) | lessons and stubs |

## 8. Asynchronous Programming

| Section | Here | Status |
|---|---|---|
| What's the Deal with Asynchrony? | [Async](../../35_Async/README.md) · [What a future is](../../35_Async/what_a_future_is/README.md) · [`Iterator` versus `Stream`](../../24_Iterators/iterator_vs_stream/README.md) | lessons |
| Ergonomic Futures, Going to Sleep, spawn | [`async fn` and `.await`](../../35_Async/async_fn_and_await/README.md) · [The Tokio runtime](../../35_Async/building_minidb/the_tokio_runtime/README.md) · [Tasks](../../35_Async/building_minidb/tasks/README.md) · [Common async pitfalls](../../35_Async/common_async_pitfalls/README.md) | lessons + stubs |

## 9. Unsafe Code

| Section | Here | Status |
|---|---|---|
| The `unsafe` Keyword | [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) | lesson |
| Great Power — raw pointers, unsafe functions, unsafe traits | [Raw pointers](../../36_Pointers/raw_pointers/README.md) · [Unsafe traits](../../09_Advanced/unsafe_traits/README.md) · [What a union is](../../09_Advanced/what_a_union_is/README.md) | lessons + stub |
| Great Responsibility — validity, panics, casting, the drop check | [Validity invariants](../../09_Advanced/validity_invariants/README.md) · [Panics in unsafe code](../../09_Advanced/panics_in_unsafe_code/README.md) · [Transmute](../../09_Advanced/transmute/README.md) · [The drop check](../../09_Advanced/the_drop_check/README.md) | stubs |
| Coping with Fear — unsafe boundaries, documentation, checking your work | [What an invariant is](../../09_Advanced/what_an_invariant_is/README.md) · [Documenting unsafe contracts](../../31_C_and_Cpp/migrating_c_to_rust/documenting_unsafe_contracts/README.md) · [Miri](../../09_Advanced/miri/README.md) | lesson + stubs |

## 10. Concurrency (and Parallelism)

| Section | Here | Status |
|---|---|---|
| The Trouble with Concurrency — correctness, performance | [Concurrency or parallelism](../../09_Advanced/concurrency_or_parallelism/README.md) · [Data races](../../31_C_and_Cpp/data_races/README.md) | stub + lesson |
| Concurrency Models — shared memory, worker pools, actors | [Spawning a thread](../../09_Advanced/spawning_a_thread/README.md) · [Worker pools](../../09_Advanced/worker_pools/README.md) · [Actors](../../09_Advanced/actors/README.md) | lesson + stubs |
| Asynchrony and Parallelism | [Rayon](../../09_Advanced/rayon/README.md) | stub |
| Lower-Level Concurrency — memory operations, atomics, ordering, compare-and-exchange | [`RwLock` and atomics](../../09_Advanced/rwlock_and_atomics/README.md) · [Atomic orderings](../../09_Advanced/atomic_orderings/README.md) · [Compare and exchange](../../09_Advanced/compare_and_exchange/README.md) · [False sharing](../../09_Advanced/false_sharing/README.md) | stubs |
| Sane Concurrency — start simple, stress tests, tools | [Testing concurrent code](../../09_Advanced/testing_concurrent_code/README.md) | stub |

## 11. Foreign Function Interfaces

| Section | Here | Status |
|---|---|---|
| Crossing Boundaries with `extern` — symbols, calling conventions | [Calling C](../../09_Advanced/calling_c/README.md) · [The C ABI](../../31_C_and_Cpp/migrating_c_to_rust/the_c_abi/README.md) · [The linker](../../20_Compilers/the_linker/README.md) | lessons + stub |
| Types Across Language Boundaries — type matching, allocations, callbacks | [FFI-safe types](../../31_C_and_Cpp/migrating_c_to_rust/ffi_safe_types/README.md) · [Data across the boundary](../../31_C_and_Cpp/migrating_c_to_rust/data_across_the_boundary/README.md) · [Callbacks across FFI](../../09_Advanced/callbacks_across_ffi/README.md) | stubs |
| bindgen and Build Scripts | [Generating bindings](../../31_C_and_Cpp/migrating_c_to_rust/generating_bindings/README.md) · [Build scripts](../../05_Tooling/build_scripts/README.md) | stubs |

## 12. Rust Without the Standard Library

The whole chapter is the [Without std](../../40_Without_std/README.md) section.

| Section | Here | Status |
|---|---|---|
| Opting Out of the Standard Library | [`core`, `alloc` and `std`](../../40_Without_std/core_alloc_and_std/README.md) | stub |
| Dynamic Memory Allocation | [Allocating without std](../../40_Without_std/allocating_without_std/README.md) · [The global allocator](../../09_Advanced/the_global_allocator/README.md) | stub + lesson |
| The Rust Runtime — panic handler, initialization, out-of-memory handler | [The panic handler](../../40_Without_std/the_panic_handler/README.md) · [Before main runs](../../40_Without_std/before_main_runs/README.md) · [Out of memory](../../40_Without_std/out_of_memory/README.md) | stubs |
| Low-Level Memory Accesses | [Memory-mapped registers](../../40_Without_std/memory_mapped_registers/README.md) | stub |
| Misuse-Resistant Hardware Abstraction | [Hardware you cannot misuse](../../40_Without_std/hardware_you_cannot_misuse/README.md) | stub |
| Cross-Compilation | [Cross-compiling](../../40_Without_std/cross_compiling/README.md) · [Targets and triples](../../20_Compilers/targets_and_triples/README.md) | stubs |

## 13. The Rust Ecosystem

| Section | Here | Status |
|---|---|---|
| What's Out There? — tools, libraries | [Cargo subcommands worth knowing](../../05_Tooling/cargo_subcommands/README.md) · [The string crates](../../14_Strings/string_crates/README.md) · [Search tools in Rust](../../11_Unix/search_tools_in_rust/README.md) | stub + lessons |
| Patterns in the Wild — index pointers, drop guards, extension traits, crate preludes | [Indices instead of references](../../18_Ownership/indices_instead_of_references/README.md) · [Drop guards](../../12_Traits/drop_guards/README.md) · [Extension traits](../../12_Traits/extension_traits/README.md) · [A crate prelude](../../27_Modules/a_crate_prelude/README.md) | stubs + lesson |
| Staying Up to Date, What Next? | [Resources](../README.md) · [Going deeper](../going_deeper/README.md) | lessons |

## See also

- [Books](../books/README.md) — the review of this book, and every other one
- [Rust by Example, chapter by chapter](../../RUST_BY_EXAMPLE.md) — the same kind of map for a free book
- [TOPICS.md](../../TOPICS.md) — every topic above under its other names
- [Going deeper](../going_deeper/README.md) — the domain shelves this book's later chapters open onto
- [Jon Gjengset's videos ↗](https://www.youtube.com/@jonhoo) — the author's *Crust of Rust* streams cover several chapters in video form

## Po polsku

Ta strona to mapa książki *Rust for Rustaceans* Jona Gjengseta, rozdział po rozdziale: przy każdej sekcji jest strona tej biblioteki, która ją omawia — gotowa lekcja, szkic (*stub*) z wypisanymi pytaniami albo nazwana luka. Książka nie jest darmowa, więc nie ma tu cytatów, tylko nazwy rozdziałów i sekcji, które nie zmieniają się między wydrukami.

**Szukaj po polsku:** Rust for Rustaceans po polsku · zaawansowany Rust · `rust for rustaceans chapter summary` · `jon gjengset crust of rust`
