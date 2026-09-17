# A type alias is not a new type

**Level:** 201 · working knowledge

**One line:** `type Kilometers = u32;` gives `u32` a second name and nothing else — a `Kilometers` and a `Miles` add together without a word from the compiler — so an alias shortens a long type, and a [newtype](../newtype_score/README.md) is what makes two types different.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The same program with `type Km = u32; type Mi = u32;` and with `struct Km(u32); struct Mi(u32);` — the first adds them, the second refuses with `E0308`
- What aliases are for: shortening `Box<dyn Fn(u32) -> Result<(), MyError> + Send>` and pinning a parameter, as `io::Result<T>` pins `E` (see [the `Result` alias](../../17_Option_and_Result/result_aliases/README.md))
- Generic aliases: `type Grid<T> = Vec<Vec<T>>;`
- Bounds in an alias are not enforced — the `type_alias_bounds` lint, and what it says
- An alias cannot carry an `impl` of its own: `impl Km { … }` is an inherent impl on `u32`, and rustc 1.98.0 refuses it with `E0390` *cannot define inherent `impl` for primitive types*
- Aliases in an API: rustdoc shows the alias, the compiler error shows the expanded type, and the reader has to connect them

## The trap it exists for

Reaching for `type UserId = u64;` to stop IDs being mixed up. It documents the intent and enforces none of it — `fn delete(user: UserId)` accepts an order number without complaint. That job needs a tuple struct.

## Where this sits

[A score is not a number: the newtype](../newtype_score/README.md) is the version that creates a distinct type. [The `Result` you are reading is probably an alias](../../17_Option_and_Result/result_aliases/README.md) is the alias you meet most. [Units are types](../../07_Clients/units_are_types/README.md) is the newtype applied to measurements.

## See also

- [A score is not a number: the newtype](../../16_Structs/newtype_score/README.md) — the distinct type an alias is not
- [The `Result` you are reading is probably an alias](../../17_Option_and_Result/result_aliases/README.md) — `io::Result<T>` and friends
- [Units are types](../../07_Clients/units_are_types/README.md) — kilometres and miles as real types
- [What a struct is](../../16_Structs/what_a_struct_is/README.md) — the tuple struct a newtype is made of
- [Wrapper types](../../39_API_Design/wrapper_types/README.md) — when a newtype should implement `Deref`
- [TYPES.md](../../TYPES.md) — where aliases sit among the ways to name a type

## If you are coming from another language

- **C.** `typedef unsigned int km;` is exactly a Rust alias — same type, new spelling, no checking.
- **Go.** Go has both, with a one-character difference: `type Km = uint32` is an alias, `type Km uint32` is a new defined type that will not mix with `Mi`. Rust spells the second as a tuple struct.
- **Python.** `Km: TypeAlias = int` is an alias for a checker; `NewType("Km", int)` is the newtype, checked only by the type checker and gone at run time.
- **C++.** `using Km = unsigned;` is an alias; a distinct type needs a wrapper struct, as in Rust.

## Po polsku

Alias typu (`type Km = u32;`) to tylko druga nazwa tego samego typu — kompilator bez słowa doda kilometry do mil. Do skracania długich typów alias jest świetny; do rozróżniania wartości potrzebny jest newtype, czyli struktura krotkowa `struct Km(u32);`.

**Szukaj po polsku:** alias typu · newtype w Ruście · `rust type alias vs newtype` · `rust type alias bounds not enforced`
