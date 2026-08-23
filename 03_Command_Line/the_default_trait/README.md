# The `Default` trait

**Level:** 101 → 201 · for newcomers

**One line:** [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) is the value a type takes when nobody said — and `..Default::default()` is how a struct of options grows a tenth field without you editing every place that built one.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `#[derive(Default)]` and what it produces field by field: every field's own default, which for numbers is zero and for `Option` is `None`
- Struct update syntax — `Config { words: true, ..Default::default() }` — and the fact that it is not inheritance, just the remaining fields filled in
- `#[default]` on an enum variant, so a derived default can pick which variant is the quiet one
- Writing the impl by hand when the type's zero is not your domain's zero
- Where it shows up without being asked: `unwrap_or_default`, `Vec::new`, and every `T: Default` bound in the standard library

## The trap it exists for

A derived `Default` is the **type's** zero, not your domain's — a `Timeout(0)` or a `Retries(0)` is a perfectly good `u32` and a terrible policy. [`unwrap_or_default`](../../01_Foundations/unwrap_or_default/README.md) already tells that story from the calling side; this page is the same fact from the *defining* side, where you can still do something about it.

## See also

- [`unwrap_or_default`](../../01_Foundations/unwrap_or_default/README.md) — the fallback the type chose, and the missing impl that is a guard rail
- [Optional function arguments](../../01_Foundations/optional_arguments/README.md) — the options-struct pattern this makes bearable
- [Flags by hand](../flags_by_hand/README.md) — the struct that wanted a default in the first place
