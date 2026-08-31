# Rust by Example, page by page

**Level:** reference · the coverage map

**One line:** [Rust by Example ↗](https://doc.rust-lang.org/rust-by-example/) is the syntax dictionary most Rust learners keep open in a tab; this page maps its 24 chapters onto the lessons here, says which ones this library goes deeper on, and names the ones it does not cover at all.

---

## How to read this

RBE and this library answer different questions, so the map is not a race. RBE shows **what the syntax is**, in a runnable block, with two sentences around it. A page here picks **one idea**, shows the mistake that makes it worth knowing, and backs every printed number with a program CI compiles and runs. Where a row says *covered*, it usually means the lesson here is several times longer than the RBE page and about the trap rather than the form.

| | meaning |
|---|---|
| **✓** | a lesson here covers this, and goes further |
| **◐** | partly covered — the note says which half is missing |
| **·** | not covered here; read RBE, or the link in the note |

A few rows name a page in **bold with no link**, marked *(in flight)*. Those are lessons another session has written and not yet committed — the link is left off deliberately, because a link to an uncommitted page fails the docs build for everybody. They become links in the commit that lands them.

Three whole chapters are deliberately **·** and always will be: *Cargo*, *Crates* and *Meta* are toolchain topics, and this library keeps those in [Tooling](05_Tooling/README.md) organised by the problem you have rather than by the feature.

## 1. Hello World

| RBE page | Here | Note |
|---|---|---|
| [Hello World ↗](https://doc.rust-lang.org/rust-by-example/hello.html) | ✓ | [Running a scratch program](15_First_Programs/rustc_without_cargo/README.md) — `rustc` alone, `cargo new`, and the edition flag Cargo passes for you |
| [Comments ↗](https://doc.rust-lang.org/rust-by-example/hello/comment.html) | ✓ | [Comments that compile](15_First_Programs/comments_that_compile/README.md) — `//`, `///`, `//!`, and the two error codes a misplaced doc comment produces |
| [Formatted print ↗](https://doc.rust-lang.org/rust-by-example/hello/print.html) | ✓ | [The format mini-language](14_Strings/the_format_language/README.md) — width, precision, alignment, `{:?}`, `{:#?}` |
| [Debug ↗](https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html) · [Display ↗](https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html) | ✓ | [Debug and Display](15_First_Programs/debug_vs_display/README.md) — which one is for you and which is for your user, and why one is derived and the other never is |
| [Formatting ↗](https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html) | ✓ | [The format mini-language](14_Strings/the_format_language/README.md) |

## 2. Primitives

| RBE page | Here | Note |
|---|---|---|
| [Primitives ↗](https://doc.rust-lang.org/rust-by-example/primitives.html) | ✓ | **Values** *(in flight)* — the scalar types, and the width that runs out |
| [Literals and operators ↗](https://doc.rust-lang.org/rust-by-example/primitives/literals.html) | ✓ | [Why hexadecimal](19_Numbers/why_hexadecimal/README.md), [Meet the byte](19_Numbers/meet_the_byte/README.md) |
| [Tuples ↗](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html) | ✓ | [Tuples](26_Collections/tuples/README.md) — the anonymous struct, destructuring, and the arity where it stops being readable |
| [Arrays and Slices ↗](https://doc.rust-lang.org/rust-by-example/primitives/array.html) | ✓ | [Arrays and slices](26_Collections/arrays_and_slices/README.md) — `[T; N]` is a *type* per length; `&[T]` is the one you write in signatures |

## 3. Custom Types

| RBE page | Here | Note |
|---|---|---|
| [Custom Types ↗](https://doc.rust-lang.org/rust-by-example/custom_types.html) | ✓ | [Structs](16_Structs/README.md) and [Enums](13_Enums/README.md) are a section each |
| [Structures ↗](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html) | ✓ | [What a struct is](16_Structs/what_a_struct_is/README.md), then seven more: [`impl` blocks](16_Structs/impl_blocks/README.md), [struct update](16_Structs/struct_update/README.md), [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md), [the newtype](16_Structs/newtype_score/README.md) |
| [Enums ↗](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html) | ✓ | [What an enum is](13_Enums/what_an_enum_is/README.md), [variants that carry data](13_Enums/variants_that_carry_data/README.md), [an enum as a state machine](13_Enums/an_enum_as_a_state_machine/README.md) |
| [use ↗](https://doc.rust-lang.org/rust-by-example/custom_types/enum/enum_use.html) | ✓ | [Bringing names in with `use`](27_Modules/the_use_declaration/README.md) |
| [C-like ↗](https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html) | ✓ | [What an enum is](13_Enums/what_an_enum_is/README.md) — discriminants, and the `as` cast that reads one |
| [Testcase: linked-list ↗](https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html) | ✓ | [Nullable pointers](17_Option_and_Result/nullable_pointers/README.md) and [a generic recursive type](22_Generics/a_generic_recursive_type/README.md) — the same list, built twice |
| [constants ↗](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html) | ✓ | [`const` and `static`](27_Modules/const_and_static/README.md) — inlined at every use, versus one address for the whole program |

## 4. Variable Bindings

| RBE page | Here | Note |
|---|---|---|
| [Variable Bindings ↗](https://doc.rust-lang.org/rust-by-example/variable_bindings.html) | ✓ | **Variables** *(in flight)* |
| [Mutability ↗](https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html) | ✓ | **Variables** *(in flight)* — `mut` is a property of the binding, not the type |
| [Scope and Shadowing ↗](https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html) | ✓ | Five pages, collected in [SHADOWING.md](SHADOWING.md) — including the three ways a shadow silently produces a wrong answer |
| [Declare first ↗](https://doc.rust-lang.org/rust-by-example/variable_bindings/declare.html) | ✓ | [Initial values](17_Option_and_Result/initial_values/README.md) — and why you almost never want to |
| [Freezing ↗](https://doc.rust-lang.org/rust-by-example/variable_bindings/freeze.html) | ◐ | The shadow-freezes-the-outer-binding trick appears in [when to shadow](18_Ownership/when_to_shadow/README.md); RBE's framing of it is not repeated |

## 5. Types

| RBE page | Here | Note |
|---|---|---|
| [Types ↗](https://doc.rust-lang.org/rust-by-example/types.html) | ✓ | [What a type annotation does](15_First_Programs/what_an_annotation_does/README.md) |
| [Casting ↗](https://doc.rust-lang.org/rust-by-example/types/cast.html) | ✓ | [Casting with `as`](29_Conversion/casting_with_as/README.md) — the four silent losses, and the two conversions that are not casts |
| [Literals ↗](https://doc.rust-lang.org/rust-by-example/types/literals.html) | ✓ | **Values** *(in flight)* |
| [Inference ↗](https://doc.rust-lang.org/rust-by-example/types/inference.html) | ✓ | **Type inference** *(in flight)*, and [when the compiler cannot infer](22_Generics/when_the_compiler_cannot_infer/README.md) |
| [Aliasing ↗](https://doc.rust-lang.org/rust-by-example/types/alias.html) | ✓ | [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md) — the alias you meet before you write one |

## 6. Conversion

| RBE page | Here | Note |
|---|---|---|
| [From and Into ↗](https://doc.rust-lang.org/rust-by-example/conversion/from_into.html) | ✓ | [`From` and `Into`](29_Conversion/from_and_into/README.md) — write one, get the other free |
| [TryFrom and TryInto ↗](https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html) | ✓ | [`TryFrom` and `TryInto`](29_Conversion/tryfrom_and_tryinto/README.md) — the conversion that is allowed to say no |
| [To and from Strings ↗](https://doc.rust-lang.org/rust-by-example/conversion/string.html) | ✓ | [Parsing out of a string](14_Strings/parsing_a_string/README.md) and [`Display`](15_First_Programs/debug_vs_display/README.md) — implement `Display`, never `ToString` |

## 7. Expressions

| RBE page | Here | Note |
|---|---|---|
| [Expressions ↗](https://doc.rust-lang.org/rust-by-example/expression.html) | ✓ | [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md) — the semicolon that turns a value into `()` |

## 8. Flow of Control

| RBE page | Here | Note |
|---|---|---|
| [if/else ↗](https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html) · [loop ↗](https://doc.rust-lang.org/rust-by-example/flow_control/loop.html) · [while ↗](https://doc.rust-lang.org/rust-by-example/flow_control/while.html) · [for ↗](https://doc.rust-lang.org/rust-by-example/flow_control/for.html) · [match ↗](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) | ◐ | The **Control flow** *(in flight)* section is stubs — outlines with their traps written down, no runnable example yet |
| [Nesting and labels ↗](https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html) · [Returning from loops ↗](https://doc.rust-lang.org/rust-by-example/flow_control/loop/return.html) | ◐ | Stubs: **`break` and `continue`**, **`loop`** |
| [Destructuring ↗](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html) — tuples, slices, enums, `ref`, structs | ◐ | [One arm, many values](17_Option_and_Result/one_arm_many_values/README.md) does the enum-and-tuple half; slices and `ref` are not covered |
| [Guards ↗](https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html) | ◐ | [Zero wins is not zero games](17_Option_and_Result/wrong_guard/README.md) is about a guard that lets the wrong case through |
| [Binding ↗](https://doc.rust-lang.org/rust-by-example/flow_control/match/binding.html) | · | The `@` binding is not covered |
| [if let ↗](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html) | ✓ | [`if let`](17_Option_and_Result/if_let/README.md) — and the arm you deleted |
| [let-else ↗](https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html) | · | Not covered |
| [while let ↗](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html) | ✓ | [`while let`](17_Option_and_Result/while_let/README.md) |

## 9. Functions

| RBE page | Here | Note |
|---|---|---|
| [Functions ↗](https://doc.rust-lang.org/rust-by-example/fn.html) | ◐ | **Functions** is a stub; the returned-value half is [a block is an expression](15_First_Programs/a_block_is_an_expression/README.md) |
| [Associated functions & Methods ↗](https://doc.rust-lang.org/rust-by-example/fn/methods.html) | ✓ | [`impl` blocks](16_Structs/impl_blocks/README.md) and [a type is not a constructor](16_Structs/a_type_is_not_a_constructor/README.md) |
| [Closures ↗](https://doc.rust-lang.org/rust-by-example/fn/closures.html) · [Capturing ↗](https://doc.rust-lang.org/rust-by-example/fn/closures/capture.html) | ✓ | [What a closure is](23_Closures/what_a_closure_is/README.md), [the `move` keyword](23_Closures/the_move_keyword/README.md) |
| [As input parameters ↗](https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html) · [Type anonymity ↗](https://doc.rust-lang.org/rust-by-example/fn/closures/anonymity.html) | ✓ | [The three closure traits](23_Closures/three_closure_traits/README.md) — `Fn`, `FnMut`, `FnOnce`, and which one you are asking for |
| [Input functions ↗](https://doc.rust-lang.org/rust-by-example/fn/closures/input_functions.html) · [Higher Order Functions ↗](https://doc.rust-lang.org/rust-by-example/fn/hof.html) | ✓ | **Function pointers** *(in flight)* |
| [As output parameters ↗](https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html) | ✓ | [Returning a trait](12_Traits/returning_a_trait/README.md) — `impl Fn` versus `Box<dyn Fn>` |
| [Examples in std ↗](https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples.html) — `Iterator::any`, searching | ✓ | [Iterators are lazy](24_Iterators/iterators_are_lazy/README.md), [`iter`, `iter_mut`, `into_iter`](24_Iterators/iter_iter_mut_into_iter/README.md) |
| [Diverging functions ↗](https://doc.rust-lang.org/rust-by-example/fn/diverging.html) | ◐ | `!` turns up in [what a panic costs](17_Option_and_Result/what_a_panic_costs/README.md); the never type is not its own page |

## 10. Modules

| RBE page | Here | Note |
|---|---|---|
| [Modules ↗](https://doc.rust-lang.org/rust-by-example/mod.html) · [Visibility ↗](https://doc.rust-lang.org/rust-by-example/mod/visibility.html) | ✓ | [Modules and visibility](27_Modules/modules_and_visibility/README.md) — private by default, and what "private" is measured against |
| [Struct visibility ↗](https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html) | ✓ | [Modules and visibility](27_Modules/modules_and_visibility/README.md), and the door it leaves open in [the newtype](16_Structs/newtype_score/README.md) |
| [The use declaration ↗](https://doc.rust-lang.org/rust-by-example/mod/use.html) | ✓ | [Bringing names in with `use`](27_Modules/the_use_declaration/README.md) — and [a trait must be in scope](12_Traits/trait_in_scope/README.md), which is the version that bites |
| [super and self ↗](https://doc.rust-lang.org/rust-by-example/mod/super.html) | ✓ | [Modules and visibility](27_Modules/modules_and_visibility/README.md) |
| [File hierarchy ↗](https://doc.rust-lang.org/rust-by-example/mod/split.html) | ✓ | [One module per file](27_Modules/one_module_per_file/README.md) — `mod.rs` versus `name.rs`, and the declaration that is not an include |

## 11. Crates

| RBE page | Here | Note |
|---|---|---|
| [Crates ↗](https://doc.rust-lang.org/rust-by-example/crates.html) · [Creating a Library ↗](https://doc.rust-lang.org/rust-by-example/crates/lib.html) · [Using a Library ↗](https://doc.rust-lang.org/rust-by-example/crates/using_lib.html) | ◐ | [One module per file](27_Modules/one_module_per_file/README.md) covers the crate root and the two crate kinds; the `rustc --crate-type` and `--extern` mechanics are RBE's, and [Adding a dependency](05_Tooling/cargo_dependencies/README.md) is what you actually do |

## 12. Cargo

| RBE page | Here | Note |
|---|---|---|
| [Dependencies ↗](https://doc.rust-lang.org/rust-by-example/cargo/deps.html) | ✓ | [Adding a dependency](05_Tooling/cargo_dependencies/README.md) — `search`, `info`, `add`, and what the caret permits |
| [Conventions ↗](https://doc.rust-lang.org/rust-by-example/cargo/conventions.html) | ✓ | [Running a scratch program](15_First_Programs/rustc_without_cargo/README.md) and [scaffolding a practice tree](05_Tooling/scaffolding/README.md) |
| [Testing ↗](https://doc.rust-lang.org/rust-by-example/cargo/test.html) | ✓ | [Where a test goes](28_Testing/where_a_test_goes/README.md) |
| [Build Scripts ↗](https://doc.rust-lang.org/rust-by-example/cargo/build_scripts.html) | · | Not covered |

## 13. Attributes

| RBE page | Here | Note |
|---|---|---|
| [Attributes ↗](https://doc.rust-lang.org/rust-by-example/attribute.html) | ✓ | [What an attribute is](27_Modules/what_an_attribute_is/README.md) |
| [dead_code ↗](https://doc.rust-lang.org/rust-by-example/attribute/unused.html) | ✓ | [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md) — including which of the four fixes is right |
| [cfg ↗](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html) | ✓ | [What an attribute is](27_Modules/what_an_attribute_is/README.md) — and `#[cfg(test)]`, which is the one you meet first, in [where a test goes](28_Testing/where_a_test_goes/README.md) |
| [Crates (`crate_type`) ↗](https://doc.rust-lang.org/rust-by-example/attribute/crate.html) | · | Superseded by Cargo; not covered |

## 14. Generics

| RBE page | Here | Note |
|---|---|---|
| [Generics ↗](https://doc.rust-lang.org/rust-by-example/generics.html) · [Functions ↗](https://doc.rust-lang.org/rust-by-example/generics/gen_fn.html) · [Implementation ↗](https://doc.rust-lang.org/rust-by-example/generics/impl.html) | ✓ | [What a generic is](22_Generics/what_a_generic_is/README.md) |
| [Traits ↗](https://doc.rust-lang.org/rust-by-example/generics/gen_trait.html) · [Bounds ↗](https://doc.rust-lang.org/rust-by-example/generics/bounds.html) · [Multiple bounds ↗](https://doc.rust-lang.org/rust-by-example/generics/multi_bounds.html) · [Where clauses ↗](https://doc.rust-lang.org/rust-by-example/generics/where.html) | ✓ | [Where the bound goes](22_Generics/where_the_bound_goes/README.md) |
| [New Type Idiom ↗](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) | ✓ | [A score is not a number](16_Structs/newtype_score/README.md) |
| [Associated items ↗](https://doc.rust-lang.org/rust-by-example/generics/assoc_items.html) · [Associated types ↗](https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html) | ✓ | [Implementing `Iterator`](24_Iterators/implementing_iterator/README.md) — `type Item` is the associated type you meet first |
| [Phantom type parameters ↗](https://doc.rust-lang.org/rust-by-example/generics/phantom.html) | ✓ | [Phantom types](12_Traits/phantom_types/README.md) |
| [Testcase: empty bounds ↗](https://doc.rust-lang.org/rust-by-example/generics/bounds/testcase_empty.html) | ✓ | [Marker traits](12_Traits/marker_traits/README.md) |

## 15. Scoping rules

| RBE page | Here | Note |
|---|---|---|
| [RAII ↗](https://doc.rust-lang.org/rust-by-example/scope/raii.html) · [Destructor ↗](https://doc.rust-lang.org/rust-by-example/scope/raii.html#destructor) | ✓ | [`Drop`, and what RAII buys](12_Traits/drop_and_raii/README.md) |
| [Ownership and moves ↗](https://doc.rust-lang.org/rust-by-example/scope/move.html) · [Mutability ↗](https://doc.rust-lang.org/rust-by-example/scope/move/mut.html) | ✓ | [Ownership and moves](18_Ownership/ownership_and_moves/README.md) |
| [Partial moves ↗](https://doc.rust-lang.org/rust-by-example/scope/move/partial_move.html) | ✓ | [Struct update syntax, and the partial move](16_Structs/struct_update/README.md) |
| [Borrowing ↗](https://doc.rust-lang.org/rust-by-example/scope/borrow.html) · [Mutability ↗](https://doc.rust-lang.org/rust-by-example/scope/borrow/mut.html) · [Aliasing ↗](https://doc.rust-lang.org/rust-by-example/scope/borrow/alias.html) | ✓ | [Borrowing](18_Ownership/borrowing/README.md) — and where a borrow ends, which RBE does not say |
| [The ref pattern ↗](https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html) | · | Largely obsolete since match ergonomics; not covered |
| [Lifetimes ↗](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html) · [Explicit annotation ↗](https://doc.rust-lang.org/rust-by-example/scope/lifetime/explicit.html) · [Functions ↗](https://doc.rust-lang.org/rust-by-example/scope/lifetime/fn.html) · [Structs ↗](https://doc.rust-lang.org/rust-by-example/scope/lifetime/struct.html) | ✓ | [Lifetime annotations](18_Ownership/lifetime_annotations/README.md), and [how to learn lifetimes](18_Ownership/how_to_learn_lifetimes/README.md) first |
| [Static ↗](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html) | ✓ | [`&'static str`](14_Strings/static_str/README.md) — the two meanings of `'static`, which is the trap |
| [Elision ↗](https://doc.rust-lang.org/rust-by-example/scope/lifetime/elision.html) | ✓ | [Lifetime annotations](18_Ownership/lifetime_annotations/README.md) |

## 16. Traits

| RBE page | Here | Note |
|---|---|---|
| [Traits ↗](https://doc.rust-lang.org/rust-by-example/trait.html) | ✓ | [What a trait is](12_Traits/what_a_trait_is/README.md) |
| [Derive ↗](https://doc.rust-lang.org/rust-by-example/trait/derive.html) | ✓ | [What an attribute is](27_Modules/what_an_attribute_is/README.md) — `derive` is the attribute you use most |
| [Returning Traits with dyn ↗](https://doc.rust-lang.org/rust-by-example/trait/dyn.html) | ✓ | [Returning a trait](12_Traits/returning_a_trait/README.md), [static vs dynamic dispatch](12_Traits/static_vs_dynamic_dispatch/README.md) |
| [Operator Overloading ↗](https://doc.rust-lang.org/rust-by-example/trait/ops.html) | ✓ | [Operators are traits](12_Traits/operators_are_traits/README.md) — `+` is `Add::add`, and the newtype that gets one |
| [Drop ↗](https://doc.rust-lang.org/rust-by-example/trait/drop.html) | ✓ | [`Drop`, and what RAII buys](12_Traits/drop_and_raii/README.md) |
| [Iterators ↗](https://doc.rust-lang.org/rust-by-example/trait/iter.html) | ✓ | [Implementing `Iterator`](24_Iterators/implementing_iterator/README.md) |
| [impl Trait ↗](https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html) | ✓ | [Returning a trait](12_Traits/returning_a_trait/README.md) |
| [Clone and Copy ↗](https://doc.rust-lang.org/rust-by-example/trait/clone.html) | ✓ | [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md), [`ToOwned`](12_Traits/to_owned/README.md), [`clone_into`](12_Traits/clone_into/README.md) |
| [Supertraits ↗](https://doc.rust-lang.org/rust-by-example/trait/supertraits.html) | ✓ | [Supertraits](12_Traits/supertraits/README.md) |
| [Disambiguating overlapping traits ↗](https://doc.rust-lang.org/rust-by-example/trait/disambiguating.html) | ◐ | The fully-qualified call form appears in [a trait must be in scope](12_Traits/trait_in_scope/README.md) |

## 17. Macros

| RBE page | Here | Note |
|---|---|---|
| [macro_rules! ↗](https://doc.rust-lang.org/rust-by-example/macros.html) · [Syntax ↗](https://doc.rust-lang.org/rust-by-example/macros/syntax.html) · [Designators ↗](https://doc.rust-lang.org/rust-by-example/macros/designators.html) · [Overload ↗](https://doc.rust-lang.org/rust-by-example/macros/overload.html) · [Repeat ↗](https://doc.rust-lang.org/rust-by-example/macros/repeat.html) | ◐ | **Macros** is a stub — what the `!` means, and the three things a macro can do that a function cannot |
| [DRY ↗](https://doc.rust-lang.org/rust-by-example/macros/dry.html) · [DSLs ↗](https://doc.rust-lang.org/rust-by-example/macros/dsl.html) · [Variadics ↗](https://doc.rust-lang.org/rust-by-example/macros/variadics.html) | · | Not covered |

## 18. Error handling

This is the chapter this library covers most heavily: two sections, thirty-odd pages, and a map of its own in [OPTION.md](OPTION.md).

| RBE page | Here | Note |
|---|---|---|
| [panic ↗](https://doc.rust-lang.org/rust-by-example/error/panic.html) · [abort and unwind ↗](https://doc.rust-lang.org/rust-by-example/error/abort_unwind.html) | ✓ | [What a panic costs](17_Option_and_Result/what_a_panic_costs/README.md) — the damage it does halfway through a job |
| [Option & unwrap ↗](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html) | ✓ | [`Some` and `None`](17_Option_and_Result/some_and_none/README.md), [`unwrap` is a TODO](02_Errors/unwrap_is_a_todo/README.md), [`expect`](17_Option_and_Result/expect/README.md) |
| [Unpacking options with ? ↗](https://doc.rust-lang.org/rust-by-example/error/option_unwrap/question_mark.html) · [Introducing ? ↗](https://doc.rust-lang.org/rust-by-example/error/result/enter_question_mark.html) | ✓ | [`main` can return a `Result`](02_Errors/main_returns_result/README.md) |
| [Combinators: map ↗](https://doc.rust-lang.org/rust-by-example/error/option_unwrap/map.html) · [and_then ↗](https://doc.rust-lang.org/rust-by-example/error/option_unwrap/and_then.html) | ✓ | [`map_or`](17_Option_and_Result/map_or/README.md), [`Option` is a one-item collection](17_Option_and_Result/option_as_collection/README.md), [what a monad is](17_Option_and_Result/what_a_monad_is/README.md) |
| [Unpacking options and defaults ↗](https://doc.rust-lang.org/rust-by-example/error/option_unwrap/defaults.html) | ✓ | Four pages: [`unwrap_or`](17_Option_and_Result/unwrap_or/README.md), [`unwrap_or_else`](17_Option_and_Result/unwrap_or_else/README.md), [`unwrap_or_default`](17_Option_and_Result/unwrap_or_default/README.md), [`map_or`](17_Option_and_Result/map_or/README.md) |
| [Result ↗](https://doc.rust-lang.org/rust-by-example/error/result.html) · [Using Result in main ↗](https://doc.rust-lang.org/rust-by-example/error/result.html#using-result-in-main) | ✓ | [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md), [`main` can return a `Result`](02_Errors/main_returns_result/README.md) |
| [aliases for Result ↗](https://doc.rust-lang.org/rust-by-example/error/result/result_alias.html) | ✓ | [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md) |
| [Multiple error types ↗](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types.html) · [Defining an error type ↗](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html) · [Boxing errors ↗](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html) · [Wrapping errors ↗](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html) | ◐ | [Not every error is an `io::Error`](02_Errors/not_every_error_is_io_error/README.md) and [`thiserror` vs `anyhow`](02_Errors/thiserror_vs_anyhow/README.md) are stubs today |
| [Iterating over Results ↗](https://doc.rust-lang.org/rust-by-example/error/iter_result.html) | ◐ | [Keep going, or stop](02_Errors/keep_going_or_stop/README.md) — stub, same four strategies |

## 19. Std library types

| RBE page | Here | Note |
|---|---|---|
| [Box, stack and heap ↗](https://doc.rust-lang.org/rust-by-example/std/box.html) | ✓ | [Stack and heap](18_Ownership/stack_and_heap/README.md), then [`Box`](26_Collections/the_box/README.md) — one value, moved off the stack |
| [Vectors ↗](https://doc.rust-lang.org/rust-by-example/std/vec.html) | ✓ | [`Vec`](26_Collections/the_vec/README.md) — three numbers, and the reallocation you can see |
| [Strings ↗](https://doc.rust-lang.org/rust-by-example/std/str.html) | ✓ | Twenty-one pages in [Strings](14_Strings/README.md), mapped in [STRINGS.md](STRINGS.md) |
| [Option ↗](https://doc.rust-lang.org/rust-by-example/std/option.html) · [Result ↗](https://doc.rust-lang.org/rust-by-example/std/result.html) · [? ↗](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html) · [panic! ↗](https://doc.rust-lang.org/rust-by-example/std/panic.html) | ✓ | [`Option` and `Result`](17_Option_and_Result/README.md), the largest section here |
| [HashMap ↗](https://doc.rust-lang.org/rust-by-example/std/hash.html) · [Alternate/custom key types ↗](https://doc.rust-lang.org/rust-by-example/std/hash/alt_key_types.html) | ✓ | [`HashMap`](26_Collections/the_hashmap/README.md) — `entry`, and what a key has to promise |
| [HashSet ↗](https://doc.rust-lang.org/rust-by-example/std/hash/hashset.html) | ✓ | [`HashSet`](26_Collections/the_hashset/README.md) |
| [Rc ↗](https://doc.rust-lang.org/rust-by-example/std/rc.html) · [Arc ↗](https://doc.rust-lang.org/rust-by-example/std/arc.html) | ✓ | [`Rc`](18_Ownership/reference_counting/README.md), [`Arc`](18_Ownership/sharing_across_threads/README.md) |

## 20. Std misc

| RBE page | Here | Note |
|---|---|---|
| [Threads ↗](https://doc.rust-lang.org/rust-by-example/std_misc/threads.html) · [Testcase: map-reduce ↗](https://doc.rust-lang.org/rust-by-example/std_misc/threads/testcase_mapreduce.html) | ✓ | [Spawning a thread](09_Advanced/spawning_a_thread/README.md) — and `thread::scope`, which RBE does not cover; then [`Arc`](18_Ownership/sharing_across_threads/README.md) and [lock poisoning](09_Advanced/mutex_poisoning/README.md) |
| [Channels ↗](https://doc.rust-lang.org/rust-by-example/std_misc/channels.html) | ✓ | [Channels](09_Advanced/channels/README.md) — `mpsc`, the `Sender` you kept, and bounded backpressure |
| [Path ↗](https://doc.rust-lang.org/rust-by-example/std_misc/path.html) | ◐ | [`Path` and `PathBuf`](04_Files/path_and_pathbuf/README.md) — stub |
| [File I/O ↗](https://doc.rust-lang.org/rust-by-example/std_misc/file.html) · [open ↗](https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html) · [create ↗](https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html) · [read_lines ↗](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html) | ◐ | [Files](04_Files/README.md) — five stubs, including [reading lines efficiently](04_Files/reading_lines_efficiently/README.md), which is RBE's "more efficient approach" |
| [Child processes ↗](https://doc.rust-lang.org/rust-by-example/std_misc/process.html) · [Pipes ↗](https://doc.rust-lang.org/rust-by-example/std_misc/process/pipe.html) · [Wait ↗](https://doc.rust-lang.org/rust-by-example/std_misc/process/wait.html) | · | Not covered |
| [Filesystem Operations ↗](https://doc.rust-lang.org/rust-by-example/std_misc/fs.html) | · | Not covered |
| [Program arguments ↗](https://doc.rust-lang.org/rust-by-example/std_misc/arg.html) · [Argument parsing ↗](https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html) | ◐ | [Command line](03_Command_Line/README.md) — six stubs, including [`clap`](03_Command_Line/clap_derive/README.md), which is what you should actually use |
| [Foreign Function Interface ↗](https://doc.rust-lang.org/rust-by-example/std_misc/ffi.html) | · | Named as future work in [Advanced](09_Advanced/README.md) |

## 21. Testing

| RBE page | Here | Note |
|---|---|---|
| [Unit testing ↗](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html) | ✓ | [What a test asserts](28_Testing/what_a_test_asserts/README.md) — and the assertion that passes for the wrong reason |
| [Documentation testing ↗](https://doc.rust-lang.org/rust-by-example/testing/doc_testing.html) | ✓ | [The example that is a test](28_Testing/doc_tests/README.md) |
| [Integration testing ↗](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html) | ✓ | [Where a test goes](28_Testing/where_a_test_goes/README.md) — `#[cfg(test)]` beside the code, versus `tests/` outside it |
| [Development dependencies ↗](https://doc.rust-lang.org/rust-by-example/testing/dev_dependencies.html) | ✓ | [Where a test goes](28_Testing/where_a_test_goes/README.md), and [cargo-nextest](05_Tooling/nextest/README.md) |

## 22. Unsafe Operations

| RBE page | Here | Note |
|---|---|---|
| [Unsafe Operations ↗](https://doc.rust-lang.org/rust-by-example/unsafe.html) | ✓ | [What `unsafe` turns off](09_Advanced/what_unsafe_turns_off/README.md) — the five powers, and the four rules that stay on; then [a union](09_Advanced/what_a_union_is/README.md) and [the global allocator](09_Advanced/the_global_allocator/README.md) as two of them in use |
| [Inline assembly ↗](https://doc.rust-lang.org/rust-by-example/unsafe/asm.html) | · | Not covered |

## 23. Compatibility

| RBE page | Here | Note |
|---|---|---|
| [Raw identifiers ↗](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html) | · | Not covered — `r#match` is real and rare |

## 24. Meta

| RBE page | Here | Note |
|---|---|---|
| [Documentation ↗](https://doc.rust-lang.org/rust-by-example/meta/doc.html) | ◐ | [Comments that compile](15_First_Programs/comments_that_compile/README.md) covers `///` and `//!`; [doc tests](28_Testing/doc_tests/README.md) covers the part that runs |
| [Playground ↗](https://doc.rust-lang.org/rust-by-example/meta/playground.html) | · | Not covered |

## What is still missing

Ordered by what it costs a reader who came here from RBE:

1. **Flow of control has no runnable page.** **`25_Control_Flow/`** *(in flight)* is eight stubs, and `if`/`match`/`for` are the first things anyone needs. Highest-value gap in the library.
2. **`macro_rules!`** — one stub, and RBE's five pages are the best short treatment anywhere.
3. **`let-else`, `@` bindings, slice patterns.** Pattern matching beyond one arm.
4. **Child processes, filesystem operations, FFI.** All three are stubs or absent.

## See also

- [Rust by Example](00_Start_Here/rust_by_example/README.md) — what RBE is for, and the two moments it is the right answer
- [The course, in order](index.md) — the reading order these lessons were written in
- [KATAS.md](KATAS.md) — the exercises, which RBE calls *Activities*

## Po polsku

*Rust by Example* to zbiór krótkich, uruchamialnych przykładów, który większość uczących się trzyma otwarty w drugiej karcie przeglądarki. Rzecz, którą lepiej wiedzieć od razu: **polskiego tłumaczenia nie ma**. Oficjalne repozytorium prowadzi przekłady na hiszpański, japoński, koreański i chiński — i na tym koniec. Nie przeoczyłeś polskiej wersji, ona nie istnieje.

Ta strona jest mapą pokrycia: przechodzi przez 24 rozdziały RBE i mówi, która lekcja tutaj im odpowiada, gdzie ta biblioteka schodzi głębiej, a czego nie porusza wcale. Korzystać z niej warto w jedną konkretną stronę — RBE odpowiada na pytanie „jak się to zapisuje", a lekcje tutaj na pytanie „dlaczego tak i gdzie to boli". Role są komplementarne, nie konkurencyjne.

Stąd rada praktyczna dla uczącego się po polsku, i to ta sama, którą powtarza [POLSKI.md](POLSKI.md): składni ucz się, skąd chcesz, ale **komunikatów kompilatora ucz się po angielsku**. `rustc` po polsku nie mówi i mówić nie będzie, a większość czasu w Ruscie spędza się na czytaniu tego, co wypisał.

**Szukaj po polsku:** kurs Rusta po polsku · dokumentacja Rusta · `rust by example` · `rust book tłumaczenie`
