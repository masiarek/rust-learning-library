# A `StateMachine` derive

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** The chapter in one macro: `#[derive(StateMachine)]` on an enum, with `#[state(initial)]` and `#[transition(to = …)]` helper attributes, generating the transition methods — and rejecting a transition to a state that does not exist at compile time, with the error on the offending token.

## What it has to cover

- Code generation over an enum's variants
- Helper attributes for the initial state and allowed transitions, parsed with `darling`
- Validation: a missing or duplicate initial state, a transition to an unknown variant — each a spanned error, recorded and under `trybuild`
- Generated identifiers (method names, a companion type) built with `format_ident!` so they cannot clash with the user's
- The expansion of a small machine, recorded

## See also

- [Helper attributes with `darling`](../helper_attributes_with_darling/README.md) — the attribute parsing
- [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) — the spanned errors
- [Absolute paths and hygiene](../absolute_paths_and_hygiene/README.md) — identifiers that never clash
- [Procedural macros](../README.md) — the chapter, in reading order
