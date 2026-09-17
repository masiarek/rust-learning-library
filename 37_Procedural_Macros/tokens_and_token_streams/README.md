# Tokens and token streams

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A `TokenStream` is a sequence of four kinds of `TokenTree` — `Ident`, `Punct`, `Literal`, and `Group`, a delimited stream inside — and every macro reads and writes nothing else.

## What it has to cover

- The four kinds, printed from a real input: kind, text, and for a `Punct` its `Spacing` (`Joint` vs `Alone`, and why `=>` is two puncts)
- `Group` and `Delimiter`: parentheses, brackets, braces, and `None`
- Doc comments: a probe on 1.98.0 showed a doc comment arrives as `#` plus `[doc = " hi"]`, and that `to_string` of the whole stream prints it back as `/// hi` for a derive or attribute input but as `#[doc = r" hi"]` inside a function-like macro — verify and record
- Spacing in `to_string`: a probe showed runs of whitespace collapse to one space and no space stays none (`x:i32` stays `x:i32`) — verify and record, and cite the docs' warning that the format may change
- Spans, briefly: every token carries a source location, which is how errors point at your code
- Why a macro never sees types, only tokens

## See also

- [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) — a macro of each kind printing its input
- [Parsing with `syn`](../parsing_with_syn/README.md) — what to use instead of walking tokens by hand
- [Procedural macros](../README.md) — the chapter, in reading order
