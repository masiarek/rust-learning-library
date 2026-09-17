# A `routes!` macro

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `routes! { GET "/users/{id}" => get_user, POST "/users" => create_user }` parsed into a table, with each path checked while compiling: a `{param}` that the handler does not take is a compile error pointing at the path.

## What it has to cover

- The DSL's grammar and its `Parse` implementation
- Generated code: a router table (or `match`) the program can call, with no runtime crate needed
- Compile-time validation of paths and duplicate routes, with spanned errors, recorded
- A `trybuild` test for the error cases

## See also

- [Parsing arbitrary tokens](../parsing_arbitrary_tokens/README.md) — the parsing machinery
- [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) — spanned errors
- [Procedural macros](../README.md) — the chapter, in reading order
