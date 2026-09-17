# Topic index

**Level:** reference · the map

**One line:** Every topic in this library under every name you might look for it by — *vector*, *hash map*, *static const*, *compound types*, *Shipyard* — with the page that is its home and the pages to read next.

A folder has one name, and it is rarely the word you typed: a *vector* lives in `26_Collections/the_vec`, `static` lives in `27_Modules/const_and_static`, and "SSH keys for a private registry" lives in `05_Tooling/registry_authentication`. This page is the index at the back of the book. Rows marked *stub* are outlines with their boundaries written down but no checked example yet.

## How to find anything here

| You have | Go to |
|---|---|
| a word or a topic | **this page**, then the site search (press `/`) |
| a whole subject you want to read in order | a map: [TYPES.md](TYPES.md) · [OPTION.md](OPTION.md) · [STRINGS.md](STRINGS.md) · [STRUCTS.md](STRUCTS.md) · [SHADOWING.md](SHADOWING.md) · [TOOLCHAIN.md](TOOLCHAIN.md) |
| an error code such as `E0502` | [ERRORS.md](ERRORS.md) |
| a term to define in one line | [GLOSSARY.md](GLOSSARY.md) |
| something to type | [KATAS.md](KATAS.md) |
| a book or course, chapter by chapter | [Rust by Example](RUST_BY_EXAMPLE.md) · [*Rust for Rustaceans*](10_Resources/rust_for_rustaceans/README.md) · [the Ultimate Rust courses](10_Resources/ultimate_rust_courses/README.md) · [*Rust in Action*, run](10_Resources/rust_in_action/README.md) |
| what to read next, section by section | [the course table on the homepage](index.md#the-course-in-order) |
| Polish terminology | [POLSKI.md](POLSKI.md) |

**How the pieces fit.** A *topic folder* holds one idea and never moves. A *section* (`14_Strings/`) groups folders by subject, and its README is a table of them. A *map* (the root `*.md` pages above) threads one subject through several sections in reading order. This index points at all three. Every lesson ends with *See also* links both ways and an *If you are coming from another language* section that links the matching page in the sibling libraries.

## A

| Topic, and other names for it | Home | Then |
|---|---|---|
| **`anyhow`**, error context | [`anyhow` and context](02_Errors/anyhow_and_context/README.md) *stub* | [`thiserror` vs `anyhow`](02_Errors/thiserror_vs_anyhow/README.md) *stub* |
| **API design**, interface design, API guidelines | [API design](39_API_Design/README.md) | [String parameters worth copying](14_Strings/string_api_design/README.md) |
| **`Arc`**, atomically reference counted | [Sharing across threads: `Arc`](18_Ownership/sharing_across_threads/README.md) | [`Rc`](18_Ownership/reference_counting/README.md) · [`Send` and `Sync`](09_Advanced/send_and_sync/README.md) *stub* |
| **Array**, fixed-size array, `[T; N]`, static array, compound type, array iterate, write array values, filter an array | [Arrays: the map](26_Collections/arrays/README.md) | [Arrays and slices](26_Collections/arrays_and_slices/README.md) · [Writing an array down](26_Collections/arrays/writing_an_array_down/README.md) · [Array or `Vec`?](26_Collections/array_or_vec/README.md) · [A `Vec` of arrays](26_Collections/vec_of_arrays/README.md) · [Const generics](22_Generics/const_generics/README.md) *stub* · [TYPES.md](TYPES.md) |
| **`as`**, casting, type cast | [Casting with `as`](29_Conversion/casting_with_as/README.md) | [Conversion](29_Conversion/README.md) |
| **`AsRef`**, `AsMut`, cheap borrow conversion | [`AsRef` and `AsMut`](29_Conversion/as_ref_and_as_mut/README.md) *stub* | [`Borrow`](12_Traits/borrow_trait/README.md) |
| **Associated types**, `type Item` | [Associated types or type parameters?](22_Generics/associated_types_or_type_parameters/README.md) *stub* | [Implementing `Iterator`](24_Iterators/implementing_iterator/README.md) |
| **Associated function**, `Type::new`, constructor | [`impl` blocks](16_Structs/impl_blocks/README.md) | [A type is not a constructor](16_Structs/a_type_is_not_a_constructor/README.md) |
| **Async**, `async`/`await`, futures, Tokio | [Async](35_Async/README.md) | [What a future is](35_Async/what_a_future_is/README.md) · [`async fn` and `.await`](35_Async/async_fn_and_await/README.md) · [Building minidb](35_Async/building_minidb/README.md) · [`Iterator` versus `Stream`](24_Iterators/iterator_vs_stream/README.md) |
| **Atomics**, `AtomicUsize`, `Ordering::SeqCst` | [`RwLock` and atomics](09_Advanced/rwlock_and_atomics/README.md) *stub* | [Atomic orderings](09_Advanced/atomic_orderings/README.md) *stub* · [Compare and exchange](09_Advanced/compare_and_exchange/README.md) *stub* |
| **Attributes**, `#[derive]`, `#[allow]` | [What an attribute is](27_Modules/what_an_attribute_is/README.md) | [Conditional compilation](27_Modules/conditional_compilation/README.md) *stub* |
| **Advent of Code** | [Advent of Code in Rust](10_Resources/exercises/advent_of_code/README.md) *stub* | [Exercises](10_Resources/exercises/README.md) |

## B

| Topic, and other names for it | Home | Then |
|---|---|---|
| **Backtrace**, `RUST_BACKTRACE` | [Reading a backtrace](17_Option_and_Result/reading_a_backtrace/README.md) | [What a panic costs](17_Option_and_Result/what_a_panic_costs/README.md) |
| **Benchmarking**, Criterion, Divan, timeit | [Timing a block](33_Time_and_Benchmarking/timing_a_block/README.md) | [Benchmark harnesses](33_Time_and_Benchmarking/benchmark_harnesses/README.md) *stub* · [`black_box` is a hint](33_Time_and_Benchmarking/black_box_is_a_hint/README.md) |
| **Bevy**, game engine, ECS | [Games with Bevy](43_Games/README.md) | [Entities and components](43_Games/entities_and_components/README.md) *stub* |
| **bindgen**, C headers | [Generating bindings](31_C_and_Cpp/migrating_c_to_rust/generating_bindings/README.md) *stub* | [Build scripts](05_Tooling/build_scripts/README.md) *stub* |
| **Blanket impl**, `impl<T: X> Y for T` | [Blanket impls](22_Generics/blanket_impls/README.md) *stub* | [Step 6: one blanket impl](12_Traits/how_to_learn_to_owned/the_blanket_to_owned/README.md) |
| **Block**, scope, `{ }` | [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md) | [Scope is about names](18_Ownership/scope_is_about_names/README.md) |
| **`bool`**, boolean | [Meet the `bool`](15_First_Programs/meet_the_bool/README.md) | [An enum instead of a bool](13_Enums/an_enum_instead_of_a_bool/README.md) |
| **Borrowing**, references, `&`, `&mut`, borrow checker | [Borrowing](18_Ownership/borrowing/README.md) | [Borrowed state](18_Ownership/borrowed_state/README.md) · [Reborrowing](18_Ownership/reborrowing/README.md) · [Where the `&` sits](18_Ownership/where_the_sigil_sits/README.md) |
| **`Box`**, heap allocation | [`Box`](26_Collections/the_box/README.md) | [Stack and heap](18_Ownership/stack_and_heap/README.md) · [The third owned form: `Box<str>`](14_Strings/boxed_str/README.md) |
| **`break`**, `continue`, loop labels | [`break`](25_Control_Flow/break_expressions/README.md) | [`continue`](25_Control_Flow/continue_expressions/README.md) · [Loop labels](25_Control_Flow/loop_labels/README.md) · [Flow control](25_Control_Flow/flow_control/README.md) |
| **`BTreeMap`**, `BTreeSet`, sorted map | [`BTreeMap` and `BTreeSet`](26_Collections/sorted_collections/README.md) | |
| **Build scripts**, `build.rs` | [Build scripts](05_Tooling/build_scripts/README.md) *stub* | [Calling C](09_Advanced/calling_c/README.md) |
| **Bytes**, `u8`, `[u8]` | [Meet the byte](19_Numbers/meet_the_byte/README.md) | [A file is bytes](04_Files/a_file_is_bytes/README.md) · [Printing bytes](19_Numbers/printing_bytes/README.md) |

## C

| Topic, and other names for it | Home | Then |
|---|---|---|
| **C and C++ bugs**, memory safety | [C and C++ — the bugs Rust is a reply to](31_C_and_Cpp/README.md) | [What Rust is](00_Start_Here/what_rust_is/README.md) *stub* |
| **Calling C**, FFI, `extern "C"` | [Calling C](09_Advanced/calling_c/README.md) | [Migrating C to Rust](31_C_and_Cpp/migrating_c_to_rust/README.md) · [FFI-safe types](31_C_and_Cpp/migrating_c_to_rust/ffi_safe_types/README.md) *stub* · [Data across the boundary](31_C_and_Cpp/migrating_c_to_rust/data_across_the_boundary/README.md) *stub* · [Callbacks across FFI](09_Advanced/callbacks_across_ffi/README.md) *stub* |
| **Cargo**, package manager, dependencies | [Adding a dependency](05_Tooling/cargo_dependencies/README.md) | [From one `.rs` file to a Cargo project](05_Tooling/from_rustc_to_cargo/README.md) · [TOOLCHAIN.md](TOOLCHAIN.md) · [`Cargo.lock`](05_Tooling/cargo_lock/README.md) · [Cargo subcommands](05_Tooling/cargo_subcommands/README.md) *stub* |
| **Cargo features**, `[features]`, optional dependencies | [Cargo features](05_Tooling/cargo_features/README.md) *stub* | [Conditional compilation](27_Modules/conditional_compilation/README.md) *stub* |
| **`Cell`**, `RefCell`, shareable mutable containers | [Interior mutability](09_Advanced/interior_mutability/README.md) *stub* | [TYPES.md: pointers](TYPES.md#pointers) |
| **`cfg`**, `#[cfg]`, `cfg!`, conditional compilation | [Conditional compilation](27_Modules/conditional_compilation/README.md) *stub* | [What an attribute is](27_Modules/what_an_attribute_is/README.md) |
| **Channels**, `mpsc`, crossbeam | [Channels](09_Advanced/channels/README.md) | [Crossbeam channels](09_Advanced/crossbeam_channels/README.md) *stub* · [Go library: channels ↗](https://masiarek.github.io/go-learning-library/02_Channels/) |
| **`char`**, character, Unicode scalar value | [Meet the `char`](14_Strings/meet_the_char/README.md) | [Why a `char` is 32 bits](14_Strings/why_char_is_32_bits/README.md) |
| **Clippy**, lints | [Clippy: the groups, the settings, the commands](34_Templates/clippy/README.md) | [Strict clippy lints](05_Tooling/strict_lints/README.md) · [Lints around tests](28_Testing/testing_lints/README.md) · [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md) |
| **`Clone`**, cloning, `.clone()` | [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) | [What a clone costs](18_Ownership/what_a_clone_costs/README.md) · [`ToOwned`](12_Traits/to_owned/README.md) · [Reading the `Clone for &T` hover](12_Traits/reading_the_clone_hover/README.md) |
| **Closures**, lambdas, anonymous functions, nested functions | [What a closure is](23_Closures/what_a_closure_is/README.md) | [The three closure traits](23_Closures/three_closure_traits/README.md) · [The `move` keyword](23_Closures/the_move_keyword/README.md) · [Items inside a function](27_Modules/items_inside_a_function/README.md) *stub* |
| **Coercion**, deref coercion, unsizing | [Coercion](29_Conversion/coercion/README.md) | [Method resolution](12_Traits/method_resolution/README.md) |
| **Collections** | [Collections](26_Collections/README.md) | [TYPES.md: built by std](TYPES.md#built-by-std) |
| **`collect`**, `FromIterator` | [`collect` and `FromIterator`](24_Iterators/collect_and_fromiterator/README.md) | [Collect the iterator into a `Vec`](24_Iterators/collect_into_a_vec/README.md) |
| **Comments**, doc comments, `///` | [Comments that compile](15_First_Programs/comments_that_compile/README.md) | [The example that is a test](28_Testing/doc_tests/README.md) |
| **Comparison**, `PartialEq`, `Ord`, sorting | [The comparison traits](12_Traits/comparison_traits/README.md) *stub* | [Comparing and sorting text](14_Strings/comparing_strings/README.md) |
| **Compile times**, slow builds | [Compile times](05_Tooling/compile_times/README.md) | [Release profiles](05_Tooling/release_profiles/README.md) *stub* |
| **Compiler**, rustc, LLVM, linker | [Compilers](20_Compilers/README.md) | [What a compiler does](20_Compilers/what_a_compiler_does/README.md) · [LLVM](20_Compilers/llvm_and_its_ir/README.md) · [The linker](20_Compilers/the_linker/README.md) |
| **Compound types**, tuples and arrays | [TYPES.md: compound types](TYPES.md#compound-types-the-two-that-are-primitive) | [Tuples](26_Collections/tuples/README.md) · [Arrays: the map](26_Collections/arrays/README.md) · [Arrays and slices](26_Collections/arrays_and_slices/README.md) |
| **Concurrency**, threads, parallelism | [Spawning a thread](09_Advanced/spawning_a_thread/README.md) | [Concurrency or parallelism](09_Advanced/concurrency_or_parallelism/README.md) *stub* · [Concurrency library ↗](https://masiarek.github.io/concurrency-learning-library/) |
| **`const`**, `static`, constants vs statics, static const | [`const` and `static`](27_Modules/const_and_static/README.md) | [`&'static str`](14_Strings/static_str/README.md) |
| **Const generics**, `const N: usize` | [Const generics](22_Generics/const_generics/README.md) *stub* | [Arrays and slices](26_Collections/arrays_and_slices/README.md) · [Arrays in and out of functions](26_Collections/arrays/arrays_in_signatures/README.md#const-generics-every-length-and-the-same-length-back) |
| **Contributing to Rust**, rustc development | [A first contribution](20_Compilers/a_first_contribution/README.md) *stub* | [The Rust project](20_Compilers/the_rust_project/README.md) *stub* · [Building the compiler](20_Compilers/building_the_compiler/README.md) *stub* |
| **Control flow**, `if`, `match`, loops | [Control flow](25_Control_Flow/README.md) | [Pattern matching](30_Pattern_Matching/README.md) |
| **`Copy`**, copying vs moving | [Copy or move?](18_Ownership/copy_or_move/README.md) | [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) · [There is no `Move` trait](18_Ownership/no_move_trait/README.md) |
| **`Cow`**, clone on write | [`Cow`: borrow until somebody writes](18_Ownership/clone_on_write/README.md) | [What `Cow` explanations get wrong](12_Traits/how_to_learn_to_owned/cow_claims_checked/README.md) |
| **Crates**, packages, `crate::` | [Packages and crates](27_Modules/packages_and_crates/README.md) *stub* | [Modules and visibility](27_Modules/modules_and_visibility/README.md) · [Publishing a crate](05_Tooling/publishing_a_crate/README.md) *stub* |
| **Cross-compiling**, targets | [Cross-compiling](40_Without_std/cross_compiling/README.md) *stub* | [Targets and triples](20_Compilers/targets_and_triples/README.md) *stub* |

## D – F

| Topic, and other names for it | Home | Then |
|---|---|---|
| **Data types**, types in Rust | [TYPES.md](TYPES.md) | [Values](15_First_Programs/values/README.md) |
| **`dbg!`** | [What `dbg!` does](15_First_Programs/what_dbg_does/README.md) | [Debugging Rust](32_Debugging/README.md) |
| **Debug and Display**, `{:?}`, `{}` | [Debug and Display](15_First_Programs/debug_vs_display/README.md) | [The format mini-language](14_Strings/the_format_language/README.md) |
| **Debugging** | [Debugging Rust](32_Debugging/README.md) | |
| **`Default`** | [The `Default` trait](03_Command_Line/the_default_trait/README.md) *stub* | [`unwrap_or_default`](17_Option_and_Result/unwrap_or_default/README.md) |
| **Destructuring** | [Destructuring structs](30_Pattern_Matching/destructuring_structs/README.md) *stub* | [Destructuring enums](30_Pattern_Matching/destructuring_enums/README.md) *stub* |
| **Divergent functions**, `-> !`, never type | [The never type `!`](15_First_Programs/the_never_type/README.md) | |
| **`Drop`**, destructors, RAII, drop guard | [`Drop`, and what RAII buys](12_Traits/drop_and_raii/README.md) | [Drop guards](12_Traits/drop_guards/README.md) *stub* · [The drop flag](18_Ownership/the_drop_flag/README.md) |
| **Dynamic dispatch**, `dyn Trait`, trait objects, vtable | [Static vs dynamic dispatch](12_Traits/static_vs_dynamic_dispatch/README.md) | [Dyn compatibility](39_API_Design/dyn_compatibility/README.md) *stub* |
| **Editions**, 2021, 2024 | [What Rust is](00_Start_Here/what_rust_is/README.md) *stub* | [`iter`, `iter_mut`, `into_iter`](24_Iterators/iter_iter_mut_into_iter/README.md) (the 2021 array change) |
| **Editors**, IDE, RustRover, Zed, Neovim | [Choosing an editor](05_Tooling/editors/README.md) | [RustRover setup](05_Tooling/rustrover_setup/README.md) · [Zed setup](05_Tooling/zed_setup/README.md) |
| **Embedded**, `no_std`, bare metal | [Without std](40_Without_std/README.md) | [`core`, `alloc` and `std`](40_Without_std/core_alloc_and_std/README.md) *stub* |
| **Enum**, variants, sum type | [What an enum is](13_Enums/what_an_enum_is/README.md) | [Variants that carry data](13_Enums/variants_that_carry_data/README.md) · [An enum as a state machine](13_Enums/an_enum_as_a_state_machine/README.md) |
| **Error codes**, `E0308`, `E0502` | [ERRORS.md](ERRORS.md) | |
| **Error handling**, `Result`, `?`, `Error` trait | [Errors](02_Errors/README.md) | [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md) · [The `?` operator](17_Option_and_Result/the_question_mark_operator/README.md) · [What makes a type an error](02_Errors/the_error_trait/README.md) |
| **Exit status**, stderr | [Standard error, and exit status](02_Errors/stderr_and_exit_status/README.md) | |
| **`expect`**, `unwrap` | [`expect`: writing down the proof](17_Option_and_Result/expect/README.md) | [`unwrap` is a TODO](02_Errors/unwrap_is_a_todo/README.md) |
| **Extension traits** | [Extension traits](12_Traits/extension_traits/README.md) | [A crate prelude](27_Modules/a_crate_prelude/README.md) *stub* |
| **Files**, `File::open`, reading lines | [Files](04_Files/README.md) | [Opening a file](04_Files/opening_a_file/README.md) |
| **Floats**, `f64`, rounding | [What a float actually stores](19_Numbers/what_a_float_stores/README.md) | [Making a float whole](19_Numbers/rounding_a_float/README.md) |
| **`fold`**, `reduce`, `sum` | [`fold` and `reduce`](24_Iterators/fold_and_reduce/README.md) | |
| **`for` loops** | [`for` loops](25_Control_Flow/for_loops/README.md) | [Loops without an index](25_Control_Flow/loops_without_an_index/README.md) · [Ranges](24_Iterators/ranges/README.md) *stub* · [`iter`, `iter_mut`, `into_iter`](24_Iterators/iter_iter_mut_into_iter/README.md) |
| **Formatting**, `format!`, `{:>8}`, rustfmt | [The format mini-language](14_Strings/the_format_language/README.md) | [Formatting: `rustfmt`](05_Tooling/formatting/README.md) |
| **`From` and `Into`**, orphan rule | [`From` and `Into`](29_Conversion/from_and_into/README.md) | [`TryFrom` and `TryInto`](29_Conversion/tryfrom_and_tryinto/README.md) |
| **Function pointers**, `fn(u32) -> u32` | [Function pointers](23_Closures/function_pointers/README.md) | |
| **Functions**, parameters, return values | [Functions](25_Control_Flow/functions/README.md) | [The call stack](18_Ownership/the_call_stack/README.md) · [Returned by value](18_Ownership/returned_by_value/README.md) · [Items inside a function](27_Modules/items_inside_a_function/README.md) *stub* |

## G – L

| Topic, and other names for it | Home | Then |
|---|---|---|
| **Games**, Bevy, game loop | [Games with Bevy](43_Games/README.md) | [What a game engine is](43_Games/what_a_game_engine_is/README.md) *stub* |
| **Generics**, `<T>`, type parameters, monomorphization | [What a generic is](22_Generics/what_a_generic_is/README.md) | [Where the bound goes](22_Generics/where_the_bound_goes/README.md) · [Generics](22_Generics/README.md) |
| **Global allocator**, heap allocation counting | [The global allocator](09_Advanced/the_global_allocator/README.md) | [Allocating without std](40_Without_std/allocating_without_std/README.md) *stub* |
| **Glossary**, vocabulary | [GLOSSARY.md](GLOSSARY.md) | |
| **`HashMap`**, hash map, dictionary, dict | [`HashMap`](26_Collections/the_hashmap/README.md) | [`Borrow`](12_Traits/borrow_trait/README.md) |
| **`HashSet`**, set | [`HashSet`](26_Collections/the_hashset/README.md) | |
| **Hexadecimal**, hex | [Why hexadecimal](19_Numbers/why_hexadecimal/README.md) | |
| **`if`**, `if let` | [`if` expressions](25_Control_Flow/if_expressions/README.md) | [`if let`](17_Option_and_Result/if_let/README.md) |
| **`impl` blocks**, methods vs functions, struct methods | [`impl` blocks](16_Structs/impl_blocks/README.md) | [Method resolution](12_Traits/method_resolution/README.md) |
| **`impl Trait`**, existential types | [Returning a trait](12_Traits/returning_a_trait/README.md) | [Returning an iterator](24_Iterators/returning_an_iterator/README.md) |
| **Indices instead of references**, graphs, arenas | [Indices instead of references](18_Ownership/indices_instead_of_references/README.md) *stub* | [`Rc`](18_Ownership/reference_counting/README.md) |
| **Integers**, `i32`, `u8`, integer types, overflow | [The integer types](19_Numbers/the_integer_types/README.md) *stub* | [Writing a number down](19_Numbers/writing_a_number_down/README.md) · [Comparing two number types](19_Numbers/comparing_two_number_types/README.md) · [Signed overflow](31_C_and_Cpp/signed_overflow/README.md) |
| **Interior mutability** | [Interior mutability](09_Advanced/interior_mutability/README.md) *stub* | |
| **Internationalization**, i18n, locale, translation | [Internationalization](14_Strings/internationalization/README.md) *stub* | [The string crates](14_Strings/string_crates/README.md) |
| **Iterators**, `iter()`, `into_iter()`, lazy | [Iterators](24_Iterators/README.md) | [Iterators are lazy](24_Iterators/iterators_are_lazy/README.md) · [Adapters by job](24_Iterators/adapters_by_job/README.md) |
| **JSON**, serde | [Deriving `Serialize` and `Deserialize`](06_Data/serde_derive/README.md) *stub* | [The round trip](06_Data/json_round_trip/README.md) *stub* |
| **Katas**, exercises | [KATAS.md](KATAS.md) | [Exercises](10_Resources/exercises/README.md) |
| **`let else`** | [`let else`](30_Pattern_Matching/let_else/README.md) *stub* | |
| **Lifetimes**, `'a`, `'static` | [Lifetime annotations](18_Ownership/lifetime_annotations/README.md) | [How to learn lifetimes](18_Ownership/how_to_learn_lifetimes/README.md) · [What a lifetime does at the call site](18_Ownership/lifetimes_at_the_call_site/README.md) |
| **Lock poisoning**, `Mutex` | [Lock poisoning](09_Advanced/mutex_poisoning/README.md) | [Forgotten unlock](31_C_and_Cpp/forgotten_unlock/README.md) |
| **`loop`** | [`loop`](25_Control_Flow/the_loop_keyword/README.md) | [`while` loops](25_Control_Flow/while_loops/README.md) |

## M – O

| Topic, and other names for it | Home | Then |
|---|---|---|
| **Macros**, `macro_rules!`, `!` | [Macros](25_Control_Flow/macros/README.md) *stub* | [Declarative macros](38_Declarative_Macros/README.md) · [std macros you have not met](38_Declarative_Macros/std_macros_you_have_not_met/README.md) *stub* |
| **`match`** | [`match` expressions](25_Control_Flow/match_expressions/README.md) | [Pattern matching](30_Pattern_Matching/README.md) · [Match ergonomics](30_Pattern_Matching/match_ergonomics/README.md) |
| **Memory layout**, `repr(C)`, padding, alignment | [Type layout](09_Advanced/type_layout/README.md) *stub* | [What is a record, in memory?](16_Structs/representing_a_record/README.md) |
| **Migrating C to Rust**, the C ABI, rewriting a module | [Migrating C to Rust](31_C_and_Cpp/migrating_c_to_rust/README.md) | [The C ABI](31_C_and_Cpp/migrating_c_to_rust/the_c_abi/README.md) *stub* · [Safe wrappers](31_C_and_Cpp/migrating_c_to_rust/safe_wrappers/README.md) *stub* |
| **Method resolution**, auto-deref | [Method resolution](12_Traits/method_resolution/README.md) | ["No method named …"](12_Traits/no_method_named/README.md) |
| **Modules**, `mod`, `use`, `pub` | [Modules and visibility](27_Modules/modules_and_visibility/README.md) | [Bringing names in with `use`](27_Modules/the_use_declaration/README.md) · [One module per file](27_Modules/one_module_per_file/README.md) |
| **Monad**, `and_then` | [What a monad is](17_Option_and_Result/what_a_monad_is/README.md) | |
| **MSRV**, `rust-version`, versioning | [MSRV](05_Tooling/msrv/README.md) *stub* | [Semver hazards](39_API_Design/semver_hazards/README.md) *stub* |
| **Newtype**, tuple struct wrapper | [A score is not a number: the newtype](16_Structs/newtype_score/README.md) | [A type alias is not a new type](16_Structs/type_aliases/README.md) *stub* · [Wrapper types](39_API_Design/wrapper_types/README.md) *stub* |
| **Null**, nullable pointer | [Nullable pointers](17_Option_and_Result/nullable_pointers/README.md) | [Null dereference](31_C_and_Cpp/null_dereference/README.md) |
| **Numbers** | [Numbers and bytes](19_Numbers/README.md) | [TYPES.md: primitive types](TYPES.md#primitive-types) |
| **Observability**, tracing, logs, metrics | [Observability](21_Observability/README.md) | |
| **Operators**, `+`, `Add`, operator overloading | [Operators are traits](12_Traits/operators_are_traits/README.md) | |
| **`Option`**, `Some`, `None` | [`Some` and `None`](17_Option_and_Result/some_and_none/README.md) | [OPTION.md](OPTION.md) |
| **`OsString`**, `CString`, kinds of string | [Six kinds of string](14_Strings/six_kinds_of_string/README.md) | [`Path` and `PathBuf`](04_Files/path_and_pathbuf/README.md) *stub* |
| **Ownership**, moves | [Ownership and moves](18_Ownership/ownership_and_moves/README.md) | [Ownership](18_Ownership/README.md) |

## P – R

| Topic, and other names for it | Home | Then |
|---|---|---|
| **Panic**, `panic!`, unwinding | [What a panic costs](17_Option_and_Result/what_a_panic_costs/README.md) | [The panic handler](40_Without_std/the_panic_handler/README.md) *stub* · [Panics in unsafe code](09_Advanced/panics_in_unsafe_code/README.md) *stub* |
| **Pointers**, address, raw pointer, `*const T` | [Pointers](36_Pointers/README.md) | [Address, pointer, reference](36_Pointers/address_pointer_reference/README.md) · [Raw pointers](36_Pointers/raw_pointers/README.md) · [Wide pointers](36_Pointers/wide_pointers/README.md) |
| **Parallelism**, rayon, `par_iter` | [Rayon](09_Advanced/rayon/README.md) *stub* | [Worker pools](09_Advanced/worker_pools/README.md) *stub* · [Concurrency or parallelism](09_Advanced/concurrency_or_parallelism/README.md) *stub* |
| **Parsing**, `parse`, `FromStr` | [Parsing out of a string](14_Strings/parsing_a_string/README.md) | |
| **Pattern matching**, patterns, `_`, `@` | [Pattern matching](30_Pattern_Matching/README.md) | [The wildcard `_`](30_Pattern_Matching/the_wildcard/README.md) · [Binding with `@`](30_Pattern_Matching/binding_at/README.md) *stub* |
| **`PhantomData`**, phantom types | [Phantom types](12_Traits/phantom_types/README.md) | [Marker traits](12_Traits/marker_traits/README.md) |
| **Prelude** | [A crate prelude](27_Modules/a_crate_prelude/README.md) *stub* | [A trait must be in scope](12_Traits/trait_in_scope/README.md) |
| **Printing**, `println!` | [Debug and Display](15_First_Programs/debug_vs_display/README.md) | [Printing borrows](18_Ownership/printing_borrows/README.md) · [The braces take a name](15_First_Programs/braces_take_a_name/README.md) |
| **Procedural macros**, derive macros, attribute macros, `syn`, `quote` | [Procedural macros](37_Procedural_Macros/README.md) | [A function, `macro_rules!`, or a procedural macro](37_Procedural_Macros/function_macro_rules_or_proc_macro/README.md) *stub* · [Declarative macros](38_Declarative_Macros/README.md) |
| **Publishing**, crates.io, `cargo publish` | [Publishing a crate](05_Tooling/publishing_a_crate/README.md) *stub* | [Private registries](05_Tooling/private_registries/README.md) *stub* |
| **`?` operator**, question mark | [The `?` operator](17_Option_and_Result/the_question_mark_operator/README.md) | [`main` can return a `Result`](02_Errors/main_returns_result/README.md) *stub* |
| **Randomness**, `rand` | [Randomness: `std` has none](15_First_Programs/randomness/README.md) | |
| **Ranges**, `0..5`, `..=` | [Ranges](24_Iterators/ranges/README.md) *stub* | [One arm, many values](17_Option_and_Result/one_arm_many_values/README.md) |
| **Raw strings**, escapes, `r"…"`, `b"…"` | [Raw strings, escapes and the literal prefixes](14_Strings/raw_strings_and_escapes/README.md) | |
| **`Rc`**, reference counting | [`Rc`: the clone that copies a pointer](18_Ownership/reference_counting/README.md) | [`Arc`](18_Ownership/sharing_across_threads/README.md) |
| **`Read` and `Write`**, I/O traits | [`Read` and `Write`](12_Traits/read_and_write/README.md) *stub* | [Reading a line from standard input](03_Command_Line/reading_stdin/README.md) |
| **Recursion**, stack overflow | [Recursion and the size of the stack](18_Ownership/recursion_and_the_stack/README.md) | [A generic recursive type](22_Generics/a_generic_recursive_type/README.md) |
| **Registries**, private registry, Shipyard, SSH key | [Private registries](05_Tooling/private_registries/README.md) *stub* | [Registry authentication](05_Tooling/registry_authentication/README.md) *stub* |
| **Release builds**, `--release`, profiles, LTO | [Release profiles](05_Tooling/release_profiles/README.md) *stub* | [What the optimizer does](20_Compilers/what_the_optimizer_does/README.md) |
| **`Result`**, `Ok`, `Err` | [`Ok` and `Err`](17_Option_and_Result/ok_and_err/README.md) | [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md) · [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md) |
| **Resources**, books, courses, videos | [Resources](10_Resources/README.md) | [Books](10_Resources/books/README.md) · [Start here](00_Start_Here/README.md) |
| **Returned by value**, return values, return slot, `sret`, `-> str`, unsized return type | [Returned by value](18_Ownership/returned_by_value/README.md) | [Drawing `sret`](18_Ownership/drawing_the_return_slot/README.md) · [The call stack](18_Ownership/the_call_stack/README.md) · [`str` is unsized](14_Strings/str_is_unsized/README.md) · [Every returned-by-value error](18_Ownership/returned_by_value_errors/README.md) · [Returning a trait](12_Traits/returning_a_trait/README.md) |
| **Rust itself**, what is Rust | [What Rust is](00_Start_Here/what_rust_is/README.md) *stub* | [Benefits of Rust](00_Start_Here/benefits_of_rust/README.md) |
| **rustup**, toolchains, nightly | [rustup](05_Tooling/rustup/README.md) | [Pinning the toolchain](05_Tooling/pinning_the_toolchain/README.md) · [Nightly by default](05_Tooling/nightly/README.md) |

## S

| Topic, and other names for it | Home | Then |
|---|---|---|
| **`Send` and `Sync`** | [`Send` and `Sync`](09_Advanced/send_and_sync/README.md) *stub* | [Unsafe traits](09_Advanced/unsafe_traits/README.md) *stub* |
| **Serialization**, serde, zero-copy deserialization | [Data](06_Data/README.md) | [Zero-copy deserialization](06_Data/zero_copy_deserialization/README.md) *stub* |
| **Shadowing**, `let x = x` | [SHADOWING.md](SHADOWING.md) | [When to shadow](18_Ownership/when_to_shadow/README.md) |
| **Signals**, Ctrl-C, SIGPIPE | [Catching a signal](09_Advanced/catching_a_signal/README.md) | [Broken pipe](02_Errors/broken_pipe/README.md) *stub* |
| **Slices**, `&[T]`, fat pointer, wide pointer | [Arrays and slices](26_Collections/arrays_and_slices/README.md) | [Wide pointers](36_Pointers/wide_pointers/README.md) · [Slices of slices](26_Collections/slice_of_slices/README.md) · [`slice` methods](26_Collections/slice_methods/README.md) |
| **Smart pointers**, `Deref`, `Box`, `Rc`, `Arc` | [Smart pointers](41_Smart_Pointers/README.md) | [What a smart pointer is](18_Ownership/what_a_smart_pointer_is/README.md) · [What a smart pointer costs](41_Smart_Pointers/what_a_smart_pointer_costs/README.md) · [TYPES.md: pointers](TYPES.md#pointers) |
| **`sret`**, hidden return pointer, out-parameter, return slot, `rdi`/`rax` on return | [Drawing `sret`](18_Ownership/drawing_the_return_slot/README.md) | [Returned by value](18_Ownership/returned_by_value/README.md) · [LLVM and its IR](20_Compilers/llvm_and_its_ir/README.md) |
| **Stack and heap** | [Stack and heap](18_Ownership/stack_and_heap/README.md) | [The call stack](18_Ownership/the_call_stack/README.md) · [A stack slot is reused](18_Ownership/a_stack_slot_is_reused/README.md) |
| **`static`**, `&'static str` | [`const` and `static`](27_Modules/const_and_static/README.md) | [`&'static str`](14_Strings/static_str/README.md) |
| **stdin**, reading input | [Reading a line from standard input](03_Command_Line/reading_stdin/README.md) | [Feeding stdin](11_Unix/feeding_stdin/README.md) |
| **Strings**, `String`, `&str`, UTF-8 | [`String` vs `&str`](14_Strings/string_vs_str/README.md) | [Drawing the owner and the view](14_Strings/drawing_the_owner_and_the_view/README.md) · [STRINGS.md](STRINGS.md) · [Strings](14_Strings/README.md) |
| **Struct**, fields, unit struct, tuple struct | [What a struct is](16_Structs/what_a_struct_is/README.md) | [STRUCTS.md](STRUCTS.md) · [When a struct refuses](16_Structs/when_a_struct_refuses/README.md) |
| **Struct update**, reusing fields from another struct, `..base` | [Struct update syntax](16_Structs/struct_update/README.md) | |
| **Supertraits** | [Supertraits](12_Traits/supertraits/README.md) | |

## T

| Topic, and other names for it | Home | Then |
|---|---|---|
| **Templates**, project starters, lint profiles | [Templates](34_Templates/README.md) | |
| **Temporary directory**, `tempfile`, `TempDir`, test fixture on disk | [Temporary directories in tests](04_Files/temp_dirs_in_tests/README.md) | [How `cargo test` runs your tests](28_Testing/how_cargo_test_runs/README.md) · [`Read` and `Write`](12_Traits/read_and_write/README.md) |
| **Test double**, mock, fake, stub, spy, `mockall`, dependency injection | [A test double is a second `impl`](28_Testing/a_test_double_by_hand/README.md) | [Static and dynamic dispatch](12_Traits/static_vs_dynamic_dispatch/README.md) · [Mocking a server](07_Clients/mocking_a_server/README.md) *stub* |
| **Test harness**, `harness = false`, `libtest-mimic`, custom test runner, setup and teardown | [A harness of your own](28_Testing/a_harness_of_your_own/README.md) | [How `cargo test` runs your tests](28_Testing/how_cargo_test_runs/README.md) · [cargo-nextest](05_Tooling/nextest/README.md) |
| **Testing**, `#[test]`, `cargo test` | [Testing](28_Testing/README.md) | [A first test, step by step](28_Testing/a_first_test_step_by_step/README.md) · [How `cargo test` runs your tests](28_Testing/how_cargo_test_runs/README.md) · [Other kinds of test](28_Testing/other_kinds_of_test/README.md) · [A test double is a second `impl`](28_Testing/a_test_double_by_hand/README.md) · [A harness of your own](28_Testing/a_harness_of_your_own/README.md) · [Errors](28_Testing/testing_errors/README.md) · [Lints](28_Testing/testing_lints/README.md) · [Courses and links](28_Testing/resources/README.md) · [Testing concurrent code](09_Advanced/testing_concurrent_code/README.md) *stub* |
| **Threads**, `thread::spawn`, `join` | [Spawning a thread](09_Advanced/spawning_a_thread/README.md) | [Channels](09_Advanced/channels/README.md) · [Worker pools](09_Advanced/worker_pools/README.md) *stub* · [Actors](09_Advanced/actors/README.md) *stub* |
| **Time**, `Instant`, `Duration`, `SystemTime` | [Two clocks](33_Time_and_Benchmarking/two_clocks/README.md) | [Time and benchmarking](33_Time_and_Benchmarking/README.md) |
| **`ToOwned`**, `to_owned()` | [How to learn `ToOwned`](12_Traits/how_to_learn_to_owned/README.md) | [`ToOwned`](12_Traits/to_owned/README.md) · [Three `ToOwned` katas, run](12_Traits/how_to_learn_to_owned/to_owned_katas_checked/README.md) |
| **Traits**, interfaces | [What a trait is](12_Traits/what_a_trait_is/README.md) | [Traits](12_Traits/README.md) · [A trait must be in scope](12_Traits/trait_in_scope/README.md) |
| **Transmute** | [Transmute](09_Advanced/transmute/README.md) *stub* | [Casting with `as`](29_Conversion/casting_with_as/README.md) |
| **Tuples**, `(a, b)` | [Tuples](26_Collections/tuples/README.md) | [TYPES.md: compound types](TYPES.md#compound-types-the-two-that-are-primitive) |
| **Type aliases**, `type X = Y` | [A type alias is not a new type](16_Structs/type_aliases/README.md) *stub* | [The `Result` alias](17_Option_and_Result/result_aliases/README.md) |
| **Type inference**, annotations, turbofish | [Type inference](15_First_Programs/type_inference/README.md) | [What a type annotation does](15_First_Programs/what_an_annotation_does/README.md) · [When the compiler cannot infer](22_Generics/when_the_compiler_cannot_infer/README.md) |

## U – Z

| Topic, and other names for it | Home | Then |
|---|---|---|
| **Undefined behaviour**, validity | [Validity invariants](09_Advanced/validity_invariants/README.md) *stub* | [What an invariant is](09_Advanced/what_an_invariant_is/README.md) · [Miri](09_Advanced/miri/README.md) *stub* |
| **Union** | [What a union is](09_Advanced/what_a_union_is/README.md) | |
| **Unit type**, `()` | [The unit type `()`](15_First_Programs/the_unit_type/README.md) | |
| **`unsafe`** | [What `unsafe` turns off](09_Advanced/what_unsafe_turns_off/README.md) | [Raw pointers](36_Pointers/raw_pointers/README.md) · [Unsafe traits](09_Advanced/unsafe_traits/README.md) *stub* · [The drop check](09_Advanced/the_drop_check/README.md) *stub* |
| **`unwrap`**, `unwrap_or`, `unwrap_or_else` | [What `unwrap` does](17_Option_and_Result/what_unwrap_does/README.md) | [`unwrap` is a TODO](02_Errors/unwrap_is_a_todo/README.md) · [`unwrap_or`](17_Option_and_Result/unwrap_or/README.md) · [`unwrap_or_else`](17_Option_and_Result/unwrap_or_else/README.md) |
| **Variables**, `let`, `mut` | [Variables](15_First_Programs/variables/README.md) | [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md) |
| **`Vec`**, vector, dynamic array, list | [`Vec`](26_Collections/the_vec/README.md) | [`Vec` methods](26_Collections/vec_methods/README.md) · [What a `Vec` guarantees](26_Collections/vec_guarantees/README.md) · [Grids and nested `Vec`s](26_Collections/vec_of_vecs/README.md) |
| **`VecDeque`**, queue | [`VecDeque`](26_Collections/the_vecdeque/README.md) | |
| **WebAssembly**, WASM, wasm32 | [WebAssembly](42_WebAssembly/README.md) | [Rust in the browser](42_WebAssembly/rust_in_the_browser/README.md) *stub* |
| **`while`**, `while let` | [`while` loops](25_Control_Flow/while_loops/README.md) | [`while let`](17_Option_and_Result/while_let/README.md) |
| **Workspaces**, monorepo | [Workspaces](05_Tooling/workspaces/README.md) *stub* | [A tree of practice projects](05_Tooling/practice_workspace/README.md) |
| **`zip`**, `enumerate` | [`zip` and `enumerate`](24_Iterators/zip_and_enumerate/README.md) | |

## When a topic is missing

Search the site first (`/`), then [TODO.md](TODO.md) for the strings vocabulary that is tracked but not written. If it is in neither, the topic has no page yet — and a stub with its boundaries written down is the cheapest way to give it one ([CONTRIBUTING.md: stubs](CONTRIBUTING.md#stubs)).

## Po polsku

To jest indeks tematów — odpowiednik indeksu na końcu książki. Nazwa katalogu rzadko jest słowem, które wpisujesz: wektor mieszka w `26_Collections/the_vec`, a `static` w `27_Modules/const_and_static`. Każdy wiersz podaje temat pod kilkoma nazwami, stronę, która jest jego domem, i strony do przeczytania potem; polskie odpowiedniki terminów zbiera [POLSKI.md](POLSKI.md).

**Szukaj po polsku:** indeks tematów · spis pojęć Rust · `rust topics index` · `rust learning roadmap`
