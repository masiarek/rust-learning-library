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
| K1 | [One file, three builds — plain, `--test` and `-O`, each predicted before you run it](01_Foundations/rustc_without_cargo/README.md#practice) | [Running a scratch program](01_Foundations/rustc_without_cargo/README.md) | 101 |
| K2 | [A favourite number that may not exist — `Some` / `None`, one `match`, and `unwrap_or`](01_Foundations/some_and_none/README.md#practice) | [`Some` and `None`](01_Foundations/some_and_none/README.md) | 101 |
| K3 | [The arm you deleted — add a variant, and watch only one of the two forms come and find you](01_Foundations/if_let/README.md#practice) | [`if let`](01_Foundations/if_let/README.md) | 101 |
| K4 | [The reason the caller could have used — the same four bad cells, once as `None` and once as a named error](01_Foundations/option_vs_result/README.md#practice) | [`Option` vs `Result`](01_Foundations/option_vs_result/README.md) | 101 |
| K5 | [The default that was built for nothing — watch an eager fallback run on the happy path](01_Foundations/unwrap_or/README.md#practice) | [`unwrap_or`](01_Foundations/unwrap_or/README.md) | 201 |
| K6 | [Fall back, but keep the reason — the one job only the `_else` closure can do](01_Foundations/unwrap_or_else/README.md#practice) | [`unwrap_or_else`](01_Foundations/unwrap_or_else/README.md) | 201 |
| K7 | [The type's zero is not your domain's zero — a blank ballot defaults into a real-looking one](01_Foundations/unwrap_or_default/README.md#practice) | [`unwrap_or_default`](01_Foundations/unwrap_or_default/README.md) | 201 |
| K8 | [Transform, or fall back — a default written first and run last](01_Foundations/map_or/README.md#practice) | [`map_or`](01_Foundations/map_or/README.md) | 201 |
| K9 | [Follow the responsibility — a `Drop` that prints, and the function where the free actually happened](01_Foundations/ownership_and_moves/README.md#practice) | [Ownership and moves](01_Foundations/ownership_and_moves/README.md) | 101 |
| K10 | [Many readers, or one writer — then move one `println!` and read `E0502`](01_Foundations/borrowing/README.md#practice) | [Borrowing](01_Foundations/borrowing/README.md) | 101 → 201 |
| K11 | [The same program with a type that is not `Copy` — read `E0382`, then fix it four ways and pick one](01_Foundations/shadowing_and_unwrap/README.md#practice) | [Shadowing and `unwrap`](01_Foundations/shadowing_and_unwrap/README.md) | 201 |
| K12 | [What the panic left behind — read the damage an `unwrap` does mid-job, then make the missing row a return value](01_Foundations/what_a_panic_costs/README.md#practice) | [What a panic costs](01_Foundations/what_a_panic_costs/README.md) | 201 |
| K13 | [The average of nothing — make a partial function total, and say what your `None` means](01_Foundations/partial_functions/README.md#practice) | [Partial functions](01_Foundations/partial_functions/README.md) | 201 |
| K14 | [Four causes, one `None` — write the operator's error message from a signature that discarded it](01_Foundations/none_on_error/README.md#practice) | [Returning `None` on error](01_Foundations/none_on_error/README.md) | 201 |
| K15 | [Declare it, then prove it — a value decided in three branches, with no `Option` and no `mut`](01_Foundations/initial_values/README.md#practice) | [Initial values](01_Foundations/initial_values/README.md) | 201 |
| K16 | [Which fields may legitimately be missing? — and telling *no ballot* apart from *an empty one*](01_Foundations/option_fields/README.md#practice) | [`Option` fields](01_Foundations/option_fields/README.md) | 101 |
| K17 | [No `match` allowed — count, total and average the ballots that exist with iterator methods only](01_Foundations/option_as_collection/README.md#practice) | [`Option` is a one-item collection](01_Foundations/option_as_collection/README.md) | 201 |
| K18 | [A loop with no counter — the body that peeks where it meant to advance](01_Foundations/while_let/README.md#practice) | [`while let`](01_Foundations/while_let/README.md) | 201 |
| K19 | [A list that ends — `Option<Box<Node>>`, and the `size_of` proof that it costs nothing](01_Foundations/nullable_pointers/README.md#practice) | [Nullable pointers](01_Foundations/nullable_pointers/README.md) | 201 |
| K20 | [One optional argument, four ways — and the signature that rejects half your callers](01_Foundations/optional_arguments/README.md#practice) | [Optional function arguments](01_Foundations/optional_arguments/README.md) | 201 |
| K21 | [Make the invalid score unbuildable — then find the door your own module still leaves open](01_Foundations/newtype_score/README.md#practice) | [A score is not a number](01_Foundations/newtype_score/README.md) | 101 → 201 |
| K22 | [The line you forgot — desync two parallel `Vec`s and get a plausible wrong answer](01_Foundations/representing_a_ballot/README.md#practice) | [What is a ballot, in memory?](01_Foundations/representing_a_ballot/README.md) | 201 |
| K23 | [Expand the alias — follow a one-parameter `Result` back to the list of things that can go wrong](01_Foundations/result_aliases/README.md#practice) | [The `Result` you are reading is probably an alias](01_Foundations/result_aliases/README.md) | 201 |
| K24 | [Guard the input that has no answer — find the case the careful-looking guard lets through](01_Foundations/wrong_guard/README.md#practice) | [Zero wins is not zero games](01_Foundations/wrong_guard/README.md) | 201 |
| K25 | [The `Result` the lock hands you — a thread dies mid-update, and you answer it three ways](09_Advanced/mutex_poisoning/README.md#practice) | [Lock poisoning](09_Advanced/mutex_poisoning/README.md) | 301 |
| K26 | [Credit a fourth knob honestly — the same linker swap, first and last in the ladder](05_Tooling/compile_times/README.md#practice) | [Compile times](05_Tooling/compile_times/README.md) | 201 |
| K27 | [The arm you didn't write — a catch-all quietly refiles two spoiled ballots as blanks](01_Foundations/six_kinds_of_zero/README.md#practice) | [Six kinds of zero](01_Foundations/six_kinds_of_zero/README.md) | 201 |
| K28 | [Eight candidates in one byte — pack an approval ballot into a `u8` with bit operations, then let a ninth sign up](01_Foundations/meet_the_byte/README.md#practice) | [Meet the byte](01_Foundations/meet_the_byte/README.md) | 101 → 201 |
| K29 | [The fingerprint that collided — one missing `0` turns two different ballot files into the same hex string, across 3,600 of the 65,536 two-byte cases](01_Foundations/why_hexadecimal/README.md#practice) | [Why hexadecimal](01_Foundations/why_hexadecimal/README.md) | 101 → 201 |
| K30 | [The results table that would not sort — meet `E0277` on purpose, then rank the same table three ways and pick the one you would ship](01_Foundations/what_a_float_stores/README.md#practice) | [What a float actually stores](01_Foundations/what_a_float_stores/README.md) | 201 |
| K31 | [Spend the entitlement, not just the token — close the sign-in hole, then count what the fix costs](09_Advanced/one_person_one_vote/README.md#practice) | [The right to vote is a value](09_Advanced/one_person_one_vote/README.md) | 301 |
| K32 | [The scale that stopped covering the election — hard-code a denominator, grow the election past it, and find the bug that changes no winner](09_Advanced/scaled_integers/README.md#practice) | [Scale the denominator away](09_Advanced/scaled_integers/README.md) | 301 |
| K33 | [The average that came out as a three-way tie — collapse three candidates onto one number, then rank them without ever dividing](09_Advanced/i128_exactness/README.md#practice) | [What `i128` is exact about](09_Advanced/i128_exactness/README.md) | 301 |
| K34 | [Build the count that always finishes — then find the coarsest rounding that still reproduces the exact winners](09_Advanced/compounding_weights/README.md#practice) | [When the denominators compound](09_Advanced/compounding_weights/README.md) | 301 |
| K35 | [The audit that has to know when to stop — five wards, one rounded count each, and the escalation loop that never returns on a tie](09_Advanced/interval_arithmetic/README.md#practice) | [Did the rounding decide it?](09_Advanced/interval_arithmetic/README.md) | 301 |
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
