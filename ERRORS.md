# The error-code map

**Level:** reference · the map

**One line:** The code in the red line is the one exact, searchable thing a stuck reader already has — and until this page nothing in the library could be searched by it. Here is every code the library teaches, and the lesson that explains it.

**If you have a code on screen, it is in [the table](#every-code-the-library-teaches).** The rest of this page is about the codes that mean too many things to be looked up that way.

Same rule as the other maps: order is presentation, so it lives here rather than in folder names — [STRUCTS.md](STRUCTS.md) explains why in full, and [STRINGS.md](STRINGS.md), [OPTION.md](OPTION.md) and [SHADOWING.md](SHADOWING.md) follow it too.

---

## The six you will actually meet

The **on** column is how many lesson pages in this library have to mention the code — a fair proxy for how often you meet it, and the reason these six lead.

| code | rustc says | on | what it usually means here |
|---|---|---|---|
| [E0277 ↗](https://doc.rust-lang.org/error_codes/E0277.html) | the trait bound is not satisfied | 38 | four unrelated struct mistakes share it, so [the code is never the diagnosis](16_Structs/when_a_struct_refuses/README.md) — read the line under it |
| [E0308 ↗](https://doc.rust-lang.org/error_codes/E0308.html) | mismatched types | 37 | the least diagnostic code in Rust: it is what you get when almost anything is wrong. Three lessons treat three unrelated causes — a `+` whose left side does not own its bytes, a shadow that changed type, and an `if` whose arms disagree over one stray `;` |
| [E0382 ↗](https://doc.rust-lang.org/error_codes/E0382.html) | use of moved value | 30 | `let b = a;` moved rather than copied. [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) is the page: a struct is never `Copy` by accident |
| [E0502 ↗](https://doc.rust-lang.org/error_codes/E0502.html) | cannot borrow as mutable | 18 | many readers **or** one writer. [Borrowing](18_Ownership/borrowing/README.md) — and the part that decides it is where the compiler thinks the borrow *ended* |
| [E0282 ↗](https://doc.rust-lang.org/error_codes/E0282.html) | type annotations needed | 14 | it is asking *which type*, and for `collect` it is asking [which collection](24_Iterators/collect_and_fromiterator/README.md). [Type inference](15_First_Programs/type_inference/README.md) is the general case |
| [E0599 ↗](https://doc.rust-lang.org/error_codes/E0599.html) | no method named … | 12 | [one code over three unrelated mistakes](12_Traits/no_method_named/README.md) — never written, not imported, not implemented — and the `help:` line tells them apart |

Notice what four of those six have in common: **one code, several causes.** That is the reason this page is a map to lessons rather than a glossary of codes — the code narrows the search, and the lesson finishes it.

## Every code the library teaches

| code | the lesson that explains it | also appears on |
|---|---|---|
| [E0004 ↗](https://doc.rust-lang.org/error_codes/E0004.html) | [Six kinds of zero](17_Option_and_Result/six_kinds_of_zero/README.md) | **8** other pages |
| [E0005 ↗](https://doc.rust-lang.org/error_codes/E0005.html) | [Irrefutable patterns](30_Pattern_Matching/irrefutable_patterns/README.md) | 1 other page |
| [E0015 ↗](https://doc.rust-lang.org/error_codes/E0015.html) | [`const` and `static`](27_Modules/const_and_static/README.md) | — |
| [E0038 ↗](https://doc.rust-lang.org/error_codes/E0038.html) | [Static vs dynamic dispatch](12_Traits/static_vs_dynamic_dispatch/README.md) | 1 other page |
| [E0040 ↗](https://doc.rust-lang.org/error_codes/E0040.html) | [`Drop`, and what RAII buys](12_Traits/drop_and_raii/README.md) | 1 other page |
| [E0063 ↗](https://doc.rust-lang.org/error_codes/E0063.html) | [When a struct refuses](16_Structs/when_a_struct_refuses/README.md) | — |
| [E0072 ↗](https://doc.rust-lang.org/error_codes/E0072.html) | [`Box`](26_Collections/the_box/README.md) | **3** other pages |
| [E0106 ↗](https://doc.rust-lang.org/error_codes/E0106.html) | [Lifetime annotations](18_Ownership/lifetime_annotations/README.md) | **6** other pages |
| [E0117 ↗](https://doc.rust-lang.org/error_codes/E0117.html) | [`From` and `Into`](29_Conversion/from_and_into/README.md) | — |
| [E0119 ↗](https://doc.rust-lang.org/error_codes/E0119.html) | [When a struct refuses](16_Structs/when_a_struct_refuses/README.md) | **2** other pages |
| [E0133 ↗](https://doc.rust-lang.org/error_codes/E0133.html) | [What `unsafe` turns off](09_Advanced/what_unsafe_turns_off/README.md) | — |
| [E0184 ↗](https://doc.rust-lang.org/error_codes/E0184.html) | [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) | 1 other page |
| [E0204 ↗](https://doc.rust-lang.org/error_codes/E0204.html) | [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) | — |
| [E0214 ↗](https://doc.rust-lang.org/error_codes/E0214.html) | [A type is not a constructor](16_Structs/a_type_is_not_a_constructor/README.md) | — |
| [E0252 ↗](https://doc.rust-lang.org/error_codes/E0252.html) | [Bringing names in with `use`](27_Modules/the_use_declaration/README.md) | — |
| [E0277 ↗](https://doc.rust-lang.org/error_codes/E0277.html) | [When a struct refuses](16_Structs/when_a_struct_refuses/README.md) | **37** other pages |
| [E0282 ↗](https://doc.rust-lang.org/error_codes/E0282.html) | [Type inference](15_First_Programs/type_inference/README.md) | **13** other pages |
| [E0284 ↗](https://doc.rust-lang.org/error_codes/E0284.html) | [What a type annotation does](15_First_Programs/what_an_annotation_does/README.md) | 1 other page |
| [E0308 ↗](https://doc.rust-lang.org/error_codes/E0308.html) | [Concatenating strings](14_Strings/concatenating_strings/README.md) — but see [below](#the-two-codes-with-no-single-home) | **36** other pages |
| [E0317 ↗](https://doc.rust-lang.org/error_codes/E0317.html) | [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md) | — |
| [E0368 ↗](https://doc.rust-lang.org/error_codes/E0368.html) | [Concatenating strings](14_Strings/concatenating_strings/README.md) | 1 other page |
| [E0369 ↗](https://doc.rust-lang.org/error_codes/E0369.html) | [Concatenating strings](14_Strings/concatenating_strings/README.md) | **4** other pages |
| [E0373 ↗](https://doc.rust-lang.org/error_codes/E0373.html) | [The `move` keyword](23_Closures/the_move_keyword/README.md) — but see [below](#the-two-codes-with-no-single-home) | **2** other pages |
| [E0381 ↗](https://doc.rust-lang.org/error_codes/E0381.html) | [A type is not a constructor](16_Structs/a_type_is_not_a_constructor/README.md) | **4** other pages |
| [E0382 ↗](https://doc.rust-lang.org/error_codes/E0382.html) | [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) | **29** other pages |
| [E0384 ↗](https://doc.rust-lang.org/error_codes/E0384.html) | [Variables](15_First_Programs/variables/README.md) | **2** other pages |
| [E0392 ↗](https://doc.rust-lang.org/error_codes/E0392.html) | [Phantom types](12_Traits/phantom_types/README.md) | **3** other pages |
| [E0408 ↗](https://doc.rust-lang.org/error_codes/E0408.html) | [One arm, many values](17_Option_and_Result/one_arm_many_values/README.md) | — |
| [E0423 ↗](https://doc.rust-lang.org/error_codes/E0423.html) | [A type is not a constructor](16_Structs/a_type_is_not_a_constructor/README.md) | **2** other pages |
| [E0425 ↗](https://doc.rust-lang.org/error_codes/E0425.html) | [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md) | **4** other pages |
| [E0428 ↗](https://doc.rust-lang.org/error_codes/E0428.html) | [One module per file](27_Modules/one_module_per_file/README.md) | — |
| [E0433 ↗](https://doc.rust-lang.org/error_codes/E0433.html) | [A throwaway that needs a crate: three commands, and the message that means you skipped them](05_Tooling/scratch_with_a_crate/README.md) | — |
| [E0434 ↗](https://doc.rust-lang.org/error_codes/E0434.html) | [What a closure is](23_Closures/what_a_closure_is/README.md) | — |
| [E0435 ↗](https://doc.rust-lang.org/error_codes/E0435.html) | [What a compiler does before your program runs](20_Compilers/what_a_compiler_does/README.md) | — |
| [E0451 ↗](https://doc.rust-lang.org/error_codes/E0451.html) | [Modules and visibility](27_Modules/modules_and_visibility/README.md) | — |
| [E0499 ↗](https://doc.rust-lang.org/error_codes/E0499.html) | [What `unsafe` turns off](09_Advanced/what_unsafe_turns_off/README.md) | **5** other pages |
| [E0502 ↗](https://doc.rust-lang.org/error_codes/E0502.html) | [Borrowing: `&T`, `&mut T`, and where a borrow ends](18_Ownership/borrowing/README.md) | **17** other pages |
| [E0505 ↗](https://doc.rust-lang.org/error_codes/E0505.html) | [A shadow does not drop](18_Ownership/shadowing_does_not_drop/README.md) | **2** other pages |
| [E0506 ↗](https://doc.rust-lang.org/error_codes/E0506.html) | [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md) | 1 other page |
| [E0507 ↗](https://doc.rust-lang.org/error_codes/E0507.html) | [Transforms instead of `match`](17_Option_and_Result/transforms_instead_of_match/README.md) | **5** other pages |
| [E0509 ↗](https://doc.rust-lang.org/error_codes/E0509.html) | [The drop flag](18_Ownership/the_drop_flag/README.md) | — |
| [E0515 ↗](https://doc.rust-lang.org/error_codes/E0515.html) | [Collect the iterator into a `Vec`](24_Iterators/collect_into_a_vec/README.md) | **6** other pages |
| [E0585 ↗](https://doc.rust-lang.org/error_codes/E0585.html) | [Comments that compile](15_First_Programs/comments_that_compile/README.md) | — |
| [E0593 ↗](https://doc.rust-lang.org/error_codes/E0593.html) | [`unwrap_or_else`: the fallback that is built only if it is needed](17_Option_and_Result/unwrap_or_else/README.md) | — |
| [E0594 ↗](https://doc.rust-lang.org/error_codes/E0594.html) | [When a struct refuses](16_Structs/when_a_struct_refuses/README.md) | — |
| [E0596 ↗](https://doc.rust-lang.org/error_codes/E0596.html) | [`impl` blocks](16_Structs/impl_blocks/README.md) | **3** other pages |
| [E0597 ↗](https://doc.rust-lang.org/error_codes/E0597.html) | [`&'static str`](14_Strings/static_str/README.md) | **2** other pages |
| [E0599 ↗](https://doc.rust-lang.org/error_codes/E0599.html) | ["No method named …"](12_Traits/no_method_named/README.md) | **11** other pages |
| [E0603 ↗](https://doc.rust-lang.org/error_codes/E0603.html) | [Modules and visibility](27_Modules/modules_and_visibility/README.md) | **2** other pages |
| [E0605 ↗](https://doc.rust-lang.org/error_codes/E0605.html) | [What an enum is](13_Enums/what_an_enum_is/README.md) | — |
| [E0616 ↗](https://doc.rust-lang.org/error_codes/E0616.html) | [Modules and visibility](27_Modules/modules_and_visibility/README.md) | 1 other page |
| [E0618 ↗](https://doc.rust-lang.org/error_codes/E0618.html) | [A type is not a constructor](16_Structs/a_type_is_not_a_constructor/README.md) | **2** other pages |
| [E0624 ↗](https://doc.rust-lang.org/error_codes/E0624.html) | ["No method named …"](12_Traits/no_method_named/README.md) | — |
| [E0659 ↗](https://doc.rust-lang.org/error_codes/E0659.html) | [Bringing names in with `use`](27_Modules/the_use_declaration/README.md) | — |
| [E0665 ↗](https://doc.rust-lang.org/error_codes/E0665.html) | [What an attribute is](27_Modules/what_an_attribute_is/README.md) | 1 other page |
| [E0700 ↗](https://doc.rust-lang.org/error_codes/E0700.html) | [Returning an iterator](24_Iterators/returning_an_iterator/README.md) | — |
| [E0716 ↗](https://doc.rust-lang.org/error_codes/E0716.html) | [`Vec::into_iter` — and the three `IntoIterator` impls](26_Collections/vec_methods/vec_into_iter/README.md) | 1 other page |
| [E0740 ↗](https://doc.rust-lang.org/error_codes/E0740.html) | [What a union is](09_Advanced/what_a_union_is/README.md) | — |
| [E0753 ↗](https://doc.rust-lang.org/error_codes/E0753.html) | [Comments that compile](15_First_Programs/comments_that_compile/README.md) | — |

## Mentioned, but not taught

Seven more codes appear once or twice in passing, without a lesson behind them. They are listed for completeness, and each links to rustc's own explanation:

[E0080 ↗](https://doc.rust-lang.org/error_codes/E0080.html) on [What a compiler does before your program runs](20_Compilers/what_a_compiler_does/README.md), [E0170 ↗](https://doc.rust-lang.org/error_codes/E0170.html) on [A typo becomes a binding](13_Enums/a_typo_becomes_a_binding/README.md), [E0283 ↗](https://doc.rust-lang.org/error_codes/E0283.html) on [Type inference](15_First_Programs/type_inference/README.md), [E0405 ↗](https://doc.rust-lang.org/error_codes/E0405.html) on [There is no `Move` trait](18_Ownership/no_move_trait/README.md), [E0432 ↗](https://doc.rust-lang.org/error_codes/E0432.html) on [A typo becomes a binding](13_Enums/a_typo_becomes_a_binding/README.md), [E0493 ↗](https://doc.rust-lang.org/error_codes/E0493.html) on [`Vec` methods](26_Collections/vec_methods/README.md), [E0658 ↗](https://doc.rust-lang.org/error_codes/E0658.html) on [`unwrap_or`: the default you already have](17_Option_and_Result/unwrap_or/README.md)

## The two codes with no single home

The table above names one lesson per code, chosen as the page that treats it at most length. For two codes that choice is a tie, and the tie is telling rather than arbitrary.

**E0308** is mismatched types — what rustc says when almost anything is wrong. It appears on **37** lesson pages — second only to E0277 — and the two that treat it most do so for entirely different reasons: [Concatenating strings](14_Strings/concatenating_strings/README.md) meets it as the near miss where `+` has a `String` on the right instead of a `&str`, and [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md) meets it when a shadow changes a binding's type. A third, [`if` expressions](25_Control_Flow/if_expressions/README.md), meets it when two branches disagree — usually over one stray `;`. There is no single lesson to send you to, because there is no single mistake.

**E0373** — closure may outlive the current function — ties between [The `move` keyword](23_Closures/the_move_keyword/README.md), which is the concept, and [Spawning a thread](09_Advanced/spawning_a_thread/README.md), which is where you meet it. Read the first if you want to know why, the second if you have a thread that will not compile.

## How this page is kept honest

Every row was derived from the library rather than written from memory: the owning lesson for each code is the page that mentions it most, and the "also appears on" count is the number of other pages carrying it. All 65 code links were checked live. If a code moves to a better home, the count moves with it — so re-derive this table rather than editing a row by hand.

## Sources

The linked explanations are rustc's own [error index ↗](https://doc.rust-lang.org/error_codes/error-index.html), which is the authority for what each code means; this page claims only which lesson in *this* library explains it.
