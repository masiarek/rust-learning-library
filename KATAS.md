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
| K3 | [A favourite number that may not exist — `Some` / `None`, one `match`, and `unwrap_or`](17_Option_and_Result/some_and_none/README.md#practice) | [`Some` and `None`](17_Option_and_Result/some_and_none/README.md) | 101 |
| K4 | [The arm you deleted — add a variant, and watch only one of the two forms come and find you](17_Option_and_Result/if_let/README.md#practice) | [`if let`](17_Option_and_Result/if_let/README.md) | 101 |
| K5 | [The reason the caller could have used — the same four bad cells, once as `None` and once as a named error](17_Option_and_Result/option_vs_result/README.md#practice) | [`Option` vs `Result`](17_Option_and_Result/option_vs_result/README.md) | 101 |
| K6 | [The default that was built for nothing — watch an eager fallback run on the happy path](17_Option_and_Result/unwrap_or/README.md#practice) | [`unwrap_or`](17_Option_and_Result/unwrap_or/README.md) | 201 |
| K7 | [Fall back, but keep the reason — the one job only the `_else` closure can do](17_Option_and_Result/unwrap_or_else/README.md#practice) | [`unwrap_or_else`](17_Option_and_Result/unwrap_or_else/README.md) | 201 |
| K8 | [The type's zero is not your domain's zero — a blank ballot defaults into a real-looking one](17_Option_and_Result/unwrap_or_default/README.md#practice) | [`unwrap_or_default`](17_Option_and_Result/unwrap_or_default/README.md) | 201 |
| K9 | [Transform, or fall back — a default written first and run last](17_Option_and_Result/map_or/README.md#practice) | [`map_or`](17_Option_and_Result/map_or/README.md) | 201 |
| K10 | [Follow the responsibility — a `Drop` that prints, and the function where the free actually happened](18_Ownership/ownership_and_moves/README.md#practice) | [Ownership and moves](18_Ownership/ownership_and_moves/README.md) | 101 |
| K11 | [Many readers, or one writer — then move one `println!` and read `E0502`](18_Ownership/borrowing/README.md#practice) | [Borrowing](18_Ownership/borrowing/README.md) | 101 → 201 |
| K12 | [Two places, or one? — compile the four-line reference test both ways, time the drops, read the optimizer's answer, then grade the three-option quiz that rejects the right line for the wrong reason](18_Ownership/a_name_is_not_a_place/README.md#practice) | [A name is not a place](18_Ownership/a_name_is_not_a_place/README.md) | 201 |
| K13 | [The same program with a type that is not `Copy` — read `E0382`, then fix it four ways and pick one](17_Option_and_Result/shadowing_and_unwrap/README.md#practice) | [Shadowing and `unwrap`](17_Option_and_Result/shadowing_and_unwrap/README.md) | 201 |
| K14 | [The value you can no longer free — shadow a buffer, watch it outlive the work, then fix it three ways](18_Ownership/shadowing_does_not_drop/README.md#practice) | [A shadow does not drop](18_Ownership/shadowing_does_not_drop/README.md) | 201 |
| K15 | [Three shadows, one of them earned — predict two wrong numbers, then fix one by deleting a `let` and the other by renaming](18_Ownership/when_to_shadow/README.md#practice) | [When to shadow](18_Ownership/when_to_shadow/README.md) | 201 |
| K16 | [The tally that never tallied — a shadowed accumulator logs three plausible running totals and names a candidate who scored zero](18_Ownership/nothing_checks_a_shadow/README.md#practice) | [Nothing checks a shadow](18_Ownership/nothing_checks_a_shadow/README.md) | 201 |
| K17 | [Time three things you cannot see — a report that closes before it writes, a guard released one identifier early, and a borrow ended three ways](18_Ownership/scope_is_about_names/README.md#practice) | [Scope is about names, not values](18_Ownership/scope_is_about_names/README.md) | 201 |
| K18 | [Four warnings, four different right answers — only one of them is an underscore, and one is still broken if you pick the wrong kind](15_First_Programs/what_a_warning_is_asking/README.md#practice) | [What a warning is asking](15_First_Programs/what_a_warning_is_asking/README.md) | 101 → 201 |
| K19 | [What the panic left behind — read the damage an `unwrap` does mid-job, then make the missing row a return value](17_Option_and_Result/what_a_panic_costs/README.md#practice) | [What a panic costs](17_Option_and_Result/what_a_panic_costs/README.md) | 201 |
| K20 | [Four sentences, one of them a hope — name the guarantor for each `expect`, then watch the one that has none die on a misspelled key](17_Option_and_Result/expect/README.md#practice) | [`expect`: writing down the proof](17_Option_and_Result/expect/README.md) | 201 |
| K21 | [Delete four unwraps — rewrite a README-shaped config parser so no line can abort, using a different technique for each](02_Errors/unwrap_is_a_todo/README.md#practice) | [`unwrap` is a TODO you forgot to remove](02_Errors/unwrap_is_a_todo/README.md) | 201 |
| K22 | [The average of nothing — make a partial function total, and say what your `None` means](17_Option_and_Result/partial_functions/README.md#practice) | [Partial functions](17_Option_and_Result/partial_functions/README.md) | 201 |
| K23 | [Four causes, one `None` — write the operator's error message from a signature that discarded it](17_Option_and_Result/none_on_error/README.md#practice) | [Returning `None` on error](17_Option_and_Result/none_on_error/README.md) | 201 |
| K24 | [Declare it, then prove it — a value decided in three branches, with no `Option` and no `mut`](17_Option_and_Result/initial_values/README.md#practice) | [Initial values](17_Option_and_Result/initial_values/README.md) | 201 |
| K25 | [Which fields may legitimately be missing? — and telling *no ballot* apart from *an empty one*](17_Option_and_Result/option_fields/README.md#practice) | [`Option` fields](17_Option_and_Result/option_fields/README.md) | 101 |
| K26 | [No `match` allowed — count, total and average the ballots that exist with iterator methods only](17_Option_and_Result/option_as_collection/README.md#practice) | [`Option` is a one-item collection](17_Option_and_Result/option_as_collection/README.md) | 201 |
| K27 | [A loop with no counter — the body that peeks where it meant to advance](17_Option_and_Result/while_let/README.md#practice) | [`while let`](17_Option_and_Result/while_let/README.md) | 201 |
| K28 | [A list that ends — `Option<Box<Node>>`, and the `size_of` proof that it costs nothing](17_Option_and_Result/nullable_pointers/README.md#practice) | [Nullable pointers](17_Option_and_Result/nullable_pointers/README.md) | 201 |
| K29 | [One optional argument, four ways — and the signature that rejects half your callers](17_Option_and_Result/optional_arguments/README.md#practice) | [Optional function arguments](17_Option_and_Result/optional_arguments/README.md) | 201 |
| K30 | [Make the invalid score unbuildable — then find the door your own module still leaves open](16_Structs/newtype_score/README.md#practice) | [A score is not a number](16_Structs/newtype_score/README.md) | 101 → 201 |
| K31 | [The line you forgot — desync two parallel `Vec`s and get a plausible wrong answer](16_Structs/representing_a_ballot/README.md#practice) | [What is a ballot, in memory?](16_Structs/representing_a_ballot/README.md) | 201 |
| K32 | [Expand the alias — follow a one-parameter `Result` back to the list of things that can go wrong](17_Option_and_Result/result_aliases/README.md#practice) | [The `Result` you are reading is probably an alias](17_Option_and_Result/result_aliases/README.md) | 201 |
| K33 | [Guard the input that has no answer — find the case the careful-looking guard lets through](17_Option_and_Result/wrong_guard/README.md#practice) | [Zero wins is not zero games](17_Option_and_Result/wrong_guard/README.md) | 201 |
| K34 | [The `Result` the lock hands you — a thread dies mid-update, and you answer it three ways](09_Advanced/mutex_poisoning/README.md#practice) | [Lock poisoning](09_Advanced/mutex_poisoning/README.md) | 301 |
| K35 | [Credit a fourth knob honestly — the same linker swap, first and last in the ladder](05_Tooling/compile_times/README.md#practice) | [Compile times](05_Tooling/compile_times/README.md) | 201 |
| K36 | [The arm you didn't write — a catch-all quietly refiles two spoiled ballots as blanks](17_Option_and_Result/six_kinds_of_zero/README.md#practice) | [Six kinds of zero](17_Option_and_Result/six_kinds_of_zero/README.md) | 201 |
| K37 | [Eight candidates in one byte — pack an approval ballot into a `u8` with bit operations, then let a ninth sign up](19_Numbers/meet_the_byte/README.md#practice) | [Meet the byte](19_Numbers/meet_the_byte/README.md) | 101 → 201 |
| K38 | [The fingerprint that collided — one missing `0` turns two different ballot files into the same hex string, across 3,600 of the 65,536 two-byte cases](19_Numbers/why_hexadecimal/README.md#practice) | [Why hexadecimal](19_Numbers/why_hexadecimal/README.md) | 101 → 201 |
| K39 | [A tic-tac-toe game in 18 bits — two nine-bit fields in one `u32`, eight win masks, then a fourth field arrives on top](19_Numbers/bit_flags/README.md#practice) | [Bit flags](19_Numbers/bit_flags/README.md) | 201 |
| K40 | [The results table that would not sort — meet `E0277` on purpose, then rank the same table three ways and pick the one you would ship](19_Numbers/what_a_float_stores/README.md#practice) | [What a float actually stores](19_Numbers/what_a_float_stores/README.md) | 201 |
| K41 | [Spend the entitlement, not just the token — close the sign-in hole, then count what the fix costs](09_Advanced/one_person_one_vote/README.md#practice) | [The right to vote is a value](09_Advanced/one_person_one_vote/README.md) | 301 |
| K42 | [The scale that stopped covering the election — hard-code a denominator, grow the election past it, and find the bug that changes no winner](09_Advanced/scaled_integers/README.md#practice) | [Scale the denominator away](09_Advanced/scaled_integers/README.md) | 301 |
| K43 | [The average that came out as a three-way tie — collapse three candidates onto one number, then rank them without ever dividing](09_Advanced/i128_exactness/README.md#practice) | [What `i128` is exact about](09_Advanced/i128_exactness/README.md) | 301 |
| K44 | [Build the count that always finishes — then find the coarsest rounding that still reproduces the exact winners](09_Advanced/compounding_weights/README.md#practice) | [When the denominators compound](09_Advanced/compounding_weights/README.md) | 301 |
| K45 | [The audit that has to know when to stop — five wards, one rounded count each, and the escalation loop that never returns on a tie](09_Advanced/interval_arithmetic/README.md#practice) | [Did the rounding decide it?](09_Advanced/interval_arithmetic/README.md) | 301 |
| K46 | [The error message nobody saw — count the five ways your error can reach a person, then fix it twice and price both fixes](15_First_Programs/debug_vs_display/README.md#practice) | [Debug and Display](15_First_Programs/debug_vs_display/README.md) | 101 → 201 |
| K47 | [The semicolon that changed the type — cause `E0308` on purpose, then seal a `mut` builder behind braces and make an `if` be the value](15_First_Programs/a_block_is_an_expression/README.md#practice) | [A block is an expression](15_First_Programs/a_block_is_an_expression/README.md) | 101 → 201 |
| K48 | [The f-string that isn't — four braces, four refusals, then fix the line three ways and defend the one you ship](15_First_Programs/braces_take_a_name/README.md#practice) | [The braces take a name](15_First_Programs/braces_take_a_name/README.md) | 101 → 201 |
| K49 | [The reformat that changed the program — find the one place a whitespace edit is not cosmetic, and watch the formatter decline to help](05_Tooling/formatting/README.md#practice) | [Formatting](05_Tooling/formatting/README.md) | 101 → 201 |
| K50 | [The hour that changed its ad — collapse twenty-six match arms into five, then break it three ways and predict which two the compiler catches](17_Option_and_Result/one_arm_many_values/README.md#practice) | [One arm, many values](17_Option_and_Result/one_arm_many_values/README.md) | 101 → 201 |
| K51 | [Three ways to make `Some(None)` compile — read `E0308` in full, then fix it by deleting, by supplying, and by widening the field, and defend the one you would ship](17_Option_and_Result/some_is_a_constructor/README.md#practice) | [`Some` is a constructor, not a flag](17_Option_and_Result/some_is_a_constructor/README.md) | 101 → 201 |
| K52 | [Three flavors, and the two things the compiler keeps apart — identical tuple structs that will not substitute, the private field that privatises a constructor, and a unit struct whose only content is behaviour](16_Structs/what_a_struct_is/README.md#practice) | [What a struct is](16_Structs/what_a_struct_is/README.md) | 101 → 201 |
| K53 | [Pick the right receiver four times — then call a `&mut self` method through a non-`mut` binding, and use a value after a method took `self`](16_Structs/impl_blocks/README.md#practice) | [`impl` blocks](16_Structs/impl_blocks/README.md) | 101 → 201 |
| K54 | [Predict which half of the base survives a `..base` — four fields, two of them still readable, and the trailing comma that is its own error](16_Structs/struct_update/README.md#practice) | [Struct update syntax](16_Structs/struct_update/README.md) | 101 → 201 |
| K55 | [One `E0382`, three fixes, and the `String` field that removes one of them — then rank what each costs the *caller*](16_Structs/copy_vs_clone/README.md#practice) | [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) | 101 → 201 |
| K56 | [One `&str` parameter, three callers — then flip it to `String` and catalogue what every call site now pays](14_Strings/string_vs_str/README.md#practice) | [`String` vs `&str`](14_Strings/string_vs_str/README.md) | 101 → 201 |
| K57 | [Cut a name in half without panicking — `len()/2` on four names, and the two ways to find a legal boundary](14_Strings/string_slices/README.md#practice) | [String slices](14_Strings/string_slices/README.md) | 101 → 201 |
| K58 | [Predict `len` and `capacity` through five pushes — which ones reallocate? — then make one up-front allocation serve all of them](14_Strings/anatomy_of_a_string/README.md#practice) | [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md) | 101 → 201 |
| K59 | [Implement `Display` once and collect four abilities — then add `impl ToString` and read the `E0119`](14_Strings/making_a_string/README.md#practice) | [Making a `String`](14_Strings/making_a_string/README.md) | 101 → 201 |
| K60 | [One greeting three ways — then go back and earn all three refusals on purpose (`E0369`, `E0308`, `E0368`)](14_Strings/concatenating_strings/README.md#practice) | [Concatenating strings](14_Strings/concatenating_strings/README.md) | 101 → 201 |
| K61 | [One line built four ways — which inputs survive, how many buffers, and which one is wrong inside a loop](14_Strings/building_a_string/README.md#practice) | [Building a `String`](14_Strings/building_a_string/README.md) | 101 → 201 |
| K62 | [One name, three lengths — a per-`char` inventory, and the combining accent that makes two identical-looking strings unequal](14_Strings/meet_the_char/README.md#practice) | [Meet the `char`](14_Strings/meet_the_char/README.md) | 101 → 201 |
| K63 | [An empty field is data — parse `"5,,0"` into abstentions, then watch `split_whitespace()` shorten the row](14_Strings/walking_a_string/README.md#practice) | [Walking a `String`](14_Strings/walking_a_string/README.md) | 101 → 201 |
| K64 | [Return a label three ways — read the `E0515`, then match, leak, or own it, and say which one leaks in a loop](14_Strings/static_str/README.md#practice) | [`&'static str`](14_Strings/static_str/README.md) | 201 |
| K65 | [Three arrivals, three types — then break the UTF-8 promise and the NUL promise on purpose, and read both refusals](14_Strings/six_kinds_of_string/README.md#practice) | [Six kinds of string](14_Strings/six_kinds_of_string/README.md) | 201 |
| K66 | [Build the same choice twice — a hand-maintained tag beside a union, then an `enum`, and the desync only one of them can have](09_Advanced/what_a_union_is/README.md#practice) | [What a union is](09_Advanced/what_a_union_is/README.md) | 301 |
| K67 | [Seven errors, five root causes, three edits — group them by cause before changing a line](16_Structs/when_a_struct_refuses/README.md#practice) | [When a struct refuses](16_Structs/when_a_struct_refuses/README.md) | 101 → 201 |
| K68 | [Both `dbg!` traps — the alternate flag a hand-written `Debug` ignores, and the move that costs you the value](15_First_Programs/what_dbg_does/README.md#practice) | [What `dbg!` does](15_First_Programs/what_dbg_does/README.md) | 101 → 201 |
| K69 | [Four spellings, four error codes — and the one that compiles into the wrong thing](16_Structs/a_type_is_not_a_constructor/README.md#practice) | [A type is not a constructor](16_Structs/a_type_is_not_a_constructor/README.md) | 101 → 201 |
| K70 | [Seed it, prove it, measure the modulo bias, then remove it — counted over the whole output space, never sampled](15_First_Programs/randomness/README.md#practice) | [Randomness](15_First_Programs/randomness/README.md) | 101 → 201 |
| K71 | [Which defence catches a silent catch-all — two lints denied, a variant added, and only one of the three edits does anything](13_Enums/a_typo_becomes_a_binding/README.md#practice) | [A typo becomes a binding](13_Enums/a_typo_becomes_a_binding/README.md) | 201 |
The numbers are labels, and they live **only in this table** — a kata's own page does not print its number, so moving one costs a single line here and nothing else. Reorder freely; the order is the order to attempt them in, not the order they were written.

Both columns are links, and they go to different places: the kata title opens the exercise itself, the lesson name opens the page it lives on. Every lesson in [`01_Foundations/`](01_Foundations/README.md), [`05_Tooling/`](05_Tooling/README.md) and [`09_Advanced/`](09_Advanced/README.md) now has one.

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
3. **Add a row above**, at the position in the sequence where it should be attempted.

The mechanics of the markers, and the rest of the house conventions, are in [CONTRIBUTING.md](CONTRIBUTING.md).
