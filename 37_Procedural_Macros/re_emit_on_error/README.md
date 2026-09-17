# Re-emit the item on error

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** When an attribute macro fails and returns only `compile_error!`, the function disappears and every call to it becomes a second error; return the error **and** the original item, and the user sees one message.

## What it has to cover

- The cascade, recorded: one bad attribute argument, and the E0425 errors at every call site
- The fix: `quote! { #error #item }` using the untouched input
- The same run after the fix, recorded
- When re-emitting is wrong (the item itself failed to parse)

## See also

- [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) — spanned errors in a derive
- [Parse, tweak, re-emit](../parse_tweak_reemit/README.md) — the happy path
- [Procedural macros](../README.md) — the chapter, in reading order
