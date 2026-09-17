# Parse, tweak, re-emit

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** An attribute macro on a function parses it into an `ItemFn`, changes what it needs to — the body, the signature, an attribute — and returns the whole function, because whatever it returns replaces what was there.

## What it has to cover

- `ItemFn`: `attrs`, `vis`, `sig`, `block`
- A worked example that wraps the body (for example timing or logging the call), keeping the signature, generics and `async` intact
- Arguments to the attribute: the first `TokenStream`, and rejecting arguments the macro does not take
- The expansion, recorded

## See also

- [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) — an attribute returns the item's replacement
- [Re-emit the item on error](../re_emit_on_error/README.md) — what to return when this fails
- [Procedural macros](../README.md) — the chapter, in reading order
