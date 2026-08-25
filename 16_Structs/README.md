# Structs

**One line:** A struct names the fields a value has; a separate `impl` block gives that value its behaviour — and the gap between those two declarations is the whole of Rust's answer to the class.

There is no constructor. There is no `new` keyword, no `Type()` call, and nothing runs implicitly when a value is made: you either write the literal out field by field, or you write an ordinary associated function that returns one. What replaces the constructor's guarantees is privacy — a private field means the only way to build the value is a function you wrote, which is the entire mechanism behind [the newtype](newtype_score/README.md).

The other half of the section is what the compiler does *with* your struct: which fields survive a `..base`, why a struct is never `Copy` by accident, and the eight errors a struct produces — where `E0277` is never the diagnosis, since one code covers four unrelated mistakes.

| Lesson | Level | What it teaches |
|---|---|---|
| [What a struct is](what_a_struct_is/README.md) | 101 → 201 | The three flavors, why behaviour lives in a separate `impl` block rather than in the struct, associated function vs method, and the privacy that is per *module* — including the private field that makes a tuple struct's constructor private |
| [A type is not a constructor](a_type_is_not_a_constructor/README.md) | 101 → 201 | There is no `Type()` call that makes a value — a literal, a tuple struct's real constructor function, or an associated function you wrote; plus why `let b: Ballot();` prints **two** errors and `let p = Precinct;` prints none |
| [`impl` blocks](impl_blocks/README.md) | 101 → 201 | Where the functions went — `self` is the only thing separating an associated function from a method, `ballot.total()` is sugar for `Ballot::total(&ballot)`, and the three receivers decide what the *caller* keeps |
| [Struct update syntax](struct_update/README.md) | 101 → 201 | `..base` fills the rest by **moving**, field by field — so the base is *partially* dead, and `Copy` decides which half survives; the compiler names `user1.username`, not `user1` |
| [`Copy` vs `Clone`](copy_vs_clone/README.md) | 101 → 201 | `Clone` is a method you call; `Copy` changes what `=` *means* — and a struct is never `Copy` by accident: every field must be, and you must opt in. Three refusals, three error codes |
| [When a struct refuses](when_a_struct_refuses/README.md) | 101 → 201 | Eight struct errors and the fix each is asking for — and why `E0277` is never the diagnosis, since one code covers a missing `Display`, a missing `Debug`, an unsized field and an `Eq` without its `PartialEq` |
| [A score is not a number](newtype_score/README.md) | 101 → 201 | The newtype: one private field, one validating door, and why privacy is per *module* |
| [What is a ballot, in memory?](representing_a_ballot/README.md) | 201 | Array vs `Vec` vs tuple vs struct vs map vs flat matrix — and which bugs each one makes writeable |

## Struct pages that live elsewhere

- [`String` vs `&str`](../14_Strings/string_vs_str/README.md) — the choice every struct with a text field has to make
- [Ownership and moves](../18_Ownership/ownership_and_moves/README.md) — what `copy_vs_clone` here is a special case of
- [What a trait is](../12_Traits/what_a_trait_is/README.md) — behaviour shared across types, once `impl` blocks are solid
- [What an enum is](../13_Enums/what_an_enum_is/README.md) — the other way to declare a type of your own

[STRUCTS.md](../STRUCTS.md) is the full reading order, including the pages in other sections.
