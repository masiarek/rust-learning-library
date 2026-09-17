# Helper attributes with `darling`

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `darling` turns attribute parsing into a struct definition: derive `FromDeriveInput` and `FromField`, name the fields you accept, and it writes the parsing, the defaults and the error messages for unknown or missing keys.

## What it has to cover

- The same attributes as the by-hand page, rewritten with `#[derive(FromDeriveInput)]` and `#[darling(attributes(...))]`
- `Option`, `#[darling(default)]`, and `darling::util::Flag`
- The errors darling writes for an unknown field, recorded, and `darling::Error::write_errors`
- Line count against the by-hand version

## See also

- [Helper attributes by hand](../helper_attributes_by_hand/README.md) — what darling replaces
- [A `#[retry]` attribute](../a_retry_attribute/README.md) — darling's `FromMeta` for an attribute macro's arguments
- [Procedural macros](../README.md) — the chapter, in reading order
