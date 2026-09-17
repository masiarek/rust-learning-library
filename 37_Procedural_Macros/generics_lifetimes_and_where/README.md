# Generics, lifetimes and `where`

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A derive on `struct Wrapper<'a, T: Clone> where T: Debug` has to repeat every parameter, bound and `where` clause in its `impl` — `Generics::split_for_impl` hands back the three pieces, and adding a bound of your own goes through `make_where_clause`.

## What it has to cover

- The break, recorded: a derive that writes `impl MyTrait for #name` on a generic type
- `split_for_impl`: `impl_generics`, `ty_generics`, `where_clause`, and where each goes
- Lifetimes and const generics coming along for free
- Adding `T: MyTrait` bounds per type parameter, and the known trap of bounding parameters instead of field types

## See also

- [Every struct and enum shape](../every_struct_and_enum_shape/README.md) — the other axis a derive has to cover
- [Parsing with `syn`](../parsing_with_syn/README.md) — the `Generics` type
- [Procedural macros](../README.md) — the chapter, in reading order
