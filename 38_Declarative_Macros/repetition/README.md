# Repetition

**Level:** 301 · deep dive

**One line:** `$( $x:expr ),*` matches a comma-separated list and `$( … )*` on the right writes one copy per item — and every metavariable has to sit under exactly as many repetitions in the transcriber as it did in the matcher.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The three operators: `*` for any number, `+` for at least one, `?` for at most one — and why `?` takes no separator: `$( $i:ident ),?` is refused with *the `?` macro repetition operator does not take a separator*
- Separators: `,` and `;` are the usual ones, but any token that is not a delimiter or an operator works; the `$(,)?` trailing-comma idiom, as in std's `vec!` rule `($($x:expr),+ $(,)?)`
- Changing the separator on the way out: matching `$( $i:ident ),*` and writing `$( $i );*`
- The three depth errors, word for word on 1.98.0, and which side each one is really about:
    - *variable `i` is still repeating at this depth* — `$i` used outside the `$( )` it was matched in
    - *attempted to repeat an expression containing no syntax variables matched as repeating at this depth* — a `$( )` in the transcriber with nothing inside to count
    - *meta-variable `i` repeats 3 times, but `j` repeats 2 times* — two lists zipped in one repetition, refused at the call that passed uneven lists
- Nested repetition: `$( $name:ident { $( $field:ident ),* } );*`, and a transcriber that mirrors both levels
- Counting items on stable: `${count($x)}` is unstable (`E0658`, issue #83527), so what do the recursive muncher and the slice-length trick each cost?
- `vec!` read as the model: its three rules — empty, `$elem; $n`, and the list — in the order std wrote them

## The trap it exists for

Writing `$x` where `$( $x ),*` was meant, reading *still repeating at this depth* as a problem with the matcher, and editing the wrong side. The message is about the transcriber: the variable is a list there, and a list cannot be pasted where one item goes.

## Where this sits

[Matchers and fragment specifiers](../macro_rules_patterns/README.md) covers what one fragment matches and how rules are chosen. This page covers lists of fragments. [`Vec`](../../26_Collections/the_vec/README.md) is where the best-known repeating macro is used.

## See also

- [Matchers and fragment specifiers](../macro_rules_patterns/README.md) — one fragment at a time
- [Expanding a macro](../expanding_a_macro/README.md) — seeing what each repetition wrote
- [`Vec`](../../26_Collections/the_vec/README.md) — `vec![…]` from the caller's side
- [A function, `macro_rules!`, or a procedural macro](../../37_Procedural_Macros/function_macro_rules_or_proc_macro/README.md) — the variadic call as a reason to write one
- [Macros by example: repetitions ↗](https://doc.rust-lang.org/reference/macros-by-example.html#r-macro.decl.repetition) — the Reference's rules for depth and counts

## If you are coming from another language

- **C.** `__VA_ARGS__` passes the remaining arguments along as one comma-separated run, and the preprocessor has no loop to walk them. `$( … )*` is that loop: each item stays a separate fragment you can wrap, reorder or separate differently.
- **Python.** `*args` collects the extra arguments into a tuple at run time. `$( $x:expr ),*` collects them at compile time, and each item stays its own expression with its own type — which is why `vec![1, 2, "three"]` is a type error at the call rather than a list of mixed values.

## Po polsku

Powtórzenie (*repetition*) w makrze zapisuje się jako `$( … )` z operatorem `*`, `+` albo `?` i opcjonalnym separatorem: `$( $x:expr ),*` to „dowolnie wiele wyrażeń oddzielonych przecinkami”. Zasada, na której wszyscy się potykają: zmienna dopasowana wewnątrz powtórzenia musi po prawej stronie stać wewnątrz tylu samo powtórzeń — inaczej kompilator zgłasza *still repeating at this depth*, a błąd leży w części transkrybującej, nie we wzorcu.

**Szukaj po polsku:** powtórzenia w makrach · zmienna liczba argumentów makra · `rust macro repetition` · `still repeating at this depth` · `rust macro trailing comma`
