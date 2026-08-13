# Katas

**Level:** reference · the practice track

**One line:** The lessons explain; the katas make you type. This page is the only place the katas are ordered — each kata itself lives on the page for the topic it teaches.

---

## Where a kata lives

**A kata belongs to its topic, not to a folder of its own.** The page that explains `Option` is the page that asks you to write one, under a `## Practice` heading near the end, with the solution folded away in a `<details>` block.

That is a deliberate choice, and the reason is that folders are URLs. A topic — `Option`, `if let`, borrowing — is stable for years; a *sequence* is not. The moment you write a kata that belongs between K1 and K2, a `K01_…/` folder either gets renumbered (breaking every link anyone saved) or starts lying about its own order. So the sequence lives here, in a table that costs nothing to reorder, and the numbers below are labels rather than addresses. It is the same rule the sidebar follows: [order is presentation](CONTRIBUTING.md), so it belongs in a page, never in a path.

The happy side effect is that a kata arrives with its explanation already written, and a topic can collect several katas over time without any of them needing to restate the background.

## The katas

| # | Kata | Where it lives | Level |
|---|---|---|---|
| K1 | A favourite number that may not exist — `Some` / `None`, one `match`, and `unwrap_or` | [`Some` and `None`](01_Foundations/some_and_none/README.md#practice) | 101 |
| K2 | The reason the caller could have used — the same four bad cells, once as `None` and once as a named error | [`Option` vs `Result`](01_Foundations/option_vs_result/README.md#practice) | 101 |
| K3 | The default that was built for nothing — watch an eager fallback run on the happy path | [`unwrap_or`](01_Foundations/unwrap_or/README.md#practice) | 101 |
| K4 | Fall back, but keep the reason — the one job only the `_else` closure can do | [`unwrap_or_else`](01_Foundations/unwrap_or_else/README.md#practice) | 101 |
| K5 | The type's zero is not your domain's zero — a blank ballot defaults into a real-looking one | [`unwrap_or_default`](01_Foundations/unwrap_or_default/README.md#practice) | 101 |
| K6 | Transform, or fall back — a default written first and run last | [`map_or`](01_Foundations/map_or/README.md#practice) | 201 |
| K7 | The same program with a type that is not `Copy` — read `E0382`, then fix it four ways and pick one | [Shadowing and `unwrap`](01_Foundations/shadowing_and_unwrap/README.md#practice) | 201 |
| K8 | What the panic left behind — read the damage an `unwrap` does mid-job, then make the missing row a return value | [What a panic costs](01_Foundations/what_a_panic_costs/README.md#practice) | 201 |
| K9 | The average of nothing — make a partial function total, and say what your `None` means | [Partial functions](01_Foundations/partial_functions/README.md#practice) | 201 |
| K10 | Four causes, one `None` — write the operator's error message from a signature that discarded it | [Returning `None` on error](01_Foundations/none_on_error/README.md#practice) | 201 |
| K11 | Declare it, then prove it — a value decided in three branches, with no `Option` and no `mut` | [Initial values](01_Foundations/initial_values/README.md#practice) | 201 |
| K12 | Which fields may legitimately be missing? — and telling *no ballot* apart from *an empty one* | [`Option` fields](01_Foundations/option_fields/README.md#practice) | 101 |
| K13 | No `match` allowed — count, total and average the ballots that exist with iterator methods only | [`Option` is a one-item collection](01_Foundations/option_as_collection/README.md#practice) | 201 |
| K14 | A list that ends — `Option<Box<Node>>`, and the `size_of` proof that it costs nothing | [Nullable pointers](01_Foundations/nullable_pointers/README.md#practice) | 201 |
| K15 | One optional argument, four ways — and the signature that rejects half your callers | [Optional function arguments](01_Foundations/optional_arguments/README.md#practice) | 201 |
The numbers are labels, and they live **only in this table** — a kata's own page does not print its number, so moving one costs a single line here and nothing else. Reorder freely; the order is the order to attempt them in, not the order they were written.

More land here as they are written. Every lesson in [`01_Foundations/`](01_Foundations/README.md) is a candidate; a lesson with no kata yet is not a gap in the library so much as an invitation.

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
