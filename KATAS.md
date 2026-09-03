# Katas

**Level:** reference · the practice track

**One line:** The lessons explain; the katas make you type. This page is the only place the katas are ordered — each kata itself lives on the page for the topic it teaches.

---

## Where a kata lives

**A kata belongs to its topic, not to a folder of its own.** The page that explains `Option` is the page that asks you to write one, under a `## Practice` heading near the end, with the solution folded away in a `<details>` block.

That is a deliberate choice, and the reason is that folders are URLs. A topic — `Option`, `if let`, borrowing — is stable for years; a *sequence* is not. The moment you write a kata that belongs between K1 and K2, a `K01_…/` folder either gets renumbered (breaking every link anyone saved) or starts lying about its own order. So the sequence lives here, in a table that costs nothing to reorder, and the numbers below are labels rather than addresses. It is the same rule the sidebar follows: [order is presentation](CONTRIBUTING.md), so it belongs in a page, never in a path.

The happy side effect is that a kata arrives with its explanation already written, and a topic can collect several katas over time without any of them needing to restate the background.

## The katas

| # | Kata | Lesson | Level |
|---|---|---|---|
| K1 | [One file, three builds — plain, `--test` and `-O`, each predicted before you run it](15_First_Programs/rustc_without_cargo/README.md#practice) | [Running a scratch program](15_First_Programs/rustc_without_cargo/README.md) | 101 |
| K2 | [Three misplaced doc comments — predict warning, `E0585` or `E0753` for each, then move them](15_First_Programs/comments_that_compile/README.md#practice) | [Comments that compile](15_First_Programs/comments_that_compile/README.md) | 101 → 201 |
| K3 | [Fibonacci, and the width that runs out — write `fib`, find the exact `n` that panics, then explain why the same `n` prints a wrong number in `--release`](15_First_Programs/values/README.md#practice) | [Values](15_First_Programs/values/README.md) | 101 → 201 |
| K4 | [The guard that stops protecting you — swap `&&` for `&` on a bounds check, then write Python's `sum(flags)` three ways and defend one of them](15_First_Programs/meet_the_bool/README.md#practice) | [Meet the `bool`](15_First_Programs/meet_the_bool/README.md) | 101 → 201 |
| K5 | [A favourite number that may not exist — `Some` / `None`, one `match`, and `unwrap_or`](17_Option_and_Result/some_and_none/README.md#practice) | [`Some` and `None`](17_Option_and_Result/some_and_none/README.md) | 101 |
| K6 | [The arm you deleted — add a variant, and watch only one of the two forms come and find you](17_Option_and_Result/if_let/README.md#practice) | [`if let`](17_Option_and_Result/if_let/README.md) | 101 |
| K7 | [The reason the caller could have used — the same four bad cells, once as `None` and once as a named error](17_Option_and_Result/option_vs_result/README.md#practice) | [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md) | 101 |
| K8 | [The default that was built for nothing — watch an eager fallback run on the happy path](17_Option_and_Result/unwrap_or/README.md#practice) | [`unwrap_or`](17_Option_and_Result/unwrap_or/README.md) | 201 |
| K9 | [Fall back, but keep the reason — the one job only the `_else` closure can do](17_Option_and_Result/unwrap_or_else/README.md#practice) | [`unwrap_or_else`](17_Option_and_Result/unwrap_or_else/README.md) | 201 |
| K10 | [The type's zero is not your domain's zero — a blank ballot defaults into a real-looking one](17_Option_and_Result/unwrap_or_default/README.md#practice) | [`unwrap_or_default`](17_Option_and_Result/unwrap_or_default/README.md) | 201 |
| K11 | [Transform, or fall back — a default written first and run last](17_Option_and_Result/map_or/README.md#practice) | [`map_or`](17_Option_and_Result/map_or/README.md) | 201 |
| K12 | [Follow the responsibility — a `Drop` that prints, and the function where the free actually happened](18_Ownership/ownership_and_moves/README.md#practice) | [Ownership and moves](18_Ownership/ownership_and_moves/README.md) | 101 |
| K13 | [Many readers, or one writer — then move one `println!` and read `E0502`](18_Ownership/borrowing/README.md#practice) | [Borrowing](18_Ownership/borrowing/README.md) | 101 → 201 |
| K14 | [Two places, or one? — compile the four-line reference test both ways, time the drops, read the optimizer's answer, then grade the three-option quiz that rejects the right line for the wrong reason](18_Ownership/a_name_is_not_a_place/README.md#practice) | [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md) | 201 |
| K15 | [The same program with a type that is not `Copy` — read `E0382`, then fix it four ways and pick one](17_Option_and_Result/shadowing_and_unwrap/README.md#practice) | [Shadowing and `unwrap`](17_Option_and_Result/shadowing_and_unwrap/README.md) | 201 |
| K16 | [The value you can no longer free — shadow a buffer, watch it outlive the work, then fix it three ways](18_Ownership/shadowing_does_not_drop/README.md#practice) | [A shadow does not drop](18_Ownership/shadowing_does_not_drop/README.md) | 201 |
| K17 | [Three shadows, one of them earned — predict two wrong numbers, then fix one by deleting a `let` and the other by renaming](18_Ownership/when_to_shadow/README.md#practice) | [When to shadow](18_Ownership/when_to_shadow/README.md) | 201 |
| K18 | [The tally that never tallied — a shadowed accumulator logs three plausible running totals and names a candidate who scored zero](18_Ownership/nothing_checks_a_shadow/README.md#practice) | [Nothing checks a shadow](18_Ownership/nothing_checks_a_shadow/README.md) | 201 |
| K19 | [Time three things you cannot see — a report that closes before it writes, a guard released one identifier early, and a borrow ended three ways](18_Ownership/scope_is_about_names/README.md#practice) | [Scope is about names, not values](18_Ownership/scope_is_about_names/README.md) | 201 |
| K20 | [Six statements, and only two of them drop anything — a shadow and a reassignment that differ by one keyword and by everything else](18_Ownership/assignment_is_a_drop/README.md#practice) | [Assignment drops the old value](18_Ownership/assignment_is_a_drop/README.md) | 201 |
| K21 | [Four warnings, four different right answers — only one of them is an underscore, and one is still broken if you pick the wrong kind](15_First_Programs/what_a_warning_is_asking/README.md#practice) | [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md) | 101 → 201 |
| K22 | [What the panic left behind — read the damage an `unwrap` does mid-job, then make the missing row a return value](17_Option_and_Result/what_a_panic_costs/README.md#practice) | [What a panic costs](17_Option_and_Result/what_a_panic_costs/README.md) | 201 |
| K23 | [Four sentences, one of them a hope — name the guarantor for each `expect`, then watch the one that has none die on a misspelled key](17_Option_and_Result/expect/README.md#practice) | [`expect`: writing down the proof](17_Option_and_Result/expect/README.md) | 201 |
| K24 | [Delete four unwraps — rewrite a README-shaped config parser so no line can abort, using a different technique for each](02_Errors/unwrap_is_a_todo/README.md#practice) | [`unwrap` is a TODO you forgot to remove](02_Errors/unwrap_is_a_todo/README.md) | 201 |
| K25 | [The average of nothing — make a partial function total, and say what your `None` means](17_Option_and_Result/partial_functions/README.md#practice) | [Partial functions](17_Option_and_Result/partial_functions/README.md) | 201 |
| K26 | [Four causes, one `None` — write the operator's error message from a signature that discarded it](17_Option_and_Result/none_on_error/README.md#practice) | [Returning `None` on error](17_Option_and_Result/none_on_error/README.md) | 201 |
| K27 | [Declare it, then prove it — a value decided in three branches, with no `Option` and no `mut`](17_Option_and_Result/initial_values/README.md#practice) | [Initial values](17_Option_and_Result/initial_values/README.md) | 201 |
| K28 | [Which fields may legitimately be missing? — and telling *no ballot* apart from *an empty one*](17_Option_and_Result/option_fields/README.md#practice) | [`Option` fields](17_Option_and_Result/option_fields/README.md) | 101 |
| K29 | [No `match` allowed — count, total and average the ballots that exist with iterator methods only](17_Option_and_Result/option_as_collection/README.md#practice) | [`Option` is a one-item collection](17_Option_and_Result/option_as_collection/README.md) | 201 |
| K30 | [A loop with no counter — the body that peeks where it meant to advance](17_Option_and_Result/while_let/README.md#practice) | [`while let`](17_Option_and_Result/while_let/README.md) | 201 |
| K31 | [A list that ends — `Option<Box<Node>>`, and the `size_of` proof that it costs nothing](17_Option_and_Result/nullable_pointers/README.md#practice) | [Nullable pointers](17_Option_and_Result/nullable_pointers/README.md) | 201 |
| K32 | [One optional argument, four ways — and the signature that rejects half your callers](17_Option_and_Result/optional_arguments/README.md#practice) | [Optional function arguments](17_Option_and_Result/optional_arguments/README.md) | 201 |
| K33 | [Make the invalid score unbuildable — then find the door your own module still leaves open](16_Structs/newtype_score/README.md#practice) | [A score is not a number](16_Structs/newtype_score/README.md) | 101 → 201 |
| K34 | [The line you forgot — desync two parallel `Vec`s and get a plausible wrong answer](16_Structs/representing_a_record/README.md#practice) | [What is a record, in memory?](16_Structs/representing_a_record/README.md) | 201 |
| K35 | [Expand the alias — follow a one-parameter `Result` back to the list of things that can go wrong](17_Option_and_Result/result_aliases/README.md#practice) | [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md) | 201 |
| K36 | [Guard the input that has no answer — find the case the careful-looking guard lets through](17_Option_and_Result/wrong_guard/README.md#practice) | [Zero wins is not zero games](17_Option_and_Result/wrong_guard/README.md) | 201 |
| K37 | [The `Result` the lock hands you — a thread dies mid-update, and you answer it three ways](09_Advanced/mutex_poisoning/README.md#practice) | [Lock poisoning](09_Advanced/mutex_poisoning/README.md) | 301 |
| K38 | [Credit a fourth knob honestly — the same linker swap, first and last in the ladder](05_Tooling/compile_times/README.md#practice) | [Compile times](05_Tooling/compile_times/README.md) | 201 |
| K39 | [The arm you didn't write — a catch-all quietly refiles two spoiled ballots as blanks](17_Option_and_Result/six_kinds_of_zero/README.md#practice) | [Six kinds of zero](17_Option_and_Result/six_kinds_of_zero/README.md) | 201 |
| K40 | [Eight candidates in one byte — pack an approval ballot into a `u8` with bit operations, then let a ninth sign up](19_Numbers/meet_the_byte/README.md#practice) | [Meet the byte](19_Numbers/meet_the_byte/README.md) | 101 → 201 |
| K41 | [Two decisions, one literal — write three fields in the base whose digits line up with them, then find the quantity whose obvious width is too narrow](19_Numbers/writing_a_number_down/README.md#practice) | [Writing a number down](19_Numbers/writing_a_number_down/README.md) | 101 → 201 |
| K42 | [The fingerprint that collided — one missing `0` turns two different ballot files into the same hex string, across 3,600 of the 65,536 two-byte cases](19_Numbers/why_hexadecimal/README.md#practice) | [Why hexadecimal](19_Numbers/why_hexadecimal/README.md) | 101 → 201 |
| K43 | [A tic-tac-toe game in 18 bits — two nine-bit fields in one `u32`, eight win masks, then a fourth field arrives on top](19_Numbers/bit_flags/README.md#practice) | [Bit flags](19_Numbers/bit_flags/README.md) | 201 |
| K44 | [The results table that would not sort — meet `E0277` on purpose, then rank the same table three ways and pick the one you would ship](19_Numbers/what_a_float_stores/README.md#practice) | [What a float actually stores](19_Numbers/what_a_float_stores/README.md) | 201 |
| K45 | [Spend the entitlement, not just the token — close the sign-in hole, then count what the fix costs](09_Advanced/one_account_one_review/README.md#practice) | [The right to post is a value](09_Advanced/one_account_one_review/README.md) | 301 |
| K46 | [The scale that stopped covering the election — hard-code a denominator, grow the election past it, and find the bug that changes no winner](09_Advanced/scaled_integers/README.md#practice) | [Scale the denominator away](09_Advanced/scaled_integers/README.md) | 301 |
| K47 | [The average that came out as a three-way tie — collapse three candidates onto one number, then rank them without ever dividing](09_Advanced/i128_exactness/README.md#practice) | [What `i128` is exact about](09_Advanced/i128_exactness/README.md) | 301 |
| K48 | [Build the count that always finishes — then find the coarsest rounding that still reproduces the exact winners](09_Advanced/compounding_weights/README.md#practice) | [When the denominators compound](09_Advanced/compounding_weights/README.md) | 301 |
| K49 | [The audit that has to know when to stop — five wards, one rounded count each, and the escalation loop that never returns on a tie](09_Advanced/interval_arithmetic/README.md#practice) | [Did the rounding decide it?](09_Advanced/interval_arithmetic/README.md) | 301 |
| K50 | [The error message nobody saw — count the five ways your error can reach a person, then fix it twice and price both fixes](15_First_Programs/debug_vs_display/README.md#practice) | [Debug and Display](15_First_Programs/debug_vs_display/README.md) | 101 → 201 |
| K51 | [The semicolon that changed the type — cause `E0308` on purpose, then seal a `mut` builder behind braces and make an `if` be the value](15_First_Programs/a_block_is_an_expression/README.md#practice) | [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md) | 101 → 201 |
| K52 | [Three places `()` shows up — take the receipt instead of the result, use `?` on a `Result<(), E>`, then build a set out of `HashMap<T, ()>`](15_First_Programs/the_unit_type/README.md#practice) | [The unit type `()`](15_First_Programs/the_unit_type/README.md) | 101 → 201 |
| K53 | [The f-string that isn't — four braces, four refusals, then fix the line three ways and defend the one you ship](15_First_Programs/braces_take_a_name/README.md#practice) | [The braces take a name](15_First_Programs/braces_take_a_name/README.md) | 101 → 201 |
| K54 | [The reformat that changed the program — find the one place a whitespace edit is not cosmetic, and watch the formatter decline to help](05_Tooling/formatting/README.md#practice) | [Formatting](05_Tooling/formatting/README.md) | 101 → 201 |
| K55 | [The hour that changed its ad — collapse twenty-six match arms into five, then break it three ways and predict which two the compiler catches](17_Option_and_Result/one_arm_many_values/README.md#practice) | [One arm, many values](17_Option_and_Result/one_arm_many_values/README.md) | 101 → 201 |
| K56 | [Three ways to make `Some(None)` compile — read `E0308` in full, then fix it by deleting, by supplying, and by widening the field, and defend the one you would ship](17_Option_and_Result/some_is_a_constructor/README.md#practice) | [`Some` is a constructor, not a flag](17_Option_and_Result/some_is_a_constructor/README.md) | 101 → 201 |
| K57 | [Three flavors, and the two things the compiler keeps apart — identical tuple structs that will not substitute, the private field that privatises a constructor, and a unit struct whose only content is behaviour](16_Structs/what_a_struct_is/README.md#practice) | [What a struct is](16_Structs/what_a_struct_is/README.md) | 101 → 201 |
| K58 | [Pick the right receiver four times — then call a `&mut self` method through a non-`mut` binding, and use a value after a method took `self`](16_Structs/impl_blocks/README.md#practice) | [`impl` blocks](16_Structs/impl_blocks/README.md) | 101 → 201 |
| K59 | [Predict which half of the base survives a `..base` — four fields, two of them still readable, and the trailing comma that is its own error](16_Structs/struct_update/README.md#practice) | [Struct update syntax](16_Structs/struct_update/README.md) | 101 → 201 |
| K60 | [One `E0382`, three fixes, and the `String` field that removes one of them — then rank what each costs the *caller*](16_Structs/copy_vs_clone/README.md#practice) | [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) | 101 → 201 |
| K61 | [One `&str` parameter, three callers — then flip it to `String` and catalogue what every call site now pays](14_Strings/string_vs_str/README.md#practice) | [`String` vs `&str`](14_Strings/string_vs_str/README.md) | 101 → 201 |
| K62 | [Cut a name in half without panicking — `len()/2` on four names, and the two ways to find a legal boundary](14_Strings/string_slices/README.md#practice) | [String slices](14_Strings/string_slices/README.md) | 101 → 201 |
| K63 | [Predict `len` and `capacity` through five pushes — which ones reallocate? — then make one up-front allocation serve all of them](14_Strings/anatomy_of_a_string/README.md#practice) | [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md) | 101 → 201 |
| K64 | [Implement `Display` once and collect four abilities — then add `impl ToString` and read the `E0119`](14_Strings/making_a_string/README.md#practice) | [Making a `String`](14_Strings/making_a_string/README.md) | 101 → 201 |
| K65 | [One greeting three ways — then go back and earn all three refusals on purpose (`E0369`, `E0308`, `E0368`)](14_Strings/concatenating_strings/README.md#practice) | [Concatenating strings](14_Strings/concatenating_strings/README.md) | 101 → 201 |
| K66 | [One line built four ways — which inputs survive, how many buffers, and which one is wrong inside a loop](14_Strings/building_a_string/README.md#practice) | [Building a `String`](14_Strings/building_a_string/README.md) | 101 → 201 |
| K67 | [One name, three lengths — a per-`char` inventory, and the combining accent that makes two identical-looking strings unequal](14_Strings/meet_the_char/README.md#practice) | [Meet the `char`](14_Strings/meet_the_char/README.md) | 101 → 201 |
| K68 | [An empty field is data — parse `"5,,0"` into abstentions, then watch `split_whitespace()` shorten the row](14_Strings/walking_a_string/README.md#practice) | [Walking a `String`](14_Strings/walking_a_string/README.md) | 101 → 201 |
| K69 | [Return a label three ways — read the `E0515`, then match, leak, or own it, and say which one leaks in a loop](14_Strings/static_str/README.md#practice) | [`&'static str`](14_Strings/static_str/README.md) | 201 |
| K70 | [Three arrivals, three types — then break the UTF-8 promise and the NUL promise on purpose, and read both refusals](14_Strings/six_kinds_of_string/README.md#practice) | [Six kinds of string](14_Strings/six_kinds_of_string/README.md) | 201 |
| K71 | [Pivot both ways, then earn `E0106` and `E0515` — a borrowed field, an owned rewrite, and a `Cow` that allocates only when the text changes](14_Strings/string_vs_str/README.md#practice) | [`String` vs `&str`](14_Strings/string_vs_str/README.md) | 101 → 201 |
| K72 | [Slice up to a boundary, then straight through one — `is_char_boundary` by hand against `floor_char_boundary`, the panic message in full, and an `Option` where a crash was](14_Strings/string_slices/README.md#practice) | [String slices](14_Strings/string_slices/README.md) | 201 |
| K73 | [Four arrivals and one pre-payment — a capacity that does not move, a `parse` that does not trim, and the bytes that turned out not to be UTF-8](14_Strings/making_a_string/README.md#practice) | [Making a `String`](14_Strings/making_a_string/README.md) | 101 → 201 |
| K74 | [Five edits, one buffer — `retain`, an insert at the middle character rather than the middle byte, `drain`, the `+` that eats its left operand, and `pop`](14_Strings/building_a_string/README.md#practice) | [Building a `String`](14_Strings/building_a_string/README.md) | 101 → 201 |
| K75 | [Run-length encoding both ways — then the run past nine, the input with a digit in it, and the string that comes out bigger](14_Strings/building_a_string/README.md#practice) | [Building a `String`](14_Strings/building_a_string/README.md) | 201 |
| K76 | [Case, whitespace, and `STARVoting` — a converter pair that is not a round trip, ASCII-only case swapping, and `'ß'` as the reason both families exist](14_Strings/meet_the_char/README.md#practice) | [Meet the `char`](14_Strings/meet_the_char/README.md) | 201 |
| K77 | [The third ruler — reverse `café` and lose the accent, count a family emoji as seven, then write the grouper and name what it still cannot do](14_Strings/meet_the_char/README.md#practice) | [Meet the `char`](14_Strings/meet_the_char/README.md) | 201 → 301 |
| K78 | [Two rulers over one string — `len()` against `chars().count()`, and the two indices that agree until the crab](14_Strings/walking_a_string/README.md#practice) | [Walking a `String`](14_Strings/walking_a_string/README.md) | 101 → 201 |
| K79 | [Six searches, and the two that want a regex — palindromes, fields, offsets, every match, then what your hand-rolled email finder gets wrong](14_Strings/walking_a_string/README.md#practice) | [Walking a `String`](14_Strings/walking_a_string/README.md) | 101 → 201 |
| K80 | [Build the same choice twice — a hand-maintained tag beside a union, then an `enum`, and the desync only one of them can have](09_Advanced/what_a_union_is/README.md#practice) | [What a union is](09_Advanced/what_a_union_is/README.md) | 301 |
| K81 | [Seven errors, five root causes, three edits — group them by cause before changing a line](16_Structs/when_a_struct_refuses/README.md#practice) | [When a struct refuses](16_Structs/when_a_struct_refuses/README.md) | 101 → 201 |
| K82 | [Both `dbg!` traps — the alternate flag a hand-written `Debug` ignores, and the move that costs you the value](15_First_Programs/what_dbg_does/README.md#practice) | [What `dbg!` does](15_First_Programs/what_dbg_does/README.md) | 101 → 201 |
| K83 | [Four spellings, four error codes — and the one that compiles into the wrong thing](16_Structs/a_type_is_not_a_constructor/README.md#practice) | [A type is not a constructor](16_Structs/a_type_is_not_a_constructor/README.md) | 101 → 201 |
| K84 | [Seed it, prove it, measure the modulo bias, then remove it — counted over the whole output space, never sampled](15_First_Programs/randomness/README.md#practice) | [Randomness](15_First_Programs/randomness/README.md) | 101 → 201 |
| K85 | [Which defence catches a silent catch-all — two lints denied, a variant added, and only one of the three edits does anything](13_Enums/a_typo_becomes_a_binding/README.md#practice) | [A typo becomes a binding](13_Enums/a_typo_becomes_a_binding/README.md) | 201 |
| K86 | [Pay only when you have to — `ensure_prefix` returning `Cow`, then prove the untouched rows were never copied](18_Ownership/clone_on_write/README.md#practice) | [`Cow`: borrow until somebody writes](18_Ownership/clone_on_write/README.md) | 201 |
| K87 | [Make the sum land on the number you would have typed — eight tenths, two groupings, and the one assertion that survives either](19_Numbers/letting_the_compiler_reorder/README.md#practice) | [Letting the compiler reorder a float sum](19_Numbers/letting_the_compiler_reorder/README.md) | 201 → 301 |
| K88 | [Let the source pick the spelling — six things converted to a `String`, and which of the five you were entitled to use](14_Strings/making_a_string/README.md#practice) | [Making a `String`](14_Strings/making_a_string/README.md) | 101 → 201 |
| K89 | [Predict the owned twin before you run it — six receivers, and the two everybody gets wrong](12_Traits/to_owned/README.md#practice) | [`ToOwned`](12_Traits/to_owned/README.md) | 201 |
| K90 | [Predict, then count — one line of output built four ways, and what `with_capacity` buys when the number is one byte short](09_Advanced/the_global_allocator/README.md#practice) | [The global allocator](09_Advanced/the_global_allocator/README.md) | 301 |
| K91 | [Four loops that all look like reuse — predict the allocation count for each, then count them](12_Traits/clone_into/README.md#practice) | [`clone_into`](12_Traits/clone_into/README.md) | 201 → 301 |
| K92 | [A container with two holes — write `Pair<A, B>`, and work out why `swap` cannot return `Self`](22_Generics/what_a_generic_is/README.md#practice) | [What a generic is](22_Generics/what_a_generic_is/README.md) | 101 → 201 |
| K93 | [Walk a linked list without recursion — and without moving or cloning anything on the way](22_Generics/a_generic_recursive_type/README.md#practice) | [A generic recursive type](22_Generics/a_generic_recursive_type/README.md) | 201 → 301 |
| K94 | [Predict the count four times, then find the edge that leaks — one roster shared by three tallies, a deep clone that changes none of the numbers, and a back-reference that stops `Drop` from running](18_Ownership/reference_counting/README.md#practice) | [`Rc`: the clone that copies a pointer](18_Ownership/reference_counting/README.md) | 201 |
| K95 | [Predict the closure-call count, then count it — seven chains over six scores, and the three that stop early for three different reasons](24_Iterators/iterators_are_lazy/README.md#practice) | [Iterators are lazy](24_Iterators/iterators_are_lazy/README.md) | 201 |
| K96 | [The `Vec` that could not outlive its string — read `E0515`, then fix it four ways and say what each one allocates](24_Iterators/collect_into_a_vec/README.md#practice) | [Collect the iterator into a `Vec`](24_Iterators/collect_into_a_vec/README.md) | 101 → 201 |
| K97 | [Six questions, one `Vec` — predict which of them needs a collection, then count how many pieces each one actually walked](24_Iterators/collect_into_a_vec/README.md#practice) | [Collect the iterator into a `Vec`](24_Iterators/collect_into_a_vec/README.md) | 101 → 201 |
| K98 | [Five references where you meant five numbers — reverse an array two ways, then read the `&&i32` in the sum that will not compile, the three routes back to `Vec<i32>`, and the two adapters `rev()` does not commute with](24_Iterators/double_ended_and_exact_size/README.md#practice) | [`DoubleEndedIterator` and `ExactSizeIterator`](24_Iterators/double_ended_and_exact_size/README.md) | 201 → 301 |
| K99 | [Three refusals, three fixes, and the one you can delete — an `Rc` in a `spawn`, a push through a shared `Arc`, a missing per-thread clone, and the total you cannot print](18_Ownership/sharing_across_threads/README.md#practice) | [Sharing across threads: `Arc`](18_Ownership/sharing_across_threads/README.md) | 201 |
| K100 | [Predict seven sizes, then predict which of seven lines allocates — the array against the `Vec`, the three numbers that all miss the heap, and a counting allocator over move, `Copy`, borrow, clone, `Box` and `Rc`](18_Ownership/stack_and_heap/README.md#practice) | [Stack and heap](18_Ownership/stack_and_heap/README.md) | 101 → 201 |
| K101 | [Four fields, and the transposition that compiles — total a ballot two ways, then swap two fields and construct the version rustc cannot catch](26_Collections/tuples/README.md#practice) | [Tuples](26_Collections/tuples/README.md) | 101 |
| K102 | [One function, four callers — write `average` twice, find the three calls the fixed-length signature turns away, and the `0 / 0` that neither panics nor stops](26_Collections/arrays_and_slices/README.md#practice) | [Arrays and slices](26_Collections/arrays_and_slices/README.md) | 101 → 201 |
| K103 | [The call that printed its own receipt — reverse an array inside a `println!`, then find which of rustc, clippy and nobody catches each of the three ways to make the same mistake](26_Collections/arrays_and_slices/README.md#practice) | [Arrays and slices](26_Collections/arrays_and_slices/README.md) | 101 → 201 |
| K104 | [Count the reallocations, then delete them — a hundred pushes three ways, and the elements each one copied](26_Collections/the_vec/README.md#practice) | [`Vec`](26_Collections/the_vec/README.md) | 101 → 201 |
| K105 | [Two lengths in one election — one is a fact about STAR and one is a fact about today; snapshot the array, then try to size the other from `len()`](26_Collections/array_or_vec/README.md#practice) | [Array or `Vec`?](26_Collections/array_or_vec/README.md) | 101 → 201 |
| K106 | [Build the same grid twice and send it the bill — a `Flat` struct against `Vec<Vec<u8>>`, the allocation count for a million cells, and the `set` that writes a real cell it was never asked for](26_Collections/vec_of_vecs/README.md#practice) | [Grids and nested `Vec`s](26_Collections/vec_of_vecs/README.md) | 201 |
| K107 | [Four ways to count, and the two that are wrong — one reports 2 for a candidate who scored 11, the other is merely three lookups per ballot](26_Collections/the_hashmap/README.md#practice) | [`HashMap`](26_Collections/the_hashmap/README.md) | 101 → 201 |
| K108 | [Who voted twice, who never voted — two answers from one operation with its arguments swapped, and the turnout formula that counts a stranger](26_Collections/the_hashset/README.md#practice) | [`HashSet`](26_Collections/the_hashset/README.md) | 201 |
| K109 | [Two orders from one tally, and the key you cannot use — the container hands you one of the two orders free, and keying on each candidate's share puts five in and gets three out](26_Collections/sorted_collections/README.md#practice) | [`BTreeMap` and `BTreeSet`](26_Collections/sorted_collections/README.md) | 101 → 201 |
| K110 | [Two walks and a drop order — the same boxed list recursively and with a cursor, then the error rustc gives when the `Box` comes out](26_Collections/the_box/README.md#practice) | [`Box`](26_Collections/the_box/README.md) | 201 |
| K111 | [Three conversions, one the compiler forbids — the orphan rule in its own words, and the `impl Into<T>` argument that also accepts the type it converts to](29_Conversion/from_and_into/README.md#practice) | [`From` and `Into`](29_Conversion/from_and_into/README.md) | 201 |
| K112 | [A ballot line parsed twice — one parser names the bad cell, the other returns a plausible ballot with a 9 on a 0-5 scale](29_Conversion/tryfrom_and_tryinto/README.md#practice) | [`TryFrom` and `TryInto`](29_Conversion/tryfrom_and_tryinto/README.md) | 201 |
| K113 | [Four silent losses — the turnout that rounds to zero, the guard a negative index walks through, and the three named adds](29_Conversion/casting_with_as/README.md#practice) | [Casting with `as`](29_Conversion/casting_with_as/README.md) | 201 |
| K114 | [Six call sites, three repairs — name the rule that stopped each one: not a reference, inside another type, or a generic parameter](29_Conversion/coercion/README.md#practice) | [Coercion: the conversion you never write](29_Conversion/coercion/README.md) | 201 |
| K115 | [The door your own module left open — three versions of one newtype, two of which let a caller build an invalid score](27_Modules/modules_and_visibility/README.md#practice) | [Modules and visibility](27_Modules/modules_and_visibility/README.md) | 201 |
| K116 | [Two traits called `Write`, and a glob that shadows — predict which line the ambiguity error lands on](27_Modules/the_use_declaration/README.md#practice) | [Bringing names in with `use`](27_Modules/the_use_declaration/README.md) | 101 → 201 |
| K117 | [Five paths to one function — `super`, `crate`, `self` and a sibling, then the fifth file that compiles and never runs](27_Modules/one_module_per_file/README.md#practice) | [One module per file](27_Modules/one_module_per_file/README.md) | 201 |
| K118 | [The address you may not rely on — two pointer comparisons that agree, only one of which is a promise](27_Modules/const_and_static/README.md#practice) | [`const` and `static`](27_Modules/const_and_static/README.md) | 201 |
| K119 | [The derive that changed behaviour when a field moved — one cosmetic diff, and every sort over the type reversed](27_Modules/what_an_attribute_is/README.md#practice) | [What an attribute is](27_Modules/what_an_attribute_is/README.md) | 201 |
| K120 | [Five assertions, two of which cannot fail — say what would have to break before each one noticed](28_Testing/what_a_test_asserts/README.md#practice) | [What a test asserts](28_Testing/what_a_test_asserts/README.md) | 201 |
| K121 | [The test that could not see it — a private helper, and a `should_panic` that goes green on the wrong panic](28_Testing/where_a_test_goes/README.md#practice) | [Where a test goes](28_Testing/where_a_test_goes/README.md) | 201 |
| K122 | [The example that documents half the sentence — then a third case the doc comment never promised at all](28_Testing/doc_tests/README.md#practice) | [The example that is a test](28_Testing/doc_tests/README.md) | 201 |
| K123 | [Four impls for one `*`, and an operator that should not exist — then three plausible readings of adding two turnout figures](12_Traits/operators_are_traits/README.md#practice) | [Operators are traits](12_Traits/operators_are_traits/README.md) | 201 |
| K124 | [Three drop orders, and the guard released one line early — two of the orders are opposites, and one binding is a bug](12_Traits/drop_and_raii/README.md#practice) | [`Drop`, and what RAII buys](12_Traits/drop_and_raii/README.md) | 201 |
| K125 | [Count the drops on four paths — two of them need a run-time decision at the closing brace, and the fourth makes that decision readable](18_Ownership/the_drop_flag/README.md#practice) | [The drop flag](18_Ownership/the_drop_flag/README.md) | 301 |
| K126 | [The same fan-out three ways — sequential, `spawn` plus `Arc`, and `scope`, then the E0373 that explains why the middle one exists](09_Advanced/spawning_a_thread/README.md#practice) | [Spawning a thread](09_Advanced/spawning_a_thread/README.md) | 201 → 301 |
| K127 | [A three-stage pipeline, and the drop that ends it — then reproduce the classic mpsc hang without hanging](09_Advanced/channels/README.md#practice) | [Channels](09_Advanced/channels/README.md) | 201 → 301 |
| K128 | [The safe line the unsafe block depends on — write `split_at_mut`, delete its assert, then find the ordinary `pub fn` that makes an unchecked read unsound](09_Advanced/what_unsafe_turns_off/README.md#practice) | [What `unsafe` turns off](09_Advanced/what_unsafe_turns_off/README.md) | 301 |
| K129 | [Three calls, three causes — one `E0599` each, and only one of them is fixed by implementing the trait the method came from](12_Traits/no_method_named/README.md#practice) | ["No method named …"](12_Traits/no_method_named/README.md) | 201 |
| K130 | [Three predictions about one call — which of the two addresses moved, where the free happened, and which of your three claims was about Rust rather than about one build](18_Ownership/the_call_stack/README.md#practice) | [The call stack](18_Ownership/the_call_stack/README.md) | 101 → 201 |
| K131 | [Prove the reuse, then get the compiler to stop you exploiting it — the same address twice, `E0106` before `E0515`, and the two fixes that are not annotations](18_Ownership/a_stack_slot_is_reused/README.md#practice) | [A stack slot is reused](18_Ownership/a_stack_slot_is_reused/README.md) | 201 |
| K132 | [Freeze a candidate column three ways — handles and buffers for `String`, `Box<str>` and an interned `Rc<str>`, then the `.to_owned()` that clones a pointer instead of the text](14_Strings/boxed_str/README.md#practice) | [The third owned form](14_Strings/boxed_str/README.md) | 201 → 301 |
The numbers are labels, and they live **only in this table** — a kata's own page does not print its number, so moving one costs a single line here and nothing else. Reorder freely; the order is the order to attempt them in, not the order they were written.

Both columns are links, and they go to different places: the kata title opens the exercise itself, the lesson name opens the page it lives on. Every lesson in [`01_Foundations/`](01_Foundations/README.md), [`05_Tooling/`](05_Tooling/README.md) and [`09_Advanced/`](09_Advanced/README.md) now has one.

## By subject

The table above is the order to attempt them in. This is the same katas grouped by the section they live in, for when the question is *what practice is there for `Option`?* rather than *what do I do next?*

<!-- by-subject:start -->

*Generated from the table above by `tools/check_katas.py --fix`. Sections in the order their first kata is reached.*

**[First programs](15_First_Programs/README.md)** — 11 katas

- K1 · [One file, three builds](15_First_Programs/rustc_without_cargo/README.md#practice)
- K2 · [Three misplaced doc comments](15_First_Programs/comments_that_compile/README.md#practice)
- K3 · [Fibonacci, and the width that runs out](15_First_Programs/values/README.md#practice)
- K4 · [The guard that stops protecting you](15_First_Programs/meet_the_bool/README.md#practice)
- K21 · [Four warnings, four different right answers](15_First_Programs/what_a_warning_is_asking/README.md#practice)
- K50 · [The error message nobody saw](15_First_Programs/debug_vs_display/README.md#practice)
- K51 · [The semicolon that changed the type](15_First_Programs/a_block_is_an_expression/README.md#practice)
- K52 · [Three places `()` shows up](15_First_Programs/the_unit_type/README.md#practice)
- K53 · [The f-string that isn't](15_First_Programs/braces_take_a_name/README.md#practice)
- K82 · [Both `dbg!` traps](15_First_Programs/what_dbg_does/README.md#practice)
- K84 · [Seed it, prove it, measure the modulo bias, then remove it](15_First_Programs/randomness/README.md#practice)

**[`Option` and `Result`](17_Option_and_Result/README.md)** — 23 katas

- K5 · [A favourite number that may not exist](17_Option_and_Result/some_and_none/README.md#practice)
- K6 · [The arm you deleted](17_Option_and_Result/if_let/README.md#practice)
- K7 · [The reason the caller could have used](17_Option_and_Result/option_vs_result/README.md#practice)
- K8 · [The default that was built for nothing](17_Option_and_Result/unwrap_or/README.md#practice)
- K9 · [Fall back, but keep the reason](17_Option_and_Result/unwrap_or_else/README.md#practice)
- K10 · [The type's zero is not your domain's zero](17_Option_and_Result/unwrap_or_default/README.md#practice)
- K11 · [Transform, or fall back](17_Option_and_Result/map_or/README.md#practice)
- K15 · [The same program with a type that is not `Copy`](17_Option_and_Result/shadowing_and_unwrap/README.md#practice)
- K22 · [What the panic left behind](17_Option_and_Result/what_a_panic_costs/README.md#practice)
- K23 · [Four sentences, one of them a hope](17_Option_and_Result/expect/README.md#practice)
- K25 · [The average of nothing](17_Option_and_Result/partial_functions/README.md#practice)
- K26 · [Four causes, one `None`](17_Option_and_Result/none_on_error/README.md#practice)
- K27 · [Declare it, then prove it](17_Option_and_Result/initial_values/README.md#practice)
- K28 · [Which fields may legitimately be missing?](17_Option_and_Result/option_fields/README.md#practice)
- K29 · [No `match` allowed](17_Option_and_Result/option_as_collection/README.md#practice)
- K30 · [A loop with no counter](17_Option_and_Result/while_let/README.md#practice)
- K31 · [A list that ends](17_Option_and_Result/nullable_pointers/README.md#practice)
- K32 · [One optional argument, four ways](17_Option_and_Result/optional_arguments/README.md#practice)
- K35 · [Expand the alias](17_Option_and_Result/result_aliases/README.md#practice)
- K36 · [Guard the input that has no answer](17_Option_and_Result/wrong_guard/README.md#practice)
- K39 · [The arm you didn't write](17_Option_and_Result/six_kinds_of_zero/README.md#practice)
- K55 · [The hour that changed its ad](17_Option_and_Result/one_arm_many_values/README.md#practice)
- K56 · [Three ways to make `Some(None)` compile](17_Option_and_Result/some_is_a_constructor/README.md#practice)

**[Ownership](18_Ownership/README.md)** — 15 katas

- K12 · [Follow the responsibility](18_Ownership/ownership_and_moves/README.md#practice)
- K13 · [Many readers, or one writer](18_Ownership/borrowing/README.md#practice)
- K14 · [Two places, or one?](18_Ownership/a_name_is_not_a_place/README.md#practice)
- K16 · [The value you can no longer free](18_Ownership/shadowing_does_not_drop/README.md#practice)
- K17 · [Three shadows, one of them earned](18_Ownership/when_to_shadow/README.md#practice)
- K18 · [The tally that never tallied](18_Ownership/nothing_checks_a_shadow/README.md#practice)
- K19 · [Time three things you cannot see](18_Ownership/scope_is_about_names/README.md#practice)
- K20 · [Six statements, and only two of them drop anything](18_Ownership/assignment_is_a_drop/README.md#practice)
- K86 · [Pay only when you have to](18_Ownership/clone_on_write/README.md#practice)
- K94 · [Predict the count four times, then find the edge that leaks](18_Ownership/reference_counting/README.md#practice)
- K99 · [Three refusals, three fixes, and the one you can delete](18_Ownership/sharing_across_threads/README.md#practice)
- K100 · [Predict seven sizes, then predict which of seven lines allocates](18_Ownership/stack_and_heap/README.md#practice)
- K125 · [Count the drops on four paths](18_Ownership/the_drop_flag/README.md#practice)
- K130 · [Three predictions about one call](18_Ownership/the_call_stack/README.md#practice)
- K131 · [Prove the reuse, then get the compiler to stop you exploiting it](18_Ownership/a_stack_slot_is_reused/README.md#practice)

**[Errors](02_Errors/README.md)** — 1 kata

- K24 · [Delete four unwraps](02_Errors/unwrap_is_a_todo/README.md#practice)

**[Structs](16_Structs/README.md)** — 8 katas

- K33 · [Make the invalid score unbuildable](16_Structs/newtype_score/README.md#practice)
- K34 · [The line you forgot](16_Structs/representing_a_record/README.md#practice)
- K57 · [Three flavors, and the two things the compiler keeps apart](16_Structs/what_a_struct_is/README.md#practice)
- K58 · [Pick the right receiver four times](16_Structs/impl_blocks/README.md#practice)
- K59 · [Predict which half of the base survives a `..base`](16_Structs/struct_update/README.md#practice)
- K60 · [One `E0382`, three fixes, and the `String` field that removes one of them](16_Structs/copy_vs_clone/README.md#practice)
- K81 · [Seven errors, five root causes, three edits](16_Structs/when_a_struct_refuses/README.md#practice)
- K83 · [Four spellings, four error codes](16_Structs/a_type_is_not_a_constructor/README.md#practice)

**[Advanced](09_Advanced/README.md)** — 11 katas

- K37 · [The `Result` the lock hands you](09_Advanced/mutex_poisoning/README.md#practice)
- K45 · [Spend the entitlement, not just the token](09_Advanced/one_account_one_review/README.md#practice)
- K46 · [The scale that stopped covering the election](09_Advanced/scaled_integers/README.md#practice)
- K47 · [The average that came out as a three-way tie](09_Advanced/i128_exactness/README.md#practice)
- K48 · [Build the count that always finishes](09_Advanced/compounding_weights/README.md#practice)
- K49 · [The audit that has to know when to stop](09_Advanced/interval_arithmetic/README.md#practice)
- K80 · [Build the same choice twice](09_Advanced/what_a_union_is/README.md#practice)
- K90 · [Predict, then count](09_Advanced/the_global_allocator/README.md#practice)
- K126 · [The same fan-out three ways](09_Advanced/spawning_a_thread/README.md#practice)
- K127 · [A three-stage pipeline, and the drop that ends it](09_Advanced/channels/README.md#practice)
- K128 · [The safe line the unsafe block depends on](09_Advanced/what_unsafe_turns_off/README.md#practice)

**[Tooling](05_Tooling/README.md)** — 2 katas

- K38 · [Credit a fourth knob honestly](05_Tooling/compile_times/README.md#practice)
- K54 · [The reformat that changed the program](05_Tooling/formatting/README.md#practice)

**[Numbers and bytes](19_Numbers/README.md)** — 6 katas

- K40 · [Eight candidates in one byte](19_Numbers/meet_the_byte/README.md#practice)
- K41 · [Two decisions, one literal](19_Numbers/writing_a_number_down/README.md#practice)
- K42 · [The fingerprint that collided](19_Numbers/why_hexadecimal/README.md#practice)
- K43 · [A tic-tac-toe game in 18 bits](19_Numbers/bit_flags/README.md#practice)
- K44 · [The results table that would not sort](19_Numbers/what_a_float_stores/README.md#practice)
- K87 · [Make the sum land on the number you would have typed](19_Numbers/letting_the_compiler_reorder/README.md#practice)

**[Strings](14_Strings/README.md)** — 21 katas

- K61 · [One `&str` parameter, three callers](14_Strings/string_vs_str/README.md#practice)
- K62 · [Cut a name in half without panicking](14_Strings/string_slices/README.md#practice)
- K63 · [Predict `len` and `capacity` through five pushes](14_Strings/anatomy_of_a_string/README.md#practice)
- K64 · [Implement `Display` once and collect four abilities](14_Strings/making_a_string/README.md#practice)
- K65 · [One greeting three ways](14_Strings/concatenating_strings/README.md#practice)
- K66 · [One line built four ways](14_Strings/building_a_string/README.md#practice)
- K67 · [One name, three lengths](14_Strings/meet_the_char/README.md#practice)
- K68 · [An empty field is data](14_Strings/walking_a_string/README.md#practice)
- K69 · [Return a label three ways](14_Strings/static_str/README.md#practice)
- K70 · [Three arrivals, three types](14_Strings/six_kinds_of_string/README.md#practice)
- K71 · [Pivot both ways, then earn `E0106` and `E0515`](14_Strings/string_vs_str/README.md#practice)
- K72 · [Slice up to a boundary, then straight through one](14_Strings/string_slices/README.md#practice)
- K73 · [Four arrivals and one pre-payment](14_Strings/making_a_string/README.md#practice)
- K74 · [Five edits, one buffer](14_Strings/building_a_string/README.md#practice)
- K75 · [Run-length encoding both ways](14_Strings/building_a_string/README.md#practice)
- K76 · [Case, whitespace, and `STARVoting`](14_Strings/meet_the_char/README.md#practice)
- K77 · [The third ruler](14_Strings/meet_the_char/README.md#practice)
- K78 · [Two rulers over one string](14_Strings/walking_a_string/README.md#practice)
- K79 · [Six searches, and the two that want a regex](14_Strings/walking_a_string/README.md#practice)
- K88 · [Let the source pick the spelling](14_Strings/making_a_string/README.md#practice)
- K132 · [Freeze a candidate column three ways](14_Strings/boxed_str/README.md#practice)

**[Enums](13_Enums/README.md)** — 1 kata

- K85 · [Which defence catches a silent catch-all](13_Enums/a_typo_becomes_a_binding/README.md#practice)

**[Traits](12_Traits/README.md)** — 5 katas

- K89 · [Predict the owned twin before you run it](12_Traits/to_owned/README.md#practice)
- K91 · [Four loops that all look like reuse](12_Traits/clone_into/README.md#practice)
- K123 · [Four impls for one `*`, and an operator that should not exist](12_Traits/operators_are_traits/README.md#practice)
- K124 · [Three drop orders, and the guard released one line early](12_Traits/drop_and_raii/README.md#practice)
- K129 · [Three calls, three causes](12_Traits/no_method_named/README.md#practice)

**[Generics](22_Generics/README.md)** — 2 katas

- K92 · [A container with two holes](22_Generics/what_a_generic_is/README.md#practice)
- K93 · [Walk a linked list without recursion](22_Generics/a_generic_recursive_type/README.md#practice)

**[Iterators](24_Iterators/README.md)** — 4 katas

- K95 · [Predict the closure-call count, then count it](24_Iterators/iterators_are_lazy/README.md#practice)
- K96 · [The `Vec` that could not outlive its string](24_Iterators/collect_into_a_vec/README.md#practice)
- K97 · [Six questions, one `Vec`](24_Iterators/collect_into_a_vec/README.md#practice)
- K98 · [Five references where you meant five numbers](24_Iterators/double_ended_and_exact_size/README.md#practice)

**[Collections](26_Collections/README.md)** — 10 katas

- K101 · [Four fields, and the transposition that compiles](26_Collections/tuples/README.md#practice)
- K102 · [One function, four callers](26_Collections/arrays_and_slices/README.md#practice)
- K103 · [The call that printed its own receipt](26_Collections/arrays_and_slices/README.md#practice)
- K104 · [Count the reallocations, then delete them](26_Collections/the_vec/README.md#practice)
- K105 · [Two lengths in one election](26_Collections/array_or_vec/README.md#practice)
- K106 · [Build the same grid twice and send it the bill](26_Collections/vec_of_vecs/README.md#practice)
- K107 · [Four ways to count, and the two that are wrong](26_Collections/the_hashmap/README.md#practice)
- K108 · [Who voted twice, who never voted](26_Collections/the_hashset/README.md#practice)
- K109 · [Two orders from one tally, and the key you cannot use](26_Collections/sorted_collections/README.md#practice)
- K110 · [Two walks and a drop order](26_Collections/the_box/README.md#practice)

**[Conversion](29_Conversion/README.md)** — 4 katas

- K111 · [Three conversions, one the compiler forbids](29_Conversion/from_and_into/README.md#practice)
- K112 · [A ballot line parsed twice](29_Conversion/tryfrom_and_tryinto/README.md#practice)
- K113 · [Four silent losses](29_Conversion/casting_with_as/README.md#practice)
- K114 · [Six call sites, three repairs](29_Conversion/coercion/README.md#practice)

**[Modules](27_Modules/README.md)** — 5 katas

- K115 · [The door your own module left open](27_Modules/modules_and_visibility/README.md#practice)
- K116 · [Two traits called `Write`, and a glob that shadows](27_Modules/the_use_declaration/README.md#practice)
- K117 · [Five paths to one function](27_Modules/one_module_per_file/README.md#practice)
- K118 · [The address you may not rely on](27_Modules/const_and_static/README.md#practice)
- K119 · [The derive that changed behaviour when a field moved](27_Modules/what_an_attribute_is/README.md#practice)

**[Testing](28_Testing/README.md)** — 3 katas

- K120 · [Five assertions, two of which cannot fail](28_Testing/what_a_test_asserts/README.md#practice)
- K121 · [The test that could not see it](28_Testing/where_a_test_goes/README.md#practice)
- K122 · [The example that documents half the sentence](28_Testing/doc_tests/README.md#practice)

<!-- by-subject:end -->

## Adding one

Three things, in this order:

1. **Write the prompt on the topic page**, under `## Practice`. State the task, not the method — and say which mistake is worth making on purpose first, because on most of these pages the compiler error *is* the lesson.
2. **Make the solution a real example.** Put it in that lesson's `examples/` folder as `<topic>_kata.rs`, record its key, and pull both the code and its output into the `<details>` block with generated markers:

   ```markdown
   <!-- source:some_and_none_kata -->
   <!-- /source -->

   <!-- output:some_and_none_kata -->
   <!-- /output -->
   ```

   Neither is hand-typed, so a solution cannot quietly stop compiling — which is the one failure a practice page must never ship.
3. **Add a row above**, at the position in the sequence where it should be attempted — *inside* the table, not merely above `## Adding one`. Two paragraphs of prose sit between the last row and that heading, and a row dropped after them renders as a second one-row table with no header. `check_katas.py` does not catch it: the row parses, its links resolve, and the numbering still reads in order, so the gate passes on a page that looks broken.

The mechanics of the markers, and the rest of the house conventions, are in [CONTRIBUTING.md](CONTRIBUTING.md).
